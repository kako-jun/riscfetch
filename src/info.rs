use std::fs;
use std::process::Command;
use sysinfo::System;

pub fn is_riscv() -> bool {
    // Check architecture via uname -m
    if let Ok(output) = Command::new("uname").arg("-m").output() {
        let arch = String::from_utf8_lossy(&output.stdout);
        if arch.contains("riscv") {
            return true;
        }
    }

    // Fallback: check /proc/cpuinfo
    if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
        if content.contains("riscv") || content.contains("RISC-V") {
            return true;
        }
    }

    false
}

pub fn get_board_info() -> String {
    // Try to get board name from device tree
    if let Ok(content) = fs::read_to_string("/proc/device-tree/model") {
        let model = content.trim_matches('\0').trim();
        if !model.is_empty() {
            return model.to_string();
        }
    }

    // Fallback to compatible string
    if let Ok(content) = fs::read_to_string("/proc/device-tree/compatible") {
        let parts: Vec<&str> = content.split('\0').collect();
        if let Some(first) = parts.first() {
            if !first.is_empty() {
                // Extract readable board name
                if first.contains("starfive") {
                    if first.contains("visionfive2") || first.contains("visionfive-2") {
                        return "StarFive VisionFive 2".to_string();
                    }
                    return "StarFive Board".to_string();
                } else if first.contains("sifive") {
                    if first.contains("unmatched") {
                        return "SiFive HiFive Unmatched".to_string();
                    } else if first.contains("unleashed") {
                        return "SiFive HiFive Unleashed".to_string();
                    }
                    return "SiFive Board".to_string();
                } else if first.contains("milkv") || first.contains("milk-v") {
                    if first.contains("mars") {
                        return "Milk-V Mars".to_string();
                    } else if first.contains("pioneer") {
                        return "Milk-V Pioneer".to_string();
                    }
                    return "Milk-V Board".to_string();
                } else if first.contains("thead") {
                    return "T-Head Board".to_string();
                }
            }
        }
    }

    String::new()
}

pub fn get_cpu_info() -> String {
    if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
        // Try to find ISA info
        for line in content.lines() {
            if line.starts_with("isa") {
                if let Some(isa) = line.split(':').nth(1) {
                    let isa = isa.trim();
                    // Detect RV32 or RV64
                    if isa.starts_with("rv64") {
                        return format!("RV64{}", extract_extensions(isa));
                    } else if isa.starts_with("rv32") {
                        return format!("RV32{}", extract_extensions(isa));
                    }
                    return isa.to_uppercase();
                }
            }
        }

        // Fallback: check model name
        for line in content.lines() {
            if line.starts_with("model name") || line.starts_with("uarch") {
                if let Some(model) = line.split(':').nth(1) {
                    return model.trim().to_string();
                }
            }
        }
    }

    "Unknown RISC-V CPU".to_string()
}

fn extract_extensions(isa: &str) -> String {
    // Extract standard extensions (letters after rv32/rv64)
    let isa_lower = isa.to_lowercase();
    if let Some(base_pos) = isa_lower.find("rv") {
        let after_base = &isa_lower[base_pos + 4..]; // Skip "rv64" or "rv32"
        let extensions: String = after_base
            .chars()
            .take_while(|c| c.is_alphabetic())
            .collect::<String>()
            .to_uppercase();
        return extensions;
    }
    "".to_string()
}

pub fn get_hart_count() -> String {
    // Count number of processor entries in /proc/cpuinfo
    if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
        let count = content
            .lines()
            .filter(|line| line.starts_with("processor"))
            .count();
        if count > 0 {
            return format!("{} hart{}", count, if count > 1 { "s" } else { "" });
        }
    }

    // Fallback: use sysinfo
    let mut sys = System::new();
    sys.refresh_cpu_all();
    let count = sys.cpus().len();
    format!("{} hart{}", count, if count > 1 { "s" } else { "" })
}

pub fn get_soc_info() -> String {
    if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
        for line in content.lines() {
            if line.starts_with("mmu") || line.starts_with("mvendorid") {
                if let Some(value) = line.split(':').nth(1) {
                    let value = value.trim();
                    if !value.is_empty() && value != "0x0" {
                        return format!("Vendor ID: {}", value);
                    }
                }
            }
        }
    }

    // Try device tree for SoC info
    if let Ok(content) = fs::read_to_string("/proc/device-tree/compatible") {
        let parts: Vec<&str> = content.split('\0').collect();
        if let Some(first) = parts.first() {
            if !first.is_empty() {
                return first.to_string();
            }
        }
    }

    "Unknown SoC".to_string()
}

pub fn get_extensions() -> String {
    if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
        for line in content.lines() {
            if line.starts_with("isa") {
                if let Some(isa) = line.split(':').nth(1) {
                    let isa = isa.trim().to_lowercase();
                    let mut exts = Vec::new();

                    // Check for standard extensions
                    if isa.contains('m') {
                        exts.push("M (Multiply)");
                    }
                    if isa.contains('a') {
                        exts.push("A (Atomic)");
                    }
                    if isa.contains('f') {
                        exts.push("F (Float)");
                    }
                    if isa.contains('d') {
                        exts.push("D (Double)");
                    }
                    if isa.contains('c') {
                        exts.push("C (Compressed)");
                    }
                    if isa.contains('v') {
                        exts.push("V (Vector)");
                    }

                    // Check for Z extensions
                    if isa.contains("zicsr") {
                        exts.push("Zicsr");
                    }
                    if isa.contains("zifencei") {
                        exts.push("Zifencei");
                    }
                    if isa.contains("zba") {
                        exts.push("Zba");
                    }
                    if isa.contains("zbb") {
                        exts.push("Zbb");
                    }

                    if !exts.is_empty() {
                        return exts.join(", ");
                    }

                    return extract_extensions(&isa)
                        .chars()
                        .map(|c| c.to_string())
                        .collect::<Vec<_>>()
                        .join(", ");
                }
            }
        }
    }

    "Standard RISC-V".to_string()
}

pub fn get_vector_info() -> String {
    if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
        for line in content.lines() {
            if line.starts_with("isa") {
                if let Some(isa) = line.split(':').nth(1) {
                    let isa = isa.trim().to_lowercase();
                    if isa.contains('v') || isa.contains("vector") {
                        return "Enabled (V extension)".to_string();
                    }
                }
            }
        }
    }
    String::new()
}

pub fn get_cache_info() -> String {
    let mut cache_parts = Vec::new();

    // Try to get cache info from /sys/devices/system/cpu/cpu0/cache/
    if let Ok(l1d_size) = fs::read_to_string("/sys/devices/system/cpu/cpu0/cache/index0/size") {
        let size = l1d_size.trim();
        if !size.is_empty() {
            cache_parts.push(format!("L1D: {}", size));
        }
    }

    if let Ok(l1i_size) = fs::read_to_string("/sys/devices/system/cpu/cpu0/cache/index1/size") {
        let size = l1i_size.trim();
        if !size.is_empty() {
            cache_parts.push(format!("L1I: {}", size));
        }
    }

    if let Ok(l2_size) = fs::read_to_string("/sys/devices/system/cpu/cpu0/cache/index2/size") {
        let size = l2_size.trim();
        if !size.is_empty() {
            cache_parts.push(format!("L2: {}", size));
        }
    }

    if cache_parts.is_empty() {
        String::new()
    } else {
        cache_parts.join(", ")
    }
}

pub fn get_memory_info() -> String {
    let mut sys = System::new();
    sys.refresh_memory();

    let total_mem = sys.total_memory();
    let used_mem = sys.used_memory();

    let total_gb = total_mem as f64 / 1_073_741_824.0;
    let used_gb = used_mem as f64 / 1_073_741_824.0;

    format!("{:.2} GiB / {:.2} GiB", used_gb, total_gb)
}

pub fn get_kernel_info() -> String {
    if let Ok(output) = Command::new("uname").arg("-r").output() {
        let kernel = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !kernel.is_empty() {
            return kernel;
        }
    }
    "Unknown".to_string()
}

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

pub fn get_uptime() -> String {
    let uptime_secs = System::uptime();
    let hours = uptime_secs / 3600;
    let minutes = (uptime_secs % 3600) / 60;

    if hours > 0 {
        format!("{}h {}m", hours, minutes)
    } else {
        format!("{}m", minutes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_extensions() {
        assert_eq!(extract_extensions("rv64imafdc"), "IMAFDC");
        assert_eq!(extract_extensions("rv32imafc"), "IMAFC");
    }

    #[test]
    fn test_get_uptime() {
        let uptime = get_uptime();
        assert!(!uptime.is_empty());
    }
}
