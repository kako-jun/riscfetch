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

use serde::Serialize;
use std::fs;
use std::process::Command;
use sysinfo::System;

/// Hardware IDs from RISC-V CSRs
#[derive(Default, Serialize, Clone, Debug)]
pub struct HardwareIds {
    pub mvendorid: String,
    pub marchid: String,
    pub mimpid: String,
}

/// Vector extension information
#[derive(Serialize, Default, Debug)]
pub struct VectorInfo {
    pub enabled: bool,
    pub vlen: Option<u32>,
    pub elen: Option<u32>,
}

/// Cache information
#[derive(Serialize, Default, Debug)]
pub struct CacheInfo {
    pub l1d: Option<String>,
    pub l1i: Option<String>,
    pub l2: Option<String>,
    pub l3: Option<String>,
}

/// Complete system information for JSON serialization
#[derive(Serialize, Debug)]
pub struct SystemInfo {
    pub isa: String,
    pub extensions: Vec<String>,
    pub z_extensions: Vec<String>,
    pub vector: VectorInfo,
    pub hart_count: usize,
    pub hardware_ids: HardwareIds,
    pub cache: CacheInfo,
    pub board: String,
    pub memory_used_bytes: u64,
    pub memory_total_bytes: u64,
    pub kernel: String,
    pub os: String,
    pub uptime_seconds: u64,
}

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

/// Strip rv32/rv64 prefix from ISA base part to get extension letters only
fn strip_rv_prefix(base: &str) -> &str {
    base.strip_prefix("rv64")
        .or_else(|| base.strip_prefix("rv32"))
        .unwrap_or(base)
}

/// Get raw ISA string (e.g., "rv64imafdcv_zicsr_...")
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

/// Parse extensions from ISA string (pure function for testing)
pub fn parse_extensions_compact(isa: &str) -> String {
    let isa = isa.to_lowercase();
    let mut exts = Vec::new();

    let standard = [
        ('i', "I"),
        ('m', "M"),
        ('a', "A"),
        ('f', "F"),
        ('d', "D"),
        ('q', "Q"),
        ('c', "C"),
        ('b', "B"),
        ('v', "V"),
        ('h', "H"),
    ];

    // Get the base part before any underscore
    let base = isa.split('_').next().unwrap_or(&isa);
    let ext_part = strip_rv_prefix(base);

    for (ch, name) in standard {
        if ext_part.contains(ch) {
            exts.push(name);
        }
    }

    exts.join(" ")
}

/// Get compact extension list (e.g., "I M A F D C V")
pub fn get_extensions_compact() -> String {
    parse_extensions_compact(&get_isa_string())
}

/// Parse Z-extensions from ISA string (pure function for testing)
pub fn parse_z_extensions(isa: &str) -> String {
    let isa = isa.to_lowercase();
    let mut z_exts = Vec::new();

    for part in isa.split('_') {
        if part.starts_with('z') || part.starts_with('s') {
            z_exts.push(part.to_string());
        }
    }

    z_exts.join(" ")
}

/// Get Z-extensions as compact string
pub fn get_z_extensions() -> String {
    parse_z_extensions(&get_isa_string())
}

/// Standard extension definitions
const STANDARD_EXTENSIONS: [(&char, &str, &str); 10] = [
    (&'i', "I", "Base Integer Instructions"),
    (&'m', "M", "Integer Multiply/Divide"),
    (&'a', "A", "Atomic Instructions"),
    (&'f', "F", "Single-Precision Float"),
    (&'d', "D", "Double-Precision Float"),
    (&'q', "Q", "Quad-Precision Float"),
    (&'c', "C", "Compressed (16-bit)"),
    (&'b', "B", "Bit Manipulation"),
    (&'v', "V", "Vector (SIMD)"),
    (&'h', "H", "Hypervisor"),
];

/// Z-extension definitions
const Z_EXTENSIONS: [(&str, &str, &str); 38] = [
    ("zicsr", "Zicsr", "CSR Instructions"),
    ("zifencei", "Zifencei", "Instruction-Fetch Fence"),
    ("zicntr", "Zicntr", "Base Counters/Timers"),
    ("zihpm", "Zihpm", "Hardware Perf Counters"),
    ("zicbom", "Zicbom", "Cache-Block Management"),
    ("zicboz", "Zicboz", "Cache-Block Zero"),
    ("zicond", "Zicond", "Conditional Operations"),
    ("zihintpause", "Zihintpause", "Pause Hint"),
    ("zba", "Zba", "Address Generation"),
    ("zbb", "Zbb", "Basic Bit Manipulation"),
    ("zbc", "Zbc", "Carry-less Multiply"),
    ("zbs", "Zbs", "Single-bit Operations"),
    ("zbkb", "Zbkb", "Bit Manip for Crypto"),
    ("zbkc", "Zbkc", "Carry-less for Crypto"),
    ("zbkx", "Zbkx", "Crossbar for Crypto"),
    ("zfh", "Zfh", "Half-Precision Float"),
    ("zfhmin", "Zfhmin", "Minimal Half-Precision"),
    ("zkt", "Zkt", "Constant-Time Execution"),
    ("zca", "Zca", "Compressed Base"),
    ("zcb", "Zcb", "Compressed Basic Ops"),
    ("zcd", "Zcd", "Compressed Double FP"),
    ("zcf", "Zcf", "Compressed Single FP"),
    ("zve32f", "Zve32f", "Vector 32-bit Float"),
    ("zve32x", "Zve32x", "Vector 32-bit Int"),
    ("zve64d", "Zve64d", "Vector 64-bit Double"),
    ("zve64f", "Zve64f", "Vector 64-bit Float"),
    ("zve64x", "Zve64x", "Vector 64-bit Int"),
    ("zvfh", "Zvfh", "Vector Half-Precision"),
    ("zvfhmin", "Zvfhmin", "Min Vector Half-Prec"),
    ("zvkt", "Zvkt", "Vector Constant-Time"),
    ("zvl128b", "Zvl128b", "VLEN >= 128 bits"),
    ("zvl256b", "Zvl256b", "VLEN >= 256 bits"),
    ("zvl512b", "Zvl512b", "VLEN >= 512 bits"),
    ("svinval", "Svinval", "Fine-Grained TLB"),
    ("svnapot", "Svnapot", "NAPOT Translation"),
    ("svpbmt", "Svpbmt", "Page-Based Mem Types"),
    ("sscofpmf", "Sscofpmf", "Count Overflow/Filter"),
    ("sstc", "Sstc", "Supervisor Timer"),
];

/// Parse extensions with explanations (pure function for testing)
pub fn parse_extensions_explained(isa: &str) -> Vec<(String, String)> {
    let isa = isa.to_lowercase();
    let base = isa.split('_').next().unwrap_or(&isa);
    let ext_part = strip_rv_prefix(base);
    let mut exts = Vec::new();

    for (ch, name, desc) in STANDARD_EXTENSIONS {
        if ext_part.contains(*ch) {
            exts.push((name.to_string(), desc.to_string()));
        }
    }

    exts
}

/// Get extensions with explanations
pub fn get_extensions_explained() -> Vec<(String, String)> {
    parse_extensions_explained(&get_isa_string())
}

/// Parse Z-extensions with explanations (pure function for testing)
pub fn parse_z_extensions_explained(isa: &str) -> Vec<(String, String)> {
    let isa = isa.to_lowercase();
    let mut z_exts = Vec::new();

    for (pattern, name, desc) in Z_EXTENSIONS {
        if isa.contains(pattern) {
            z_exts.push((name.to_string(), desc.to_string()));
        }
    }

    z_exts
}

/// Get Z-extensions with explanations
pub fn get_z_extensions_explained() -> Vec<(String, String)> {
    parse_z_extensions_explained(&get_isa_string())
}

/// Parse vector details from ISA string (pure function for testing)
/// Returns None if no vector extension, Some(details) otherwise
pub fn parse_vector_from_isa(isa: &str) -> Option<String> {
    let isa = isa.to_lowercase();
    let base = isa.split('_').next().unwrap_or(&isa);
    let ext_part = strip_rv_prefix(base);

    // Check for V extension in the extension part, or zve in Z-extensions
    if !ext_part.contains('v') && !isa.contains("zve") {
        return None;
    }

    let mut details = vec!["Enabled".to_string()];

    if isa.contains("zvl512b") {
        details.push("VLEN>=512".to_string());
    } else if isa.contains("zvl256b") {
        details.push("VLEN>=256".to_string());
    } else if isa.contains("zvl128b") {
        details.push("VLEN>=128".to_string());
    }

    Some(details.join(", "))
}

/// Get vector extension details (VLEN, ELEN)
pub fn get_vector_detail() -> String {
    let isa = get_isa_string();
    let mut result = parse_vector_from_isa(&isa).unwrap_or_default();

    // Try to get actual VLEN from sysfs
    if !result.is_empty() {
        if let Ok(vlen) = fs::read_to_string("/sys/devices/system/cpu/cpu0/riscv/vlen") {
            result.push_str(&format!(", VLEN={}", vlen.trim()));
        }
    }

    result
}

/// Get hardware IDs (mvendorid, marchid, mimpid)
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
pub fn get_board_info() -> String {
    if let Ok(content) = fs::read_to_string("/proc/device-tree/model") {
        let model = content.trim_matches('\0').trim();
        if !model.is_empty() {
            return model.to_string();
        }
    }

    if let Ok(content) = fs::read_to_string("/proc/device-tree/compatible") {
        let parts: Vec<&str> = content.split('\0').collect();
        if let Some(first) = parts.first() {
            if !first.is_empty() {
                return first.to_string();
            }
        }
    }

    String::new()
}

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
        // Check some known extensions are found with correct descriptions
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
}
