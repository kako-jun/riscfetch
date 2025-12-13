//! Logo generation using figlet-rs
//!
//! Dynamically generates ASCII art logos for vendors using FIGlet fonts.

use figlet_rs::FIGfont;

/// Available vendor logos
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogoVendor {
    Default,
    SiFive,
    StarFive,
    Kendryte,
    Allwinner,
    Espressif,
    SpacemiT,
    THead,
    MilkV,
    Sipeed,
    Sophgo,
}

/// Logo display styles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogoStyle {
    Normal,
    Small,
    None,
}

impl LogoVendor {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "sifive" => Self::SiFive,
            "starfive" => Self::StarFive,
            "kendryte" | "canaan" => Self::Kendryte,
            "allwinner" => Self::Allwinner,
            "espressif" | "esp" => Self::Espressif,
            "spacemit" => Self::SpacemiT,
            "thead" | "t-head" | "alibaba" => Self::THead,
            "milkv" | "milk-v" => Self::MilkV,
            "sipeed" => Self::Sipeed,
            "sophgo" => Self::Sophgo,
            _ => Self::Default,
        }
    }

    /// Get the display name for this vendor
    fn display_name(&self) -> &'static str {
        match self {
            Self::Default => "RISC-V",
            Self::SiFive => "SiFive",
            Self::StarFive => "StarFive",
            Self::Kendryte => "Kendryte",
            Self::Allwinner => "Allwinner",
            Self::Espressif => "Espressif",
            Self::SpacemiT => "SpacemiT",
            Self::THead => "T-Head",
            Self::MilkV => "Milk-V",
            Self::Sipeed => "Sipeed",
            Self::Sophgo => "Sophgo",
        }
    }

    /// Get the subtitle for this vendor
    fn subtitle(&self) -> &'static str {
        match self {
            Self::Default => "Architecture Info",
            Self::SiFive => "RISC-V by SiFive",
            Self::StarFive => "RISC-V by StarFive",
            Self::Kendryte => "RISC-V by Kendryte",
            Self::Allwinner => "RISC-V by Allwinner",
            Self::Espressif => "RISC-V by Espressif",
            Self::SpacemiT => "RISC-V by SpacemiT",
            Self::THead => "RISC-V by T-Head",
            Self::MilkV => "RISC-V by Milk-V",
            Self::Sipeed => "RISC-V by Sipeed",
            Self::Sophgo => "RISC-V by Sophgo",
        }
    }
}

impl LogoStyle {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "small" | "compact" => Self::Small,
            "none" | "off" => Self::None,
            _ => Self::Normal,
        }
    }
}

/// Generate ASCII art logo for the specified vendor
pub fn generate_logo(vendor: LogoVendor, style: LogoStyle) -> String {
    match style {
        LogoStyle::None => String::new(),
        LogoStyle::Small => format!("  {} - {}", vendor.display_name(), vendor.subtitle()),
        LogoStyle::Normal => generate_figlet_logo(vendor),
    }
}

/// Generate FIGlet ASCII art logo
fn generate_figlet_logo(vendor: LogoVendor) -> String {
    let standard_font = FIGfont::standard();

    match standard_font {
        Ok(font) => {
            let name = vendor.display_name();
            match font.convert(name) {
                Some(figure) => {
                    let fig_str = figure.to_string();
                    // Get the width of the first non-empty line
                    let logo_width = fig_str
                        .lines()
                        .filter(|l| !l.trim().is_empty())
                        .map(|l| l.len())
                        .max()
                        .unwrap_or(0);

                    let subtitle = vendor.subtitle();
                    let padding = if logo_width > subtitle.len() {
                        (logo_width - subtitle.len()) / 2
                    } else {
                        0
                    };

                    let mut result = String::new();
                    result.push('\n');
                    result.push_str(&fig_str);
                    result.push_str(&format!(
                        "{:>width$}\n",
                        subtitle,
                        width = padding + subtitle.len()
                    ));
                    result
                }
                None => fallback_logo(vendor),
            }
        }
        Err(_) => fallback_logo(vendor),
    }
}

/// Fallback if FIGlet fails
fn fallback_logo(vendor: LogoVendor) -> String {
    format!(
        "\n  === {} ===\n       {}\n",
        vendor.display_name(),
        vendor.subtitle()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vendor_from_str() {
        assert_eq!(LogoVendor::from_str("default"), LogoVendor::Default);
        assert_eq!(LogoVendor::from_str("sifive"), LogoVendor::SiFive);
        assert_eq!(LogoVendor::from_str("SIFIVE"), LogoVendor::SiFive);
        assert_eq!(LogoVendor::from_str("starfive"), LogoVendor::StarFive);
        assert_eq!(LogoVendor::from_str("kendryte"), LogoVendor::Kendryte);
        assert_eq!(LogoVendor::from_str("canaan"), LogoVendor::Kendryte);
        assert_eq!(LogoVendor::from_str("espressif"), LogoVendor::Espressif);
        assert_eq!(LogoVendor::from_str("esp"), LogoVendor::Espressif);
        assert_eq!(LogoVendor::from_str("spacemit"), LogoVendor::SpacemiT);
        assert_eq!(LogoVendor::from_str("thead"), LogoVendor::THead);
        assert_eq!(LogoVendor::from_str("t-head"), LogoVendor::THead);
        assert_eq!(LogoVendor::from_str("milkv"), LogoVendor::MilkV);
        assert_eq!(LogoVendor::from_str("milk-v"), LogoVendor::MilkV);
        assert_eq!(LogoVendor::from_str("unknown"), LogoVendor::Default);
    }

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
        let logo = generate_logo(LogoVendor::Default, LogoStyle::None);
        assert!(logo.is_empty());
    }

    #[test]
    fn test_generate_logo_small_style() {
        let logo = generate_logo(LogoVendor::Default, LogoStyle::Small);
        assert!(logo.contains("RISC-V"));
        assert!(logo.contains("Architecture Info"));
    }

    #[test]
    fn test_generate_logo_small_vendor() {
        let logo = generate_logo(LogoVendor::SiFive, LogoStyle::Small);
        assert!(logo.contains("SiFive"));
        assert!(logo.contains("RISC-V by SiFive"));
    }

    #[test]
    fn test_generate_logo_normal_not_empty() {
        let logo = generate_logo(LogoVendor::Default, LogoStyle::Normal);
        assert!(!logo.is_empty());
        assert!(logo.contains("Architecture Info"));
    }

    #[test]
    fn test_all_vendors_have_logos() {
        let vendors = [
            LogoVendor::Default,
            LogoVendor::SiFive,
            LogoVendor::StarFive,
            LogoVendor::Kendryte,
            LogoVendor::Allwinner,
            LogoVendor::Espressif,
            LogoVendor::SpacemiT,
            LogoVendor::THead,
            LogoVendor::MilkV,
            LogoVendor::Sipeed,
            LogoVendor::Sophgo,
        ];

        for vendor in vendors {
            let logo = generate_logo(vendor, LogoStyle::Normal);
            assert!(
                !logo.is_empty(),
                "Logo for {:?} should not be empty",
                vendor
            );
        }
    }
}
