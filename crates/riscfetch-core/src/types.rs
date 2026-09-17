//! Data types for RISC-V system information

use serde::Serialize;

/// Extension entry with name and description
#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub struct ExtensionEntry {
    pub name: String,
    pub description: String,
    /// `true` if this extension was not reported directly in the ISA string but was
    /// inferred through implication or composition from other extensions that are
    /// present (see issue #10 / `riscfetch_core::implications`).
    pub derived: bool,
}

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

/// RISC-V specific information only (excludes generic system info)
#[derive(Serialize, Debug)]
pub struct RiscvInfo {
    pub isa: String,
    pub extensions: Vec<ExtensionEntry>,
    pub z_extensions: Vec<ExtensionEntry>,
    pub vector: VectorInfo,
    pub hart_count: usize,
    pub hardware_ids: HardwareIds,
    pub cache: CacheInfo,
}

/// Complete system information for JSON serialization
#[derive(Serialize, Debug)]
pub struct SystemInfo {
    pub isa: String,
    pub extensions: Vec<ExtensionEntry>,
    pub z_extensions: Vec<ExtensionEntry>,
    pub s_extensions: Vec<ExtensionEntry>,
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
