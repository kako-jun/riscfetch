//! General system information (memory, uptime, kernel, OS)

use std::fs;
use std::process::Command;
use sysinfo::System;

/// Get memory usage as formatted string
pub fn get_memory_info() -> String {
    let mut sys = System::new();
    sys.refresh_memory();

    let total_mem = sys.total_memory();
    let used_mem = sys.used_memory();

    let total_gb = total_mem as f64 / 1_073_741_824.0;
    let used_gb = used_mem as f64 / 1_073_741_824.0;

    format!("{used_gb:.2} GiB / {total_gb:.2} GiB")
}

/// Get memory information as bytes
pub fn get_memory_bytes() -> (u64, u64) {
    let mut sys = System::new();
    sys.refresh_memory();
    (sys.used_memory(), sys.total_memory())
}

/// Get kernel version
pub fn get_kernel_info() -> String {
    if let Ok(output) = Command::new("uname").arg("-r").output() {
        let kernel = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !kernel.is_empty() {
            return kernel;
        }
    }
    "Unknown".to_string()
}

/// Get OS name from /etc/os-release
pub fn get_os_info() -> String {
    if let Ok(content) = fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            if line.starts_with("PRETTY_NAME=") {
                if let Some(name) = line.split('=').nth(1) {
                    return name.trim_matches('"').to_string();
                }
            }
        }
    }

    "Linux".to_string()
}

/// Get uptime as formatted string
pub fn get_uptime() -> String {
    let uptime_secs = System::uptime();
    let hours = uptime_secs / 3600;
    let minutes = (uptime_secs % 3600) / 60;

    if hours > 0 {
        format!("{hours}h {minutes}m")
    } else {
        format!("{minutes}m")
    }
}

/// Get uptime in seconds
pub fn get_uptime_seconds() -> u64 {
    System::uptime()
}
