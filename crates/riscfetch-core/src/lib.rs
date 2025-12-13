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

mod hardware;
mod parsing;
mod system;
mod types;

// Re-export types
pub use types::{CacheInfo, HardwareIds, SystemInfo, VectorInfo};

// Re-export parsing functions
pub use parsing::{
    parse_extensions_compact, parse_extensions_explained, parse_vector_from_isa,
    parse_z_extensions, parse_z_extensions_explained,
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
pub fn get_extensions_compact() -> String {
    parse_extensions_compact(&get_isa_string())
}

/// Get Z-extensions as compact string
pub fn get_z_extensions() -> String {
    parse_z_extensions(&get_isa_string())
}

/// Get extensions with explanations
pub fn get_extensions_explained() -> Vec<(String, String)> {
    parse_extensions_explained(&get_isa_string())
}

/// Get Z-extensions with explanations
pub fn get_z_extensions_explained() -> Vec<(String, String)> {
    parse_z_extensions_explained(&get_isa_string())
}

/// Collect all information into a single struct
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

    let hw_ids = get_hardware_ids();
    let isa_lower = isa.to_lowercase();
    let base = isa_lower.split('_').next().unwrap_or(&isa_lower);

    SystemInfo {
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

    // Real ISA strings from actual RISC-V systems for testing
    const ISA_VISIONFIVE2: &str = "rv64imafdc_zicntr_zicsr_zifencei_zihpm_zba_zbb";
    const ISA_SPACEMIT_K1: &str = "rv64imafdcv_zicbom_zicboz_zicntr_zicsr_zifencei_zihintpause_zihpm_zba_zbb_zbc_zbs_zkt_zvkt_zvl128b_zvl256b_zvl32b_zvl64b";
    const ISA_MINIMAL: &str = "rv64imac";
    const ISA_RV32: &str = "rv32imc";

    // === ISA Parsing Tests ===

    #[test]
    fn test_parse_extensions_compact_visionfive2() {
        let result = parse_extensions_compact(ISA_VISIONFIVE2);
        assert_eq!(result, "I M A F D C");
    }

    #[test]
    fn test_parse_extensions_compact_spacemit() {
        let result = parse_extensions_compact(ISA_SPACEMIT_K1);
        assert_eq!(result, "I M A F D C V");
    }

    #[test]
    fn test_parse_extensions_compact_minimal() {
        let result = parse_extensions_compact(ISA_MINIMAL);
        assert_eq!(result, "I M A C");
    }

    #[test]
    fn test_parse_extensions_compact_rv32() {
        let result = parse_extensions_compact(ISA_RV32);
        assert_eq!(result, "I M C");
    }

    #[test]
    fn test_parse_extensions_compact_unknown() {
        let result = parse_extensions_compact("unknown");
        assert_eq!(result, "");
    }

    // === Z-Extension Tests ===

    #[test]
    fn test_parse_z_extensions_visionfive2() {
        let result = parse_z_extensions(ISA_VISIONFIVE2);
        assert!(result.contains("zicntr"));
        assert!(result.contains("zicsr"));
        assert!(result.contains("zifencei"));
        assert!(result.contains("zba"));
        assert!(result.contains("zbb"));
    }

    #[test]
    fn test_parse_z_extensions_spacemit() {
        let result = parse_z_extensions(ISA_SPACEMIT_K1);
        assert!(result.contains("zicbom"));
        assert!(result.contains("zicboz"));
        assert!(result.contains("zbc"));
        assert!(result.contains("zbs"));
        assert!(result.contains("zvl256b"));
    }

    #[test]
    fn test_parse_z_extensions_minimal() {
        let result = parse_z_extensions(ISA_MINIMAL);
        assert!(result.is_empty());
    }

    // === Explained Extensions Tests ===

    #[test]
    fn test_parse_extensions_explained_visionfive2() {
        let result = parse_extensions_explained(ISA_VISIONFIVE2);
        assert_eq!(result.len(), 6); // I M A F D C
        assert!(result.iter().any(|(n, _)| n == "I"));
        assert!(result.iter().any(|(n, _)| n == "M"));
        assert!(result.iter().any(|(n, _)| n == "F"));
        assert!(result.iter().any(|(n, _)| n == "D"));
        assert!(result.iter().any(|(n, _)| n == "C"));
    }

    #[test]
    fn test_parse_z_extensions_explained_spacemit() {
        let result = parse_z_extensions_explained(ISA_SPACEMIT_K1);
        assert!(result
            .iter()
            .any(|(n, d)| n == "Zba" && d == "Address Generation"));
        assert!(result
            .iter()
            .any(|(n, d)| n == "Zbb" && d == "Basic Bit Manipulation"));
        assert!(result
            .iter()
            .any(|(n, d)| n == "Zbc" && d == "Carry-less Multiply"));
    }

    // === Vector Tests ===

    #[test]
    fn test_parse_vector_no_vector() {
        let result = parse_vector_from_isa(ISA_VISIONFIVE2);
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_vector_with_v() {
        let result = parse_vector_from_isa(ISA_SPACEMIT_K1);
        assert!(result.is_some());
        let detail = result.unwrap();
        assert!(detail.contains("Enabled"));
        assert!(detail.contains("VLEN>=256"));
    }

    #[test]
    fn test_parse_vector_zve_only() {
        let isa = "rv64imac_zve32x";
        let result = parse_vector_from_isa(isa);
        assert!(result.is_some());
        assert!(result.unwrap().contains("Enabled"));
    }

    // === System Info Tests (these work on any system) ===

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

    // === Edge Cases ===

    #[test]
    fn test_parse_extensions_case_insensitive() {
        let upper = parse_extensions_compact("RV64IMAFDC");
        let lower = parse_extensions_compact("rv64imafdc");
        assert_eq!(upper, lower);
    }

    #[test]
    fn test_parse_empty_isa() {
        let result = parse_extensions_compact("");
        assert_eq!(result, "");
    }

    #[test]
    fn test_parse_z_extensions_order_preserved() {
        let result = parse_z_extensions("rv64i_zba_zbb_zbc");
        let parts: Vec<&str> = result.split(' ').collect();
        assert_eq!(parts, vec!["zba", "zbb", "zbc"]);
    }

    // =======================================================
    // Specification-based tests (from SPEC.md)
    // =======================================================

    mod spec_tests {
        use super::*;

        #[test]
        fn spec_g_expansion() {
            assert_eq!(parse_extensions_compact("rv64gc"), "I M A F D C");
        }

        #[test]
        fn spec_g_expansion_uppercase() {
            assert_eq!(parse_extensions_compact("RV64GC"), "I M A F D C");
        }

        #[test]
        fn spec_e_extension() {
            assert_eq!(parse_extensions_compact("rv32e"), "E");
        }

        #[test]
        fn spec_e_with_c() {
            assert_eq!(parse_extensions_compact("rv32ec"), "E C");
        }

        #[test]
        fn spec_standard_extensions() {
            assert_eq!(parse_extensions_compact("rv64imafdc"), "I M A F D C");
        }

        #[test]
        fn spec_with_vector() {
            assert_eq!(parse_extensions_compact("rv64imafdcv"), "I M A F D C V");
        }

        #[test]
        fn spec_rv64_prefix_not_vector() {
            let result = parse_extensions_compact("rv64imafdc");
            assert!(!result.contains("V"));
        }

        #[test]
        fn spec_z_extensions_ignored() {
            assert_eq!(
                parse_extensions_compact("rv64imafdc_zba_zbb"),
                "I M A F D C"
            );
        }

        #[test]
        fn spec_empty_input() {
            assert_eq!(parse_extensions_compact(""), "");
        }

        #[test]
        fn spec_invalid_input() {
            assert_eq!(parse_extensions_compact("unknown"), "");
        }

        #[test]
        fn spec_rv64_only() {
            assert_eq!(parse_extensions_compact("rv64"), "");
        }

        #[test]
        fn spec_z_extensions_basic() {
            assert_eq!(parse_z_extensions("rv64i_zicsr_zifencei"), "zicsr zifencei");
        }

        #[test]
        fn spec_z_extensions_order() {
            assert_eq!(parse_z_extensions("rv64i_zba_zbb_zbc"), "zba zbb zbc");
        }

        #[test]
        fn spec_s_extensions() {
            let result = parse_z_extensions("rv64i_sstc");
            assert!(result.contains("sstc"));
        }

        #[test]
        fn spec_z_extensions_none() {
            assert_eq!(parse_z_extensions("rv64imafdc"), "");
        }

        #[test]
        fn spec_z_extensions_g_implies() {
            assert_eq!(parse_z_extensions("rv64gc"), "zicsr zifencei");
        }

        #[test]
        fn spec_z_extensions_case() {
            let result = parse_z_extensions("rv64i_Zicsr");
            assert_eq!(result, "zicsr");
        }

        #[test]
        fn spec_vector_with_v() {
            let result = parse_vector_from_isa("rv64imafdcv");
            assert!(result.is_some());
            assert!(result.unwrap().contains("Enabled"));
        }

        #[test]
        fn spec_vector_none() {
            let result = parse_vector_from_isa("rv64imafdc");
            assert!(result.is_none());
        }

        #[test]
        fn spec_vector_zve() {
            let result = parse_vector_from_isa("rv64imac_zve32x");
            assert!(result.is_some());
        }

        #[test]
        fn spec_vector_vlen_256() {
            let result = parse_vector_from_isa("rv64imafdcv_zvl256b");
            assert!(result.is_some());
            assert!(result.unwrap().contains("VLEN>=256"));
        }

        #[test]
        fn spec_vector_vlen_largest() {
            let result = parse_vector_from_isa("rv64imafdcv_zvl128b_zvl256b");
            assert!(result.is_some());
            assert!(result.unwrap().contains("VLEN>=256"));
        }

        #[test]
        fn spec_vector_no_default_vlen() {
            let result = parse_vector_from_isa("rv64imafdcv");
            assert!(result.is_some());
            let detail = result.unwrap();
            assert!(detail.contains("Enabled"));
            assert!(!detail.contains("VLEN"));
        }
    }

    // =======================================================
    // RISC-V Hardware Tests (only run on actual RISC-V)
    // =======================================================

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
    }
}
