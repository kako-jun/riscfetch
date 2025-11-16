use clap::Parser;
use colored::*;
use std::fs;
use std::io::{self, Write};
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};
use sysinfo::System;

#[derive(Parser, Debug)]
#[command(name = "riscfetch")]
#[command(author, version, about = "RISC-V architecture information display tool", long_about = None)]
struct Args {
    /// Logo style (default, sifive, kendryte)
    #[arg(short, long, default_value = "default")]
    logo: String,

    /// Run simple benchmarks
    #[arg(short, long)]
    benchmark: bool,

    /// Show animated splash screen on startup
    #[arg(short, long)]
    splash: bool,
}

fn main() {
    let args = Args::parse();

    if !is_riscv() {
        println!("\n{}\n", "Sorry, not RISC-V 😢".red().bold());
        std::process::exit(1);
    }

    if args.splash {
        show_splash_animation();
    }

    display_riscv_info(&args.logo);

    if args.benchmark {
        println!();
        run_benchmarks();
    }
}

fn is_riscv() -> bool {
    // Check architecture via uname -m
    if let Ok(output) = Command::new("uname").arg("-m").output() {
        let arch = String::from_utf8_lossy(&output.stdout);
        if arch.contains("riscv") {
            return true;
        }
    }

    // Fallback: check /proc/cpuinfo
    if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
        if content.contains("riscv") || content.contains("RISC-V") {
            return true;
        }
    }

    false
}

fn display_riscv_info(logo_style: &str) {
    println!();
    display_logo(logo_style);
    println!();

    // Gather all RISC-V specific information
    let board_info = get_board_info();
    let cpu_info = get_cpu_info();
    let hart_count = get_hart_count();
    let soc_info = get_soc_info();
    let extensions = get_extensions();
    let vector_info = get_vector_info();
    let cache_info = get_cache_info();
    let memory_info = get_memory_info();
    let kernel_info = get_kernel_info();
    let os_info = get_os_info();
    let uptime = get_uptime();

    // Display RISC-V specific information (fastfetch style)
    if !board_info.is_empty() {
        println!("{} {}", "🖥️  Board:".bright_blue().bold(), board_info.bright_white());
    }
    println!("{} {}", "🧠 CPU:".bright_cyan().bold(), cpu_info.bright_white());
    println!("{} {}", "⚙️  Harts:".bright_white().bold(), hart_count.bright_white());
    println!("{} {}", "🏗️  SoC:".bright_green().bold(), soc_info.bright_white());
    println!("{} {}", "🧪 ISA:".bright_yellow().bold(), extensions.bright_white());

    if !vector_info.is_empty() {
        println!("{} {}", "📐 Vector:".bright_magenta().bold(), vector_info.bright_white());
    }

    if !cache_info.is_empty() {
        println!("{} {}", "💾 Cache:".bright_cyan().bold(), cache_info.bright_white());
    }

    println!("{} {}", "🧮 Memory:".bright_red().bold(), memory_info.bright_white());
    println!("{} {}", "🐧 Kernel:".bright_green().bold(), kernel_info.bright_white());
    println!("{} {}", "🕹️  OS:".bright_magenta().bold(), os_info.bright_white());
    println!("{} {}", "🚀 Uptime:".bright_blue().bold(), uptime.bright_white());
    println!();
}

fn display_logo(style: &str) {
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
"#.to_string()
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
"#.to_string()
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
"#.to_string()
}

fn get_cpu_info() -> String {
    if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
        // Try to find ISA info
        for line in content.lines() {
            if line.starts_with("isa") {
                if let Some(isa) = line.split(':').nth(1) {
                    let isa = isa.trim();
                    // Detect RV32 or RV64
                    if isa.starts_with("rv64") {
                        return format!("RV64{}", extract_extensions(isa));
                    } else if isa.starts_with("rv32") {
                        return format!("RV32{}", extract_extensions(isa));
                    }
                    return isa.to_uppercase();
                }
            }
        }

        // Fallback: check model name
        for line in content.lines() {
            if line.starts_with("model name") || line.starts_with("uarch") {
                if let Some(model) = line.split(':').nth(1) {
                    return model.trim().to_string();
                }
            }
        }
    }

    "Unknown RISC-V CPU".to_string()
}

fn extract_extensions(isa: &str) -> String {
    // Extract standard extensions (letters after rv32/rv64)
    let isa_lower = isa.to_lowercase();
    if let Some(base_pos) = isa_lower.find("rv") {
        let after_base = &isa_lower[base_pos + 4..]; // Skip "rv64" or "rv32"
        let extensions: String = after_base
            .chars()
            .take_while(|c| c.is_alphabetic())
            .collect::<String>()
            .to_uppercase();
        return extensions;
    }
    "".to_string()
}

fn get_soc_info() -> String {
    if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
        for line in content.lines() {
            if line.starts_with("mmu") || line.starts_with("mvendorid") {
                if let Some(value) = line.split(':').nth(1) {
                    let value = value.trim();
                    if !value.is_empty() && value != "0x0" {
                        return format!("Vendor ID: {}", value);
                    }
                }
            }
        }
    }

    // Try device tree for SoC info
    if let Ok(content) = fs::read_to_string("/proc/device-tree/compatible") {
        let parts: Vec<&str> = content.split('\0').collect();
        if let Some(first) = parts.first() {
            if !first.is_empty() {
                return first.to_string();
            }
        }
    }

    "Unknown SoC".to_string()
}

fn get_extensions() -> String {
    if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
        for line in content.lines() {
            if line.starts_with("isa") {
                if let Some(isa) = line.split(':').nth(1) {
                    let isa = isa.trim().to_lowercase();
                    let mut exts = Vec::new();

                    // Check for standard extensions
                    if isa.contains('m') { exts.push("M (Multiply)"); }
                    if isa.contains('a') { exts.push("A (Atomic)"); }
                    if isa.contains('f') { exts.push("F (Float)"); }
                    if isa.contains('d') { exts.push("D (Double)"); }
                    if isa.contains('c') { exts.push("C (Compressed)"); }
                    if isa.contains('v') { exts.push("V (Vector)"); }

                    // Check for Z extensions
                    if isa.contains("zicsr") { exts.push("Zicsr"); }
                    if isa.contains("zifencei") { exts.push("Zifencei"); }
                    if isa.contains("zba") { exts.push("Zba"); }
                    if isa.contains("zbb") { exts.push("Zbb"); }

                    if !exts.is_empty() {
                        return exts.join(", ");
                    }

                    return extract_extensions(&isa).chars()
                        .map(|c| c.to_string())
                        .collect::<Vec<_>>()
                        .join(", ");
                }
            }
        }
    }

    "Standard RISC-V".to_string()
}

fn get_os_info() -> String {
    // Get OS name only
    if let Ok(content) = fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            if line.starts_with("PRETTY_NAME=") {
                if let Some(name) = line.split('=').nth(1) {
                    return name.trim_matches('"').to_string();
                }
            }
        }
    }

    "Linux".to_string()
}

fn get_kernel_info() -> String {
    if let Ok(output) = Command::new("uname").arg("-r").output() {
        let kernel = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !kernel.is_empty() {
            return kernel;
        }
    }
    "Unknown".to_string()
}

fn get_uptime() -> String {
    let uptime_secs = System::uptime();
    let hours = uptime_secs / 3600;
    let minutes = (uptime_secs % 3600) / 60;

    if hours > 0 {
        format!("{}h {}m", hours, minutes)
    } else {
        format!("{}m", minutes)
    }
}

fn get_board_info() -> String {
    // Try to get board name from device tree
    if let Ok(content) = fs::read_to_string("/proc/device-tree/model") {
        let model = content.trim_matches('\0').trim();
        if !model.is_empty() {
            return model.to_string();
        }
    }

    // Fallback to compatible string
    if let Ok(content) = fs::read_to_string("/proc/device-tree/compatible") {
        let parts: Vec<&str> = content.split('\0').collect();
        if let Some(first) = parts.first() {
            if !first.is_empty() {
                // Extract readable board name
                if first.contains("starfive") {
                    if first.contains("visionfive2") || first.contains("visionfive-2") {
                        return "StarFive VisionFive 2".to_string();
                    }
                    return "StarFive Board".to_string();
                } else if first.contains("sifive") {
                    if first.contains("unmatched") {
                        return "SiFive HiFive Unmatched".to_string();
                    } else if first.contains("unleashed") {
                        return "SiFive HiFive Unleashed".to_string();
                    }
                    return "SiFive Board".to_string();
                } else if first.contains("milkv") || first.contains("milk-v") {
                    if first.contains("mars") {
                        return "Milk-V Mars".to_string();
                    } else if first.contains("pioneer") {
                        return "Milk-V Pioneer".to_string();
                    }
                    return "Milk-V Board".to_string();
                } else if first.contains("thead") {
                    return "T-Head Board".to_string();
                }
            }
        }
    }

    String::new()
}

fn get_hart_count() -> String {
    // Count number of processor entries in /proc/cpuinfo
    if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
        let count = content.lines()
            .filter(|line| line.starts_with("processor"))
            .count();
        if count > 0 {
            return format!("{} hart{}", count, if count > 1 { "s" } else { "" });
        }
    }

    // Fallback: use sysinfo
    let mut sys = System::new();
    sys.refresh_cpu_all();
    let count = sys.cpus().len();
    format!("{} hart{}", count, if count > 1 { "s" } else { "" })
}

fn get_vector_info() -> String {
    if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
        for line in content.lines() {
            if line.starts_with("isa") {
                if let Some(isa) = line.split(':').nth(1) {
                    let isa = isa.trim().to_lowercase();
                    if isa.contains('v') || isa.contains("vector") {
                        // Try to get VLEN from device tree or sysfs
                        // Default: just indicate vector is present
                        return "Enabled (V extension)".to_string();
                    }
                }
            }
        }
    }
    String::new()
}

fn get_cache_info() -> String {
    let mut cache_parts = Vec::new();

    // Try to get cache info from /sys/devices/system/cpu/cpu0/cache/
    if let Ok(l1d_size) = fs::read_to_string("/sys/devices/system/cpu/cpu0/cache/index0/size") {
        let size = l1d_size.trim();
        if !size.is_empty() {
            cache_parts.push(format!("L1D: {}", size));
        }
    }

    if let Ok(l1i_size) = fs::read_to_string("/sys/devices/system/cpu/cpu0/cache/index1/size") {
        let size = l1i_size.trim();
        if !size.is_empty() {
            cache_parts.push(format!("L1I: {}", size));
        }
    }

    if let Ok(l2_size) = fs::read_to_string("/sys/devices/system/cpu/cpu0/cache/index2/size") {
        let size = l2_size.trim();
        if !size.is_empty() {
            cache_parts.push(format!("L2: {}", size));
        }
    }

    if cache_parts.is_empty() {
        String::new()
    } else {
        cache_parts.join(", ")
    }
}

fn get_memory_info() -> String {
    let mut sys = System::new();
    sys.refresh_memory();

    let total_mem = sys.total_memory();
    let used_mem = sys.used_memory();

    let total_gb = total_mem as f64 / 1_073_741_824.0; // Convert KB to GB
    let used_gb = used_mem as f64 / 1_073_741_824.0;

    format!("{:.2} GiB / {:.2} GiB", used_gb, total_gb)
}

fn run_benchmarks() {
    println!("{}", "⚡ Running RISC-V Benchmarks...".bright_yellow().bold());
    println!();

    // Integer multiplication benchmark (M extension)
    let int_score = benchmark_integer_ops();
    println!("{} {} {}",
        "🔢 Integer Ops (M):".bright_cyan().bold(),
        format!("{:.2}", int_score).bright_white(),
        "MOPS".bright_white().dimmed()
    );

    // Floating-point benchmark (F/D extension)
    let float_score = benchmark_float_ops();
    println!("{} {} {}",
        "🎯 Float Ops (F/D):".bright_green().bold(),
        format!("{:.2}", float_score).bright_white(),
        "MFLOPS".bright_white().dimmed()
    );

    // Memory bandwidth benchmark
    let mem_score = benchmark_memory();
    println!("{} {} {}",
        "💾 Memory Bandwidth:".bright_magenta().bold(),
        format!("{:.2}", mem_score).bright_white(),
        "MB/s".bright_white().dimmed()
    );

    println!();
    println!("{}", "✨ Benchmarks complete!".bright_yellow().bold());
    println!();
}

fn benchmark_integer_ops() -> f64 {
    const ITERATIONS: u64 = 10_000_000;
    let start = Instant::now();

    let mut result: u64 = 1;
    for i in 1..ITERATIONS {
        result = result.wrapping_mul(i).wrapping_add(i);
    }

    let elapsed = start.elapsed();
    let ops_per_sec = (ITERATIONS as f64 / elapsed.as_secs_f64()) / 1_000_000.0;

    // Use result to prevent optimization
    if result == 0 { println!(""); }

    ops_per_sec
}

fn benchmark_float_ops() -> f64 {
    const ITERATIONS: u64 = 5_000_000;
    let start = Instant::now();

    let mut result: f64 = 1.0;
    for i in 1..ITERATIONS {
        let x = i as f64;
        result = (result * x).sqrt() + x.sin();
    }

    let elapsed = start.elapsed();
    let ops_per_sec = (ITERATIONS as f64 * 2.0 / elapsed.as_secs_f64()) / 1_000_000.0;

    // Use result to prevent optimization
    if result == 0.0 { println!(""); }

    ops_per_sec
}

fn benchmark_memory() -> f64 {
    const SIZE: usize = 10_000_000;
    let mut data = vec![0u8; SIZE];

    let start = Instant::now();

    // Write benchmark
    for i in 0..SIZE {
        data[i] = (i & 0xFF) as u8;
    }

    // Read benchmark
    let mut sum: u64 = 0;
    for &byte in &data {
        sum = sum.wrapping_add(byte as u64);
    }

    let elapsed = start.elapsed();
    let mb_per_sec = (SIZE as f64 * 2.0 / elapsed.as_secs_f64()) / 1_000_000.0;

    // Use sum to prevent optimization
    if sum == 0 { println!(""); }

    mb_per_sec
}

fn show_splash_animation() {
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
"#.to_string(),

        // Frame 2 - Slight rotation
        r#"
        ███████╗ ██╗███████╗ ██████╗     ██╗   ██╗
        ██╔════╝ ██║██╔════╝██╔════╝     ██║   ██║
        ███████╗ ██║███████╗██║     ████╗██║   ██║
        ██╔═══██╗██║╚════██║██║     ╚═══╝ ██╗ ██╔╝
        ███████╔╝██║███████║╚██████╗       ╚███╔╝
        ╚══════╝ ╚═╝╚══════╝ ╚═════╝        ╚══╝
"#.to_string(),

        // Frame 3 - More rotation
        r#"
        ████████╗██╗███████╗ ██████╗     ██╗   ██╗
        ╚══██╔══╝██║██╔════╝██╔════╝     ██║   ██║
           ██║   ██║███████╗██║     ████╗██║   ██║
           ██║   ██║╚════██║██║     ╚═══╝ ██╗ ██╔╝
        ████████╗██║███████║╚██████╗       ╚███╔╝
        ╚═══════╝╚═╝╚══════╝ ╚═════╝        ╚══╝
"#.to_string(),

        // Frame 4 - Side view
        r#"
        ██████╗  ██╗ ██████╗  ██████╗    ██╗   ██╗
        ██╔══██╗ ██║██╔════╝ ██╔════╝    ██║   ██║
        ██████╔╝ ██║╚█████╗  ██║    ████╗██║   ██║
        ██╔══██╗ ██║ ╚═══██╗ ██║    ╚═══╝ ██╗ ██╔╝
        ██║  ██║ ██║██████╔╝ ╚██████╗     ╚███╔╝
        ╚═╝  ╚═╝ ╚═╝╚═════╝   ╚═════╝      ╚══╝
"#.to_string(),

        // Frame 5 - Continuing rotation
        r#"
        ██████╗  ██╗ ██████╗ ██████╗     ██╗   ██╗
        ██╔══██╗ ██║██╔════╝██╔════╝     ██║   ██║
        ██████╔╝ ██║╚█████╗ ██║     ████╗██║   ██║
        ██╔═══╝  ██║ ╚═══██╗██║     ╚═══╝ ██╗ ██╔╝
        ██║      ██║██████╔╝╚██████╗      ╚███╔╝
        ╚═╝      ╚═╝╚═════╝  ╚═════╝       ╚══╝
"#.to_string(),

        // Frame 6 - Almost back to front
        r#"
        ██████╗  ██╗ ██████╗ ██████╗     ██╗   ██╗
        ██╔══██╗ ██║██╔════╝██╔════╝     ██║   ██║
        ██████╔╝ ██║███████╗██║     ████╗██║   ██║
        ██╔══██╗ ██║╚════██║██║     ╚═══╝ ██╗ ██╔╝
        ██║  ██║ ██║███████║╚██████╗      ╚███╔╝
        ╚═╝  ╚═╝ ╚═╝╚══════╝ ╚═════╝       ╚══╝
"#.to_string(),
    ]
}
