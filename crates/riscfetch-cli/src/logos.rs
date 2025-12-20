//! Logo generation using figlet-rs
//!
//! Dynamically generates ASCII art logos for vendors using `FIGlet` fonts.

use crate::vendors::{get_default_vendor, get_vendor_info};
use figlet_rs::FIGfont;
use std::fmt::Write;

/// RISC-V block letter logo
const RISCV_LOGO: &str = r"
        ██████╗ ██╗███████╗ ██████╗      ██╗   ██╗
        ██╔══██╗██║██╔════╝██╔════╝      ██║   ██║
        ██████╔╝██║███████╗██║     █████╗██║   ██║
        ██╔══██╗██║╚════██║██║     ╚════╝╚██╗ ██╔╝
        ██║  ██║██║███████║╚██████╗       ╚████╔╝
        ╚═╝  ╚═╝╚═╝╚══════╝ ╚═════╝        ╚═══╝
                    Architecture Info
";

/// Logo display styles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogoStyle {
    Normal,
    Small,
    None,
}

impl LogoStyle {
    #[must_use]
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "small" | "compact" => Self::Small,
            "none" | "off" => Self::None,
            _ => Self::Normal,
        }
    }
}

/// Generate ASCII art logo for the specified vendor
#[must_use]
pub fn generate_logo(vendor: &str, style: LogoStyle) -> String {
    let (display_name, subtitle) = get_vendor_info(vendor).unwrap_or_else(get_default_vendor);

    match style {
        LogoStyle::None => String::new(),
        LogoStyle::Small => format!("  {display_name} - {subtitle}"),
        LogoStyle::Normal => {
            if display_name == "RISC-V" {
                RISCV_LOGO.to_string()
            } else {
                generate_figlet_logo(display_name, subtitle)
            }
        }
    }
}

/// Generate `FIGlet` ASCII art logo
fn generate_figlet_logo(display_name: &str, subtitle: &str) -> String {
    let standard_font = FIGfont::standard();

    match standard_font {
        Ok(font) => match font.convert(display_name) {
            Some(figure) => {
                let fig_str = figure.to_string();
                // Get the width of the first non-empty line
                let logo_width = fig_str
                    .lines()
                    .filter(|l| !l.trim().is_empty())
                    .map(str::len)
                    .max()
                    .unwrap_or(0);

                let padding = if logo_width > subtitle.len() {
                    (logo_width - subtitle.len()) / 2
                } else {
                    0
                };

                let mut result = String::new();
                result.push('\n');
                result.push_str(&fig_str);
                let _ = writeln!(
                    result,
                    "{subtitle:>width$}",
                    width = padding + subtitle.len()
                );
                result
            }
            None => fallback_logo(display_name, subtitle),
        },
        Err(_) => fallback_logo(display_name, subtitle),
    }
}

/// Fallback if `FIGlet` fails
fn fallback_logo(display_name: &str, subtitle: &str) -> String {
    format!("\n  === {display_name} ===\n       {subtitle}\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vendors::VENDORS;

    #[test]
    fn test_style_from_str() {
        assert_eq!(LogoStyle::from_str("normal"), LogoStyle::Normal);
        assert_eq!(LogoStyle::from_str("small"), LogoStyle::Small);
        assert_eq!(LogoStyle::from_str("compact"), LogoStyle::Small);
        assert_eq!(LogoStyle::from_str("none"), LogoStyle::None);
        assert_eq!(LogoStyle::from_str("off"), LogoStyle::None);
        assert_eq!(LogoStyle::from_str("unknown"), LogoStyle::Normal);
    }

    #[test]
    fn test_generate_logo_none_style() {
        let logo = generate_logo("default", LogoStyle::None);
        assert!(logo.is_empty());
    }

    #[test]
    fn test_generate_logo_small_style() {
        let logo = generate_logo("default", LogoStyle::Small);
        assert!(logo.contains("RISC-V"));
        assert!(logo.contains("Architecture Info"));
    }

    #[test]
    fn test_generate_logo_small_vendor() {
        let logo = generate_logo("sifive", LogoStyle::Small);
        assert!(logo.contains("SiFive"));
        assert!(logo.contains("RISC-V by SiFive"));
    }

    #[test]
    fn test_generate_logo_normal_not_empty() {
        let logo = generate_logo("default", LogoStyle::Normal);
        assert!(!logo.is_empty());
        // Should contain block letters from RISCV_LOGO
        assert!(logo.contains("██████╗"));
    }

    #[test]
    fn test_unknown_vendor_uses_default() {
        let logo = generate_logo("unknown_vendor", LogoStyle::Small);
        assert!(logo.contains("RISC-V"));
        assert!(logo.contains("Architecture Info"));
    }

    #[test]
    fn test_all_vendors_have_logos() {
        for (aliases, _, _) in VENDORS {
            let vendor = aliases[0];
            let logo = generate_logo(vendor, LogoStyle::Normal);
            assert!(!logo.is_empty(), "Logo for {vendor} should not be empty");
        }
    }

    #[test]
    fn test_new_vendors_logos() {
        // Pine64
        let logo = generate_logo("pine64", LogoStyle::Small);
        assert!(logo.contains("Pine64"));

        // WCH
        let logo = generate_logo("wch", LogoStyle::Small);
        assert!(logo.contains("WCH"));
    }
}
