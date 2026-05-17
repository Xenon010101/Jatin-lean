# Code Examples for jatin-lean v2.0 Transformation

This document provides copy-paste ready code examples for implementing the transformation.

---

## 1. New CLI Structure (src/main.rs)

### Replace Current CLI Structure

```rust
//! jatin-lean — Universal System Optimization Platform
//! 
//! Comprehensive optimization toolkit for Node.js projects, system performance,
//! network acceleration, and memory optimization.

mod allocator;
mod analytics;
mod benchmark;
mod cache;
mod compress;
mod config;
mod dedup;
mod deleter;
mod display;
mod health;
mod lockfile;
mod mmap;
mod network;
mod plugin;
mod policy;
mod profiler;
mod rules;
mod scanner;
mod simd;
mod snapshot;
mod syscall;
mod tracer;
mod treeshake;
mod visualizer;
mod watcher;
mod ringbuffer;
mod strategy;
mod distributed_cache;
mod analyzer;
mod xdp_middleware;
mod shared_memory_ipc;
mod zero_copy_serde;
mod request_coalescing;
mod adaptive_engine;
mod unified_gateway;
mod simd_json;
mod memory_pool;
mod maglev;
mod io_uring;
mod cpu_cache;
mod hardware_tuning;
mod bpf_verifier;
mod pcie_bottleneck;
mod hedging;
mod mmap_ipc;
mod static_plugins;

// New modules
mod cli;
mod output;
mod ai_context;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use console::style;
use std::path::PathBuf;

/// ⚡ jatin-lean — Universal System Optimization Platform
#[derive(Parser, Debug)]
#[command(
    name = "jatin-lean",
    version,
    about = "Universal system optimization platform",
    long_about = "Comprehensive optimization toolkit for Node.js projects, system performance,\nnetwork acceleration, and memory optimization.\n\nOriginally focused on node_modules optimization, jatin-lean has evolved into\na full-featured system optimization platform with advanced HPC capabilities."
)]
struct Cli {
    /// Output in JSON format (for AI/automation)
    #[arg(long, global = true)]
    json: bool,

    /// Pretty-print JSON output
    #[arg(long, global = true)]
    json_pretty: bool,

    /// Verbose output
    #[arg(long, short = 'v', global = true)]
    verbose: bool,

    /// Enable performance profiling
    #[arg(long, global = true)]
    profile: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Node.js ecosystem optimization (node_modules, packages, dependencies)
    Node {
        #[command(subcommand)]
        command: cli::NodeCommands,
    },

    /// System-level optimization (CPU, memory, kernel tuning)
    System {
        #[command(subcommand)]
        command: cli::SystemCommands,
    },

    /// Network & eBPF tools (XDP, BPF, gateway, DPI)
    Network {
        #[command(subcommand)]
        command: cli::NetworkCommands,
    },

    /// Memory & IPC optimization (shared memory, mmap, PCIe)
    Memory {
        #[command(subcommand)]
        command: cli::MemoryCommands,
    },

    /// Comprehensive benchmarking suite
    Bench {
        #[command(subcommand)]
        command: cli::BenchCommands,
    },

    /// Analysis and reporting tools
    Analyze {
        #[command(subcommand)]
        command: cli::AnalyzeCommands,
    },

    /// Generate AI-friendly context about the tool and current project
    AiContext {
        /// Path to project directory
        #[arg(default_value = ".")]
        path: PathBuf,
    },

    // Legacy commands (hidden, deprecated)
    #[command(flatten)]
    legacy: cli::LegacyCommands,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Set up output context
    let output_ctx = output::OutputContext {
        json: cli.json,
        pretty: cli.json_pretty,
        verbose: cli.verbose,
    };

    match cli.command {
        Some(Commands::Node { command }) => {
            cli::handle_node_command(command, &output_ctx)
        }
        Some(Commands::System { command }) => {
            cli::handle_system_command(command, &output_ctx)
        }
        Some(Commands::Network { command }) => {
            cli::handle_network_command(command, &output_ctx)
        }
        Some(Commands::Memory { command }) => {
            cli::handle_memory_command(command, &output_ctx)
        }
        Some(Commands::Bench { command }) => {
            cli::handle_bench_command(command, &output_ctx)
        }
        Some(Commands::Analyze { command }) => {
            cli::handle_analyze_command(command, &output_ctx)
        }
        Some(Commands::AiContext { path }) => {
            ai_context::handle_ai_context(path, &output_ctx)
        }
        Some(Commands::Legacy(legacy)) => {
            cli::handle_legacy_command(legacy, &output_ctx)
        }
        None => {
            // Default: show help
            if !output_ctx.json {
                display::print_banner();
                println!();
                println!("  {} No command specified. Use {} for help.", 
                    style("ℹ").blue(), 
                    style("--help").yellow());
                println!();
                println!("  {} Quick start:", style("→").dim());
                println!("    {} - Scan node_modules", 
                    style("jatin-lean node scan").green());
                println!("    {} - System assessment", 
                    style("jatin-lean system optimize --assess").green());
                println!("    {} - AI context", 
                    style("jatin-lean ai-context").green());
                println!();
            }
            Ok(())
        }
    }
}
```

---

## 2. CLI Module (src/cli/mod.rs)

```rust
//! CLI command definitions and handlers

pub mod node;
pub mod system;
pub mod network;
pub mod memory;
pub mod bench;
pub mod analyze;
pub mod legacy;

pub use node::NodeCommands;
pub use system::SystemCommands;
pub use network::NetworkCommands;
pub use memory::MemoryCommands;
pub use bench::BenchCommands;
pub use analyze::AnalyzeCommands;
pub use legacy::LegacyCommands;

use crate::output::OutputContext;
use anyhow::Result;

pub fn handle_node_command(command: NodeCommands, ctx: &OutputContext) -> Result<()> {
    node::handle_command(command, ctx)
}

pub fn handle_system_command(command: SystemCommands, ctx: &OutputContext) -> Result<()> {
    system::handle_command(command, ctx)
}

pub fn handle_network_command(command: NetworkCommands, ctx: &OutputContext) -> Result<()> {
    network::handle_command(command, ctx)
}

pub fn handle_memory_command(command: MemoryCommands, ctx: &OutputContext) -> Result<()> {
    memory::handle_command(command, ctx)
}

pub fn handle_bench_command(command: BenchCommands, ctx: &OutputContext) -> Result<()> {
    bench::handle_command(command, ctx)
}

pub fn handle_analyze_command(command: AnalyzeCommands, ctx: &OutputContext) -> Result<()> {
    analyze::handle_command(command, ctx)
}

pub fn handle_legacy_command(command: LegacyCommands, ctx: &OutputContext) -> Result<()> {
    legacy::handle_command(command, ctx)
}
```

---

## 3. Node Commands (src/cli/node.rs)

```rust
//! Node.js ecosystem optimization commands

use clap::Subcommand;
use std::path::PathBuf;
use anyhow::Result;
use crate::output::OutputContext;
use crate::{scanner, health, dedup, lockfile, compress, treeshake, network, analyzer};
use console::style;

#[derive(Subcommand, Debug)]
pub enum NodeCommands {
    /// Scan node_modules for optimization opportunities
    Scan {
        /// Path to project directory
        #[arg(default_value = ".")]
        path: PathBuf,
    },

    /// Prune non-essential files from node_modules
    Prune {
        /// Path to project directory
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Execute deletion (default is dry-run)
        #[arg(long, short = 'f')]
        force: bool,

        /// Skip confirmation prompt
        #[arg(long, short = 'y')]
        yes: bool,

        /// Create snapshot before deletion
        #[arg(long)]
        snapshot: bool,
    },

    /// Run comprehensive health check
    Health {
        #[arg(default_value = ".")]
        path: PathBuf,
    },

    /// Find duplicate files across packages
    Dedup {
        #[arg(default_value = ".")]
        path: PathBuf,
    },

    /// Analyze dependency graph from lock files
    Deps {
        #[arg(default_value = ".")]
        path: PathBuf,
    },

    /// Analyze compression potential (gzip/brotli)
    Compress {
        #[arg(default_value = ".")]
        path: PathBuf,
    },

    /// Analyze tree-shaking potential and dead exports
    Treeshake {
        #[arg(default_value = ".")]
        path: PathBuf,
    },

    /// Audit installed packages
    Audit {
        #[arg(default_value = ".")]
        path: PathBuf,
    },

    /// Analyze project structure and detect frameworks
    Analyze {
        #[arg(default_value = ".")]
        path: PathBuf,
    },

    /// Watch node_modules for changes and auto-prune
    Watch {
        #[arg(default_value = ".")]
        path: PathBuf,

        #[arg(long, default_value = "5")]
        interval: u64,

        #[arg(long)]
        auto_prune: bool,
    },

    /// Enforce dependency policies
    Policy {
        #[arg(long, value_name = "FILE")]
        file: Option<PathBuf>,

        #[arg(long, value_name = "FILE")]
        init: Option<PathBuf>,

        #[arg(default_value = ".")]
        path: PathBuf,
    },

    /// Render visual analysis of node_modules
    Visualize {
        #[arg(default_value = ".")]
        path: PathBuf,

        #[arg(long)]
        treemap: bool,

        #[arg(long)]
        sparklines: bool,
    },
}

pub fn handle_command(command: NodeCommands, ctx: &OutputContext) -> Result<()> {
    match command {
        NodeCommands::Health { path } => handle_health(path, ctx),
        NodeCommands::Dedup { path } => handle_dedup(path, ctx),
        NodeCommands::Deps { path } => handle_deps(path, ctx),
        NodeCommands::Compress { path } => handle_compress(path, ctx),
        NodeCommands::Treeshake { path } => handle_treeshake(path, ctx),
        NodeCommands::Audit { path } => handle_audit(path, ctx),
        NodeCommands::Analyze { path } => handle_analyze(path, ctx),
        // ... other commands
        _ => {
            println!("Command not yet implemented in new structure");
            Ok(())
        }
    }
}

fn handle_health(path: PathBuf, ctx: &OutputContext) -> Result<()> {
    let target = std::fs::canonicalize(&path)?;
    let nm_path = target.join("node_modules");
    
    if !nm_path.exists() {
        if ctx.json {
            crate::output::output_error("node health", "No node_modules found", ctx)?;
        } else {
            println!("  {} No node_modules found at {}",
                style("✗").red().bold(),
                style(target.display()).dim());
        }
        return Ok(());
    }

    let report = health::check_health(&nm_path)?;

    if ctx.json {
        use serde::Serialize;
        
        #[derive(Serialize)]
        struct HealthOutput {
            grade: String,
            score: u32,
            total_packages: usize,
            total_size_bytes: u64,
            issues: Vec<HealthIssue>,
            license_distribution: std::collections::HashMap<String, usize>,
        }

        #[derive(Serialize)]
        struct HealthIssue {
            severity: String,
            category: String,
            package: String,
            message: String,
            size_bytes: Option<u64>,
        }

        let output = HealthOutput {
            grade: report.grade.to_string(),
            score: report.score,
            total_packages: report.packages.len(),
            total_size_bytes: report.total_size,
            issues: report.issues.iter().map(|i| HealthIssue {
                severity: i.severity.to_string(),
                category: i.category.to_string(),
                package: i.package.clone(),
                message: i.message.clone(),
                size_bytes: i.size,
            }).collect(),
            license_distribution: report.license_dist.clone(),
        };

        crate::output::output_result("node health", output, ctx)?;
    } else {
        health::print_health_report(&report);
    }

    Ok(())
}

// Similar implementations for other commands...
```

---

## 4. Output Module (src/output.rs)

```rust
//! Output formatting and JSON serialization

use serde::Serialize;
use anyhow::Result;
use console::style;

pub struct OutputContext {
    pub json: bool,
    pub pretty: bool,
    pub verbose: bool,
}

#[derive(Serialize)]
struct JsonOutput<T> {
    command: String,
    timestamp: String,
    version: String,
    success: bool,
    result: T,
}

#[derive(Serialize)]
struct JsonError {
    command: String,
    timestamp: String,
    version: String,
    success: bool,
    error: String,
}

pub fn output_result<T: Serialize>(
    command: &str,
    result: T,
    ctx: &OutputContext,
) -> Result<()> {
    if ctx.json {
        let output = JsonOutput {
            command: command.to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            success: true,
            result,
        };

        let json_str = if ctx.pretty {
            serde_json::to_string_pretty(&output)?
        } else {
            serde_json::to_string(&output)?
        };

        println!("{}", json_str);
    }
    // If not JSON, the caller should have already printed human-readable output

    Ok(())
}

pub fn output_error(
    command: &str,
    error: &str,
    ctx: &OutputContext,
) -> Result<()> {
    if ctx.json {
        let output = JsonError {
            command: command.to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            success: false,
            error: error.to_string(),
        };

        let json_str = if ctx.pretty {
            serde_json::to_string_pretty(&output)?
        } else {
            serde_json::to_string(&output)?
        };

        eprintln!("{}", json_str);
    } else {
        eprintln!("  {} {}", style("✗").red().bold(), error);
    }

    Ok(())
}
```

---

## 5. AI Context Command (src/ai_context.rs)

```rust
//! AI-friendly context generation

use serde::Serialize;
use std::path::PathBuf;
use anyhow::Result;
use crate::output::OutputContext;
use crate::{scanner, rules, analyzer, simd};

#[derive(Serialize)]
pub struct AiContext {
    tool: String,
    version: String,
    capabilities: Vec<String>,
    quick_commands: QuickCommands,
    project_context: Option<ProjectContext>,
    system_context: SystemContext,
}

#[derive(Serialize)]
struct QuickCommands {
    scan_node_modules: String,
    prune_node_modules: String,
    system_assessment: String,
    network_benchmark: String,
    memory_benchmark: String,
}

#[derive(Serialize)]
struct ProjectContext {
    has_node_modules: bool,
    node_modules_size_bytes: u64,
    node_modules_packages: usize,
    potential_savings_bytes: u64,
    frameworks_detected: Vec<String>,
    package_manager: String,
}

#[derive(Serialize)]
struct SystemContext {
    os: String,
    arch: String,
    cpu_cores: usize,
    simd_tier: String,
}

pub fn handle_ai_context(path: PathBuf, ctx: &OutputContext) -> Result<()> {
    let target = std::fs::canonicalize(&path)?;
    let nm_path = target.join("node_modules");

    // Detect project context
    let project_ctx = if nm_path.exists() {
        let rules = rules::PruneRules::new();
        let scan_result = scanner::scan_node_modules(&nm_path, &rules, None)?;
        let analysis = analyzer::analyze_project(&nm_path)?;

        Some(ProjectContext {
            has_node_modules: true,
            node_modules_size_bytes: scan_result.total_size,
            node_modules_packages: scan_result.total_packages as usize,
            potential_savings_bytes: scan_result.savings(),
            frameworks_detected: analysis.frameworks.iter()
                .map(|f| f.label().to_string())
                .collect(),
            package_manager: detect_package_manager(&target),
        })
    } else {
        None
    };

    // Detect system context
    let caps = simd::CpuCapabilities::detect();
    let system_ctx = SystemContext {
        os: std::env::consts::OS.to_string(),
        arch: caps.arch.clone(),
        cpu_cores: num_cpus::get(),
        simd_tier: caps.tier_name().to_string(),
    };

    let ai_context = AiContext {
        tool: "jatin-lean".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        capabilities: vec![
            "node_modules_optimization".to_string(),
            "system_performance_tuning".to_string(),
            "network_ebpf_tools".to_string(),
            "memory_ipc_optimization".to_string(),
            "comprehensive_benchmarking".to_string(),
        ],
        quick_commands: QuickCommands {
            scan_node_modules: "jatin-lean node scan [path]".to_string(),
            prune_node_modules: "jatin-lean node prune [path] --force".to_string(),
            system_assessment: "jatin-lean system optimize --assess".to_string(),
            network_benchmark: "jatin-lean network xdp --bench".to_string(),
            memory_benchmark: "jatin-lean memory ipc --bench".to_string(),
        },
        project_context: project_ctx,
        system_context: system_ctx,
    };

    crate::output::output_result("ai-context", ai_context, ctx)?;

    Ok(())
}

fn detect_package_manager(path: &PathBuf) -> String {
    if path.join("package-lock.json").exists() {
        "npm".to_string()
    } else if path.join("yarn.lock").exists() {
        "yarn".to_string()
    } else if path.join("pnpm-lock.yaml").exists() {
        "pnpm".to_string()
    } else if path.join("bun.lockb").exists() {
        "bun".to_string()
    } else {
        "unknown".to_string()
    }
}
```

---

## 6. Legacy Commands (src/cli/legacy.rs)

```rust
//! Legacy command support with deprecation warnings

use clap::Subcommand;
use std::path::PathBuf;
use anyhow::Result;
use console::style;
use crate::output::OutputContext;
use crate::cli::{NodeCommands, SystemCommands, NetworkCommands, MemoryCommands, BenchCommands, AnalyzeCommands};

#[derive(Subcommand, Debug)]
pub enum LegacyCommands {
    #[command(hide = true)]
    Dedup { path: PathBuf },
    
    #[command(hide = true)]
    Health { path: PathBuf },
    
    #[command(hide = true)]
    Deps { path: PathBuf },
    
    #[command(hide = true)]
    Compress { path: PathBuf },
    
    #[command(hide = true)]
    Treeshake { path: PathBuf },
    
    #[command(hide = true)]
    Audit { path: PathBuf },
    
    #[command(hide = true)]
    Analyze { path: PathBuf },
    
    #[command(hide = true)]
    Xdp {
        #[arg(long)]
        compare: bool,
        #[arg(long)]
        bench: bool,
        #[arg(long, default_value = "1000000")]
        packets: u64,
        #[arg(long, default_value = "none")]
        obfuscation: String,
    },
    
    // Add all other legacy commands...
}

pub fn handle_command(command: LegacyCommands, ctx: &OutputContext) -> Result<()> {
    // Show deprecation warning (unless JSON mode)
    if !ctx.json {
        show_deprecation_warning(&command);
    }

    // Route to new command handlers
    match command {
        LegacyCommands::Dedup { path } => {
            crate::cli::handle_node_command(NodeCommands::Dedup { path }, ctx)
        }
        LegacyCommands::Health { path } => {
            crate::cli::handle_node_command(NodeCommands::Health { path }, ctx)
        }
        LegacyCommands::Deps { path } => {
            crate::cli::handle_node_command(NodeCommands::Deps { path }, ctx)
        }
        LegacyCommands::Compress { path } => {
            crate::cli::handle_node_command(NodeCommands::Compress { path }, ctx)
        }
        LegacyCommands::Treeshake { path } => {
            crate::cli::handle_node_command(NodeCommands::Treeshake { path }, ctx)
        }
        LegacyCommands::Audit { path } => {
            crate::cli::handle_node_command(NodeCommands::Audit { path }, ctx)
        }
        LegacyCommands::Analyze { path } => {
            crate::cli::handle_node_command(NodeCommands::Analyze { path }, ctx)
        }
        LegacyCommands::Xdp { compare, bench, packets, obfuscation } => {
            crate::cli::handle_network_command(
                NetworkCommands::Xdp { compare, bench, packets, obfuscation },
                ctx
            )
        }
        // ... other commands
    }
}

fn show_deprecation_warning(command: &LegacyCommands) {
    eprintln!();
    eprintln!("  {} This command syntax is deprecated and will be removed in v3.0.0",
        style("⚠️").yellow().bold());
    
    let new_syntax = match command {
        LegacyCommands::Dedup { .. } => "jatin-lean node dedup",
        LegacyCommands::Health { .. } => "jatin-lean node health",
        LegacyCommands::Deps { .. } => "jatin-lean node deps",
        LegacyCommands::Compress { .. } => "jatin-lean node compress",
        LegacyCommands::Treeshake { .. } => "jatin-lean node treeshake",
        LegacyCommands::Audit { .. } => "jatin-lean node audit",
        LegacyCommands::Analyze { .. } => "jatin-lean node analyze",
        LegacyCommands::Xdp { .. } => "jatin-lean network xdp",
        // ... other commands
    };
    
    eprintln!("  {} Use: {}", style("→").dim(), style(new_syntax).green());
    eprintln!();
}
```

---

## 7. Update Cargo.toml

```toml
[package]
name = "jatin-lean"
version = "2.0.0"
edition = "2021"
description = "Universal system optimization platform with node_modules pruning, eBPF networking, memory IPC, and hardware-level performance tuning"
authors = ["Jatin Jalandhra"]
license = "MIT"
repository = "https://github.com/decodejatin/jatin-lean"
homepage = "https://github.com/decodejatin/jatin-lean"
documentation = "https://github.com/decodejatin/jatin-lean#readme"
readme = "README.md"
keywords = ["optimization", "performance", "node_modules", "ebpf", "system-tuning"]
categories = ["command-line-utilities", "development-tools", "performance"]

# ... rest of dependencies (add chrono for timestamps)

[dependencies]
# ... existing dependencies ...

# Add for JSON timestamps
chrono = { version = "0.4", features = ["serde"] }
```

---

## 8. Testing Examples

```rust
// tests/cli_tests.rs

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use predicates::prelude::*;

    #[test]
    fn test_new_node_health_command() {
        let mut cmd = Command::cargo_bin("jatin-lean").unwrap();
        cmd.arg("node").arg("health").arg("--json");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("\"command\":\"node health\""));
    }

    #[test]
    fn test_legacy_health_command_still_works() {
        let mut cmd = Command::cargo_bin("jatin-lean").unwrap();
        cmd.arg("health");
        cmd.assert()
            .success()
            .stderr(predicate::str::contains("deprecated"));
    }

    #[test]
    fn test_ai_context_command() {
        let mut cmd = Command::cargo_bin("jatin-lean").unwrap();
        cmd.arg("ai-context").arg("--json");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("\"tool\":\"jatin-lean\""));
    }

    #[test]
    fn test_json_output_is_valid() {
        let mut cmd = Command::cargo_bin("jatin-lean").unwrap();
        cmd.arg("node").arg("health").arg("--json");
        let output = cmd.output().unwrap();
        let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(json["success"], true);
    }
}
```

---

## Usage Examples

### Before (v1.x)
```bash
jatin-lean health .
jatin-lean xdp --bench
jatin-lean optimize --assess
```

### After (v2.0)
```bash
# New syntax (recommended)
jatin-lean node health .
jatin-lean network xdp --bench
jatin-lean system optimize --assess

# With JSON output
jatin-lean node health --json
jatin-lean ai-context --json-pretty

# Old syntax still works (with deprecation warning)
jatin-lean health .  # Shows: "Use: jatin-lean node health"
```

---

These code examples provide a complete foundation for implementing the transformation. Copy and adapt them as needed!
