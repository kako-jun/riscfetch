//! ISA string parsing functions

use crate::extensions::{
    STANDARD_EXTENSIONS, S_CATEGORY_NAMES, S_EXTENSIONS, Z_CATEGORY_NAMES, Z_EXTENSIONS,
};

/// Extension info with category
#[derive(Debug, Clone)]
pub struct ExtensionInfo {
    pub name: String,
    pub description: String,
    pub category: String,
}

/// Strip rv32/rv64 prefix from ISA base part to get extension letters only
#[must_use]
pub fn strip_rv_prefix(base: &str) -> &str {
    base.strip_prefix("rv64")
        .or_else(|| base.strip_prefix("rv32"))
        .unwrap_or(base)
}

/// Parse extensions from ISA string (pure function for testing)
#[must_use]
pub fn parse_extensions_compact(isa: &str) -> String {
    let isa = isa.to_lowercase();
    let mut exts = Vec::new();

    // Get the base part before any underscore
    let base = isa.split('_').next().unwrap_or(&isa);
    let ext_part = strip_rv_prefix(base);

    // G is shorthand for IMAFD (per RISC-V spec)
    let has_g = ext_part.contains('g');

    // Standard extensions in canonical order
    // Note: E and I are mutually exclusive
    let standard = [
        ('i', "I", false), // (char, name, implied_by_g)
        ('e', "E", false), // E = embedded (16 registers)
        ('m', "M", true),
        ('a', "A", true),
        ('f', "F", true),
        ('d', "D", true),
        ('q', "Q", false),
        ('c', "C", false),
        ('b', "B", false),
        ('v', "V", false),
        ('h', "H", false),
    ];

    for (ch, name, implied_by_g) in standard {
        if ext_part.contains(ch) || (has_g && implied_by_g) {
            exts.push(name);
        }
    }

    // If G is present but I wasn't explicitly added, add I (G implies IMAFD)
    if has_g && !exts.contains(&"I") && !exts.contains(&"E") {
        exts.insert(0, "I");
    }

    exts.join(" ")
}

/// Parse Z-extensions from ISA string (pure function for testing)
#[must_use]
pub fn parse_z_extensions(isa: &str) -> String {
    let isa = isa.to_lowercase();
    let mut z_exts = Vec::new();

    // Check if G is present (G implies Zicsr_Zifencei per RISC-V spec)
    let base = isa.split('_').next().unwrap_or(&isa);
    let ext_part = strip_rv_prefix(base);
    let has_g = ext_part.contains('g');

    // Add implied Z-extensions from G
    if has_g {
        z_exts.push("zicsr".to_string());
        z_exts.push("zifencei".to_string());
    }

    // Add explicit Z-extensions (z prefix only)
    for part in isa.split('_') {
        if part.starts_with('z') && !z_exts.contains(&part.to_string()) {
            z_exts.push(part.to_string());
        }
    }

    z_exts.join(" ")
}

/// Parse S-extensions from ISA string (pure function for testing)
#[must_use]
pub fn parse_s_extensions(isa: &str) -> String {
    let isa = isa.to_lowercase();
    let mut s_exts = Vec::new();

    // Add explicit S-extensions (s prefix only)
    for part in isa.split('_') {
        if part.starts_with('s') && !s_exts.contains(&part.to_string()) {
            s_exts.push(part.to_string());
        }
    }

    s_exts.join(" ")
}

/// Parse extensions with explanations (pure function for testing)
#[must_use]
pub fn parse_extensions_explained(isa: &str) -> Vec<(String, String)> {
    let isa = isa.to_lowercase();
    let base = isa.split('_').next().unwrap_or(&isa);
    let ext_part = strip_rv_prefix(base);
    let mut exts = Vec::new();

    for &(ch, name, desc) in STANDARD_EXTENSIONS {
        if ext_part.contains(ch) {
            exts.push((name.to_string(), desc.to_string()));
        }
    }

    exts
}

/// Parse Z-extensions with explanations (pure function for testing)
#[must_use]
pub fn parse_z_extensions_explained(isa: &str) -> Vec<(String, String)> {
    let isa = isa.to_lowercase();
    let mut z_exts = Vec::new();

    for &(pattern, name, desc, _category) in Z_EXTENSIONS {
        if isa.contains(pattern) {
            z_exts.push((name.to_string(), desc.to_string()));
        }
    }

    z_exts
}

/// Parse S-extensions with explanations (pure function for testing)
#[must_use]
pub fn parse_s_extensions_explained(isa: &str) -> Vec<(String, String)> {
    let isa = isa.to_lowercase();
    let mut s_exts = Vec::new();

    for &(pattern, name, desc, _category) in S_EXTENSIONS {
        if isa.contains(pattern) {
            s_exts.push((name.to_string(), desc.to_string()));
        }
    }

    s_exts
}

/// Parse Z-extensions with category info
#[must_use]
pub fn parse_z_extensions_with_category(isa: &str) -> Vec<ExtensionInfo> {
    let isa = isa.to_lowercase();
    let mut z_exts = Vec::new();

    // Check if G is present (G implies Zicsr_Zifencei per RISC-V spec)
    let base = isa.split('_').next().unwrap_or(&isa);
    let ext_part = strip_rv_prefix(base);
    let has_g = ext_part.contains('g');

    // Add implied Z-extensions from G
    if has_g {
        z_exts.push(ExtensionInfo {
            name: "Zicsr".to_string(),
            description: "CSR Instructions".to_string(),
            category: "base".to_string(),
        });
        z_exts.push(ExtensionInfo {
            name: "Zifencei".to_string(),
            description: "Instruction-Fetch Fence".to_string(),
            category: "base".to_string(),
        });
    }

    for &(pattern, name, desc, category) in Z_EXTENSIONS {
        if isa.contains(pattern) {
            // Skip if already added (implied by G)
            if !z_exts.iter().any(|e| e.name.eq_ignore_ascii_case(name)) {
                z_exts.push(ExtensionInfo {
                    name: name.to_string(),
                    description: desc.to_string(),
                    category: category.to_string(),
                });
            }
        }
    }

    z_exts
}

/// Parse S-extensions with category info
#[must_use]
pub fn parse_s_extensions_with_category(isa: &str) -> Vec<ExtensionInfo> {
    let isa = isa.to_lowercase();
    let mut s_exts = Vec::new();

    for &(pattern, name, desc, category) in S_EXTENSIONS {
        if isa.contains(pattern) {
            s_exts.push(ExtensionInfo {
                name: name.to_string(),
                description: desc.to_string(),
                category: category.to_string(),
            });
        }
    }

    s_exts
}

/// Get category display name for Z-extensions
#[must_use]
pub fn get_z_category_name(category: &str) -> &'static str {
    Z_CATEGORY_NAMES
        .iter()
        .find(|(id, _)| *id == category)
        .map_or("Other", |(_, name)| *name)
}

/// Get category display name for S-extensions
#[must_use]
pub fn get_s_category_name(category: &str) -> &'static str {
    S_CATEGORY_NAMES
        .iter()
        .find(|(id, _)| *id == category)
        .map_or("Other", |(_, name)| *name)
}

/// Group extensions by category
#[must_use]
pub fn group_by_category(extensions: &[ExtensionInfo]) -> Vec<(String, Vec<&ExtensionInfo>)> {
    use std::collections::BTreeMap;
    let mut groups: BTreeMap<String, Vec<&ExtensionInfo>> = BTreeMap::new();

    for ext in extensions {
        groups.entry(ext.category.clone()).or_default().push(ext);
    }

    groups.into_iter().collect()
}

/// Parse vector details from ISA string (pure function for testing)
/// Returns None if no vector extension, Some(details) otherwise
#[must_use]
pub fn parse_vector_from_isa(isa: &str) -> Option<String> {
    let isa = isa.to_lowercase();
    let base = isa.split('_').next().unwrap_or(&isa);
    let ext_part = strip_rv_prefix(base);

    // Check for V extension in the extension part, or zve in Z-extensions
    if !ext_part.contains('v') && !isa.contains("zve") {
        return None;
    }

    let mut details = vec!["Enabled".to_string()];

    // Detect VLEN from zvl* extensions (use largest value)
    // If no zvl* specified, VLEN is implementation-defined (do not display)
    if isa.contains("zvl65536b") {
        details.push("VLEN>=65536".to_string());
    } else if isa.contains("zvl32768b") {
        details.push("VLEN>=32768".to_string());
    } else if isa.contains("zvl16384b") {
        details.push("VLEN>=16384".to_string());
    } else if isa.contains("zvl8192b") {
        details.push("VLEN>=8192".to_string());
    } else if isa.contains("zvl4096b") {
        details.push("VLEN>=4096".to_string());
    } else if isa.contains("zvl2048b") {
        details.push("VLEN>=2048".to_string());
    } else if isa.contains("zvl1024b") {
        details.push("VLEN>=1024".to_string());
    } else if isa.contains("zvl512b") {
        details.push("VLEN>=512".to_string());
    } else if isa.contains("zvl256b") {
        details.push("VLEN>=256".to_string());
    } else if isa.contains("zvl128b") {
        details.push("VLEN>=128".to_string());
    } else if isa.contains("zvl64b") {
        details.push("VLEN>=64".to_string());
    } else if isa.contains("zvl32b") {
        details.push("VLEN>=32".to_string());
    }
    // No default VLEN - it's implementation-defined per RISC-V spec

    Some(details.join(", "))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Real ISA strings from actual RISC-V systems
    const ISA_VISIONFIVE2: &str = "rv64imafdc_zicntr_zicsr_zifencei_zihpm_zba_zbb";
    const ISA_SPACEMIT_K1: &str = "rv64imafdcv_zicbom_zicboz_zicntr_zicsr_zifencei_zihintpause_zihpm_zba_zbb_zbc_zbs_zkt_zvkt_zvl128b_zvl256b_zvl32b_zvl64b";
    const ISA_MINIMAL: &str = "rv64imac";
    const ISA_RV32: &str = "rv32imc";

    // === parse_extensions_compact tests ===

    #[test]
    fn test_visionfive2() {
        assert_eq!(parse_extensions_compact(ISA_VISIONFIVE2), "I M A F D C");
    }

    #[test]
    fn test_spacemit() {
        assert_eq!(parse_extensions_compact(ISA_SPACEMIT_K1), "I M A F D C V");
    }

    #[test]
    fn test_minimal() {
        assert_eq!(parse_extensions_compact(ISA_MINIMAL), "I M A C");
    }

    #[test]
    fn test_rv32() {
        assert_eq!(parse_extensions_compact(ISA_RV32), "I M C");
    }

    #[test]
    fn test_unknown() {
        assert_eq!(parse_extensions_compact("unknown"), "");
    }

    #[test]
    fn test_case_insensitive() {
        assert_eq!(
            parse_extensions_compact("RV64IMAFDC"),
            parse_extensions_compact("rv64imafdc")
        );
    }

    #[test]
    fn test_empty() {
        assert_eq!(parse_extensions_compact(""), "");
    }

    // === Specification-based tests (from SPEC.md) ===

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
    fn spec_with_vector() {
        assert_eq!(parse_extensions_compact("rv64imafdcv"), "I M A F D C V");
    }

    #[test]
    fn spec_rv64_prefix_not_vector() {
        let result = parse_extensions_compact("rv64imafdc");
        assert!(!result.contains('V'));
    }

    #[test]
    fn spec_z_extensions_ignored() {
        assert_eq!(
            parse_extensions_compact("rv64imafdc_zba_zbb"),
            "I M A F D C"
        );
    }

    #[test]
    fn spec_rv64_only() {
        assert_eq!(parse_extensions_compact("rv64"), "");
    }

    // === parse_z_extensions tests ===

    #[test]
    fn test_z_extensions_visionfive2() {
        let result = parse_z_extensions(ISA_VISIONFIVE2);
        assert!(result.contains("zicntr"));
        assert!(result.contains("zicsr"));
        assert!(result.contains("zifencei"));
        assert!(result.contains("zba"));
        assert!(result.contains("zbb"));
    }

    #[test]
    fn test_z_extensions_spacemit() {
        let result = parse_z_extensions(ISA_SPACEMIT_K1);
        assert!(result.contains("zicbom"));
        assert!(result.contains("zicboz"));
        assert!(result.contains("zbc"));
        assert!(result.contains("zbs"));
        assert!(result.contains("zvl256b"));
    }

    #[test]
    fn test_z_extensions_minimal() {
        assert!(parse_z_extensions(ISA_MINIMAL).is_empty());
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
    fn spec_z_extensions_none() {
        assert_eq!(parse_z_extensions("rv64imafdc"), "");
    }

    #[test]
    fn spec_z_extensions_g_implies() {
        assert_eq!(parse_z_extensions("rv64gc"), "zicsr zifencei");
    }

    #[test]
    fn spec_z_extensions_case() {
        assert_eq!(parse_z_extensions("rv64i_Zicsr"), "zicsr");
    }

    // === parse_s_extensions tests ===

    #[test]
    fn spec_s_extensions() {
        let result = parse_s_extensions("rv64i_sstc");
        assert!(result.contains("sstc"));
    }

    // === parse_extensions_explained tests ===

    #[test]
    fn test_explained_visionfive2() {
        let result = parse_extensions_explained(ISA_VISIONFIVE2);
        assert_eq!(result.len(), 6); // I M A F D C
        assert!(result.iter().any(|(n, _)| n == "I"));
        assert!(result.iter().any(|(n, _)| n == "M"));
        assert!(result.iter().any(|(n, _)| n == "F"));
        assert!(result.iter().any(|(n, _)| n == "D"));
        assert!(result.iter().any(|(n, _)| n == "C"));
    }

    #[test]
    fn test_z_explained_spacemit() {
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

    // === parse_vector_from_isa tests ===

    #[test]
    fn test_vector_no_vector() {
        assert!(parse_vector_from_isa(ISA_VISIONFIVE2).is_none());
    }

    #[test]
    fn test_vector_with_v() {
        let result = parse_vector_from_isa(ISA_SPACEMIT_K1);
        assert!(result.is_some());
        let detail = result.unwrap();
        assert!(detail.contains("Enabled"));
        assert!(detail.contains("VLEN>=256"));
    }

    #[test]
    fn test_vector_zve_only() {
        let result = parse_vector_from_isa("rv64imac_zve32x");
        assert!(result.is_some());
        assert!(result.unwrap().contains("Enabled"));
    }

    #[test]
    fn spec_vector_with_v() {
        let result = parse_vector_from_isa("rv64imafdcv");
        assert!(result.is_some());
        assert!(result.unwrap().contains("Enabled"));
    }

    #[test]
    fn spec_vector_none() {
        assert!(parse_vector_from_isa("rv64imafdc").is_none());
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
