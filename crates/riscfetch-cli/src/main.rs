mod benchmark;
mod cli;
mod display;
mod logos;

use clap::Parser;
use cli::Args;
use colored::Colorize;
use riscfetch_core as info;

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
        output_json(args.riscv_only);
        return;
    }

    if args.splash {
        display::show_splash_animation();
    }

    display_riscv_info(&args.logo, &args.style, args.explain, args.riscv_only);

    if args.benchmark {
        println!();
        benchmark::run_benchmarks();
    }
}

fn output_json(riscv_only: bool) {
    if riscv_only {
        let data = info::collect_riscv_info();
        println!(
            "{}",
            serde_json::to_string_pretty(&data).unwrap_or_else(|_| "{}".to_string())
        );
    } else {
        let data = info::collect_all_info();
        println!(
            "{}",
            serde_json::to_string_pretty(&data).unwrap_or_else(|_| "{}".to_string())
        );
    }
}

fn display_riscv_info(vendor: &str, style: &str, explain: bool, riscv_only: bool) {
    println!();
    display::display_logo(vendor, style);
    println!();

    // === RISC-V Specific Information ===
    let isa_string = info::get_isa_string();
    let extensions_compact = info::get_extensions_compact();
    let z_exts_with_cat = info::get_z_extensions_with_category();
    let s_exts_with_cat = info::get_s_extensions_with_category();
    let vector_info = info::get_vector_detail();
    let hart_count = info::get_hart_count();
    let hw_ids = info::get_hardware_ids();
    let cache_info = info::get_cache_info();

    // ISA (base architecture)
    println!("{} {}", "ISA:".bright_cyan().bold(), isa_string.white());

    // Extensions
    if explain {
        // Detailed mode with category groups and aligned columns
        display_extensions_explained(&extensions_compact, &z_exts_with_cat, &s_exts_with_cat);
    } else {
        // Compact mode with category groups
        display_extensions_compact(&extensions_compact, &z_exts_with_cat, &s_exts_with_cat);
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
    println!("{} {}", "Harts:".bright_cyan().bold(), hart_count.white());

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
        println!("{} {}", "Cache:".bright_cyan().bold(), cache_info.white());
    }

    // Skip general system info if --riscv-only flag is set
    if riscv_only {
        println!();
        return;
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
        println!("{} {}", "Board:".bright_blue().bold(), board_info.white());
    }

    // OS
    println!("{} {}", "OS:".bright_blue().bold(), os_info.white());

    // Kernel
    println!("{} {}", "Kernel:".bright_blue().bold(), kernel_info.white());

    // Memory
    println!("{} {}", "Memory:".bright_blue().bold(), memory_info.white());

    // Uptime
    println!("{} {}", "Uptime:".bright_blue().bold(), uptime.white());

    // User@Hostname
    let user = std::env::var("USER").unwrap_or_else(|_| "unknown".to_string());
    let hostname = gethostname::gethostname().to_string_lossy().to_string();
    println!(
        "{} {}@{}",
        "User:".bright_blue().bold(),
        user.white(),
        hostname.white()
    );

    println!();
}

/// Display extensions in compact mode (category-grouped multiple lines)
fn display_extensions_compact(
    std_exts: &str,
    z_exts: &[info::ExtensionInfo],
    s_exts: &[info::ExtensionInfo],
) {
    // Standard extensions
    if !std_exts.is_empty() {
        println!("{} {}", "Ext:".bright_yellow().bold(), std_exts.white());
    }

    // Z-extensions grouped by category
    let z_groups = info::group_by_category(z_exts);
    for (category, exts) in &z_groups {
        let cat_name = info::get_z_category_name(category);
        let ext_names: Vec<&str> = exts.iter().map(|e| e.name.as_str()).collect();
        println!(
            "{} {}",
            format!("Z-{cat_name}:").bright_yellow().bold(),
            ext_names.join(" ").white()
        );
    }

    // S-extensions grouped by category
    let s_groups = info::group_by_category(s_exts);
    for (category, exts) in &s_groups {
        let cat_name = info::get_s_category_name(category);
        let ext_names: Vec<&str> = exts.iter().map(|e| e.name.as_str()).collect();
        println!(
            "{} {}",
            format!("S-{cat_name}:").bright_magenta().bold(),
            ext_names.join(" ").white()
        );
    }
}

/// Display extensions in explained mode (category-grouped with aligned columns)
fn display_extensions_explained(
    _std_exts: &str,
    z_exts: &[info::ExtensionInfo],
    s_exts: &[info::ExtensionInfo],
) {
    // Standard extensions
    println!("{}", "Extensions:".bright_yellow().bold());
    for (ext, desc) in info::get_extensions_explained() {
        println!("  {:<10} {}", ext.bright_green(), desc);
    }

    // Z-extensions grouped by category
    let z_groups = info::group_by_category(z_exts);
    for (category, exts) in &z_groups {
        let cat_name = info::get_z_category_name(category);
        println!();
        println!(
            "{}",
            format!("Z-Extensions ({cat_name}):").bright_yellow().bold()
        );
        for ext in exts {
            println!("  {:<10} {}", ext.name.bright_green(), ext.description);
        }
    }

    // S-extensions grouped by category
    let s_groups = info::group_by_category(s_exts);
    for (category, exts) in &s_groups {
        let cat_name = info::get_s_category_name(category);
        println!();
        println!(
            "{}",
            format!("S-Extensions ({cat_name}):")
                .bright_magenta()
                .bold()
        );
        for ext in exts {
            println!("  {:<10} {}", ext.name.bright_green(), ext.description);
        }
    }
}
