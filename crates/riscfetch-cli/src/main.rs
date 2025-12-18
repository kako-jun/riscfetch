mod benchmark;
mod cli;
mod display;
mod logos;
mod vendors;

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
        output_json(args.riscv_only, args.all);
        return;
    }

    if args.splash {
        display::show_splash_animation();
    }

    display_riscv_info(
        &args.logo,
        &args.style,
        args.explain,
        args.riscv_only,
        args.all,
    );

    if args.benchmark {
        println!();
        benchmark::run_benchmarks();
    }
}

fn output_json(riscv_only: bool, show_all: bool) {
    if show_all {
        output_json_all(riscv_only);
    } else if riscv_only {
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

fn output_json_all(riscv_only: bool) {
    use serde_json::json;

    let isa_string = info::get_isa_string();
    let all_std = info::get_all_standard_extensions_with_status(&isa_string);
    let all_z = info::get_all_z_extensions_with_status(&isa_string);
    let all_s = info::get_all_s_extensions_with_status(&isa_string);

    let std_json: Vec<_> = all_std
        .iter()
        .map(|(name, desc, supported)| {
            json!({
                "name": name,
                "description": desc,
                "supported": supported
            })
        })
        .collect();

    let z_json: Vec<_> = all_z
        .iter()
        .map(|e| {
            json!({
                "name": e.name,
                "description": e.description,
                "category": e.category,
                "supported": e.supported
            })
        })
        .collect();

    let s_json: Vec<_> = all_s
        .iter()
        .map(|e| {
            json!({
                "name": e.name,
                "description": e.description,
                "category": e.category,
                "supported": e.supported
            })
        })
        .collect();

    let hw_ids = info::get_hardware_ids();
    let vector_info = info::get_vector_detail();
    let cache_info = info::get_cache_info();

    let mut output = json!({
        "isa": isa_string,
        "extensions": std_json,
        "z_extensions": z_json,
        "s_extensions": s_json,
        "vector": vector_info,
        "hart_count": info::get_hart_count_num(),
        "hardware_ids": {
            "mvendorid": hw_ids.mvendorid,
            "marchid": hw_ids.marchid,
            "mimpid": hw_ids.mimpid
        },
        "cache": cache_info
    });

    if !riscv_only {
        let (mem_used, mem_total) = info::get_memory_bytes();
        output["board"] = json!(info::get_board_info());
        output["memory_used_bytes"] = json!(mem_used);
        output["memory_total_bytes"] = json!(mem_total);
        output["kernel"] = json!(info::get_kernel_info());
        output["os"] = json!(info::get_os_info());
        output["uptime_seconds"] = json!(info::get_uptime_seconds());
    }

    println!(
        "{}",
        serde_json::to_string_pretty(&output).unwrap_or_else(|_| "{}".to_string())
    );
}

fn display_riscv_info(vendor: &str, style: &str, explain: bool, riscv_only: bool, show_all: bool) {
    println!();
    display::display_logo(vendor, style);
    println!();

    // === RISC-V Specific Information ===
    let isa_string = info::get_isa_string();
    let vector_info = info::get_vector_detail();
    let hart_count = info::get_hart_count();
    let hw_ids = info::get_hardware_ids();
    let cache_info = info::get_cache_info();

    // ISA (base architecture)
    println!("{} {}", "ISA:".bright_cyan().bold(), isa_string.white());

    // Extensions
    if show_all {
        // Show ALL extensions with checkmarks
        let all_std = info::get_all_standard_extensions_with_status(&isa_string);
        let all_z = info::get_all_z_extensions_with_status(&isa_string);
        let all_s = info::get_all_s_extensions_with_status(&isa_string);

        if explain {
            display_all_extensions_explained(&all_std, &all_z, &all_s);
        } else {
            display_all_extensions_compact(&all_std, &all_z, &all_s);
        }
    } else {
        // Show only detected extensions
        let extensions_compact = info::get_extensions_compact();
        let z_exts_with_cat = info::get_z_extensions_with_category();
        let s_exts_with_cat = info::get_s_extensions_with_category();

        if explain {
            display_extensions_explained(&extensions_compact, &z_exts_with_cat, &s_exts_with_cat);
        } else {
            display_extensions_compact(&extensions_compact, &z_exts_with_cat, &s_exts_with_cat);
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

/// Format extension with checkmark based on support status
fn format_ext_with_check(name: &str, supported: bool) -> String {
    if supported {
        format!("{}{}", "✓".bright_green().bold(), name.bright_green())
    } else {
        format!("{}{}", "✗".bright_black(), name.bright_black())
    }
}

/// Display ALL extensions in compact mode with checkmarks
fn display_all_extensions_compact(
    std_exts: &[(String, String, bool)],
    z_exts: &[info::ExtensionInfo],
    s_exts: &[info::ExtensionInfo],
) {
    // Standard extensions with checkmarks
    let std_parts: Vec<String> = std_exts
        .iter()
        .map(|(name, _, supported)| format_ext_with_check(name, *supported))
        .collect();
    println!("{} {}", "Ext:".bright_yellow().bold(), std_parts.join(" "));

    // Z-extensions grouped by category
    let z_groups = info::group_by_category(z_exts);
    for (category, exts) in &z_groups {
        let cat_name = info::get_z_category_name(category);
        let ext_parts: Vec<String> = exts
            .iter()
            .map(|e| format_ext_with_check(&e.name, e.supported))
            .collect();
        println!(
            "{} {}",
            format!("Z-{cat_name}:").bright_yellow().bold(),
            ext_parts.join(" ")
        );
    }

    // S-extensions grouped by category
    let s_groups = info::group_by_category(s_exts);
    for (category, exts) in &s_groups {
        let cat_name = info::get_s_category_name(category);
        let ext_parts: Vec<String> = exts
            .iter()
            .map(|e| format_ext_with_check(&e.name, e.supported))
            .collect();
        println!(
            "{} {}",
            format!("S-{cat_name}:").bright_magenta().bold(),
            ext_parts.join(" ")
        );
    }
}

/// Display ALL extensions in explained mode with checkmarks
fn display_all_extensions_explained(
    std_exts: &[(String, String, bool)],
    z_exts: &[info::ExtensionInfo],
    s_exts: &[info::ExtensionInfo],
) {
    // Standard extensions
    println!("{}", "Extensions:".bright_yellow().bold());
    for (name, desc, supported) in std_exts {
        let mark = if *supported {
            "✓".bright_green().bold()
        } else {
            "✗".bright_black()
        };
        let name_colored = if *supported {
            name.bright_green()
        } else {
            name.bright_black()
        };
        let desc_colored = if *supported {
            desc.normal()
        } else {
            desc.bright_black()
        };
        println!(" {mark} {name_colored:<10} {desc_colored}");
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
            let mark = if ext.supported {
                "✓".bright_green().bold()
            } else {
                "✗".bright_black()
            };
            let name_colored = if ext.supported {
                ext.name.bright_green()
            } else {
                ext.name.bright_black()
            };
            let desc_colored = if ext.supported {
                ext.description.normal()
            } else {
                ext.description.bright_black()
            };
            println!(" {mark} {name_colored:<10} {desc_colored}");
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
            let mark = if ext.supported {
                "✓".bright_green().bold()
            } else {
                "✗".bright_black()
            };
            let name_colored = if ext.supported {
                ext.name.bright_green()
            } else {
                ext.name.bright_black()
            };
            let desc_colored = if ext.supported {
                ext.description.normal()
            } else {
                ext.description.bright_black()
            };
            println!(" {mark} {name_colored:<10} {desc_colored}");
        }
    }
}
