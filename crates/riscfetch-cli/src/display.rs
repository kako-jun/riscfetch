use crate::logos::{generate_logo, LogoStyle};
use colored::{Color, Colorize};

pub fn display_logo(vendor: &str, style: &str) {
    let logo_style = LogoStyle::from_str(style);

    let logo = generate_logo(vendor, logo_style);
    if logo.is_empty() {
        return;
    }

    // For default RISC-V logo, apply rainbow gradient
    if vendor == "default" || vendor == "riscv" || vendor == "risc-v" {
        let colors = [
            Color::BrightCyan,
            Color::BrightBlue,
            Color::BrightMagenta,
            Color::BrightRed,
            Color::BrightYellow,
            Color::BrightGreen,
        ];

        for (i, line) in logo.lines().enumerate() {
            if !line.is_empty() {
                let color_idx = i % colors.len();
                println!("{}", line.color(colors[color_idx]).bold());
            }
        }
    } else {
        println!("{}", logo.bright_cyan().bold());
    }
}
