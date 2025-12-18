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
