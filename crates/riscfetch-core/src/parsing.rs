//! ISA string parsing functions

use crate::extensions::{
    STANDARD_EXTENSIONS, S_CATEGORY_NAMES, S_EXTENSIONS, Z_CATEGORY_NAMES, Z_EXTENSIONS,
};
use crate::implications::compute_derived;
use std::collections::BTreeSet;

/// Extension info with category and support status
#[derive(Debug, Clone)]
pub struct ExtensionInfo {
    pub name: String,
    pub description: String,
    pub category: String,
    pub supported: bool,
    /// `true` if this extension was not reported directly in the ISA string but was
    /// inferred through implication or composition from other extensions that are
    /// present (see the `implications` module). `supported` keeps its existing meaning
    /// regardless of this flag: a derived extension is still "supported".
    pub derived: bool,
}

/// Strip rv32/rv64 prefix from ISA base part to get extension letters only
#[must_use]
pub fn strip_rv_prefix(base: &str) -> &str {
    base.strip_prefix("rv64")
        .or_else(|| base.strip_prefix("rv32"))
        .unwrap_or(base)
}

/// Check if an ISA string contains a multi-letter extension by exact part matching.
/// Extensions are underscore-separated; this avoids false positives from substring
/// matching (e.g. "zk" matching inside "zkn", or "sha" inside "shvstvala").
fn isa_has_extension(isa: &str, pattern: &str) -> bool {
    isa.split('_').any(|part| part == pattern)
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
        if isa_has_extension(&isa, pattern) {
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
        if isa_has_extension(&isa, pattern) {
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
            supported: true,
            derived: false,
        });
        z_exts.push(ExtensionInfo {
            name: "Zifencei".to_string(),
            description: "Instruction-Fetch Fence".to_string(),
            category: "base".to_string(),
            supported: true,
            derived: false,
        });
    }

    for &(pattern, name, desc, category) in Z_EXTENSIONS {
        if isa_has_extension(&isa, pattern) {
            // Skip if already added (implied by G)
            if !z_exts.iter().any(|e| e.name.eq_ignore_ascii_case(name)) {
                z_exts.push(ExtensionInfo {
                    name: name.to_string(),
                    description: desc.to_string(),
                    category: category.to_string(),
                    supported: true,
                    derived: false,
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
        if isa_has_extension(&isa, pattern) {
            s_exts.push(ExtensionInfo {
                name: name.to_string(),
                description: desc.to_string(),
                category: category.to_string(),
                supported: true,
                derived: false,
            });
        }
    }

    s_exts
}

/// Collect the canonical names of every standard/Z/S extension the ISA string reports
/// directly, including the ones implied by the `G` shorthand (I, M, A, F, D, Zicsr,
/// Zifencei). Names match the `name` field of `STANDARD_EXTENSIONS`/`Z_EXTENSIONS`/
/// `S_EXTENSIONS` exactly, so they can be fed straight into the implication tables.
fn explicit_extension_names(isa: &str) -> BTreeSet<String> {
    let isa_lower = isa.to_lowercase();
    let base = isa_lower.split('_').next().unwrap_or(&isa_lower);
    let ext_part = strip_rv_prefix(base);
    let has_g = ext_part.contains('g');

    let mut names: BTreeSet<String> = BTreeSet::new();

    for name in parse_extensions_compact(isa).split_whitespace() {
        names.insert(name.to_string());
    }

    for &(pattern, name, _desc, _category) in Z_EXTENSIONS {
        if isa_has_extension(&isa_lower, pattern)
            || (has_g && (pattern == "zicsr" || pattern == "zifencei"))
        {
            names.insert(name.to_string());
        }
    }

    for &(pattern, name, _desc, _category) in S_EXTENSIONS {
        if isa_has_extension(&isa_lower, pattern) {
            names.insert(name.to_string());
        }
    }

    names
}

/// Compute every extension name that is implied or composed from the extensions the
/// ISA string reports directly, but that the ISA string itself does not mention (see
/// the `implications` module). These are the names that should be shown in parentheses.
#[must_use]
pub fn compute_derived_extension_names(isa: &str) -> BTreeSet<String> {
    compute_derived(&explicit_extension_names(isa))
}

/// Get standard (single-letter, including `G` expansion) extensions as `ExtensionInfo`,
/// including ones inferred via implication/composition (e.g. `B` from `Zba`+`Zbb`+`Zbs`,
/// per issue #10). Extensions the ISA string names directly have `derived: false`;
/// inferred ones have `derived: true`.
#[must_use]
pub fn get_extensions_with_derived(isa: &str) -> Vec<ExtensionInfo> {
    let explicit: BTreeSet<String> = parse_extensions_compact(isa)
        .split_whitespace()
        .map(str::to_string)
        .collect();
    let derived_names = compute_derived_extension_names(isa);

    STANDARD_EXTENSIONS
        .iter()
        .filter_map(|&(_ch, name, desc)| {
            let is_explicit = explicit.contains(name);
            let is_derived = !is_explicit && derived_names.contains(name);
            if is_explicit || is_derived {
                Some(ExtensionInfo {
                    name: name.to_string(),
                    description: desc.to_string(),
                    category: "std".to_string(),
                    supported: true,
                    derived: is_derived,
                })
            } else {
                None
            }
        })
        .collect()
}

/// Same as [`parse_z_extensions_with_category`], plus Z-extensions inferred via
/// implication/composition (see issue #10). Inferred entries have `derived: true`.
#[must_use]
pub fn parse_z_extensions_with_category_and_derived(isa: &str) -> Vec<ExtensionInfo> {
    let mut z_exts = parse_z_extensions_with_category(isa);
    let derived_names = compute_derived_extension_names(isa);

    for &(_pattern, name, desc, category) in Z_EXTENSIONS {
        if derived_names.contains(name) && !z_exts.iter().any(|e| e.name.eq_ignore_ascii_case(name))
        {
            z_exts.push(ExtensionInfo {
                name: name.to_string(),
                description: desc.to_string(),
                category: category.to_string(),
                supported: true,
                derived: true,
            });
        }
    }

    z_exts
}

/// Same as [`parse_s_extensions_with_category`], plus S-extensions inferred via
/// implication/composition (see issue #10). Inferred entries have `derived: true`.
#[must_use]
pub fn parse_s_extensions_with_category_and_derived(isa: &str) -> Vec<ExtensionInfo> {
    let mut s_exts = parse_s_extensions_with_category(isa);
    let derived_names = compute_derived_extension_names(isa);

    for &(_pattern, name, desc, category) in S_EXTENSIONS {
        if derived_names.contains(name) && !s_exts.iter().any(|e| e.name.eq_ignore_ascii_case(name))
        {
            s_exts.push(ExtensionInfo {
                name: name.to_string(),
                description: desc.to_string(),
                category: category.to_string(),
                supported: true,
                derived: true,
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

/// Get ALL Z-extensions with support status based on ISA string
#[must_use]
pub fn get_all_z_extensions_with_status(isa: &str) -> Vec<ExtensionInfo> {
    let isa = isa.to_lowercase();
    let base = isa.split('_').next().unwrap_or(&isa);
    let ext_part = strip_rv_prefix(base);
    let has_g = ext_part.contains('g');

    Z_EXTENSIONS
        .iter()
        .map(|&(pattern, name, desc, category)| {
            let supported = isa_has_extension(&isa, pattern)
                || (has_g && (pattern == "zicsr" || pattern == "zifencei"));
            ExtensionInfo {
                name: name.to_string(),
                description: desc.to_string(),
                category: category.to_string(),
                supported,
                derived: false,
            }
        })
        .collect()
}

/// Get ALL S-extensions with support status based on ISA string
#[must_use]
pub fn get_all_s_extensions_with_status(isa: &str) -> Vec<ExtensionInfo> {
    let isa = isa.to_lowercase();

    S_EXTENSIONS
        .iter()
        .map(|&(pattern, name, desc, category)| {
            let supported = isa_has_extension(&isa, pattern);
            ExtensionInfo {
                name: name.to_string(),
                description: desc.to_string(),
                category: category.to_string(),
                supported,
                derived: false,
            }
        })
        .collect()
}

/// Get ALL standard extensions with support status
#[must_use]
pub fn get_all_standard_extensions_with_status(isa: &str) -> Vec<(String, String, bool)> {
    let isa = isa.to_lowercase();
    let base = isa.split('_').next().unwrap_or(&isa);
    let ext_part = strip_rv_prefix(base);
    let has_g = ext_part.contains('g');

    STANDARD_EXTENSIONS
        .iter()
        .map(|&(char, name, desc)| {
            let supported =
                ext_part.contains(char) || (has_g && matches!(char, 'i' | 'm' | 'a' | 'f' | 'd'));
            (name.to_string(), desc.to_string(), supported)
        })
        .collect()
}

/// Parse vector details from ISA string (pure function for testing)
/// Returns None if no vector extension, Some(details) otherwise
#[must_use]
pub fn parse_vector_from_isa(isa: &str) -> Option<String> {
    let isa = isa.to_lowercase();
    let base = isa.split('_').next().unwrap_or(&isa);
    let ext_part = strip_rv_prefix(base);

    // Check for V extension in the extension part, or zve* in Z-extensions
    let has_zve = isa.split('_').any(|part| part.starts_with("zve"));
    if !ext_part.contains('v') && !has_zve {
        return None;
    }

    let mut details = vec!["Enabled".to_string()];

    // Detect VLEN from zvl* extensions (use largest value)
    // If no zvl* specified, VLEN is implementation-defined (do not display)
    if isa_has_extension(&isa, "zvl65536b") {
        details.push("VLEN>=65536".to_string());
    } else if isa_has_extension(&isa, "zvl32768b") {
        details.push("VLEN>=32768".to_string());
    } else if isa_has_extension(&isa, "zvl16384b") {
        details.push("VLEN>=16384".to_string());
    } else if isa_has_extension(&isa, "zvl8192b") {
        details.push("VLEN>=8192".to_string());
    } else if isa_has_extension(&isa, "zvl4096b") {
        details.push("VLEN>=4096".to_string());
    } else if isa_has_extension(&isa, "zvl2048b") {
        details.push("VLEN>=2048".to_string());
    } else if isa_has_extension(&isa, "zvl1024b") {
        details.push("VLEN>=1024".to_string());
    } else if isa_has_extension(&isa, "zvl512b") {
        details.push("VLEN>=512".to_string());
    } else if isa_has_extension(&isa, "zvl256b") {
        details.push("VLEN>=256".to_string());
    } else if isa_has_extension(&isa, "zvl128b") {
        details.push("VLEN>=128".to_string());
    } else if isa_has_extension(&isa, "zvl64b") {
        details.push("VLEN>=64".to_string());
    } else if isa_has_extension(&isa, "zvl32b") {
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

    #[test]
    fn test_z_explained_ratified_2026_09() {
        let isa = "rv64i_ziccid_zicfilp_zicfiss";
        let result = parse_z_extensions_explained(isa);
        assert!(result
            .iter()
            .any(|(n, d)| n == "Ziccid" && d == "Inst/Data Coherence"));
        assert!(result
            .iter()
            .any(|(n, d)| n == "Zicfilp" && d == "CFI Landing Pads"));
        assert!(result
            .iter()
            .any(|(n, d)| n == "Zicfiss" && d == "CFI Shadow Stack"));
    }

    #[test]
    fn test_s_explained_sspmp() {
        let isa = "rv64i_sspmp";
        let result = parse_s_extensions_explained(isa);
        assert!(result
            .iter()
            .any(|(n, d)| n == "Sspmp" && d == "S-mode Phys Mem Protection"));
    }

    #[test]
    fn test_extension_names_no_prefix_collision() {
        // "sspm"/"sspmp" and "ziccif"/"ziccid" share a prefix; isa_has_extension
        // must match the full underscore-separated part only, not a substring.
        let isa_d_p = "rv64imafdc_ziccid_sspmp";
        let z_result = parse_z_extensions_explained(isa_d_p);
        let s_result = parse_s_extensions_explained(isa_d_p);
        assert!(z_result
            .iter()
            .any(|(n, d)| n == "Ziccid" && d == "Inst/Data Coherence"));
        assert!(s_result
            .iter()
            .any(|(n, d)| n == "Sspmp" && d == "S-mode Phys Mem Protection"));
        assert!(!z_result.iter().any(|(n, _)| n == "Ziccif"));
        assert!(!s_result.iter().any(|(n, _)| n == "Sspm"));

        let isa_f_np = "rv64imafdc_ziccif_sspm";
        let z_result2 = parse_z_extensions_explained(isa_f_np);
        let s_result2 = parse_s_extensions_explained(isa_f_np);
        assert!(z_result2
            .iter()
            .any(|(n, d)| n == "Ziccif" && d == "Inst Fetch Coherence"));
        assert!(s_result2
            .iter()
            .any(|(n, d)| n == "Sspm" && d == "Pointer Masking"));
        assert!(!z_result2.iter().any(|(n, _)| n == "Ziccid"));
        assert!(!s_result2.iter().any(|(n, _)| n == "Sspmp"));
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

    // === False positive prevention tests ===

    #[test]
    fn test_zk_does_not_false_match_zkn() {
        // "zk" (Scalar Crypto All) must not be reported when only "zkn" is present
        let isa = "rv64i_zkn";
        let result = parse_z_extensions_explained(isa);
        assert!(
            result.iter().any(|(n, _)| n == "Zkn"),
            "Zkn should be found"
        );
        assert!(
            !result.iter().any(|(n, _)| n == "Zk"),
            "Zk should NOT be found (false positive)"
        );
    }

    #[test]
    fn test_zks_does_not_false_match_zksed() {
        // "zks" must not be reported when only "zksed" is present
        let isa = "rv64i_zksed";
        let result = parse_z_extensions_explained(isa);
        assert!(
            result.iter().any(|(n, _)| n == "Zksed"),
            "Zksed should be found"
        );
        assert!(
            !result.iter().any(|(n, _)| n == "Zks"),
            "Zks should NOT be found (false positive)"
        );
        assert!(
            !result.iter().any(|(n, _)| n == "Zk"),
            "Zk should NOT be found (false positive)"
        );
    }

    #[test]
    fn test_s_extension_no_collision_with_z() {
        // S-extensions should not appear when only Z-extensions with 's' in name are present
        let isa = "rv64i_zbs_zks";
        let result = parse_s_extensions_explained(isa);
        assert!(
            result.is_empty(),
            "No S-extensions should be found in Z-only ISA: {:?}",
            result
        );
    }

    #[test]
    fn test_s_extension_exact_match() {
        // "sstc" should match exactly, not as substring
        let isa = "rv64i_sstc_svnapot";
        let result = parse_s_extensions_explained(isa);
        assert!(
            result.iter().any(|(n, _)| n == "Sstc"),
            "Sstc should be found"
        );
        assert!(
            result.iter().any(|(n, _)| n == "Svnapot"),
            "Svnapot should be found"
        );
    }

    #[test]
    fn test_zvks_does_not_false_match_zvksc() {
        // "zvks" must not be reported when only "zvksc" is present
        let isa = "rv64iv_zvksc";
        let result = parse_z_extensions_explained(isa);
        assert!(
            result.iter().any(|(n, _)| n == "Zvksc"),
            "Zvksc should be found"
        );
        assert!(
            !result.iter().any(|(n, _)| n == "Zvks"),
            "Zvks should NOT be found (false positive)"
        );
    }

    #[test]
    fn test_zicsr_not_false_positive_for_c() {
        // Having "zicsr" in the ISA should not cause "C" to appear in standard extensions
        // (C should only come from the base part before underscores)
        let isa = "rv64ima_zicsr";
        let result = parse_extensions_compact(isa);
        assert_eq!(result, "I M A", "C should not appear from zicsr");
    }

    #[test]
    fn spec_vector_no_default_vlen() {
        let result = parse_vector_from_isa("rv64imafdcv");
        assert!(result.is_some());
        let detail = result.unwrap();
        assert!(detail.contains("Enabled"));
        assert!(!detail.contains("VLEN"));
    }

    // === Derived extensions (issue #10) ===
    //
    // Real ISA strings from actual RISC-V boards, taken from external bug reports.
    const ISA_ORANGEPI_RV2: &str = "rv64imafdcv_zicbom_zicboz_zicntr_zicond_zicsr_zifencei_zihintpause_zihpm_zfh_zfhmin_zca_zcd_zba_zbb_zbc_zbs_zkt_zve32f_zve32x_zve64d_zve64f_zve64x_zvfh_zvfhmin_zvkt_sscofpmf_sstc_svinval_svnapot_svpbmt";
    const ISA_MANGOPI_MQ_PRO: &str = "rv64imafdc";

    #[test]
    fn derived_m_implies_zmmul() {
        let z_exts = parse_z_extensions_with_category_and_derived("rv64ima");
        let zmmul = z_exts.iter().find(|e| e.name == "Zmmul").unwrap();
        assert!(zmmul.derived);
    }

    #[test]
    fn derived_b_from_zba_zbb_zbs_composition() {
        let std_exts = get_extensions_with_derived("rv64i_zba_zbb_zbs");
        let b = std_exts.iter().find(|e| e.name == "B").unwrap();
        assert!(b.derived);

        // Zba alone must not compose into B.
        let std_exts_partial = get_extensions_with_derived("rv64i_zba");
        assert!(!std_exts_partial.iter().any(|e| e.name == "B"));
    }

    #[test]
    fn derived_v_transitively_reaches_zve32x() {
        let z_exts = parse_z_extensions_with_category_and_derived("rv64imafdv");
        let zve32x = z_exts.iter().find(|e| e.name == "Zve32x").unwrap();
        assert!(zve32x.derived);
    }

    #[test]
    fn explicit_extension_stays_not_derived() {
        // "zba" is spelled out directly in the ISA string, so it must never be marked
        // derived even though it also participates in composing B.
        let isa = "rv64i_zba_zbb_zbs";
        let z_exts = parse_z_extensions_with_category_and_derived(isa);
        let zba = z_exts.iter().find(|e| e.name == "Zba").unwrap();
        assert!(!zba.derived);
    }

    #[test]
    fn composition_with_explicit_shorthand_does_not_mark_shorthand_derived() {
        // When "b" is already spelled out in the ISA string, B itself must stay
        // derived: false (it's explicit), while its components (not spelled out here)
        // become derived: true.
        let isa = "rv64ib";
        let std_exts = get_extensions_with_derived(isa);
        let b = std_exts.iter().find(|e| e.name == "B").unwrap();
        assert!(!b.derived);

        let z_exts = parse_z_extensions_with_category_and_derived(isa);
        for name in ["Zba", "Zbb", "Zbs"] {
            let ext = z_exts.iter().find(|e| e.name == name).unwrap();
            assert!(ext.derived, "{name} should be derived from explicit B");
        }
    }

    #[test]
    fn regression_orangepi_rv2_spacemit_k1_shows_derived_b() {
        // Reported in issue #10: Zba/Zbb/Zbs are present in cpuinfo but B was not shown.
        let std_exts = get_extensions_with_derived(ISA_ORANGEPI_RV2);
        let b = std_exts
            .iter()
            .find(|e| e.name == "B")
            .expect("B should be present (derived) for Orange Pi RV2 / SpacemiT K1");
        assert!(b.derived);
    }

    #[test]
    fn regression_mangopi_mq_pro_d1_shows_derived_extensions() {
        // Reported in issue #10: M is present in cpuinfo but Zmmul was not shown.
        let z_exts = parse_z_extensions_with_category_and_derived(ISA_MANGOPI_MQ_PRO);
        for name in ["Zmmul", "Zca", "Zicsr"] {
            let ext = z_exts
                .iter()
                .find(|e| e.name == name)
                .unwrap_or_else(|| panic!("{name} should be derived for MangoPi MQ-Pro / D1"));
            assert!(ext.derived, "{name} should be marked derived");
        }
    }

    #[test]
    fn zihpm_implies_zicsr() {
        // FeatureStdExtZihpm -> FeatureStdExtZicsr in LLVM's RISCVFeatures.td.
        let z_exts = parse_z_extensions_with_category_and_derived("rv64i_zihpm");
        let zicsr = z_exts
            .iter()
            .find(|e| e.name == "Zicsr")
            .expect("Zicsr should be derived from Zihpm alone");
        assert!(zicsr.derived);
    }

    #[test]
    fn orangepi_rv2_zihpm_does_not_spuriously_derive_anything_new() {
        // Orange Pi RV2 already spells out zicsr directly, so adding the
        // Zihpm -> Zicsr implication must not change its derived set: Zicsr was
        // already derived via Zicntr -> Zicsr (and is also explicit here).
        let z_exts = parse_z_extensions_with_category_and_derived(ISA_ORANGEPI_RV2);
        let zicsr = z_exts.iter().find(|e| e.name == "Zicsr").unwrap();
        assert!(
            !zicsr.derived,
            "zicsr is spelled out explicitly in this ISA string"
        );
    }
}
