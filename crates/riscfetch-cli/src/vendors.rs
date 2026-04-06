//! RISC-V vendor definitions
//!
//! This module contains the constant definitions for all supported RISC-V vendors.
//! To add a new vendor, simply add a new entry to the VENDORS array.

/// Vendor definitions
/// Format: (aliases, display_name, subtitle)
///
/// - aliases: lowercase strings used to match CLI input (first one is primary)
/// - display_name: name shown in logo
/// - subtitle: text shown below logo
pub const VENDORS: &[(&[&str], &str, &str)] = &[
    // Default
    (
        &["default", "riscv", "risc-v"],
        "RISC-V",
        "Architecture Info",
    ),
    // Major IP/SoC Providers
    (&["sifive"], "SiFive", "RISC-V by SiFive"),
    (&["starfive"], "StarFive", "RISC-V by StarFive"),
    (
        &["thead", "t-head", "alibaba"],
        "T-Head",
        "RISC-V by T-Head",
    ),
    // Board Manufacturers
    (&["milkv", "milk-v"], "Milk-V", "RISC-V by Milk-V"),
    (&["sipeed"], "Sipeed", "RISC-V by Sipeed"),
    (&["pine64", "pine"], "Pine64", "RISC-V by Pine64"),
    // SoC Vendors
    (&["eswin"], "ESWIN", "RISC-V by ESWIN"),
    (&["ultrarisc"], "UltraRISC", "RISC-V by UltraRISC"),
    (&["kendryte", "canaan"], "Kendryte", "RISC-V by Kendryte"),
    (&["allwinner"], "Allwinner", "RISC-V by Allwinner"),
    (&["espressif", "esp"], "Espressif", "RISC-V by Espressif"),
    (&["spacemit"], "SpacemiT", "RISC-V by SpacemiT"),
    (&["sophgo"], "Sophgo", "RISC-V by Sophgo"),
    // MCU Vendors
    (&["wch", "winchiphead"], "WCH", "RISC-V by WCH"),
];

/// Get vendor info by alias
/// Returns (display_name, subtitle) or None if not found
#[must_use]
pub fn get_vendor_info(alias: &str) -> Option<(&'static str, &'static str)> {
    let alias_lower = alias.to_lowercase();
    for (aliases, display_name, subtitle) in VENDORS {
        if aliases.contains(&alias_lower.as_str()) {
            return Some((display_name, subtitle));
        }
    }
    None
}

/// Get default vendor info
#[must_use]
pub fn get_default_vendor() -> (&'static str, &'static str) {
    // First entry is always default
    let (_, display_name, subtitle) = VENDORS[0];
    (display_name, subtitle)
}

/// Detection keywords for auto-detecting vendor from board/compatible strings.
/// Broader than CLI aliases — includes board names and SoC identifiers.
/// Format: (keyword, vendor_primary_alias)
///
/// **Order matters**: more specific keywords (SoC IDs, board names) must come
/// before generic vendor names. The first match wins, so e.g. "eic7700" (ESWIN)
/// must precede "sifive" to correctly identify HiFive Premier P550.
const VENDOR_KEYWORDS: &[(&str, &str)] = &[
    // SoC / board-specific (most specific first)
    ("eic7700", "eswin"),
    ("eic7702", "eswin"),
    ("ebc77", "eswin"),
    ("hifive premier", "eswin"),
    ("starpro64", "eswin"),
    ("megrez", "eswin"),
    ("dc-roma", "eswin"),
    ("ur-dp1000", "ultrarisc"),
    ("urdp1000", "ultrarisc"),
    ("milkv titan", "ultrarisc"),
    ("milk-v titan", "ultrarisc"),
    ("visionfive", "starfive"),
    ("jh7110", "starfive"),
    ("xuantie", "thead"),
    ("lichee", "sipeed"),
    ("maix", "sipeed"),
    ("star64", "pine64"),
    ("nezha", "allwinner"),
    ("esp32", "espressif"),
    ("cv1800", "sophgo"),
    ("sg2000", "sophgo"),
    ("ch32v", "wch"),
    // Vendor names (generic, checked after specific keywords)
    ("eswin", "eswin"),
    ("ultrarisc", "ultrarisc"),
    ("hifive", "sifive"),
    ("sifive", "sifive"),
    ("starfive", "starfive"),
    ("thead", "thead"),
    ("t-head", "thead"),
    ("milkv", "milkv"),
    ("milk-v", "milkv"),
    ("sipeed", "sipeed"),
    ("pine64", "pine64"),
    ("kendryte", "kendryte"),
    ("canaan", "kendryte"),
    ("allwinner", "allwinner"),
    ("espressif", "espressif"),
    ("spacemit", "spacemit"),
    ("sophgo", "sophgo"),
    ("wch", "wch"),
    ("winchiphead", "wch"),
];

/// Auto-detect vendor from board model and device-tree compatible strings.
/// Returns the vendor alias if a known keyword is found, or None.
#[must_use]
pub fn detect_vendor(board_info: &str, compatible: &str) -> Option<&'static str> {
    let combined = format!("{board_info} {compatible}").to_lowercase();
    for &(keyword, vendor) in VENDOR_KEYWORDS {
        if combined.contains(keyword) {
            return Some(vendor);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_vendor_info_exact() {
        let (name, _) = get_vendor_info("sifive").unwrap();
        assert_eq!(name, "SiFive");
    }

    #[test]
    fn test_get_vendor_info_alias() {
        let (name, _) = get_vendor_info("canaan").unwrap();
        assert_eq!(name, "Kendryte");

        let (name, _) = get_vendor_info("esp").unwrap();
        assert_eq!(name, "Espressif");

        let (name, _) = get_vendor_info("t-head").unwrap();
        assert_eq!(name, "T-Head");
    }

    #[test]
    fn test_get_vendor_info_case_insensitive() {
        let (name, _) = get_vendor_info("SIFIVE").unwrap();
        assert_eq!(name, "SiFive");

        let (name, _) = get_vendor_info("Pine64").unwrap();
        assert_eq!(name, "Pine64");
    }

    #[test]
    fn test_get_vendor_info_unknown() {
        assert!(get_vendor_info("unknown_vendor").is_none());
    }

    #[test]
    fn test_get_default_vendor() {
        let (name, subtitle) = get_default_vendor();
        assert_eq!(name, "RISC-V");
        assert_eq!(subtitle, "Architecture Info");
    }

    #[test]
    fn test_new_vendors_exist() {
        // Pine64
        let (name, _) = get_vendor_info("pine64").unwrap();
        assert_eq!(name, "Pine64");

        // WCH
        let (name, _) = get_vendor_info("wch").unwrap();
        assert_eq!(name, "WCH");

        // ESWIN
        let (name, _) = get_vendor_info("eswin").unwrap();
        assert_eq!(name, "ESWIN");

        // UltraRISC
        let (name, _) = get_vendor_info("ultrarisc").unwrap();
        assert_eq!(name, "UltraRISC");
    }

    #[test]
    fn test_detect_vendor_from_model() {
        assert_eq!(detect_vendor("StarFive VisionFive 2", ""), Some("starfive"));
        assert_eq!(detect_vendor("Milk-V Mars", ""), Some("milkv"));
        assert_eq!(detect_vendor("", ""), None);
    }

    #[test]
    fn test_detect_vendor_from_compatible() {
        assert_eq!(detect_vendor("", "sipeed,licheerv-nano"), Some("sipeed"));
        assert_eq!(
            detect_vendor("", "starfive,visionfive-2-jh7110"),
            Some("starfive")
        );
    }

    #[test]
    fn test_detect_eswin_boards() {
        assert_eq!(detect_vendor("", "eswin,eic7700x"), Some("eswin"));
        assert_eq!(detect_vendor("Pine64 StarPro64", ""), Some("eswin"));
        assert_eq!(detect_vendor("Milk-V Megrez", ""), Some("eswin"));
        assert_eq!(detect_vendor("DeepComputing DC-ROMA II", ""), Some("eswin"));
        assert_eq!(detect_vendor("ESWIN EBC7702", ""), Some("eswin"));
    }

    #[test]
    fn test_detect_ultrarisc_boards() {
        assert_eq!(detect_vendor("Milk-V Titan", ""), Some("ultrarisc"));
        assert_eq!(detect_vendor("", "ultrarisc,ur-dp1000"), Some("ultrarisc"));
    }

    #[test]
    fn test_hifive_unmatched_still_sifive() {
        // HiFive Unmatched uses SiFive FU740, not ESWIN
        assert_eq!(detect_vendor("HiFive Unmatched", ""), Some("sifive"));
        assert_eq!(detect_vendor("HiFive Unleashed", ""), Some("sifive"));
    }

    #[test]
    fn test_detect_vendor_case_insensitive() {
        assert_eq!(detect_vendor("SIFIVE HIFIVE UNMATCHED", ""), Some("sifive"));
        // HiFive Premier P550 uses ESWIN EIC7700X, not SiFive silicon
        assert_eq!(
            detect_vendor("SiFive HiFive Premier P550", ""),
            Some("eswin")
        );
    }

    #[test]
    fn test_detect_vendor_unknown_board() {
        assert_eq!(detect_vendor("Some Unknown Board", "unknown,board"), None);
    }

    #[test]
    fn test_all_vendors_have_info() {
        for (aliases, display_name, subtitle) in VENDORS {
            assert!(!aliases.is_empty(), "Vendor must have at least one alias");
            assert!(!display_name.is_empty(), "Vendor must have display name");
            assert!(!subtitle.is_empty(), "Vendor must have subtitle");
        }
    }
}
