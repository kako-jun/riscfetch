//! RISC-V system information library
//!
//! Provides functions to detect and query RISC-V specific system information
//! including ISA extensions, hardware IDs, vector capabilities, and more.
//!
//! # Example
//!
//! ```no_run
//! use riscfetch_core::*;
//!
//! if is_riscv() {
//!     println!("ISA: {}", get_isa_string());
//!     println!("Extensions: {}", get_extensions_compact());
//! }
//! ```

mod extensions;
mod hardware;
mod parsing;
mod system;
mod types;

// Re-export types
pub use types::{CacheInfo, HardwareIds, RiscvInfo, SystemInfo, VectorInfo};

// Re-export extension definitions
pub use extensions::{
    STANDARD_EXTENSIONS, S_CATEGORY_NAMES, S_EXTENSIONS, Z_CATEGORY_NAMES, Z_EXTENSIONS,
};

// Re-export parsing functions and types
pub use parsing::{
    get_all_s_extensions_with_status, get_all_standard_extensions_with_status,
    get_all_z_extensions_with_status, get_s_category_name, get_z_category_name, group_by_category,
    parse_extensions_compact, parse_extensions_explained, parse_s_extensions,
    parse_s_extensions_explained, parse_s_extensions_with_category, parse_vector_from_isa,
    parse_z_extensions, parse_z_extensions_explained, parse_z_extensions_with_category,
    ExtensionInfo,
};

// Re-export hardware functions
pub use hardware::{
    get_board_info, get_cache_info, get_hardware_ids, get_hart_count, get_hart_count_num,
    get_isa_string, get_vector_detail,
};

// Re-export system functions
pub use system::{
    get_kernel_info, get_memory_bytes, get_memory_info, get_os_info, get_uptime, get_uptime_seconds,
};

use std::fs;
use std::process::Command;
use sysinfo::System;

/// Check if the current system is RISC-V architecture
#[must_use]
pub fn is_riscv() -> bool {
    if let Ok(output) = Command::new("uname").arg("-m").output() {
        let arch = String::from_utf8_lossy(&output.stdout);
        if arch.contains("riscv") {
            return true;
        }
    }

    if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
        if content.contains("riscv") || content.contains("RISC-V") {
            return true;
        }
    }

    false
}

/// Get compact extension list (e.g., "I M A F D C V")
#[must_use]
pub fn get_extensions_compact() -> String {
    parse_extensions_compact(&get_isa_string())
}

/// Get Z-extensions as compact string
#[must_use]
pub fn get_z_extensions() -> String {
    parse_z_extensions(&get_isa_string())
}

/// Get extensions with explanations
#[must_use]
pub fn get_extensions_explained() -> Vec<(String, String)> {
    parse_extensions_explained(&get_isa_string())
}

/// Get Z-extensions with explanations
#[must_use]
pub fn get_z_extensions_explained() -> Vec<(String, String)> {
    parse_z_extensions_explained(&get_isa_string())
}

/// Get S-extensions as compact string
#[must_use]
pub fn get_s_extensions() -> String {
    parse_s_extensions(&get_isa_string())
}

/// Get S-extensions with explanations
#[must_use]
pub fn get_s_extensions_explained() -> Vec<(String, String)> {
    parse_s_extensions_explained(&get_isa_string())
}

/// Get Z-extensions with category info
#[must_use]
pub fn get_z_extensions_with_category() -> Vec<ExtensionInfo> {
    parse_z_extensions_with_category(&get_isa_string())
}

/// Get S-extensions with category info
#[must_use]
pub fn get_s_extensions_with_category() -> Vec<ExtensionInfo> {
    parse_s_extensions_with_category(&get_isa_string())
}

/// Collect RISC-V specific information only (excludes generic system info)
#[must_use]
pub fn collect_riscv_info() -> RiscvInfo {
    let mut sys = System::new();
    sys.refresh_cpu_all();

    let isa = get_isa_string();
    let exts: Vec<String> = get_extensions_explained()
        .into_iter()
        .map(|(name, _)| name)
        .collect();
    let z_exts: Vec<String> = get_z_extensions_explained()
        .into_iter()
        .map(|(name, _)| name)
        .collect();

    let hw_ids = get_hardware_ids();
    let isa_lower = isa.to_lowercase();
    let base = isa_lower.split('_').next().unwrap_or(&isa_lower);

    RiscvInfo {
        isa,
        extensions: exts,
        z_extensions: z_exts,
        vector: VectorInfo {
            enabled: base.contains('v') || isa_lower.contains("zve"),
            vlen: None,
            elen: None,
        },
        hart_count: sys.cpus().len(),
        hardware_ids: hw_ids,
        cache: CacheInfo::default(),
    }
}

/// Collect all information into a single struct
#[must_use]
pub fn collect_all_info() -> SystemInfo {
    let mut sys = System::new();
    sys.refresh_memory();
    sys.refresh_cpu_all();

    let isa = get_isa_string();
    let exts: Vec<String> = get_extensions_explained()
        .into_iter()
        .map(|(name, _)| name)
        .collect();
    let z_exts: Vec<String> = get_z_extensions_explained()
        .into_iter()
        .map(|(name, _)| name)
        .collect();
    let s_exts: Vec<String> = get_s_extensions_explained()
        .into_iter()
        .map(|(name, _)| name)
        .collect();

    let hw_ids = get_hardware_ids();
    let isa_lower = isa.to_lowercase();
    let base = isa_lower.split('_').next().unwrap_or(&isa_lower);

    SystemInfo {
        isa,
        extensions: exts,
        z_extensions: z_exts,
        s_extensions: s_exts,
        vector: VectorInfo {
            enabled: base.contains('v') || isa_lower.contains("zve"),
            vlen: None,
            elen: None,
        },
        hart_count: sys.cpus().len(),
        hardware_ids: hw_ids,
        cache: CacheInfo::default(),
        board: get_board_info(),
        memory_used_bytes: sys.used_memory(),
        memory_total_bytes: sys.total_memory(),
        kernel: get_kernel_info(),
        os: get_os_info(),
        uptime_seconds: System::uptime(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // === System Info Tests (work on any system) ===

    #[test]
    fn test_get_uptime() {
        let uptime = get_uptime();
        assert!(!uptime.is_empty());
    }

    #[test]
    fn test_get_uptime_seconds() {
        let secs = get_uptime_seconds();
        assert!(secs > 0);
    }

    #[test]
    fn test_get_memory_bytes() {
        let (used, total) = get_memory_bytes();
        assert!(total > 0);
        assert!(used <= total);
    }

    #[test]
    fn test_get_kernel_info() {
        let kernel = get_kernel_info();
        assert!(!kernel.is_empty());
    }

    #[test]
    fn test_get_os_info() {
        let os = get_os_info();
        assert!(!os.is_empty());
    }

    #[test]
    fn test_hardware_ids_default() {
        let ids = HardwareIds::default();
        assert!(ids.mvendorid.is_empty());
        assert!(ids.marchid.is_empty());
        assert!(ids.mimpid.is_empty());
    }

    // === RISC-V Hardware Tests (only run on actual RISC-V) ===

    #[cfg(target_arch = "riscv64")]
    mod riscv_hardware_tests {
        use super::*;

        #[test]
        fn hw_is_riscv() {
            assert!(is_riscv());
        }

        #[test]
        fn hw_isa_string_valid() {
            let isa = get_isa_string();
            assert!(isa.starts_with("rv64") || isa.starts_with("rv32"));
        }

        #[test]
        fn hw_isa_string_has_base() {
            let isa = get_isa_string();
            assert!(isa.contains('i') || isa.contains('e'));
        }

        #[test]
        fn hw_extensions_not_empty() {
            let ext = get_extensions_compact();
            assert!(!ext.is_empty());
            assert!(ext.contains('I') || ext.contains('E'));
        }

        #[test]
        fn hw_hart_count_positive() {
            let hart_str = get_hart_count();
            assert!(hart_str.contains("hart"));
            let num: String = hart_str
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect();
            let count: usize = num.parse().unwrap_or(0);
            assert!(count > 0);
        }

        #[test]
        fn hw_hardware_ids_present() {
            let ids = get_hardware_ids();
            let has_any =
                !ids.mvendorid.is_empty() || !ids.marchid.is_empty() || !ids.mimpid.is_empty();
            assert!(has_any);
        }

        #[test]
        fn hw_collect_all_info() {
            let info = collect_all_info();
            assert!(!info.isa.is_empty());
            assert!(!info.extensions.is_empty());
            assert!(info.hart_count > 0);
        }

        #[test]
        fn hw_collect_riscv_info() {
            let info = collect_riscv_info();
            assert!(!info.isa.is_empty());
            assert!(!info.extensions.is_empty());
            assert!(info.hart_count > 0);
        }

        #[test]
        fn hw_riscv_info_excludes_system_fields() {
            let riscv_info = collect_riscv_info();
            let all_info = collect_all_info();

            // RiscvInfo should have the same RISC-V specific fields
            assert_eq!(riscv_info.isa, all_info.isa);
            assert_eq!(riscv_info.extensions, all_info.extensions);
            assert_eq!(riscv_info.z_extensions, all_info.z_extensions);
            assert_eq!(riscv_info.hart_count, all_info.hart_count);

            // SystemInfo has additional fields that RiscvInfo doesn't have
            // (board, memory_*, kernel, os, uptime_seconds)
            // This is verified by the type system - RiscvInfo simply doesn't have these fields
        }
    }
}
