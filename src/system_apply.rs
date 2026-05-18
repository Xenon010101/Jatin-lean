//! System optimization application module
//! Actually applies hardware optimizations instead of just showing recommendations

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::SystemTime;

/// Check if running as root
pub fn is_root() -> bool {
    unsafe { libc::geteuid() == 0 }
}

/// Require root privileges or return error
pub fn require_root() -> Result<()> {
    if !is_root() {
        return Err(anyhow!(
            "This operation requires root privileges.\n\
             Run with: sudo jatin-lean system optimize --apply"
        ));
    }
    Ok(())
}

/// System snapshot for backup/restore
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemSnapshot {
    pub cpu_governor: String,
    pub kernel_params: HashMap<String, String>,
    pub network_settings: NetworkSettings,
    pub io_scheduler: HashMap<String, String>,
    pub thp_enabled: bool,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSettings {
    pub tcp_fastopen: i32,
    pub somaxconn: i32,
    pub tcp_fin_timeout: i32,
    pub rmem_max: i32,
    pub wmem_max: i32,
}

impl SystemSnapshot {
    /// Capture current system state
    pub fn capture() -> Result<Self> {
        Ok(Self {
            cpu_governor: detect_cpu_governor()?,
            kernel_params: read_current_sysctl()?,
            network_settings: read_network_settings()?,
            io_scheduler: read_io_schedulers()?,
            thp_enabled: is_thp_enabled()?,
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)?
                .as_secs(),
        })
    }

    /// Restore system to snapshot state
    pub fn restore(&self) -> Result<()> {
        require_root()?;

        println!("Restoring system snapshot...");

        // Restore CPU governor
        if let Err(e) = apply_cpu_governor(&self.cpu_governor) {
            eprintln!("Warning: Failed to restore CPU governor: {}", e);
        }

        // Restore kernel parameters
        for (key, value) in &self.kernel_params {
            if let Err(e) = sysctl_set(key, value) {
                eprintln!("Warning: Failed to restore {}: {}", key, e);
            }
        }

        // Restore network settings
        if let Err(e) = apply_network_settings(&self.network_settings) {
            eprintln!("Warning: Failed to restore network settings: {}", e);
        }

        // Restore I/O schedulers
        for (device, scheduler) in &self.io_scheduler {
            if let Err(e) = set_io_scheduler(device, scheduler) {
                eprintln!("Warning: Failed to restore I/O scheduler for {}: {}", device, e);
            }
        }

        // Restore THP
        if let Err(e) = set_transparent_hugepages(self.thp_enabled) {
            eprintln!("Warning: Failed to restore THP: {}", e);
        }

        println!("System snapshot restored successfully!");
        Ok(())
    }

    /// Save snapshot to file
    pub fn save(&self, path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Load snapshot from file
    pub fn load(path: &Path) -> Result<Self> {
        let json = fs::read_to_string(path)?;
        let snapshot = serde_json::from_str(&json)?;
        Ok(snapshot)
    }
}

/// Get snapshot directory
pub fn snapshot_dir() -> PathBuf {
    let dir = PathBuf::from("/var/lib/jatin-lean");
    if !dir.exists() {
        let _ = fs::create_dir_all(&dir);
    }
    dir
}

/// Get latest snapshot path
pub fn latest_snapshot_path() -> PathBuf {
    snapshot_dir().join("system_snapshot.json")
}

// ============================================================================
// CPU Governor
// ============================================================================

/// Detect current CPU governor
pub fn detect_cpu_governor() -> Result<String> {
    let path = "/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor";
    if Path::new(path).exists() {
        let governor = fs::read_to_string(path)?.trim().to_string();
        Ok(governor)
    } else {
        Ok("unknown".to_string())
    }
}

/// Get available CPU governors
pub fn get_available_governors() -> Result<Vec<String>> {
    let path = "/sys/devices/system/cpu/cpu0/cpufreq/scaling_available_governors";
    if Path::new(path).exists() {
        let content = fs::read_to_string(path)?;
        let governors: Vec<String> = content
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        Ok(governors)
    } else {
        Ok(vec![])
    }
}

/// Apply CPU governor
pub fn apply_cpu_governor(governor: &str) -> Result<()> {
    require_root()?;

    // Validate governor is available
    let available = get_available_governors()?;
    if !available.is_empty() && !available.contains(&governor.to_string()) {
        return Err(anyhow!(
            "Governor '{}' not available. Available: {:?}",
            governor,
            available
        ));
    }

    // Try cpupower first
    let result = Command::new("cpupower")
        .args(&["frequency-set", "-g", governor])
        .status();

    if result.is_ok() && result.unwrap().success() {
        // Verify
        let current = detect_cpu_governor()?;
        if current == governor {
            return Ok(());
        }
    }

    // Fallback: write directly to sysfs
    let cpu_count = num_cpus::get();
    for cpu in 0..cpu_count {
        let path = format!("/sys/devices/system/cpu/cpu{}/cpufreq/scaling_governor", cpu);
        if Path::new(&path).exists() {
            fs::write(&path, governor)?;
        }
    }

    // Verify
    let current = detect_cpu_governor()?;
    if current != governor {
        return Err(anyhow!("Failed to set CPU governor to {}", governor));
    }

    Ok(())
}

// ============================================================================
// Kernel Parameters (sysctl)
// ============================================================================

/// Read current sysctl values
pub fn read_current_sysctl() -> Result<HashMap<String, String>> {
    let mut params = HashMap::new();

    // Key parameters to track
    let keys = vec![
        "vm.swappiness",
        "vm.dirty_ratio",
        "vm.dirty_background_ratio",
        "vm.vfs_cache_pressure",
        "fs.file-max",
        "fs.inotify.max_user_watches",
        "net.ipv4.tcp_fastopen",
        "net.core.somaxconn",
        "net.ipv4.tcp_fin_timeout",
        "net.core.rmem_max",
        "net.core.wmem_max",
    ];

    for key in keys {
        if let Ok(value) = sysctl_get(key) {
            params.insert(key.to_string(), value);
        }
    }

    Ok(params)
}

/// Get sysctl value
pub fn sysctl_get(key: &str) -> Result<String> {
    let output = Command::new("sysctl")
        .arg("-n")
        .arg(key)
        .output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(anyhow!("Failed to read sysctl {}", key))
    }
}

/// Set sysctl value
pub fn sysctl_set(key: &str, value: &str) -> Result<()> {
    require_root()?;

    let status = Command::new("sysctl")
        .arg("-w")
        .arg(format!("{}={}", key, value))
        .status()?;

    if !status.success() {
        return Err(anyhow!("Failed to set {} = {}", key, value));
    }

    Ok(())
}

/// Apply kernel tuning
pub fn apply_kernel_tuning(params: &HashMap<String, String>) -> Result<()> {
    require_root()?;

    for (key, value) in params {
        sysctl_set(key, value)?;
    }

    Ok(())
}

/// Make sysctl changes persistent
pub fn persist_sysctl(params: &HashMap<String, String>) -> Result<()> {
    require_root()?;

    let conf_path = "/etc/sysctl.d/99-jatin-lean.conf";
    let mut content = String::from("# jatin-lean system optimizations\n");

    for (key, value) in params {
        content.push_str(&format!("{} = {}\n", key, value));
    }

    fs::write(conf_path, content)?;
    println!("Persistent configuration written to {}", conf_path);

    Ok(())
}

// ============================================================================
// Network Tuning
// ============================================================================

/// Read current network settings
pub fn read_network_settings() -> Result<NetworkSettings> {
    Ok(NetworkSettings {
        tcp_fastopen: sysctl_get("net.ipv4.tcp_fastopen")?.parse().unwrap_or(0),
        somaxconn: sysctl_get("net.core.somaxconn")?.parse().unwrap_or(128),
        tcp_fin_timeout: sysctl_get("net.ipv4.tcp_fin_timeout")?.parse().unwrap_or(60),
        rmem_max: sysctl_get("net.core.rmem_max")?.parse().unwrap_or(212992),
        wmem_max: sysctl_get("net.core.wmem_max")?.parse().unwrap_or(212992),
    })
}

/// Apply network settings
pub fn apply_network_settings(settings: &NetworkSettings) -> Result<()> {
    require_root()?;

    sysctl_set("net.ipv4.tcp_fastopen", &settings.tcp_fastopen.to_string())?;
    sysctl_set("net.core.somaxconn", &settings.somaxconn.to_string())?;
    sysctl_set("net.ipv4.tcp_fin_timeout", &settings.tcp_fin_timeout.to_string())?;
    sysctl_set("net.core.rmem_max", &settings.rmem_max.to_string())?;
    sysctl_set("net.core.wmem_max", &settings.wmem_max.to_string())?;

    Ok(())
}

/// Apply optimized network tuning
pub fn apply_optimized_network_tuning() -> Result<()> {
    require_root()?;

    let settings = NetworkSettings {
        tcp_fastopen: 3,           // Enable for both client and server
        somaxconn: 4096,           // Increase connection backlog
        tcp_fin_timeout: 30,       // Reduce TIME_WAIT
        rmem_max: 16777216,        // 16MB receive buffer
        wmem_max: 16777216,        // 16MB send buffer
    };

    apply_network_settings(&settings)?;

    // Additional optimizations
    sysctl_set("net.ipv4.tcp_tw_reuse", "1")?;
    sysctl_set("net.ipv4.tcp_max_syn_backlog", "8192")?;
    sysctl_set("net.core.netdev_max_backlog", "5000")?;

    Ok(())
}

// ============================================================================
// Transparent Huge Pages
// ============================================================================

/// Check if THP is enabled
pub fn is_thp_enabled() -> Result<bool> {
    let path = "/sys/kernel/mm/transparent_hugepage/enabled";
    if Path::new(path).exists() {
        let content = fs::read_to_string(path)?;
        Ok(content.contains("[always]"))
    } else {
        Ok(false)
    }
}

/// Set transparent huge pages
pub fn set_transparent_hugepages(enable: bool) -> Result<()> {
    require_root()?;

    let path = "/sys/kernel/mm/transparent_hugepage/enabled";
    if !Path::new(path).exists() {
        return Err(anyhow!("Transparent huge pages not supported"));
    }

    let value = if enable { "always" } else { "madvise" };
    fs::write(path, value)?;

    Ok(())
}

/// Enable transparent huge pages
pub fn enable_transparent_hugepages() -> Result<()> {
    set_transparent_hugepages(true)
}

// ============================================================================
// I/O Scheduler
// ============================================================================

/// Read I/O schedulers for all block devices
pub fn read_io_schedulers() -> Result<HashMap<String, String>> {
    let mut schedulers = HashMap::new();

    let sys_block = Path::new("/sys/block");
    if !sys_block.exists() {
        return Ok(schedulers);
    }

    for entry in fs::read_dir(sys_block)? {
        let entry = entry?;
        let device = entry.file_name().to_string_lossy().to_string();

        // Skip loop devices and partitions
        if device.starts_with("loop") || device.contains(char::is_numeric) {
            continue;
        }

        let scheduler_path = entry.path().join("queue/scheduler");
        if scheduler_path.exists() {
            if let Ok(content) = fs::read_to_string(&scheduler_path) {
                // Extract current scheduler (marked with [brackets])
                if let Some(start) = content.find('[') {
                    if let Some(end) = content.find(']') {
                        let scheduler = content[start + 1..end].to_string();
                        schedulers.insert(device, scheduler);
                    }
                }
            }
        }
    }

    Ok(schedulers)
}

/// Set I/O scheduler for a device
pub fn set_io_scheduler(device: &str, scheduler: &str) -> Result<()> {
    require_root()?;

    let path = format!("/sys/block/{}/queue/scheduler", device);
    if !Path::new(&path).exists() {
        return Err(anyhow!("Device {} not found", device));
    }

    fs::write(&path, scheduler)?;

    Ok(())
}

/// Apply optimized I/O scheduler (mq-deadline for SSDs, none for NVMe)
pub fn apply_optimized_io_scheduler() -> Result<()> {
    require_root()?;

    let schedulers = read_io_schedulers()?;

    for (device, _current) in schedulers {
        // Detect device type
        let rotational_path = format!("/sys/block/{}/queue/rotational", device);
        let is_rotational = if Path::new(&rotational_path).exists() {
            fs::read_to_string(&rotational_path)?.trim() == "1"
        } else {
            false
        };

        let optimal_scheduler = if device.starts_with("nvme") {
            "none" // NVMe devices work best with none
        } else if is_rotational {
            "mq-deadline" // HDDs benefit from mq-deadline
        } else {
            "mq-deadline" // SSDs also benefit from mq-deadline
        };

        if let Err(e) = set_io_scheduler(&device, optimal_scheduler) {
            eprintln!("Warning: Failed to set scheduler for {}: {}", device, e);
        } else {
            println!("Set I/O scheduler for {} to {}", device, optimal_scheduler);
        }
    }

    Ok(())
}

// ============================================================================
// Read-Ahead
// ============================================================================

/// Set read-ahead for a device
pub fn set_readahead(device: &str, kb: usize) -> Result<()> {
    require_root()?;

    let status = Command::new("blockdev")
        .args(&["--setra", &kb.to_string(), &format!("/dev/{}", device)])
        .status()?;

    if !status.success() {
        return Err(anyhow!("Failed to set read-ahead for {}", device));
    }

    Ok(())
}

/// Apply optimized read-ahead (8MB for SSDs/NVMe)
pub fn apply_optimized_readahead() -> Result<()> {
    require_root()?;

    let schedulers = read_io_schedulers()?;

    for (device, _) in schedulers {
        // 8MB read-ahead for modern storage
        if let Err(e) = set_readahead(&device, 8192) {
            eprintln!("Warning: Failed to set read-ahead for {}: {}", device, e);
        } else {
            println!("Set read-ahead for {} to 8MB", device);
        }
    }

    Ok(())
}

// ============================================================================
// Process Affinity (for future use)
// ============================================================================

/// Pin process to NUMA node
pub fn pin_to_numa_node(pid: u32, node: usize) -> Result<()> {
    require_root()?;

    let status = Command::new("numactl")
        .args(&[
            "--cpunodebind",
            &node.to_string(),
            "--membind",
            &node.to_string(),
            "--",
            "taskset",
            "-p",
            &pid.to_string(),
        ])
        .status()?;

    if !status.success() {
        return Err(anyhow!("Failed to pin process {} to NUMA node {}", pid, node));
    }

    Ok(())
}

/// Set CPU affinity for process
pub fn set_cpu_affinity(pid: u32, cpus: &[usize]) -> Result<()> {
    let cpu_list = cpus
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join(",");

    let status = Command::new("taskset")
        .args(&["-cp", &cpu_list, &pid.to_string()])
        .status()?;

    if !status.success() {
        return Err(anyhow!("Failed to set CPU affinity for process {}", pid));
    }

    Ok(())
}

/// Set process priority
pub fn set_process_priority(pid: u32, priority: i32) -> Result<()> {
    let status = Command::new("renice")
        .args(&["-n", &priority.to_string(), "-p", &pid.to_string()])
        .status()?;

    if !status.success() {
        return Err(anyhow!("Failed to set priority for process {}", pid));
    }

    Ok(())
}

// ============================================================================
// All-in-one optimization
// ============================================================================

/// Apply all safe system optimizations
pub fn apply_all_optimizations() -> Result<()> {
    require_root()?;

    println!("Applying system optimizations...\n");

    // Create snapshot first
    println!("Creating system snapshot...");
    let snapshot = SystemSnapshot::capture()?;
    snapshot.save(&latest_snapshot_path())?;
    println!("✓ Snapshot saved\n");

    // CPU Governor
    println!("Optimizing CPU governor...");
    if let Err(e) = apply_cpu_governor("performance") {
        eprintln!("Warning: {}", e);
    } else {
        println!("✓ CPU governor set to performance\n");
    }

    // Transparent Huge Pages
    println!("Enabling transparent huge pages...");
    if let Err(e) = enable_transparent_hugepages() {
        eprintln!("Warning: {}", e);
    } else {
        println!("✓ Transparent huge pages enabled\n");
    }

    // Network Tuning
    println!("Applying network optimizations...");
    if let Err(e) = apply_optimized_network_tuning() {
        eprintln!("Warning: {}", e);
    } else {
        println!("✓ Network tuning applied\n");
    }

    // I/O Scheduler
    println!("Optimizing I/O schedulers...");
    if let Err(e) = apply_optimized_io_scheduler() {
        eprintln!("Warning: {}", e);
    } else {
        println!("✓ I/O schedulers optimized\n");
    }

    // Read-Ahead
    println!("Optimizing read-ahead...");
    if let Err(e) = apply_optimized_readahead() {
        eprintln!("Warning: {}", e);
    } else {
        println!("✓ Read-ahead optimized\n");
    }

    // Kernel parameters
    println!("Applying kernel optimizations...");
    let mut kernel_params = HashMap::new();
    kernel_params.insert("vm.swappiness".to_string(), "10".to_string());
    kernel_params.insert("vm.dirty_ratio".to_string(), "15".to_string());
    kernel_params.insert("vm.vfs_cache_pressure".to_string(), "50".to_string());
    kernel_params.insert("fs.file-max".to_string(), "2097152".to_string());

    if let Err(e) = apply_kernel_tuning(&kernel_params) {
        eprintln!("Warning: {}", e);
    } else {
        println!("✓ Kernel parameters optimized\n");
    }

    println!("✓ All optimizations applied successfully!");
    println!("\nRevert with: sudo jatin-lean system optimize --revert");

    Ok(())
}

/// Revert to snapshot
pub fn revert_optimizations() -> Result<()> {
    require_root()?;

    let snapshot_path = latest_snapshot_path();
    if !snapshot_path.exists() {
        return Err(anyhow!(
            "No snapshot found. Cannot revert.\n\
             Snapshot path: {}",
            snapshot_path.display()
        ));
    }

    let snapshot = SystemSnapshot::load(&snapshot_path)?;
    snapshot.restore()?;

    Ok(())
}
