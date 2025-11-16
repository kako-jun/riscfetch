use std::process::Command;

#[test]
fn test_help_command() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("RISC-V architecture information display tool"));
    assert!(stdout.contains("--logo"));
    assert!(stdout.contains("--benchmark"));
    assert!(stdout.contains("--splash"));
}

#[test]
fn test_version_command() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--version"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("riscfetch"));
}

#[test]
fn test_non_riscv_detection() {
    // This test will fail on actual RISC-V systems
    // On non-RISC-V systems, it should show the "Sorry, not RISC-V" message
    let output = Command::new("cargo")
        .args(&["run", "--"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // On non-RISC-V, it should exit with error and show message
    // On RISC-V, it should succeed and show system info
    if !output.status.success() {
        assert!(stdout.contains("Sorry, not RISC-V"));
    }
}
