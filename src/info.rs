use serde::Serialize;
use std::fs;
use std::process::Command;
use sysinfo::System;

/// Hardware IDs from RISC-V CSRs
#[derive(Default, Serialize, Clone)]
pub struct HardwareIds {
    pub mvendorid: String,
    pub marchid: String,
    pub mimpid: String,
}

/// All system information for JSON output
#[derive(Serialize)]
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

#[derive(Serialize, Default)]
pub struct VectorInfo {
    pub enabled: bool,
    pub vlen: Option<u32>,
    pub elen: Option<u32>,
}

#[derive(Serialize, Default)]
pub struct CacheInfo {
    pub l1d: Option<String>,
    pub l1i: Option<String>,
    pub l2: Option<String>,
    pub l3: Option<String>,
}

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

/// Get compact extension list (e.g., "I M A F D C V")
pub fn get_extensions_compact() -> String {
    let isa = get_isa_string().to_lowercase();
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

    let base = isa.split('_').next().unwrap_or(&isa);
    for (ch, name) in standard {
        if base.contains(ch) {
            exts.push(name);
        }
    }

    exts.join(" ")
}

/// Get Z-extensions as compact string
pub fn get_z_extensions() -> String {
    let isa = get_isa_string().to_lowercase();
    let mut z_exts = Vec::new();

    for part in isa.split('_') {
        if part.starts_with('z') || part.starts_with('s') {
            z_exts.push(part.to_string());
        }
    }

    z_exts.join(" ")
}

/// Get extensions with explanations
pub fn get_extensions_explained() -> Vec<(String, String)> {
    let isa = get_isa_string().to_lowercase();
    let base = isa.split('_').next().unwrap_or(&isa);
    let mut exts = Vec::new();

    let explanations = [
        ('i', "I", "Base Integer Instructions"),
        ('m', "M", "Integer Multiply/Divide"),
        ('a', "A", "Atomic Instructions"),
        ('f', "F", "Single-Precision Float"),
        ('d', "D", "Double-Precision Float"),
        ('q', "Q", "Quad-Precision Float"),
        ('c', "C", "Compressed (16-bit)"),
        ('b', "B", "Bit Manipulation"),
        ('v', "V", "Vector (SIMD)"),
        ('h', "H", "Hypervisor"),
    ];

    for (ch, name, desc) in explanations {
        if base.contains(ch) {
            exts.push((name.to_string(), desc.to_string()));
        }
    }

    exts
}

/// Get Z-extensions with explanations
pub fn get_z_extensions_explained() -> Vec<(String, String)> {
    let isa = get_isa_string().to_lowercase();
    let mut z_exts = Vec::new();

    let z_explanations = [
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
        ("zvl128b", "Zvl128b", "VLEN ≥ 128 bits"),
        ("zvl256b", "Zvl256b", "VLEN ≥ 256 bits"),
        ("zvl512b", "Zvl512b", "VLEN ≥ 512 bits"),
        ("svinval", "Svinval", "Fine-Grained TLB"),
        ("svnapot", "Svnapot", "NAPOT Translation"),
        ("svpbmt", "Svpbmt", "Page-Based Mem Types"),
        ("sscofpmf", "Sscofpmf", "Count Overflow/Filter"),
        ("sstc", "Sstc", "Supervisor Timer"),
    ];

    for (pattern, name, desc) in z_explanations {
        if isa.contains(pattern) {
            z_exts.push((name.to_string(), desc.to_string()));
        }
    }

    z_exts
}

/// Get vector extension details (VLEN, ELEN)
pub fn get_vector_detail() -> String {
    let isa = get_isa_string().to_lowercase();
    let base = isa.split('_').next().unwrap_or(&isa);

    if !base.contains('v') && !isa.contains("zve") {
        return String::new();
    }

    let mut details = vec!["Enabled".to_string()];

    if isa.contains("zvl512b") {
        details.push("VLEN≥512".to_string());
    } else if isa.contains("zvl256b") {
        details.push("VLEN≥256".to_string());
    } else if isa.contains("zvl128b") {
        details.push("VLEN≥128".to_string());
    }

    if let Ok(vlen) = fs::read_to_string("/sys/devices/system/cpu/cpu0/riscv/vlen") {
        details.push(format!("VLEN={}", vlen.trim()));
    }

    details.join(", ")
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

pub fn get_memory_info() -> String {
    let mut sys = System::new();
    sys.refresh_memory();

    let total_mem = sys.total_memory();
    let used_mem = sys.used_memory();

    let total_gb = total_mem as f64 / 1_073_741_824.0;
    let used_gb = used_mem as f64 / 1_073_741_824.0;

    format!("{used_gb:.2} GiB / {total_gb:.2} GiB")
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
        format!("{hours}h {minutes}m")
    } else {
        format!("{minutes}m")
    }
}

/// Collect all information for JSON output
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

// Legacy functions for backward compatibility
#[allow(dead_code)]
pub fn get_cpu_info() -> String {
    get_isa_string()
}

#[allow(dead_code)]
pub fn get_soc_info() -> String {
    let ids = get_hardware_ids();
    if !ids.mvendorid.is_empty() {
        format!("Vendor ID: {}", ids.mvendorid)
    } else {
        "Unknown SoC".to_string()
    }
}

#[allow(dead_code)]
pub fn get_extensions() -> String {
    get_extensions_explained()
        .into_iter()
        .map(|(name, desc)| {
            format!(
                "{} ({})",
                name,
                desc.split_whitespace().next().unwrap_or("")
            )
        })
        .collect::<Vec<_>>()
        .join(", ")
}

#[allow(dead_code)]
pub fn get_vector_info() -> String {
    get_vector_detail()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_uptime() {
        let uptime = get_uptime();
        assert!(!uptime.is_empty());
    }
}
