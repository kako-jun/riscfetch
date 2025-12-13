mod benchmark;
mod display;
mod info;

use clap::Parser;
use colored::*;

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

    /// Show detailed explanation of each ISA extension
    #[arg(short, long)]
    explain: bool,

    /// Output in JSON format (machine-readable)
    #[arg(short, long)]
    json: bool,
}

fn main() {
    let args = Args::parse();

    if !info::is_riscv() {
        if args.json {
            println!(r#"{{"error": "not_riscv", "message": "This system is not RISC-V"}}"#);
        } else {
            println!("\n{}\n", "Sorry, not RISC-V".red().bold());
        }
        std::process::exit(1);
    }

    if args.json {
        output_json();
        return;
    }

    if args.splash {
        display::show_splash_animation();
    }

    display_riscv_info(&args.logo, args.explain);

    if args.benchmark {
        println!();
        benchmark::run_benchmarks();
    }
}

fn output_json() {
    let data = info::collect_all_info();
    println!(
        "{}",
        serde_json::to_string_pretty(&data).unwrap_or_else(|_| "{}".to_string())
    );
}

fn display_riscv_info(logo_style: &str, explain: bool) {
    println!();
    display::display_logo(logo_style);
    println!();

    // === RISC-V Specific Information ===
    let isa_string = info::get_isa_string();
    let extensions_compact = info::get_extensions_compact();
    let z_extensions = info::get_z_extensions();
    let vector_info = info::get_vector_detail();
    let hart_count = info::get_hart_count();
    let hw_ids = info::get_hardware_ids();
    let cache_info = info::get_cache_info();

    // ISA (base architecture)
    println!(
        "{} {}",
        "ISA:".bright_cyan().bold(),
        isa_string.white()
    );

    // Extensions
    if explain {
        println!("{}", "Extensions:".bright_yellow().bold());
        for (ext, desc) in info::get_extensions_explained() {
            println!("  {} - {}", ext.bright_green(), desc);
        }
        if !z_extensions.is_empty() {
            println!("{}", "Z-Extensions:".bright_yellow().bold());
            for (ext, desc) in info::get_z_extensions_explained() {
                println!("  {} - {}", ext.bright_green(), desc);
            }
        }
    } else {
        println!(
            "{} {}",
            "Ext:".bright_yellow().bold(),
            extensions_compact.white()
        );
        if !z_extensions.is_empty() {
            println!(
                "{} {}",
                "Z-Ext:".bright_yellow().bold(),
                z_extensions.white()
            );
        }
    }

    // Vector extension
    if !vector_info.is_empty() {
        println!(
            "{} {}",
            "Vector:".bright_magenta().bold(),
            vector_info.white()
        );
    }

    // Hart count
    println!(
        "{} {}",
        "Harts:".bright_cyan().bold(),
        hart_count.white()
    );

    // Hardware IDs (CSR values)
    if !hw_ids.mvendorid.is_empty() || !hw_ids.marchid.is_empty() || !hw_ids.mimpid.is_empty() {
        let mut ids = Vec::new();
        if !hw_ids.mvendorid.is_empty() {
            ids.push(format!("vendor:{}", hw_ids.mvendorid));
        }
        if !hw_ids.marchid.is_empty() {
            ids.push(format!("arch:{}", hw_ids.marchid));
        }
        if !hw_ids.mimpid.is_empty() {
            ids.push(format!("impl:{}", hw_ids.mimpid));
        }
        println!(
            "{} {}",
            "HW IDs:".bright_green().bold(),
            ids.join(" ").white()
        );
    }

    // Cache info
    if !cache_info.is_empty() {
        println!(
            "{} {}",
            "Cache:".bright_cyan().bold(),
            cache_info.white()
        );
    }

    // === Separator ===
    println!();
    println!("{}", "--------------------------------".bright_black());
    println!();

    // === General System Information ===
    let board_info = info::get_board_info();
    let os_info = info::get_os_info();
    let kernel_info = info::get_kernel_info();
    let memory_info = info::get_memory_info();
    let uptime = info::get_uptime();

    // Board/Model
    if !board_info.is_empty() {
        println!(
            "{} {}",
            "Board:".bright_blue().bold(),
            board_info.white()
        );
    }

    // OS
    println!(
        "{} {}",
        "OS:".bright_blue().bold(),
        os_info.white()
    );

    // Kernel
    println!(
        "{} {}",
        "Kernel:".bright_blue().bold(),
        kernel_info.white()
    );

    // Memory
    println!(
        "{} {}",
        "Memory:".bright_blue().bold(),
        memory_info.white()
    );

    // Uptime
    println!(
        "{} {}",
        "Uptime:".bright_blue().bold(),
        uptime.white()
    );

    println!();
}
