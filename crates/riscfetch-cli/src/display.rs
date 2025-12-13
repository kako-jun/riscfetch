use colored::*;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

pub fn display_logo(style: &str) {
    let logo = match style {
        "sifive" => get_sifive_logo(),
        "kendryte" => get_kendryte_logo(),
        _ => get_default_logo(),
    };

    println!("{}", logo.bright_cyan().bold());
}

fn get_default_logo() -> String {
    r#"
      ____  ____  ____   ____      __  __
     / __ \/_  _\/ ___\ / ___|    / / / /
    / /_/ / / /  \___ \/ /   ____/ / / /
   / _, _/ / /  /___/ / /___/___/ /_/ /
  /_/ |_| /_/  /_____/\____/    \____/

        RISC-V Architecture Info
"#
    .to_string()
}

fn get_sifive_logo() -> String {
    r#"
   _____ _ ______ _
  / ____(_)  ____(_)
 | (___  _| |__   ___   _____
  \___ \| |  __| | \ \ / / _ \
  ____) | | |    | |\ V /  __/
 |_____/|_|_|    |_| \_/ \___|

      RISC-V by SiFive
"#
    .to_string()
}

fn get_kendryte_logo() -> String {
    r#"
  _  __              _            _
 | |/ /___ _ __   __| |_ __ _   _| |_ ___
 | ' // _ \ '_ \ / _` | '__| | | | __/ _ \
 | . \  __/ | | | (_| | |  | |_| | ||  __/
 |_|\_\___|_| |_|\__,_|_|   \__, |\__\___|
                            |___/
      RISC-V by Kendryte
"#
    .to_string()
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
