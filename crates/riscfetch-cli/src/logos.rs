//! Logo definitions for RISC-V vendors and styles
//!
//! To add a new vendor logo:
//! 1. Add a new constant below (e.g., `LOGO_NEWVENDOR`)
//! 2. Add the variant to `LogoVendor` enum
//! 3. Add the match arm in `get_vendor_logo()`

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

/// Get logo for the specified vendor
pub fn get_vendor_logo(vendor: LogoVendor) -> &'static str {
    match vendor {
        LogoVendor::Default => LOGO_DEFAULT,
        LogoVendor::SiFive => LOGO_SIFIVE,
        LogoVendor::StarFive => LOGO_STARFIVE,
        LogoVendor::Kendryte => LOGO_KENDRYTE,
        LogoVendor::Allwinner => LOGO_ALLWINNER,
        LogoVendor::Espressif => LOGO_ESPRESSIF,
        LogoVendor::SpacemiT => LOGO_SPACEMIT,
        LogoVendor::THead => LOGO_THEAD,
        LogoVendor::MilkV => LOGO_MILKV,
        LogoVendor::Sipeed => LOGO_SIPEED,
        LogoVendor::Sophgo => LOGO_SOPHGO,
    }
}

/// Get small logo for the specified vendor
pub fn get_vendor_logo_small(vendor: LogoVendor) -> &'static str {
    match vendor {
        LogoVendor::Default => LOGO_DEFAULT_SMALL,
        LogoVendor::SiFive => LOGO_SIFIVE_SMALL,
        LogoVendor::StarFive => LOGO_STARFIVE_SMALL,
        LogoVendor::Kendryte => LOGO_KENDRYTE_SMALL,
        LogoVendor::Allwinner => LOGO_ALLWINNER_SMALL,
        LogoVendor::Espressif => LOGO_ESPRESSIF_SMALL,
        LogoVendor::SpacemiT => LOGO_SPACEMIT_SMALL,
        LogoVendor::THead => LOGO_THEAD_SMALL,
        LogoVendor::MilkV => LOGO_MILKV_SMALL,
        LogoVendor::Sipeed => LOGO_SIPEED_SMALL,
        LogoVendor::Sophgo => LOGO_SOPHGO_SMALL,
    }
}

// =============================================================================
// Default RISC-V Logo
// =============================================================================

const LOGO_DEFAULT: &str = r#"
      ____  ____  ____   ____      __  __
     / __ \/_  _\/ ___\ / ___|    / / / /
    / /_/ / / /  \___ \/ /   ____/ / / /
   / _, _/ / /  /___/ / /___/___/ /_/ /
  /_/ |_| /_/  /_____/\____/    \____/

        RISC-V Architecture Info
"#;

const LOGO_DEFAULT_SMALL: &str = r#"
  RISC-V
"#;

// =============================================================================
// SiFive (USA) - HiFive Unmatched, HiFive Unleashed
// =============================================================================

const LOGO_SIFIVE: &str = r#"
   _____ _ ______ _
  / ____(_)  ____(_)
 | (___  _| |__   ___   _____
  \___ \| |  __| | \ \ / / _ \
  ____) | | |    | |\ V /  __/
 |_____/|_|_|    |_| \_/ \___|

      RISC-V by SiFive
"#;

const LOGO_SIFIVE_SMALL: &str = r#"
  SiFive RISC-V
"#;

// =============================================================================
// StarFive (China) - VisionFive, VisionFive 2
// =============================================================================

const LOGO_STARFIVE: &str = r#"
  ____  _              _____ _
 / ___|| |_ __ _ _ __ |  ___(_)_   _____
 \___ \| __/ _` | '__|| |_  | \ \ / / _ \
  ___) | || (_| | |   |  _| | |\ V /  __/
 |____/ \__\__,_|_|   |_|   |_| \_/ \___|

      RISC-V by StarFive
"#;

const LOGO_STARFIVE_SMALL: &str = r#"
  StarFive RISC-V
"#;

// =============================================================================
// Kendryte/Canaan (China) - K210, K510
// =============================================================================

const LOGO_KENDRYTE: &str = r#"
  _  __              _            _
 | |/ /___ _ __   __| |_ __ _   _| |_ ___
 | ' // _ \ '_ \ / _` | '__| | | | __/ _ \
 | . \  __/ | | | (_| | |  | |_| | ||  __/
 |_|\_\___|_| |_|\__,_|_|   \__, |\__\___|
                            |___/
      RISC-V by Kendryte
"#;

const LOGO_KENDRYTE_SMALL: &str = r#"
  Kendryte RISC-V
"#;

// =============================================================================
// Allwinner (China) - D1 chip
// =============================================================================

const LOGO_ALLWINNER: &str = r#"
     _    _ _         _
    / \  | | |_      _(_)_ __  _ __   ___ _ __
   / _ \ | | \ \ /\ / / | '_ \| '_ \ / _ \ '__|
  / ___ \| | |\ V  V /| | | | | | | |  __/ |
 /_/   \_\_|_| \_/\_/ |_|_| |_|_| |_|\___|_|

      RISC-V by Allwinner
"#;

const LOGO_ALLWINNER_SMALL: &str = r#"
  Allwinner RISC-V
"#;

// =============================================================================
// Espressif (China) - ESP32-C3, ESP32-C6
// =============================================================================

const LOGO_ESPRESSIF: &str = r#"
  _____                         _  __
 | ____|___ _ __  _ __ ___  ___(_)/ _|
 |  _| / __| '_ \| '__/ _ \/ __| | |_
 | |___\__ \ |_) | | |  __/\__ \ |  _|
 |_____|___/ .__/|_|  \___||___/_|_|
           |_|
      RISC-V by Espressif
"#;

const LOGO_ESPRESSIF_SMALL: &str = r#"
  Espressif RISC-V
"#;

// =============================================================================
// SpacemiT (China) - K1 chip (Orange Pi RV2, BananaPi BPI-F3)
// =============================================================================

const LOGO_SPACEMIT: &str = r#"
  ____                           _ _____
 / ___| _ __   __ _  ___ ___ _ __ (_)_   _|
 \___ \| '_ \ / _` |/ __/ _ \ '_ \| | | |
  ___) | |_) | (_| | (_|  __/ | | | | | |
 |____/| .__/ \__,_|\___\___|_| |_|_| |_|
       |_|
      RISC-V by SpacemiT
"#;

const LOGO_SPACEMIT_SMALL: &str = r#"
  SpacemiT RISC-V
"#;

// =============================================================================
// T-Head/Alibaba (China) - XuanTie C906, C910
// =============================================================================

const LOGO_THEAD: &str = r#"
  _____ _   _                _
 |_   _| | | | ___  __ _  __| |
   | | | |_| |/ _ \/ _` |/ _` |
   | | |  _  |  __/ (_| | (_| |
   |_| |_| |_|\___|\__,_|\__,_|

      RISC-V by T-Head
"#;

const LOGO_THEAD_SMALL: &str = r#"
  T-Head RISC-V
"#;

// =============================================================================
// Milk-V (China) - Milk-V Duo, Mars, Pioneer
// =============================================================================

const LOGO_MILKV: &str = r#"
  __  __ _ _ _     __     __
 |  \/  (_) | | __ \ \   / /
 | |\/| | | | |/ /  \ \ / /
 | |  | | | |   <    \ V /
 |_|  |_|_|_|_|\_\    \_/

      RISC-V by Milk-V
"#;

const LOGO_MILKV_SMALL: &str = r#"
  Milk-V RISC-V
"#;

// =============================================================================
// Sipeed (China) - Lichee, Maix series
// =============================================================================

const LOGO_SIPEED: &str = r#"
  ____  _                     _
 / ___|(_)_ __   ___  ___  __| |
 \___ \| | '_ \ / _ \/ _ \/ _` |
  ___) | | |_) |  __/  __/ (_| |
 |____/|_| .__/ \___|\___|\__,_|
         |_|
      RISC-V by Sipeed
"#;

const LOGO_SIPEED_SMALL: &str = r#"
  Sipeed RISC-V
"#;

// =============================================================================
// Sophgo (China) - CV1800B, SG2000
// =============================================================================

const LOGO_SOPHGO: &str = r#"
  ____              _
 / ___|  ___  _ __ | |__   __ _  ___
 \___ \ / _ \| '_ \| '_ \ / _` |/ _ \
  ___) | (_) | |_) | | | | (_| | (_) |
 |____/ \___/| .__/|_| |_|\__, |\___/
             |_|          |___/
      RISC-V by Sophgo
"#;

const LOGO_SOPHGO_SMALL: &str = r#"
  Sophgo RISC-V
"#;
