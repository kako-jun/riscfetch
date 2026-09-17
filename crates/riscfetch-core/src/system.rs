//! General system information (memory, uptime, kernel, OS)

use std::fs;
use std::process::Command;
use sysinfo::System;

/// Get memory usage as formatted string
#[must_use]
#[allow(clippy::cast_precision_loss)]
pub fn get_memory_info() -> String {
    let mut sys = System::new();
    sys.refresh_memory();

    let total_mem = sys.total_memory();
    let used_mem = sys.used_memory();

    let total_gb = total_mem as f64 / 1_073_741_824.0;
    let used_gb = used_mem as f64 / 1_073_741_824.0;

    format!("{used_gb:.2} GiB / {total_gb:.2} GiB")
}

/// Get memory information as bytes
#[must_use]
pub fn get_memory_bytes() -> (u64, u64) {
    let mut sys = System::new();
    sys.refresh_memory();
    (sys.used_memory(), sys.total_memory())
}

/// Get kernel version
#[must_use]
pub fn get_kernel_info() -> String {
    if let Ok(output) = Command::new("uname").arg("-r").output() {
        let kernel = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !kernel.is_empty() {
            return kernel;
        }
    }
    "Unknown".to_string()
}

/// Get OS name from /etc/os-release
#[must_use]
pub fn get_os_info() -> String {
    if let Ok(content) = fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            if line.starts_with("PRETTY_NAME=") {
                if let Some(name) = line.split('=').nth(1) {
                    return name.trim_matches('"').to_string();
                }
            }
        }
    }

    "Linux".to_string()
}

/// Get uptime as formatted string
#[must_use]
pub fn get_uptime() -> String {
    let uptime_secs = System::uptime();
    let hours = uptime_secs / 3600;
    let minutes = (uptime_secs % 3600) / 60;

    if hours > 0 {
        format!("{hours}h {minutes}m")
    } else {
        format!("{minutes}m")
    }
}

/// Get uptime in seconds
#[must_use]
pub fn get_uptime_seconds() -> u64 {
    System::uptime()
}

// uname is a child process and /etc/os-release describes the host, so these two
// readers reported whatever machine the suite happened to run on. The mocks
// below pin them to chosen values.
#[cfg(test)]
#[cfg(all(
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos", target_os = "windows")
))]
mod os_tests {
    use super::*;
    use shimforge::{mock, Session};
    use std::io;
    use std::process::Output;

    fn exit_ok() -> std::process::ExitStatus {
        #[cfg(unix)]
        use std::os::unix::process::ExitStatusExt;
        #[cfg(windows)]
        use std::os::windows::process::ExitStatusExt;
        std::process::ExitStatus::from_raw(0)
    }

    #[test]
    fn kernel_info_comes_from_uname() {
        let mut session = Session::new();
        // The uname binary is never launched; the call that would launch it is.
        let output = mock!(
            session,
            Command::output,
            fn(&mut Command) -> io::Result<Output>
        );
        output.expect().once().returning(|_| {
            Ok(Output {
                status: exit_ok(),
                stdout: b"6.6.63-riscv64\n".to_vec(),
                stderr: Vec::new(),
            })
        });
        assert_eq!(get_kernel_info(), "6.6.63-riscv64");
    }

    #[test]
    fn kernel_info_is_unknown_when_uname_says_nothing() {
        let mut session = Session::new();
        let output = mock!(
            session,
            Command::output,
            fn(&mut Command) -> io::Result<Output>
        );
        output.expect().once().returning(|_| {
            Ok(Output {
                status: exit_ok(),
                stdout: Vec::new(),
                stderr: Vec::new(),
            })
        });
        assert_eq!(get_kernel_info(), "Unknown");
    }

    #[test]
    fn os_info_reads_the_pretty_name() {
        let mut session = Session::new();
        let read = mock!(
            session,
            fs::read_to_string::<&str>,
            fn(&str) -> io::Result<String>
        );
        read.expect()
            .with(|path| *path == "/etc/os-release")
            .once()
            .returning(|_| {
                Ok(
                    "NAME=\"Debian GNU/Linux\"\nPRETTY_NAME=\"Debian GNU/Linux 12 (bookworm)\"\n"
                        .to_string(),
                )
            });
        assert_eq!(get_os_info(), "Debian GNU/Linux 12 (bookworm)");
    }

    #[test]
    fn os_info_falls_back_without_os_release() {
        let mut session = Session::new();
        let read = mock!(
            session,
            fs::read_to_string::<&str>,
            fn(&str) -> io::Result<String>
        );
        read.expect()
            .with(|path| *path == "/etc/os-release")
            .once()
            .returning(|_| Err(io::Error::from(io::ErrorKind::NotFound)));
        assert_eq!(get_os_info(), "Linux");
    }
}
