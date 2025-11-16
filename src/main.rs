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
}

fn main() {
    let args = Args::parse();

    if !info::is_riscv() {
        println!("\n{}\n", "Sorry, not RISC-V 😢".red().bold());
        std::process::exit(1);
    }

    if args.splash {
        display::show_splash_animation();
    }

    display_riscv_info(&args.logo);

    if args.benchmark {
        println!();
        benchmark::run_benchmarks();
    }
}

fn display_riscv_info(logo_style: &str) {
    println!();
    display::display_logo(logo_style);
    println!();

    // Gather all RISC-V specific information
    let board_info = info::get_board_info();
    let cpu_info = info::get_cpu_info();
    let hart_count = info::get_hart_count();
    let soc_info = info::get_soc_info();
    let extensions = info::get_extensions();
    let vector_info = info::get_vector_info();
    let cache_info = info::get_cache_info();
    let memory_info = info::get_memory_info();
    let kernel_info = info::get_kernel_info();
    let os_info = info::get_os_info();
    let uptime = info::get_uptime();

    // Display RISC-V specific information (fastfetch style)
    if !board_info.is_empty() {
        println!(
            "{} {}",
            "🖥️  Board:".bright_blue().bold(),
            board_info.bright_white()
        );
    }
    println!(
        "{} {}",
        "🧠 CPU:".bright_cyan().bold(),
        cpu_info.bright_white()
    );
    println!(
        "{} {}",
        "⚙️  Harts:".bright_white().bold(),
        hart_count.bright_white()
    );
    println!(
        "{} {}",
        "🏗️  SoC:".bright_green().bold(),
        soc_info.bright_white()
    );
    println!(
        "{} {}",
        "🧪 ISA:".bright_yellow().bold(),
        extensions.bright_white()
    );

    if !vector_info.is_empty() {
        println!(
            "{} {}",
            "📐 Vector:".bright_magenta().bold(),
            vector_info.bright_white()
        );
    }

    if !cache_info.is_empty() {
        println!(
            "{} {}",
            "💾 Cache:".bright_cyan().bold(),
            cache_info.bright_white()
        );
    }

    println!(
        "{} {}",
        "🧮 Memory:".bright_red().bold(),
        memory_info.bright_white()
    );
    println!(
        "{} {}",
        "🐧 Kernel:".bright_green().bold(),
        kernel_info.bright_white()
    );
    println!(
        "{} {}",
        "🕹️  OS:".bright_magenta().bold(),
        os_info.bright_white()
    );
    println!(
        "{} {}",
        "🚀 Uptime:".bright_blue().bold(),
        uptime.bright_white()
    );
    println!();
}
