use clap::CommandFactory;
use clap_complete::{generate_to, Shell};
use clap_mangen::Man;
use std::env;
use std::fs;
use std::path::PathBuf;

include!("src/cli.rs");

fn main() {
    let out_dir = match env::var_os("OUT_DIR") {
        Some(dir) => PathBuf::from(dir),
        None => return,
    };

    // Generate shell completions
    let completions_dir = out_dir.join("completions");
    fs::create_dir_all(&completions_dir).unwrap();

    let mut cmd = Args::command();
    for shell in [Shell::Bash, Shell::Zsh, Shell::Fish, Shell::PowerShell] {
        generate_to(shell, &mut cmd, "riscfetch", &completions_dir).unwrap();
    }

    // Generate man page
    let man_dir = out_dir.join("man");
    fs::create_dir_all(&man_dir).unwrap();

    let cmd = Args::command();
    let man = Man::new(cmd);
    let mut buffer = Vec::new();
    man.render(&mut buffer).unwrap();
    fs::write(man_dir.join("riscfetch.1"), buffer).unwrap();

    println!("cargo:rerun-if-changed=src/cli.rs");
}
