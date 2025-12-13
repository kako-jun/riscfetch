use crate::logos::{get_vendor_logo, get_vendor_logo_small, LogoStyle, LogoVendor};
use colored::*;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

pub fn display_logo(vendor: &str, style: &str) {
    let vendor = LogoVendor::from_str(vendor);
    let style = LogoStyle::from_str(style);

    let logo = match style {
        LogoStyle::None => return,
        LogoStyle::Small => get_vendor_logo_small(vendor),
        LogoStyle::Normal => get_vendor_logo(vendor),
    };

    println!("{}", logo.bright_cyan().bold());
}

pub fn show_splash_animation() {
    // Clear screen and hide cursor
    print!("\x1B[2J\x1B[1;1H\x1B[?25l");
    io::stdout().flush().unwrap();

    let frames = get_animation_frames();
    let colors = [
        Color::BrightCyan,
        Color::BrightBlue,
        Color::BrightMagenta,
        Color::BrightRed,
        Color::BrightYellow,
        Color::BrightGreen,
    ];

    // Animation loop - 2 full rotations
    for cycle in 0..2 {
        for (i, frame) in frames.iter().enumerate() {
            // Move cursor to top
            print!("\x1B[1;1H");

            let color_idx = (i + cycle * frames.len()) % colors.len();

            // Print each line with color
            for line in frame.lines() {
                println!("{}", line.color(colors[color_idx]));
            }

            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(80));
        }
    }

    // Show final frame with gradient effect
    print!("\x1B[1;1H");
    let final_frame = &frames[0];
    let lines: Vec<&str> = final_frame.lines().collect();

    for (i, line) in lines.iter().enumerate() {
        let color_idx = i % colors.len();
        println!("{}", line.color(colors[color_idx]).bold());
    }

    io::stdout().flush().unwrap();
    thread::sleep(Duration::from_millis(500));

    // Show cursor again and clear
    print!("\x1B[?25h\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn get_animation_frames() -> Vec<String> {
    vec![
        // Frame 1 - Front view
        r#"
        ██████╗ ██╗███████╗ ██████╗      ██╗   ██╗
        ██╔══██╗██║██╔════╝██╔════╝      ██║   ██║
        ██████╔╝██║███████╗██║     █████╗██║   ██║
        ██╔══██╗██║╚════██║██║     ╚════╝╚██╗ ██╔╝
        ██║  ██║██║███████║╚██████╗       ╚████╔╝
        ╚═╝  ╚═╝╚═╝╚══════╝ ╚═════╝        ╚═══╝
"#
        .to_string(),
        // Frame 2 - Slight rotation
        r#"
        ███████╗ ██╗███████╗ ██████╗     ██╗   ██╗
        ██╔════╝ ██║██╔════╝██╔════╝     ██║   ██║
        ███████╗ ██║███████╗██║     ████╗██║   ██║
        ██╔═══██╗██║╚════██║██║     ╚═══╝ ██╗ ██╔╝
        ███████╔╝██║███████║╚██████╗       ╚███╔╝
        ╚══════╝ ╚═╝╚══════╝ ╚═════╝        ╚══╝
"#
        .to_string(),
        // Frame 3 - More rotation
        r#"
        ████████╗██╗███████╗ ██████╗     ██╗   ██╗
        ╚══██╔══╝██║██╔════╝██╔════╝     ██║   ██║
           ██║   ██║███████╗██║     ████╗██║   ██║
           ██║   ██║╚════██║██║     ╚═══╝ ██╗ ██╔╝
        ████████╗██║███████║╚██████╗       ╚███╔╝
        ╚═══════╝╚═╝╚══════╝ ╚═════╝        ╚══╝
"#
        .to_string(),
        // Frame 4 - Side view
        r#"
        ██████╗  ██╗ ██████╗  ██████╗    ██╗   ██╗
        ██╔══██╗ ██║██╔════╝ ██╔════╝    ██║   ██║
        ██████╔╝ ██║╚█████╗  ██║    ████╗██║   ██║
        ██╔══██╗ ██║ ╚═══██╗ ██║    ╚═══╝ ██╗ ██╔╝
        ██║  ██║ ██║██████╔╝ ╚██████╗     ╚███╔╝
        ╚═╝  ╚═╝ ╚═╝╚═════╝   ╚═════╝      ╚══╝
"#
        .to_string(),
        // Frame 5 - Continuing rotation
        r#"
        ██████╗  ██╗ ██████╗ ██████╗     ██╗   ██╗
        ██╔══██╗ ██║██╔════╝██╔════╝     ██║   ██║
        ██████╔╝ ██║╚█████╗ ██║     ████╗██║   ██║
        ██╔═══╝  ██║ ╚═══██╗██║     ╚═══╝ ██╗ ██╔╝
        ██║      ██║██████╔╝╚██████╗      ╚███╔╝
        ╚═╝      ╚═╝╚═════╝  ╚═════╝       ╚══╝
"#
        .to_string(),
        // Frame 6 - Almost back to front
        r#"
        ██████╗  ██╗ ██████╗ ██████╗     ██╗   ██╗
        ██╔══██╗ ██║██╔════╝██╔════╝     ██║   ██║
        ██████╔╝ ██║███████╗██║     ████╗██║   ██║
        ██╔══██╗ ██║╚════██║██║     ╚═══╝ ██╗ ██╔╝
        ██║  ██║ ██║███████║╚██████╗      ╚███╔╝
        ╚═╝  ╚═╝ ╚═╝╚══════╝ ╚═════╝       ╚══╝
"#
        .to_string(),
    ]
}
