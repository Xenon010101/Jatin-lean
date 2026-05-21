//! Example: Apply system optimizations
//!
//! This example shows how to use the actionable optimization features.
//!
//! Run with: cargo run --example apply_optimizations
//! (Requires root: sudo cargo run --example apply_optimizations)

use anyhow::Result;
use jatin_lean::{impact_measurement, optimization, system_apply};

fn main() -> Result<()> {
    println!("=== jatin-lean Actionable Optimizations Example ===\n");

    // Check if running as root
    if !system_apply::is_root() {
        eprintln!("Error: This example requires root privileges.");
        eprintln!("Run with: sudo cargo run --example apply_optimizations");
        std::process::exit(1);
    }

    // Example 1: Create and restore snapshot
    example_snapshot()?;

    // Example 2: Apply individual optimizations
    example_individual_optimizations()?;

    // Example 3: Apply optimization profile
    example_profiles()?;

    // Example 4: Measure impact
    example_measure_impact()?;

    // Example 5: Monitor performance
    example_monitor()?;

    // Example 6: Apply all optimizations
    example_apply_all()?;

    Ok(())
}

fn example_snapshot() -> Result<()> {
    println!("\n--- Example 1: Snapshot and Restore ---\n");

    // Create snapshot
    println!("Creating system snapshot...");
    let snapshot = system_apply::SystemSnapshot::capture()?;
    println!("✓ Snapshot created");
    println!("  CPU Governor: {}", snapshot.cpu_governor);
    println!("  THP Enabled: {}", snapshot.thp_enabled);
    println!("  Kernel params: {} tracked", snapshot.kernel_params.len());

    // Save snapshot
    let snapshot_path = system_apply::latest_snapshot_path();
    snapshot.save(&snapshot_path)?;
    println!("✓ Snapshot saved to: {}", snapshot_path.display());

    // Load and restore
    println!("\nLoading snapshot...");
    let loaded = system_apply::SystemSnapshot::load(&snapshot_path)?;
    println!("✓ Snapshot loaded (timestamp: {})", loaded.timestamp);

    Ok(())
}

fn example_individual_optimizations() -> Result<()> {
    println!("\n--- Example 2: Individual Optimizations ---\n");

    // CPU Governor
    println!("1. CPU Governor Optimization");
    let opt = optimization::Optimization::CpuGovernor("performance".to_string());
    println!("   Description: {}", opt.description());
    println!("   Current: {}", opt.current_value());
    println!("   New: {}", opt.new_value());

    // Validate
    match opt.validate() {
        Ok(_) => println!("   ✓ Validation passed"),
        Err(e) => println!("   ✗ Validation failed: {}", e),
    }

    // Apply (commented out to avoid actually changing system)
    // opt.apply()?;
    // println!("   ✓ Applied");

    // Transparent Huge Pages
    println!("\n2. Transparent Huge Pages");
    let opt = optimization::Optimization::TransparentHugePages(true);
    println!("   Description: {}", opt.description());
    println!("   Current: {}", opt.current_value());
    println!("   New: {}", opt.new_value());

    // Network Tuning
    println!("\n3. Network Tuning");
    let opt = optimization::Optimization::NetworkTuning;
    println!("   Description: {}", opt.description());

    // Kernel Parameter
    println!("\n4. Kernel Parameter");
    let opt =
        optimization::Optimization::KernelParam("vm.swappiness".to_string(), "10".to_string());
    println!("   Description: {}", opt.description());
    println!("   Current: {}", opt.current_value());
    println!("   New: {}", opt.new_value());

    Ok(())
}

fn example_profiles() -> Result<()> {
    println!("\n--- Example 3: Optimization Profiles ---\n");

    // Development profile
    let dev_profile = optimization::OptimizationProfile::development();
    println!("Profile: {}", dev_profile.name);
    println!("Description: {}", dev_profile.description);
    println!("Optimizations:");
    for opt in &dev_profile.optimizations {
        println!("  - {}", opt.description());
    }

    // Server profile
    println!();
    let server_profile = optimization::OptimizationProfile::server();
    println!("Profile: {}", server_profile.name);
    println!("Description: {}", server_profile.description);
    println!(
        "Optimizations: {} items",
        server_profile.optimizations.len()
    );

    // Balanced profile
    println!();
    let balanced_profile = optimization::OptimizationProfile::balanced();
    println!("Profile: {}", balanced_profile.name);
    println!("Description: {}", balanced_profile.description);

    // Apply profile (commented out)
    // dev_profile.apply()?;

    Ok(())
}

fn example_measure_impact() -> Result<()> {
    println!("\n--- Example 4: Measure Impact ---\n");

    println!("Measuring impact of CPU governor change...");

    // Measure impact (commented out to avoid actually changing system)
    /*
    let report = impact_measurement::measure_impact(
        "CPU Governor: powersave → performance",
        || {
            system_apply::apply_cpu_governor("performance")
        }
    )?;

    report.print();
    */

    println!("(Skipped - would require root and system changes)");

    Ok(())
}

fn example_monitor() -> Result<()> {
    println!("\n--- Example 5: Monitor Performance ---\n");

    println!("Collecting current metrics...");
    let metrics = impact_measurement::Metrics::collect()?;

    println!("Current System Metrics:");
    println!("  CPU Frequency: {:.0} MHz", metrics.cpu_freq_mhz);
    println!("  CPU Usage: {:.1}%", metrics.cpu_usage_pct);
    println!(
        "  Context Switches: {}/sec",
        metrics.context_switches_per_sec
    );
    println!("  Cache Miss Rate: {:.1}%", metrics.cache_miss_rate_pct);
    println!("  I/O Throughput: {:.1} MB/s", metrics.io_throughput_mbps);
    println!("  Network Latency: {:.0} μs", metrics.network_latency_us);
    println!("  Available Memory: {} MB", metrics.memory_available_mb);

    // Monitor for 5 seconds (commented out)
    // println!("\nMonitoring for 5 seconds...");
    // impact_measurement::monitor_performance(5)?;

    Ok(())
}

fn example_apply_all() -> Result<()> {
    println!("\n--- Example 6: Apply All Optimizations ---\n");

    println!("This would apply all system optimizations:");
    println!("  1. CPU governor → performance");
    println!("  2. Enable transparent huge pages");
    println!("  3. Optimize network settings");
    println!("  4. Optimize I/O schedulers");
    println!("  5. Optimize read-ahead");
    println!("  6. Tune kernel parameters");

    // Apply all (commented out)
    // system_apply::apply_all_optimizations()?;

    println!("\n(Skipped - would require root and make system changes)");
    println!("\nTo actually apply:");
    println!("  sudo jatin-lean system optimize --apply");

    Ok(())
}
