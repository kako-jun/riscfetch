//! Hardware information reading from /proc and /sys

use crate::parsing::parse_vector_from_isa;
use crate::types::HardwareIds;
use std::fmt::Write;
use std::fs;
use sysinfo::System;

/// Get raw ISA string (e.g., `rv64imafdcv_zicsr_...`)
#[must_use]
pub fn get_isa_string() -> String {
    if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
        for line in content.lines() {
            if line.starts_with("isa") {
                if let Some(isa) = line.split(':').nth(1) {
                    return isa.trim().to_string();
                }
            }
        }
    }
    "unknown".to_string()
}

/// Get hardware IDs (mvendorid, marchid, mimpid)
#[must_use]
pub fn get_hardware_ids() -> HardwareIds {
    let mut ids = HardwareIds::default();

    if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
        for line in content.lines() {
            if line.starts_with("mvendorid") {
                if let Some(val) = line.split(':').nth(1) {
                    let val = val.trim();
                    if !val.is_empty() && val != "0x0" {
                        ids.mvendorid = val.to_string();
                    }
                }
            } else if line.starts_with("marchid") {
                if let Some(val) = line.split(':').nth(1) {
                    let val = val.trim();
                    if !val.is_empty() && val != "0x0" {
                        ids.marchid = val.to_string();
                    }
                }
            } else if line.starts_with("mimpid") {
                if let Some(val) = line.split(':').nth(1) {
                    let val = val.trim();
                    if !val.is_empty() && val != "0x0" {
                        ids.mimpid = val.to_string();
                    }
                }
            }
        }
    }

    ids
}

/// Get hart count as formatted string
#[must_use]
pub fn get_hart_count() -> String {
    if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
        let count = content
            .lines()
            .filter(|line| line.starts_with("processor"))
            .count();
        if count > 0 {
            return format!("{count} hart{}", if count > 1 { "s" } else { "" });
        }
    }

    let mut sys = System::new();
    sys.refresh_cpu_all();
    let count = sys.cpus().len();
    format!("{count} hart{}", if count > 1 { "s" } else { "" })
}

/// Get hart count as number
#[must_use]
pub fn get_hart_count_num() -> usize {
    if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
        let count = content
            .lines()
            .filter(|line| line.starts_with("processor"))
            .count();
        if count > 0 {
            return count;
        }
    }

    let mut sys = System::new();
    sys.refresh_cpu_all();
    sys.cpus().len()
}

/// Get cache information
#[must_use]
pub fn get_cache_info() -> String {
    let mut cache_parts = Vec::new();

    if let Ok(l1d_size) = fs::read_to_string("/sys/devices/system/cpu/cpu0/cache/index0/size") {
        let size = l1d_size.trim();
        if !size.is_empty() {
            cache_parts.push(format!("L1D:{size}"));
        }
    }

    if let Ok(l1i_size) = fs::read_to_string("/sys/devices/system/cpu/cpu0/cache/index1/size") {
        let size = l1i_size.trim();
        if !size.is_empty() {
            cache_parts.push(format!("L1I:{size}"));
        }
    }

    if let Ok(l2_size) = fs::read_to_string("/sys/devices/system/cpu/cpu0/cache/index2/size") {
        let size = l2_size.trim();
        if !size.is_empty() {
            cache_parts.push(format!("L2:{size}"));
        }
    }

    if let Ok(l3_size) = fs::read_to_string("/sys/devices/system/cpu/cpu0/cache/index3/size") {
        let size = l3_size.trim();
        if !size.is_empty() {
            cache_parts.push(format!("L3:{size}"));
        }
    }

    cache_parts.join(" ")
}

/// Get board/model information from device tree
#[must_use]
pub fn get_board_info() -> String {
    if let Ok(content) = fs::read_to_string("/proc/device-tree/model") {
        let model = content.trim_matches('\0').trim();
        if !model.is_empty() {
            return model.to_string();
        }
    }

    if let Ok(content) = fs::read_to_string("/proc/device-tree/compatible") {
        let parts: Vec<&str> = content.split('\0').collect();
        if let Some(&first) = parts.first() {
            if !first.is_empty() {
                return first.to_string();
            }
        }
    }

    String::new()
}

/// Get vector extension details (VLEN, ELEN)
#[must_use]
pub fn get_vector_detail() -> String {
    let isa = get_isa_string();
    let mut result = parse_vector_from_isa(&isa).unwrap_or_default();

    // Try to get actual VLEN from sysfs
    if !result.is_empty() {
        if let Ok(vlen) = fs::read_to_string("/sys/devices/system/cpu/cpu0/riscv/vlen") {
            let _ = write!(result, ", VLEN={}", vlen.trim());
        }
    }

    result
}
