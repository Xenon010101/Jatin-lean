//! Structural package analysis: detects frameworks, patterns, and package types.
//!
//! Provides intelligent classification of packages to enable framework-aware
//! pruning rules and better optimization decisions.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Detected framework or tool type.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Framework {
    React,
    Vue,
    Angular,
    Svelte,
    Next,
    Nuxt,
    Express,
    Fastify,
    Nest,
    Electron,
    ReactNative,
    Jest,
    Mocha,
    Webpack,
    Vite,
    Rollup,
    ESBuild,
    Babel,
    TypeScript,
    Prettier,
    ESLint,
    Tailwind,
    Unknown,
}

impl Framework {
    pub fn label(&self) -> &'static str {
        match self {
            Framework::React => "React",
            Framework::Vue => "Vue.js",
            Framework::Angular => "Angular",
            Framework::Svelte => "Svelte",
            Framework::Next => "Next.js",
            Framework::Nuxt => "Nuxt",
            Framework::Express => "Express",
            Framework::Fastify => "Fastify",
            Framework::Nest => "NestJS",
            Framework::Electron => "Electron",
            Framework::ReactNative => "React Native",
            Framework::Jest => "Jest",
            Framework::Mocha => "Mocha",
            Framework::Webpack => "Webpack",
            Framework::Vite => "Vite",
            Framework::Rollup => "Rollup",
            Framework::ESBuild => "esbuild",
            Framework::Babel => "Babel",
            Framework::TypeScript => "TypeScript",
            Framework::Prettier => "Prettier",
            Framework::ESLint => "ESLint",
            Framework::Tailwind => "Tailwind CSS",
            Framework::Unknown => "Unknown",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Framework::React | Framework::ReactNative => "⚛️",
            Framework::Vue | Framework::Nuxt => "🟩",
            Framework::Angular => "🅰️",
            Framework::Svelte => "🔶",
            Framework::Next => "▲",
            Framework::Express | Framework::Fastify | Framework::Nest => "🖥️",
            Framework::Electron => "⚡",
            Framework::Jest | Framework::Mocha => "🧪",
            Framework::Webpack | Framework::Vite | Framework::Rollup | Framework::ESBuild => "📦",
            Framework::Babel | Framework::TypeScript => "🔧",
            Framework::Prettier | Framework::ESLint => "✨",
            Framework::Tailwind => "🎨",
            Framework::Unknown => "❓",
        }
    }

    /// Category for grouping (UI, Server, Build, Test, Lint)
    pub fn category(&self) -> &'static str {
        match self {
            Framework::React
            | Framework::Vue
            | Framework::Angular
            | Framework::Svelte
            | Framework::Next
            | Framework::Nuxt
            | Framework::Tailwind => "UI Framework",
            Framework::Express | Framework::Fastify | Framework::Nest => "Server Framework",
            Framework::Electron | Framework::ReactNative => "Platform",
            Framework::Jest | Framework::Mocha => "Testing",
            Framework::Webpack
            | Framework::Vite
            | Framework::Rollup
            | Framework::ESBuild
            | Framework::Babel
            | Framework::TypeScript => "Build Tool",
            Framework::Prettier | Framework::ESLint => "Linting",
            Framework::Unknown => "Other",
        }
    }
}

/// Package type classification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageType {
    /// Library (provides functions/classes for import)
    Library,
    /// CLI tool (provides executable commands)
    CliTool,
    /// Framework (provides structural patterns)
    FrameworkCore,
    /// Plugin/addon for a framework
    Plugin,
    /// TypeScript type definitions (@types/*)
    TypeDefinition,
    /// Build/dev tooling
    DevTool,
    /// Polyfill/shim
    Polyfill,
    /// Utility (small helper)
    Utility,
    /// Unknown
    Unknown,
}

impl PackageType {
    pub fn label(&self) -> &'static str {
        match self {
            PackageType::Library => "Library",
            PackageType::CliTool => "CLI Tool",
            PackageType::FrameworkCore => "Framework",
            PackageType::Plugin => "Plugin",
            PackageType::TypeDefinition => "Type Definition",
            PackageType::DevTool => "Dev Tool",
            PackageType::Polyfill => "Polyfill",
            PackageType::Utility => "Utility",
            PackageType::Unknown => "Unknown",
        }
    }
}

/// Full analysis result for a package.
#[derive(Debug, Clone)]
pub struct PackageAnalysis {
    pub name: String,
    pub version: String,
    pub frameworks: Vec<Framework>,
    pub package_type: PackageType,
    pub has_bin: bool,
    pub has_native: bool,
    pub has_types: bool,
    pub entry_points: Vec<String>,
    pub dependency_count: usize,
    pub dev_dependency_count: usize,
    pub file_count: u64,
    pub total_size: u64,
    /// Extra prunable patterns specific to this framework
    pub extra_prunable_patterns: Vec<String>,
}

/// Analyze the project's node_modules to detect frameworks and patterns.
pub fn analyze_project(nm_path: &Path) -> anyhow::Result<ProjectAnalysis> {
    let mut analyses = Vec::new();
    let mut framework_counts: HashMap<Framework, usize> = HashMap::new();
    let mut type_counts: HashMap<String, usize> = HashMap::new();

    if let Ok(entries) = fs::read_dir(nm_path) {
        for entry in entries.flatten() {
            let pkg_path = entry.path();
            if !pkg_path.is_dir() {
                continue;
            }

            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with('.') {
                continue;
            }

            if name.starts_with('@') {
                // Scoped packages
                if let Ok(scoped) = fs::read_dir(&pkg_path) {
                    for se in scoped.flatten() {
                        if se.path().is_dir() {
                            let full_name =
                                format!("{}/{}", name, se.file_name().to_string_lossy());
                            if let Some(analysis) = analyze_package(&se.path(), &full_name) {
                                for fw in &analysis.frameworks {
                                    *framework_counts.entry(fw.clone()).or_default() += 1;
                                }
                                *type_counts
                                    .entry(analysis.package_type.label().to_string())
                                    .or_default() += 1;
                                analyses.push(analysis);
                            }
                        }
                    }
                }
            } else {
                if let Some(analysis) = analyze_package(&pkg_path, &name) {
                    for fw in &analysis.frameworks {
                        *framework_counts.entry(fw.clone()).or_default() += 1;
                    }
                    *type_counts
                        .entry(analysis.package_type.label().to_string())
                        .or_default() += 1;
                    analyses.push(analysis);
                }
            }
        }
    }

    Ok(ProjectAnalysis {
        total_packages: analyses.len(),
        package_analyses: analyses,
        framework_distribution: framework_counts,
        type_distribution: type_counts,
    })
}

/// Analyze a single package directory.
pub fn analyze_package(pkg_path: &Path, name: &str) -> Option<PackageAnalysis> {
    let pkg_json_path = pkg_path.join("package.json");
    if !pkg_json_path.exists() {
        return None;
    }

    let content = fs::read_to_string(&pkg_json_path).ok()?;
    let json: serde_json::Value = serde_json::from_str(&content).ok()?;

    let version = json
        .get("version")
        .and_then(|v| v.as_str())
        .unwrap_or("0.0.0")
        .to_string();

    let has_bin = json.get("bin").is_some();
    let has_types = json.get("types").is_some() || json.get("typings").is_some();
    let has_native = pkg_path.join("binding.gyp").exists()
        || json
            .get("gypfile")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

    let dep_count = json
        .get("dependencies")
        .and_then(|v| v.as_object())
        .map(|o| o.len())
        .unwrap_or(0);

    let dev_dep_count = json
        .get("devDependencies")
        .and_then(|v| v.as_object())
        .map(|o| o.len())
        .unwrap_or(0);

    // Collect entry points
    let mut entry_points = Vec::new();
    if let Some(main) = json.get("main").and_then(|v| v.as_str()) {
        entry_points.push(main.to_string());
    }
    if let Some(module) = json.get("module").and_then(|v| v.as_str()) {
        entry_points.push(module.to_string());
    }

    // Detect frameworks
    let frameworks = detect_frameworks(name, &json);
    let package_type = classify_package_type(name, &json, has_bin, has_native);
    let extra_patterns = get_extra_prunable_patterns(&frameworks);

    Some(PackageAnalysis {
        name: name.to_string(),
        version,
        frameworks,
        package_type,
        has_bin,
        has_native,
        has_types,
        entry_points,
        dependency_count: dep_count,
        dev_dependency_count: dev_dep_count,
        file_count: 0, // Filled later if needed
        total_size: 0,
        extra_prunable_patterns: extra_patterns,
    })
}

/// Detect frameworks from package name and package.json.
fn detect_frameworks(name: &str, json: &serde_json::Value) -> Vec<Framework> {
    let mut detected = Vec::new();
    let lower = name.to_lowercase();

    // Direct name matching
    let name_map: &[(&str, Framework)] = &[
        ("react", Framework::React),
        ("react-dom", Framework::React),
        ("react-native", Framework::ReactNative),
        ("vue", Framework::Vue),
        ("@vue/", Framework::Vue),
        ("@angular/", Framework::Angular),
        ("svelte", Framework::Svelte),
        ("next", Framework::Next),
        ("nuxt", Framework::Nuxt),
        ("express", Framework::Express),
        ("fastify", Framework::Fastify),
        ("@nestjs/", Framework::Nest),
        ("electron", Framework::Electron),
        ("jest", Framework::Jest),
        ("mocha", Framework::Mocha),
        ("webpack", Framework::Webpack),
        ("vite", Framework::Vite),
        ("rollup", Framework::Rollup),
        ("esbuild", Framework::ESBuild),
        ("@babel/", Framework::Babel),
        ("babel-", Framework::Babel),
        ("typescript", Framework::TypeScript),
        ("prettier", Framework::Prettier),
        ("eslint", Framework::ESLint),
        ("tailwindcss", Framework::Tailwind),
    ];

    for (pattern, fw) in name_map {
        if lower.contains(pattern) && !detected.contains(fw) {
            detected.push(fw.clone());
        }
    }

    // Check keywords in package.json
    if let Some(keywords) = json.get("keywords").and_then(|v| v.as_array()) {
        for kw in keywords {
            if let Some(kw_str) = kw.as_str() {
                let kw_lower = kw_str.to_lowercase();
                if kw_lower.contains("react") && !detected.contains(&Framework::React) {
                    detected.push(Framework::React);
                }
                if kw_lower.contains("vue") && !detected.contains(&Framework::Vue) {
                    detected.push(Framework::Vue);
                }
                if kw_lower.contains("angular") && !detected.contains(&Framework::Angular) {
                    detected.push(Framework::Angular);
                }
            }
        }
    }

    detected
}

/// Classify the package type.
fn classify_package_type(
    name: &str,
    json: &serde_json::Value,
    has_bin: bool,
    has_native: bool,
) -> PackageType {
    if name.starts_with("@types/") {
        return PackageType::TypeDefinition;
    }

    if has_bin && !json.get("main").is_some() {
        return PackageType::CliTool;
    }

    let lower = name.to_lowercase();

    // Framework detection
    let framework_names = [
        "react", "vue", "angular", "svelte", "express", "fastify", "next", "nuxt", "electron",
    ];
    for fw in &framework_names {
        if lower == *fw {
            return PackageType::FrameworkCore;
        }
    }

    // Plugin detection
    if lower.contains("-plugin")
        || lower.contains("-loader")
        || lower.contains("-preset")
        || lower.contains("-adapter")
    {
        return PackageType::Plugin;
    }

    // Polyfill detection
    if lower.contains("polyfill") || lower.contains("shim") || lower.contains("ponyfill") {
        return PackageType::Polyfill;
    }

    // Dev tool detection
    let dev_tools = [
        "eslint", "prettier", "webpack", "babel", "jest", "mocha", "rollup", "vite",
    ];
    for tool in &dev_tools {
        if lower.contains(tool) {
            return PackageType::DevTool;
        }
    }

    if has_native {
        return PackageType::Library;
    }

    // Small utility heuristic
    let dep_count = json
        .get("dependencies")
        .and_then(|v| v.as_object())
        .map(|o| o.len())
        .unwrap_or(0);

    if dep_count == 0 && !has_bin {
        return PackageType::Utility;
    }

    PackageType::Library
}

/// Get extra prunable patterns for detected frameworks.
fn get_extra_prunable_patterns(frameworks: &[Framework]) -> Vec<String> {
    let mut patterns = Vec::new();

    for fw in frameworks {
        match fw {
            Framework::React => {
                patterns.extend_from_slice(&[
                    "cjs/".to_string(),
                    "umd/".to_string(),
                    "__fixtures__/".to_string(),
                ]);
            }
            Framework::TypeScript => {
                patterns.extend_from_slice(&[
                    "lib/tsc.js".to_string(),
                    "lib/typescriptServices.js".to_string(),
                ]);
            }
            Framework::Babel => {
                patterns.extend_from_slice(&["src/".to_string()]);
            }
            Framework::Jest => {
                patterns.extend_from_slice(&["build/".to_string()]);
            }
            _ => {}
        }
    }

    patterns
}

/// Full project analysis result.
#[derive(Debug)]
pub struct ProjectAnalysis {
    pub total_packages: usize,
    pub package_analyses: Vec<PackageAnalysis>,
    pub framework_distribution: HashMap<Framework, usize>,
    pub type_distribution: HashMap<String, usize>,
}

/// Print analysis results.
pub fn print_analysis(analysis: &ProjectAnalysis) {
    use console::style;

    println!();
    println!(
        "  {} {}",
        style("Package Analysis").cyan().bold(),
        style("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━").dim()
    );
    println!(
        "  {} Total packages analyzed: {}",
        style("◉").cyan(),
        style(analysis.total_packages).white().bold(),
    );

    // Framework distribution
    if !analysis.framework_distribution.is_empty() {
        println!();
        println!(
            "  {} {}",
            style("Detected Frameworks").white().bold(),
            style("─────────────────────────────").dim()
        );

        let mut sorted: Vec<_> = analysis.framework_distribution.iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(a.1));

        for (fw, count) in sorted.iter().take(15) {
            println!(
                "  {} {} {} ({})",
                fw.icon(),
                style(fw.label()).yellow(),
                style("×").dim(),
                style(count).white().bold(),
            );
        }
    }

    // Type distribution
    if !analysis.type_distribution.is_empty() {
        println!();
        println!(
            "  {} {}",
            style("Package Types").white().bold(),
            style("─────────────────────────────").dim()
        );

        let mut sorted: Vec<_> = analysis.type_distribution.iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(a.1));

        for (ptype, count) in &sorted {
            println!(
                "  {} {}: {}",
                style("▸").dim(),
                style(ptype).cyan(),
                style(count).white().bold(),
            );
        }
    }

    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_framework_detection_react() {
        let json: serde_json::Value = serde_json::json!({
            "name": "react",
            "version": "18.2.0"
        });
        let fws = detect_frameworks("react", &json);
        assert!(fws.contains(&Framework::React));
    }

    #[test]
    fn test_framework_detection_vue() {
        let json: serde_json::Value = serde_json::json!({
            "name": "vue",
            "version": "3.0.0"
        });
        let fws = detect_frameworks("vue", &json);
        assert!(fws.contains(&Framework::Vue));
    }

    #[test]
    fn test_package_type_types() {
        let json = serde_json::json!({ "name": "@types/node" });
        let pt = classify_package_type("@types/node", &json, false, false);
        assert_eq!(pt, PackageType::TypeDefinition);
    }

    #[test]
    fn test_package_type_utility() {
        let json = serde_json::json!({ "name": "inherits", "main": "inherits.js" });
        let pt = classify_package_type("inherits", &json, false, false);
        assert_eq!(pt, PackageType::Utility);
    }

    #[test]
    fn test_package_type_framework() {
        let json = serde_json::json!({ "name": "react", "main": "index.js" });
        let pt = classify_package_type("react", &json, false, false);
        assert_eq!(pt, PackageType::FrameworkCore);
    }

    #[test]
    fn test_package_type_plugin() {
        let json = serde_json::json!({ "name": "babel-plugin-transform", "main": "lib/index.js" });
        let pt = classify_package_type("babel-plugin-transform", &json, false, false);
        assert_eq!(pt, PackageType::Plugin);
    }

    #[test]
    fn test_framework_labels() {
        assert_eq!(Framework::React.label(), "React");
        assert_eq!(Framework::Vue.label(), "Vue.js");
        assert_eq!(Framework::Angular.label(), "Angular");
    }

    #[test]
    fn test_framework_categories() {
        assert_eq!(Framework::React.category(), "UI Framework");
        assert_eq!(Framework::Express.category(), "Server Framework");
        assert_eq!(Framework::Jest.category(), "Testing");
        assert_eq!(Framework::Webpack.category(), "Build Tool");
    }
}
