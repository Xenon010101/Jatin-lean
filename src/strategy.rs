//! Adaptive strategy engine for smart routing of packages.
//!
//! Determines the optimal scan strategy based on package characteristics
//! such as size, file count, known framework patterns, and cache state.

use std::path::Path;
use std::time::Duration;

/// Scan strategy to apply to a package.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanStrategy {
    /// Fast path: skip deep analysis for small/known-safe packages
    FastPath,
    /// Deep analysis: full scan with dependency tracing
    DeepAnalysis,
    /// Use cached results (package hasn't changed)
    Cached,
    /// Skip entirely (package is in the exclude list)
    Skip,
}

impl ScanStrategy {
    /// Human-readable label for the strategy.
    pub fn label(&self) -> &'static str {
        match self {
            ScanStrategy::FastPath => "Fast Path",
            ScanStrategy::DeepAnalysis => "Deep Analysis",
            ScanStrategy::Cached => "Cached",
            ScanStrategy::Skip => "Skip",
        }
    }
}

/// Profile of a package used for strategy selection.
#[derive(Debug, Clone)]
pub struct PackageProfile {
    /// Package name
    pub name: String,
    /// Estimated file count (-1 if unknown)
    pub file_count: i64,
    /// Estimated total size in bytes
    pub total_size: u64,
    /// Whether the package has a package.json
    pub has_package_json: bool,
    /// Whether the package has native bindings
    pub has_native_bindings: bool,
    /// Whether the package is scoped (@org/name)
    pub is_scoped: bool,
    /// Whether the package is cached
    pub is_cached: bool,
    /// Detected framework (if any)
    pub framework: Option<String>,
    /// Previous scan time (if available)
    pub previous_scan_time: Option<Duration>,
}

impl PackageProfile {
    /// Quick profile from directory metadata.
    pub fn from_path(pkg_path: &Path, pkg_name: &str) -> Self {
        let has_package_json = pkg_path.join("package.json").exists();
        let has_native = pkg_path.join("binding.gyp").exists()
            || pkg_path.join("build").join("Release").exists();
        let is_scoped = pkg_name.starts_with('@');

        // Quick file count estimate from top-level entries
        let file_count = std::fs::read_dir(pkg_path)
            .map(|entries| entries.count() as i64)
            .unwrap_or(-1);

        Self {
            name: pkg_name.to_string(),
            file_count,
            total_size: 0,
            has_package_json,
            has_native_bindings: has_native,
            is_scoped,
            is_cached: false,
            framework: None,
            previous_scan_time: None,
        }
    }
}

/// Heuristics configuration for strategy selection.
#[derive(Debug, Clone)]
pub struct StrategyHeuristics {
    /// Max file count for fast path
    pub fast_path_max_files: i64,
    /// Max size for fast path (bytes)
    pub fast_path_max_size: u64,
    /// Known small packages that can always use fast path
    pub fast_path_packages: Vec<String>,
    /// Packages to always skip
    pub skip_packages: Vec<String>,
    /// Packages that require deep analysis
    pub deep_analysis_packages: Vec<String>,
}

impl Default for StrategyHeuristics {
    fn default() -> Self {
        Self {
            fast_path_max_files: 20,
            fast_path_max_size: 100 * 1024, // 100KB
            fast_path_packages: vec![
                "inherits".to_string(),
                "ms".to_string(),
                "escape-html".to_string(),
                "path-to-regexp".to_string(),
                "merge-descriptors".to_string(),
                "utils-merge".to_string(),
                "isarray".to_string(),
                "safer-buffer".to_string(),
            ],
            skip_packages: vec![
                ".bin".to_string(),
                ".cache".to_string(),
                ".package-lock.json".to_string(),
            ],
            deep_analysis_packages: vec![
                "webpack".to_string(),
                "babel-core".to_string(),
                "@babel/core".to_string(),
                "typescript".to_string(),
                "next".to_string(),
                "react-scripts".to_string(),
                "@angular/core".to_string(),
                "vue".to_string(),
            ],
        }
    }
}

/// Strategy selector engine.
pub struct StrategyEngine {
    heuristics: StrategyHeuristics,
}

impl StrategyEngine {
    /// Create a new strategy engine with default heuristics.
    pub fn new() -> Self {
        Self {
            heuristics: StrategyHeuristics::default(),
        }
    }

    /// Create with custom heuristics.
    pub fn with_heuristics(heuristics: StrategyHeuristics) -> Self {
        Self { heuristics }
    }

    /// Select the optimal strategy for a package.
    pub fn select_strategy(&self, profile: &PackageProfile) -> ScanStrategy {
        // 1. Check skip list
        if self.heuristics.skip_packages.contains(&profile.name) {
            return ScanStrategy::Skip;
        }

        // 2. Check cache
        if profile.is_cached {
            return ScanStrategy::Cached;
        }

        // 3. Check deep analysis list
        if self
            .heuristics
            .deep_analysis_packages
            .contains(&profile.name)
        {
            return ScanStrategy::DeepAnalysis;
        }

        // 4. Check fast path list
        if self.heuristics.fast_path_packages.contains(&profile.name) {
            return ScanStrategy::FastPath;
        }

        // 5. Heuristic-based decision
        if profile.has_native_bindings {
            return ScanStrategy::DeepAnalysis;
        }

        if profile.file_count >= 0 && profile.file_count <= self.heuristics.fast_path_max_files {
            return ScanStrategy::FastPath;
        }

        if profile.total_size > 0 && profile.total_size <= self.heuristics.fast_path_max_size {
            return ScanStrategy::FastPath;
        }

        // Default to deep analysis
        ScanStrategy::DeepAnalysis
    }

    /// Batch-select strategies for multiple packages.
    pub fn select_batch(&self, profiles: &[PackageProfile]) -> Vec<(String, ScanStrategy)> {
        profiles
            .iter()
            .map(|p| (p.name.clone(), self.select_strategy(p)))
            .collect()
    }

    /// Get a summary of strategy distribution.
    pub fn strategy_summary(&self, strategies: &[(String, ScanStrategy)]) -> StrategySummary {
        let mut summary = StrategySummary::default();
        for (_, strategy) in strategies {
            match strategy {
                ScanStrategy::FastPath => summary.fast_path += 1,
                ScanStrategy::DeepAnalysis => summary.deep_analysis += 1,
                ScanStrategy::Cached => summary.cached += 1,
                ScanStrategy::Skip => summary.skipped += 1,
            }
        }
        summary.total = strategies.len();
        summary
    }
}

impl Default for StrategyEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Summary of strategy distribution across packages.
#[derive(Debug, Clone, Default)]
pub struct StrategySummary {
    pub total: usize,
    pub fast_path: usize,
    pub deep_analysis: usize,
    pub cached: usize,
    pub skipped: usize,
}

impl StrategySummary {
    /// Print summary to console.
    pub fn print(&self) {
        use console::style;
        println!(
            "  {} Strategy Distribution: {} fast | {} deep | {} cached | {} skipped",
            style("📊").dim(),
            style(self.fast_path).green(),
            style(self.deep_analysis).yellow(),
            style(self.cached).cyan(),
            style(self.skipped).dim(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skip_strategy() {
        let engine = StrategyEngine::new();
        let profile = PackageProfile {
            name: ".bin".to_string(),
            file_count: 0,
            total_size: 0,
            has_package_json: false,
            has_native_bindings: false,
            is_scoped: false,
            is_cached: false,
            framework: None,
            previous_scan_time: None,
        };
        assert_eq!(engine.select_strategy(&profile), ScanStrategy::Skip);
    }

    #[test]
    fn test_cached_strategy() {
        let engine = StrategyEngine::new();
        let profile = PackageProfile {
            name: "lodash".to_string(),
            file_count: 100,
            total_size: 1_000_000,
            has_package_json: true,
            has_native_bindings: false,
            is_scoped: false,
            is_cached: true,
            framework: None,
            previous_scan_time: None,
        };
        assert_eq!(engine.select_strategy(&profile), ScanStrategy::Cached);
    }

    #[test]
    fn test_fast_path_small_package() {
        let engine = StrategyEngine::new();
        let profile = PackageProfile {
            name: "tiny-pkg".to_string(),
            file_count: 5,
            total_size: 1024,
            has_package_json: true,
            has_native_bindings: false,
            is_scoped: false,
            is_cached: false,
            framework: None,
            previous_scan_time: None,
        };
        assert_eq!(engine.select_strategy(&profile), ScanStrategy::FastPath);
    }

    #[test]
    fn test_deep_analysis_large_package() {
        let engine = StrategyEngine::new();
        let profile = PackageProfile {
            name: "webpack".to_string(),
            file_count: 500,
            total_size: 5_000_000,
            has_package_json: true,
            has_native_bindings: false,
            is_scoped: false,
            is_cached: false,
            framework: None,
            previous_scan_time: None,
        };
        assert_eq!(engine.select_strategy(&profile), ScanStrategy::DeepAnalysis);
    }

    #[test]
    fn test_native_bindings_deep_analysis() {
        let engine = StrategyEngine::new();
        let profile = PackageProfile {
            name: "node-sass".to_string(),
            file_count: 10,
            total_size: 500,
            has_package_json: true,
            has_native_bindings: true,
            is_scoped: false,
            is_cached: false,
            framework: None,
            previous_scan_time: None,
        };
        assert_eq!(engine.select_strategy(&profile), ScanStrategy::DeepAnalysis);
    }

    #[test]
    fn test_strategy_summary() {
        let engine = StrategyEngine::new();
        let strategies = vec![
            ("a".into(), ScanStrategy::FastPath),
            ("b".into(), ScanStrategy::DeepAnalysis),
            ("c".into(), ScanStrategy::Cached),
            ("d".into(), ScanStrategy::Skip),
            ("e".into(), ScanStrategy::FastPath),
        ];
        let summary = engine.strategy_summary(&strategies);
        assert_eq!(summary.total, 5);
        assert_eq!(summary.fast_path, 2);
        assert_eq!(summary.deep_analysis, 1);
        assert_eq!(summary.cached, 1);
        assert_eq!(summary.skipped, 1);
    }
}
