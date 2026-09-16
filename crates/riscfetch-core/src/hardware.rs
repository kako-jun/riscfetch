//! Hardware information reading from /proc and /sys

use crate::parsing::parse_vector_from_isa;
use crate::types::HardwareIds;
use std::fmt::Write;
use std::fs;
use sysinfo::System;

/// Get raw ISA string (e.g., `rv64imafdcv_zicsr_...`)
#[must_use]
pub fn get_isa_string() -> String {
    if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
        for line in content.lines() {
            if line.starts_with("isa") {
                if let Some(isa) = line.split(':').nth(1) {
                    return isa.trim().to_string();
                }
            }
        }
    }
    "unknown".to_string()
}

/// Get hardware IDs (mvendorid, marchid, mimpid)
#[must_use]
pub fn get_hardware_ids() -> HardwareIds {
    let mut ids = HardwareIds::default();

    if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
        for line in content.lines() {
            if line.starts_with("mvendorid") {
                if let Some(val) = line.split(':').nth(1) {
                    let val = val.trim();
                    if !val.is_empty() && val != "0x0" {
                        ids.mvendorid = val.to_string();
                    }
                }
            } else if line.starts_with("marchid") {
                if let Some(val) = line.split(':').nth(1) {
                    let val = val.trim();
                    if !val.is_empty() && val != "0x0" {
                        ids.marchid = val.to_string();
                    }
                }
            } else if line.starts_with("mimpid") {
                if let Some(val) = line.split(':').nth(1) {
                    let val = val.trim();
                    if !val.is_empty() && val != "0x0" {
                        ids.mimpid = val.to_string();
                    }
                }
            }
        }
    }

    ids
}

/// Get hart count as formatted string
#[must_use]
pub fn get_hart_count() -> String {
    if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
        let count = content
            .lines()
            .filter(|line| line.starts_with("processor"))
            .count();
        if count > 0 {
            return format!("{count} hart{}", if count > 1 { "s" } else { "" });
        }
    }

    let mut sys = System::new();
    sys.refresh_cpu_all();
    let count = sys.cpus().len();
    format!("{count} hart{}", if count > 1 { "s" } else { "" })
}

/// Get hart count as number
#[must_use]
pub fn get_hart_count_num() -> usize {
    if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
        let count = content
            .lines()
            .filter(|line| line.starts_with("processor"))
            .count();
        if count > 0 {
            return count;
        }
    }

    let mut sys = System::new();
    sys.refresh_cpu_all();
    sys.cpus().len()
}

/// Get cache information
#[must_use]
pub fn get_cache_info() -> String {
    let mut cache_parts = Vec::new();

    if let Ok(l1d_size) = fs::read_to_string("/sys/devices/system/cpu/cpu0/cache/index0/size") {
        let size = l1d_size.trim();
        if !size.is_empty() {
            cache_parts.push(format!("L1D:{size}"));
        }
    }

    if let Ok(l1i_size) = fs::read_to_string("/sys/devices/system/cpu/cpu0/cache/index1/size") {
        let size = l1i_size.trim();
        if !size.is_empty() {
            cache_parts.push(format!("L1I:{size}"));
        }
    }

    if let Ok(l2_size) = fs::read_to_string("/sys/devices/system/cpu/cpu0/cache/index2/size") {
        let size = l2_size.trim();
        if !size.is_empty() {
            cache_parts.push(format!("L2:{size}"));
        }
    }

    if let Ok(l3_size) = fs::read_to_string("/sys/devices/system/cpu/cpu0/cache/index3/size") {
        let size = l3_size.trim();
        if !size.is_empty() {
            cache_parts.push(format!("L3:{size}"));
        }
    }

    cache_parts.join(" ")
}

/// Get board/model information from device tree
#[must_use]
pub fn get_board_info() -> String {
    if let Ok(content) = fs::read_to_string("/proc/device-tree/model") {
        let model = content.trim_matches('\0').trim();
        if !model.is_empty() {
            return model.to_string();
        }
    }

    if let Ok(content) = fs::read_to_string("/proc/device-tree/compatible") {
        let parts: Vec<&str> = content.split('\0').collect();
        if let Some(&first) = parts.first() {
            if !first.is_empty() {
                return first.to_string();
            }
        }
    }

    String::new()
}

/// Get vector extension details (VLEN, ELEN)
#[must_use]
pub fn get_vector_detail() -> String {
    let isa = get_isa_string();
    let mut result = parse_vector_from_isa(&isa).unwrap_or_default();

    // Try to get actual VLEN from sysfs
    if !result.is_empty() {
        if let Ok(vlen) = fs::read_to_string("/sys/devices/system/cpu/cpu0/riscv/vlen") {
            let _ = write!(result, ", VLEN={}", vlen.trim());
        }
    }

    result
}

// These readers only return real data on a RISC-V board, so they have never had
// tests. Mocking fs::read_to_string per thread feeds them genuine board
// contents on any machine, leaving the production code untouched.
#[cfg(test)]
#[cfg(all(
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos", target_os = "windows")
))]
mod os_tests {
    use super::*;
    use shimforge::{mock, Session};
    use std::io;

    /// /proc/cpuinfo as reported by a SpacemiT K1 (Banana Pi F3, Orange Pi RV2).
    const K1_CPUINFO: &str = "processor\t: 0
hart\t\t: 0
isa\t\t: rv64imafdcv_zicsr_zifencei_zvl256b
mvendorid\t: 0x710
marchid\t\t: 0x8000000058000001
mimpid\t\t: 0x1000000049772200

processor\t: 1
hart\t\t: 1
isa\t\t: rv64imafdcv_zicsr_zifencei_zvl256b
";

    /// Every rule for one target has to live on a single mock handle: each
    /// `mock!` site binds once per session, so this is called once per test and
    /// adds one rule per file. The first matching rule wins.
    fn read_files(session: &mut Session, files: Vec<(&'static str, io::Result<String>)>) {
        let read = mock!(
            session,
            fs::read_to_string::<&str>,
            fn(&str) -> io::Result<String>
        );
        for (path, content) in files {
            let mut content = Some(content);
            read.expect()
                .with(move |requested| *requested == path)
                .once()
                .returning(move |_| content.take().expect("one call per rule"));
        }
    }

    fn missing() -> io::Result<String> {
        Err(io::Error::from(io::ErrorKind::NotFound))
    }

    #[test]
    fn isa_string_is_read_from_cpuinfo() {
        let mut session = Session::new();
        read_files(
            &mut session,
            vec![("/proc/cpuinfo", Ok(K1_CPUINFO.to_string()))],
        );
        assert_eq!(get_isa_string(), "rv64imafdcv_zicsr_zifencei_zvl256b");
    }

    #[test]
    fn isa_string_is_unknown_without_cpuinfo() {
        let mut session = Session::new();
        read_files(&mut session, vec![("/proc/cpuinfo", missing())]);
        assert_eq!(get_isa_string(), "unknown");
    }

    #[test]
    fn hardware_ids_are_read_from_cpuinfo() {
        let mut session = Session::new();
        read_files(
            &mut session,
            vec![("/proc/cpuinfo", Ok(K1_CPUINFO.to_string()))],
        );
        let ids = get_hardware_ids();
        assert_eq!(ids.mvendorid, "0x710");
        assert_eq!(ids.marchid, "0x8000000058000001");
        assert_eq!(ids.mimpid, "0x1000000049772200");
    }

    #[test]
    fn zeroed_hardware_ids_are_left_empty() {
        let mut session = Session::new();
        let cpuinfo = "processor\t: 0\nmvendorid\t: 0x0\nmarchid\t\t: 0x0\nmimpid\t\t: 0x0\n";
        read_files(
            &mut session,
            vec![("/proc/cpuinfo", Ok(cpuinfo.to_string()))],
        );
        let ids = get_hardware_ids();
        assert_eq!(ids.mvendorid, "");
        assert_eq!(ids.marchid, "");
        assert_eq!(ids.mimpid, "");
    }

    #[test]
    fn several_harts_are_pluralised() {
        let mut session = Session::new();
        read_files(
            &mut session,
            vec![("/proc/cpuinfo", Ok(K1_CPUINFO.to_string()))],
        );
        assert_eq!(get_hart_count(), "2 harts");
    }

    #[test]
    fn a_single_hart_is_not_pluralised() {
        let mut session = Session::new();
        read_files(
            &mut session,
            vec![("/proc/cpuinfo", Ok("processor\t: 0\n".to_string()))],
        );
        assert_eq!(get_hart_count(), "1 hart");
    }

    #[test]
    fn hart_count_is_also_available_as_a_number() {
        let mut session = Session::new();
        read_files(
            &mut session,
            vec![("/proc/cpuinfo", Ok(K1_CPUINFO.to_string()))],
        );
        assert_eq!(get_hart_count_num(), 2);
    }

    #[test]
    fn cache_levels_that_are_absent_are_skipped() {
        let mut session = Session::new();
        read_files(
            &mut session,
            vec![
                (
                    "/sys/devices/system/cpu/cpu0/cache/index0/size",
                    Ok("32K\n".to_string()),
                ),
                (
                    "/sys/devices/system/cpu/cpu0/cache/index1/size",
                    Ok("32K\n".to_string()),
                ),
                (
                    "/sys/devices/system/cpu/cpu0/cache/index2/size",
                    Ok("512K\n".to_string()),
                ),
                // A board with no L3: the file is simply not there.
                ("/sys/devices/system/cpu/cpu0/cache/index3/size", missing()),
            ],
        );
        assert_eq!(get_cache_info(), "L1D:32K L1I:32K L2:512K");
    }

    #[test]
    fn board_info_prefers_the_device_tree_model() {
        let mut session = Session::new();
        // Device tree strings carry a trailing NUL.
        read_files(
            &mut session,
            vec![("/proc/device-tree/model", Ok("Orange Pi RV2\0".to_string()))],
        );
        assert_eq!(get_board_info(), "Orange Pi RV2");
    }

    #[test]
    fn board_info_falls_back_to_the_compatible_node() {
        let mut session = Session::new();
        read_files(
            &mut session,
            vec![
                ("/proc/device-tree/model", missing()),
                (
                    "/proc/device-tree/compatible",
                    Ok("spacemit,k1\0spacemit,k1-x\0".to_string()),
                ),
            ],
        );
        assert_eq!(get_board_info(), "spacemit,k1");
    }

    #[test]
    fn vector_detail_adds_the_vlen_reported_by_sysfs() {
        let mut session = Session::new();
        read_files(
            &mut session,
            vec![
                ("/proc/cpuinfo", Ok(K1_CPUINFO.to_string())),
                (
                    "/sys/devices/system/cpu/cpu0/riscv/vlen",
                    Ok("256\n".to_string()),
                ),
            ],
        );
        let detail = get_vector_detail();
        assert!(detail.contains("Enabled"), "{detail}");
        assert!(detail.contains("VLEN=256"), "{detail}");
    }
}
