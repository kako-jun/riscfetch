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
                    let mut result = String::new();
                    result.push('\n');
                    result.push_str(&figure.to_string());
                    result.push_str(&format!("       {}\n", vendor.subtitle()));
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
