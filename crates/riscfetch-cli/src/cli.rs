use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "riscfetch")]
#[command(author, version, about = "RISC-V architecture information display tool", long_about = None)]
#[allow(clippy::struct_excessive_bools)]
pub struct Args {
    /// Vendor logo (default, sifive, starfive, thead, milkv, sipeed, pine64, kendryte, allwinner, espressif, spacemit, sophgo, wch)
    #[arg(short, long, default_value = "default")]
    pub logo: String,

    /// Logo style (normal, small, none)
    #[arg(long, default_value = "normal")]
    pub style: String,

    /// Run simple benchmarks
    #[arg(short, long)]
    pub benchmark: bool,

    /// Show detailed explanation of each ISA extension
    #[arg(short, long)]
    pub explain: bool,

    /// Output in JSON format (machine-readable)
    #[arg(short, long)]
    pub json: bool,

    /// Show only RISC-V specific info (exclude generic system info like OS, memory, uptime)
    #[arg(short, long)]
    pub riscv_only: bool,

    /// Show all extensions with checkmarks for supported ones
    #[arg(short, long)]
    pub all: bool,
}
