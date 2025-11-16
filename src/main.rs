use clap::Parser;
use colored::*;
use std::fs;
use std::process::Command;
use std::time::Instant;
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
}

fn main() {
    let args = Args::parse();

    if !is_riscv() {
        println!("\n{}\n", "Sorry, not RISC-V 😢".red().bold());
        std::process::exit(1);
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

    let cpu_info = get_cpu_info();
    let soc_info = get_soc_info();
    let extensions = get_extensions();
    let os_info = get_os_info();
    let uptime = get_uptime();

    println!("{} {}", "🧠 CPU:".bright_cyan().bold(), cpu_info.bright_white());
    println!("{} {}", "🏗️  SoC:".bright_green().bold(), soc_info.bright_white());
    println!("{} {}", "🧪 Extensions:".bright_yellow().bold(), extensions.bright_white());
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
    let mut os_parts = Vec::new();

    // Get OS name
    if let Ok(content) = fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            if line.starts_with("PRETTY_NAME=") {
                if let Some(name) = line.split('=').nth(1) {
                    os_parts.push(name.trim_matches('"').to_string());
                    break;
                }
            }
        }
    }

    // Get kernel version
    if let Ok(output) = Command::new("uname").arg("-r").output() {
        let kernel = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !kernel.is_empty() {
            os_parts.push(kernel);
        }
    }

    if os_parts.is_empty() {
        "Linux".to_string()
    } else {
        os_parts.join(" - ")
    }
}

fn get_uptime() -> String {
    let mut sys = System::new_all();
    sys.refresh_all();

    let uptime_secs = System::uptime();
    let hours = uptime_secs / 3600;
    let minutes = (uptime_secs % 3600) / 60;

    if hours > 0 {
        format!("{}h {}m", hours, minutes)
    } else {
        format!("{}m", minutes)
    }
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
