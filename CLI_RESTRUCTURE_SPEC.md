# CLI Restructure Implementation Specification

## Overview

This document provides detailed implementation guidance for restructuring the jatin-lean CLI from a flat command structure to a hierarchical, category-based structure.

---

## Current vs New Structure

### Current (Flat)
```
jatin-lean [path] [--flags]
jatin-lean <subcommand> [--flags]
```

### New (Hierarchical)
```
jatin-lean <category> <command> [--flags]
```

---

## Implementation Strategy

### Phase 1: Add New Structure (Non-Breaking)

Create new command categories while keeping old commands working.

```rust
// src/main.rs

#[derive(Parser, Debug)]
#[command(name = "jatin-lean", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    
    /// Global JSON output flag
    #[arg(long, global = true)]
    json: bool,
    
    /// Pretty-print JSON
    #[arg(long, global = true)]
    json_pretty: bool,
    
    /// Verbose output
    #[arg(long, short = 'v', global = true)]
    verbose: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Node.js ecosystem optimization
    Node {
        #[command(subcommand)]
        command: NodeCommands,
    },
    
    /// System-level optimization
    System {
        #[command(subcommand)]
        command: SystemCommands,
    },
    
    /// Network & eBPF tools
    Network {
        #[command(subcommand)]
        command: NetworkCommands,
    },
    
    /// Memory & IPC optimization
    Memory {
        #[command(subcommand)]
        command: MemoryCommands,
    },
    
    /// Benchmarking suite
    Bench {
        #[command(subcommand)]
        command: BenchCommands,
    },
    
    /// Analysis tools
    Analyze {
        #[command(subcommand)]
        command: AnalyzeCommands,
    },
    
    // Legacy commands (hidden, deprecated)
    #[command(flatten)]
    legacy: LegacyCommands,
}
```

### Phase 2: Define Category Commands

#### Node Commands
```rust
#[derive(Subcommand, Debug)]
enum NodeCommands {
    /// Scan node_modules for optimization opportunities
    Scan {
        /// Path to project directory
        #[arg(default_value = ".")]
        path: PathBuf,
        
        /// Show individual files
        #[arg(long, short = 'v')]
        verbose: bool,
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
    
    /// Run health check on node_modules
    Health {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    
    /// Find duplicate files
    Dedup {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    
    /// Analyze dependency graph
    Deps {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    
    /// Analyze compression potential
    Compress {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    
    /// Analyze tree-shaking potential
    Treeshake {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    
    /// Audit packages
    Audit {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    
    /// Analyze project structure
    Analyze {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    
    /// Watch node_modules for changes
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
    
    /// Visualize node_modules
    Visualize {
        #[arg(default_value = ".")]
        path: PathBuf,
        
        #[arg(long)]
        treemap: bool,
        
        #[arg(long)]
        sparklines: bool,
    },
}
```

#### System Commands
```rust
#[derive(Subcommand, Debug)]
enum SystemCommands {
    /// System optimization assessment and tuning
    Optimize {
        /// Run full system assessment
        #[arg(long)]
        assess: bool,
        
        /// Show NUMA topology
        #[arg(long)]
        numa: bool,
        
        /// Show network tuning
        #[arg(long)]
        network: bool,
        
        /// Show kernel parameters
        #[arg(long)]
        kernel: bool,
        
        /// Generate tuning commands
        #[arg(long)]
        generate: bool,
    },
    
    /// CPU cache optimization
    CpuCache {
        /// Run cache benchmark
        #[arg(long)]
        bench: bool,
        
        /// Show cache hierarchy
        #[arg(long)]
        info: bool,
        
        /// Analyze TLB pressure
        #[arg(long)]
        tlb: bool,
        
        #[arg(long, default_value = "8192")]
        working_set_kb: usize,
    },
    
    /// I/O statistics
    Io {
        #[arg(default_value = ".")]
        path: PathBuf,
        
        #[arg(long)]
        fs_info: bool,
        
        #[arg(long)]
        process: bool,
    },
}
```

#### Network Commands
```rust
#[derive(Subcommand, Debug)]
enum NetworkCommands {
    /// XDP/eBPF network middleware
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
    
    /// BPF verifier and DPI analysis
    Bpf {
        #[arg(long)]
        verify: bool,
        
        #[arg(long)]
        dpi: bool,
        
        #[arg(long, value_name = "PACKETS")]
        skbuff: Option<u64>,
    },
    
    /// Unified gateway pipeline
    Gateway {
        #[arg(long)]
        bench: bool,
        
        #[arg(long, default_value = "10000")]
        requests: u64,
        
        #[arg(long, default_value = "1024")]
        payload_size: usize,
    },
}
```

#### Memory Commands
```rust
#[derive(Subcommand, Debug)]
enum MemoryCommands {
    /// Shared memory IPC
    Ipc {
        #[arg(long)]
        bench: bool,
        
        #[arg(long, default_value = "1024")]
        capacity: usize,
        
        #[arg(long, default_value = "100000")]
        messages: u64,
        
        #[arg(long)]
        layout: bool,
    },
    
    /// mmap ring buffer
    Mmap {
        #[arg(long)]
        bench: bool,
        
        #[arg(long, default_value = "4096")]
        capacity: usize,
        
        #[arg(long, default_value = "256")]
        msg_size: usize,
        
        #[arg(long)]
        compare: bool,
    },
    
    /// Arena allocator
    Arena {
        #[arg(long)]
        bench: bool,
        
        #[arg(long, default_value = "1024")]
        capacity_kb: usize,
        
        #[arg(long, default_value = "100000")]
        allocations: u64,
    },
    
    /// PCIe and CUDA memory
    Pcie {
        #[arg(long)]
        compare: bool,
        
        #[arg(long, default_value = "1")]
        size_gb: u64,
        
        #[arg(long, value_name = "LAYERS")]
        offload: Option<usize>,
        
        #[arg(long)]
        grace_hopper: bool,
    },
}
```

#### Bench Commands
```rust
#[derive(Subcommand, Debug)]
enum BenchCommands {
    /// Run all benchmarks
    All {
        #[arg(long)]
        timer: bool,
    },
    
    /// SIMD operations
    Simd,
    
    /// Serialization (rkyv vs JSON)
    Serde {
        #[arg(long)]
        bench: bool,
        
        #[arg(long, default_value = "100")]
        entities: usize,
        
        #[arg(long, default_value = "1000")]
        iterations: u64,
        
        #[arg(long)]
        compare: bool,
    },
    
    /// JSON parsing
    Json {
        #[arg(long, value_name = "FILE")]
        file: Option<PathBuf>,
        
        #[arg(long)]
        input: Option<String>,
        
        #[arg(long)]
        keys: bool,
    },
    
    /// io_uring async I/O
    IoUring {
        #[arg(long)]
        bench: bool,
        
        #[arg(long, default_value = "10000")]
        files: u64,
        
        #[arg(long)]
        compare: bool,
        
        #[arg(long)]
        nvme: bool,
    },
    
    /// Request coalescing
    Coalesce {
        #[arg(long)]
        demo: bool,
        
        #[arg(long, default_value = "1000")]
        requests: u64,
        
        #[arg(long, default_value = "10")]
        keys: u64,
    },
    
    /// Request hedging
    Hedge {
        #[arg(long)]
        bench: bool,
        
        #[arg(long, default_value = "10000")]
        requests: u64,
        
        #[arg(long)]
        cache_demo: bool,
    },
    
    /// Maglev consistent hashing
    Maglev {
        #[arg(long, default_value = "server-1,server-2,server-3,server-4,server-5")]
        backends: String,
        
        #[arg(long, default_value = "65537")]
        table_size: usize,
        
        #[arg(long)]
        analyze: bool,
    },
    
    /// Static dispatch
    Dispatch {
        #[arg(long)]
        bench: bool,
    },
}
```

#### Analyze Commands
```rust
#[derive(Subcommand, Debug)]
enum AnalyzeCommands {
    /// Project analysis
    Project {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    
    /// Cache statistics
    Cache {
        #[arg(long)]
        clear: bool,
        
        #[arg(long)]
        stats: bool,
        
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    
    /// Distributed cache
    DistCache {
        #[arg(long)]
        stats: bool,
        
        #[arg(long)]
        clear: bool,
        
        #[arg(long)]
        evict: bool,
    },
    
    /// Adaptive engine
    Engine {
        #[arg(long)]
        analyze: bool,
        
        #[arg(long)]
        grace_hopper: bool,
        
        #[arg(long)]
        compare: bool,
    },
    
    /// Snapshot management
    Snapshots {
        #[arg(long)]
        list: bool,
        
        #[arg(long, value_name = "ID")]
        restore: Option<String>,
        
        #[arg(long, value_name = "ID")]
        delete: Option<String>,
        
        #[arg(long, value_name = "DAYS")]
        cleanup: Option<u64>,
    },
    
    /// Analytics dashboard
    Analytics {
        #[arg(long)]
        clear: bool,
    },
    
    /// Undo last operation
    Undo,
    
    /// Restore snapshot
    Restore {
        snapshot_id: String,
    },
}
```

### Phase 3: Legacy Command Support

```rust
#[derive(Subcommand, Debug)]
enum LegacyCommands {
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
    
    // ... all other legacy commands
}

fn handle_legacy_command(cmd: LegacyCommands, json: bool) -> Result<()> {
    // Show deprecation warning (unless JSON mode)
    if !json {
        eprintln!("{} This command syntax is deprecated.", style("⚠️").yellow());
        match &cmd {
            LegacyCommands::Dedup { .. } => {
                eprintln!("   Use: {}", style("jatin-lean node dedup").green());
            }
            LegacyCommands::Health { .. } => {
                eprintln!("   Use: {}", style("jatin-lean node health").green());
            }
            // ... other commands
            _ => {}
        }
        eprintln!();
    }
    
    // Execute the command using new handler
    match cmd {
        LegacyCommands::Dedup { path } => {
            handle_node_command(NodeCommands::Dedup { path }, json)
        }
        LegacyCommands::Health { path } => {
            handle_node_command(NodeCommands::Health { path }, json)
        }
        // ... other commands
        _ => Ok(())
    }
}
```

### Phase 4: JSON Output Support

```rust
// src/output.rs

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct JsonOutput<T> {
    command: String,
    timestamp: String,
    version: String,
    result: T,
}

pub fn output_result<T: Serialize>(
    command: &str,
    result: T,
    json: bool,
    pretty: bool,
) -> Result<()> {
    if json {
        let output = JsonOutput {
            command: command.to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            result,
        };
        
        let json_str = if pretty {
            serde_json::to_string_pretty(&output)?
        } else {
            serde_json::to_string(&output)?
        };
        
        println!("{}", json_str);
    } else {
        // Use existing human-readable output
        // (existing print functions)
    }
    
    Ok(())
}

// Example usage in command handler
fn handle_node_health(path: PathBuf, json: bool, pretty: bool) -> Result<()> {
    let nm_path = path.join("node_modules");
    let report = health::check_health(&nm_path)?;
    
    if json {
        #[derive(Serialize)]
        struct HealthResult {
            grade: String,
            score: u32,
            total_packages: usize,
            total_size_bytes: u64,
            issues: Vec<HealthIssue>,
            license_distribution: HashMap<String, usize>,
        }
        
        let result = HealthResult {
            grade: report.grade.to_string(),
            score: report.score,
            total_packages: report.total_packages,
            total_size_bytes: report.total_size,
            issues: report.issues.into_iter().map(|i| i.into()).collect(),
            license_distribution: report.license_dist,
        };
        
        output_result("node health", result, json, pretty)?;
    } else {
        health::print_health_report(&report);
    }
    
    Ok(())
}
```

---

## Command Handler Routing

```rust
fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Some(Commands::Node { command }) => {
            handle_node_command(command, cli.json, cli.json_pretty)
        }
        Some(Commands::System { command }) => {
            handle_system_command(command, cli.json, cli.json_pretty)
        }
        Some(Commands::Network { command }) => {
            handle_network_command(command, cli.json, cli.json_pretty)
        }
        Some(Commands::Memory { command }) => {
            handle_memory_command(command, cli.json, cli.json_pretty)
        }
        Some(Commands::Bench { command }) => {
            handle_bench_command(command, cli.json, cli.json_pretty)
        }
        Some(Commands::Analyze { command }) => {
            handle_analyze_command(command, cli.json, cli.json_pretty)
        }
        Some(Commands::Legacy(legacy)) => {
            handle_legacy_command(legacy, cli.json)
        }
        None => {
            // Default behavior: show help
            Cli::command().print_help()?;
            Ok(())
        }
    }
}

fn handle_node_command(
    command: NodeCommands,
    json: bool,
    pretty: bool,
) -> Result<()> {
    match command {
        NodeCommands::Scan { path, verbose } => {
            // Implementation
        }
        NodeCommands::Prune { path, force, yes, snapshot } => {
            // Implementation
        }
        NodeCommands::Health { path } => {
            handle_node_health(path, json, pretty)
        }
        // ... other commands
    }
}

// Similar handlers for other categories
```

---

## JSON Output Schemas

The JSON output schemas have been standardized across all top-level CLI categories. All outputs are gated via the global `--json` flag and utilize a unified `{ "command": "...", "timestamp": "...", "data": { ... } }` wrapper.

### Standardized Wrapper
```json
{
  "command": "node health",
  "timestamp": "2026-05-17T12:00:00Z",
  "data": {
    // Command-specific schema
  }
}
```

### Module Examples

#### System Module (`system io --json`)
```json
{
  "data": {
    "io_operations": {
      "process_stats": {
        "read_bytes": 1048576,
        "write_bytes": 2048,
        "read_syscalls": 150,
        "write_syscalls": 12
      }
    }
  }
}
```

#### Network Module (`network xdp --json`)
```json
{
  "data": {
    "xdp": {
      "mode": "Simulated",
      "throughput_gbps": 12.5,
      "latency_ns": 45,
      "dropped_packets": 0
    }
  }
}
```

#### Memory Module (`memory pcie --json`)
```json
{
  "data": {
    "bottleneck_analysis": {
      "theoretical_bandwidth_gbps": 64.0,
      "effective_bandwidth_gbps": 52.3,
      "offload_plan": [
        {
          "layer_id": "transformer_0",
          "placement": "VRAM",
          "reason": "Hot path"
        }
      ]
    }
  }
}
```

#### Bench Module (`bench simd-json --json`)
```json
{
  "data": {
    "scan": {
      "bytes": 5242880,
      "structural_chars_found": 15000,
      "max_nesting_depth": 5
    }
  }
}
```

#### Analyze Module (`analyze cache --json`)
```json
{
  "data": {
    "cache_info": {
      "cached_count": 42,
      "age_seconds": 3600,
      "cache_path": "/path/to/project/node_modules/.jatin-lean/scan_cache.json"
    }
  }
}
```

#### Node Module (`node treeshake --json`)
```json
{
  "data": {
    "total_exports": 1520,
    "unused_exports": 340,
    "estimated_dead_bytes": 85000
  }
}
```

---

## Testing Strategy

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_command_structure() {
        let cli = Cli::parse_from(&["jatin-lean", "node", "scan"]);
        assert!(matches!(cli.command, Some(Commands::Node { .. })));
    }
    
    #[test]
    fn test_legacy_command_still_works() {
        let cli = Cli::parse_from(&["jatin-lean", "health", "."]);
        assert!(matches!(cli.command, Some(Commands::Legacy(_))));
    }
    
    #[test]
    fn test_json_output_flag() {
        let cli = Cli::parse_from(&["jatin-lean", "node", "health", "--json"]);
        assert!(cli.json);
    }
}
```

### Integration Tests
```bash
#!/bin/bash
# tests/cli_integration.sh

# Test new commands
jatin-lean node scan --json > /tmp/output.json
jq . /tmp/output.json  # Validate JSON

# Test legacy commands still work
jatin-lean health . > /tmp/legacy.txt
grep -q "Health Check" /tmp/legacy.txt

# Test deprecation warnings
jatin-lean health . 2>&1 | grep -q "deprecated"

echo "All tests passed!"
```

---

## Migration Timeline

### Week 1-2: Implementation
- Implement new command structure
- Add JSON output support
- Maintain legacy commands

### Week 3: Testing
- Unit tests
- Integration tests
- Performance benchmarks

### Week 4: Documentation
- Update README
- Create migration guide
- Update examples

### Week 5-6: Beta Release
- Beta release to npm
- Gather feedback
- Fix issues

### Week 7: Stable Release
- Release v2.0.0
- Announce deprecation timeline
- Monitor adoption

---

## Success Criteria

✅ All new commands work correctly  
✅ All legacy commands still work  
✅ JSON output for all commands  
✅ No performance regression  
✅ Documentation updated  
✅ Tests pass  
✅ Positive user feedback

---

**Document Version**: 1.0  
**Last Updated**: May 17, 2026  
**Status**: Ready for Implementation
