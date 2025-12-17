use std::process::Command;

#[test]
fn test_help_command() {
    let output = Command::new("cargo")
        .args(["run", "--", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("RISC-V architecture information display tool"));
    assert!(stdout.contains("--logo"));
    assert!(stdout.contains("--benchmark"));
    assert!(stdout.contains("--splash"));
    assert!(stdout.contains("--explain"));
    assert!(stdout.contains("--json"));
}

#[test]
fn test_version_command() {
    let output = Command::new("cargo")
        .args(["run", "--", "--version"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("riscfetch"));
}

#[test]
fn test_non_riscv_detection() {
    let output = Command::new("cargo")
        .args(["run", "--"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // On non-RISC-V, it should exit with error and show message
    // On RISC-V, it should succeed and show system info
    if !output.status.success() {
        assert!(stdout.contains("Sorry, not RISC-V"));
    }
}

#[test]
fn test_json_flag_non_riscv() {
    let output = Command::new("cargo")
        .args(["run", "--", "--json"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // On non-RISC-V, JSON error should be returned
    if !output.status.success() {
        assert!(stdout.contains(r#""error""#));
        assert!(stdout.contains(r#""not_riscv""#));
    }
}

#[test]
fn test_explain_flag_exists() {
    let output = Command::new("cargo")
        .args(["run", "--", "--help"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--explain"));
    assert!(stdout.contains("detailed explanation"));
}

#[test]
fn test_short_flags() {
    let output = Command::new("cargo")
        .args(["run", "--", "--help"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Check short flags exist
    assert!(stdout.contains("-e"));
    assert!(stdout.contains("-j"));
    assert!(stdout.contains("-s"));
    assert!(stdout.contains("-b"));
    assert!(stdout.contains("-l"));
    assert!(stdout.contains("-r"));
}

#[test]
fn test_riscv_only_flag_exists() {
    let output = Command::new("cargo")
        .args(["run", "--", "--help"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--riscv-only"));
    assert!(stdout.contains("RISC-V specific info"));
}

#[test]
fn test_riscv_only_json_non_riscv() {
    let output = Command::new("cargo")
        .args(["run", "--", "--json", "--riscv-only"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // On non-RISC-V, JSON error should be returned even with --riscv-only
    if !output.status.success() {
        assert!(stdout.contains(r#""error""#));
        assert!(stdout.contains(r#""not_riscv""#));
    }
}
