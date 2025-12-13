use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "riscfetch")]
#[command(author, version, about = "RISC-V architecture information display tool", long_about = None)]
pub struct Args {
    /// Vendor logo (default, sifive, starfive, kendryte, allwinner, espressif, spacemit, thead, milkv, sipeed, sophgo)
    #[arg(short, long, default_value = "default")]
    pub logo: String,

    /// Logo style (normal, small, none)
    #[arg(long, default_value = "normal")]
    pub style: String,

    /// Run simple benchmarks
    #[arg(short, long)]
    pub benchmark: bool,

    /// Show animated splash screen on startup
    #[arg(short, long)]
    pub splash: bool,

    /// Show detailed explanation of each ISA extension
    #[arg(short, long)]
    pub explain: bool,

    /// Output in JSON format (machine-readable)
    #[arg(short, long)]
    pub json: bool,
}
