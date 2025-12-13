# riscfetch CLI Specification

This document defines the expected behavior of the riscfetch command-line tool.

## Overview

riscfetch is a RISC-V system information display tool. It only runs on RISC-V architecture and exits with an error on other systems.

## Command Line Interface

```
riscfetch [OPTIONS]
```

### Options

| Short | Long | Description |
|-------|------|-------------|
| `-e` | `--explain` | Show detailed explanation of each ISA extension |
| `-j` | `--json` | Output in JSON format |
| `-s` | `--splash` | Show animated splash screen |
| `-b` | `--benchmark` | Run ISA-specific benchmarks |
| `-l` | `--logo <STYLE>` | Logo style: default, sifive, kendryte |
| `-h` | `--help` | Show help message |
| `-V` | `--version` | Show version |

---

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Not running on RISC-V architecture |

---

## Behavior by Architecture

### On RISC-V Systems

- Display system information (see Output Format below)
- Exit with code 0

### On Non-RISC-V Systems

**Normal mode:**
```
Sorry, not RISC-V
```
- Exit with code 1

**JSON mode (`--json`):**
```json
{"error":"not_riscv","message":"This tool only runs on RISC-V architecture"}
```
- Exit with code 1

---

## Output Format (Normal Mode)

### Standard Output

```
ISA:    rv64imafdc_zicsr_zifencei_zba_zbb
Ext:    I M A F D C
Z-Ext:  zicsr zifencei zba zbb
Vector: Not detected
Harts:  4 harts
HW IDs: vendor:0x489 arch:0x8000000000000007 impl:0x0
Cache:  L1D:32K L1I:32K L2:2048K

--------------------------------

Board:  StarFive VisionFive 2
OS:     Ubuntu 24.04 LTS
Kernel: 6.8.0-riscv64
Memory: 3.45 GiB / 8.00 GiB
Uptime: 3h 42m
```

### Field Definitions

| Field | Description | Example |
|-------|-------------|---------|
| ISA | Full ISA string from /proc/cpuinfo | `rv64imafdc_zicsr_zifencei` |
| Ext | Standard extensions (space-separated) | `I M A F D C V` |
| Z-Ext | Z-extensions (space-separated) | `zicsr zifencei zba zbb` |
| Vector | Vector extension status and VLEN | `Enabled, VLEN>=256` or `Not detected` |
| Harts | Number of hardware threads | `4 harts` |
| HW IDs | Hardware identifiers | `vendor:0x489 arch:0x... impl:0x...` |
| Cache | Cache sizes | `L1D:32K L1I:32K L2:2048K` |
| Board | Device tree model name | `StarFive VisionFive 2` |
| OS | Operating system name and version | `Ubuntu 24.04 LTS` |
| Kernel | Kernel version | `6.8.0-riscv64` |
| Memory | Used / Total memory | `3.45 GiB / 8.00 GiB` |
| Uptime | System uptime | `3h 42m` |

### Separator

A line of dashes (`--------------------------------`) separates RISC-V specific info (above) from general system info (below).

---

## Output Format (--explain Mode)

Shows each extension with its description:

```
ISA Extensions:
  I   Base Integer Instructions
  M   Integer Multiply/Divide
  A   Atomic Instructions
  F   Single-Precision Float
  D   Double-Precision Float
  C   Compressed (16-bit)

Z-Extensions:
  Zicsr     CSR Instructions
  Zifencei  Instruction-Fetch Fence
  Zba       Address Generation
  Zbb       Basic Bit Manipulation
```

---

## Output Format (--json Mode)

### Success (on RISC-V)

```json
{
  "isa": "rv64imafdc_zicsr_zifencei",
  "extensions": ["I", "M", "A", "F", "D", "C"],
  "z_extensions": ["zicsr", "zifencei"],
  "vector": {
    "enabled": false,
    "vlen": null,
    "elen": null
  },
  "hart_count": 4,
  "hardware_ids": {
    "mvendorid": "0x489",
    "marchid": "0x8000000000000007",
    "mimpid": "0x0"
  },
  "cache": {
    "l1d": "32K",
    "l1i": "32K",
    "l2": "2048K",
    "l3": null
  },
  "board": "StarFive VisionFive 2",
  "memory_used_bytes": 3707764736,
  "memory_total_bytes": 8589934592,
  "kernel": "6.8.0-riscv64",
  "os": "Ubuntu 24.04 LTS",
  "uptime_seconds": 13320
}
```

### Error (on non-RISC-V)

```json
{
  "error": "not_riscv",
  "message": "This tool only runs on RISC-V architecture"
}
```

---

## Output Format (--benchmark Mode)

```
RISC-V ISA Benchmarks
=====================

Integer (I):      1234.56 MIPS
Multiply (M):     567.89 MIPS
Atomic (A):       45.67 Mops/s
Float-SP (F):     234.56 MFLOPS
Float-DP (D):     123.45 MFLOPS
Vector (V):       Not available

Total score: 1234
```

- Only benchmark extensions that are present
- Show "Not available" for missing extensions

---

## Output Format (--splash Mode)

- Display animated ASCII art logo
- Duration: approximately 2 seconds
- Then display normal output

---

## Logo Styles (--logo)

| Style | Description |
|-------|-------------|
| default | Standard RISC-V logo |
| sifive | SiFive style |
| kendryte | Kendryte style |

---

## Help Output (--help)

Must include:
- Program description: "RISC-V architecture information display tool"
- All options with short and long forms
- Brief description of each option

```
RISC-V architecture information display tool - Show off your RISC-V setup!

Usage: riscfetch [OPTIONS]

Options:
  -l, --logo <STYLE>  Logo style [default: default] [possible values: default, sifive, kendryte]
  -b, --benchmark     Run ISA-specific benchmarks
  -s, --splash        Show animated splash
  -e, --explain       Show detailed explanation of ISA extensions
  -j, --json          Output in JSON format
  -h, --help          Print help
  -V, --version       Print version
```

---

## Version Output (--version)

```
riscfetch <version>
```

Example: `riscfetch 0.2.0`

---

## Test Requirements

### Tests That Work on Any System

1. `--help` output contains expected strings
2. `--version` output contains program name
3. On non-RISC-V: exits with code 1
4. On non-RISC-V: shows appropriate error message
5. On non-RISC-V with `--json`: outputs valid JSON error

### Tests That Require RISC-V Hardware

1. Normal output format matches specification
2. `--json` outputs valid JSON with all required fields
3. `--explain` shows extension descriptions
4. Exit code is 0
5. All displayed values are accurate (match /proc/cpuinfo, etc.)

---

## Data Sources

| Field | Source |
|-------|--------|
| ISA | `/proc/cpuinfo` (isa line) |
| Hardware IDs | `/proc/cpuinfo` (mvendorid, marchid, mimpid) |
| Hart count | `/proc/cpuinfo` (count of processor entries) |
| Board | `/proc/device-tree/model` or `/sys/firmware/devicetree/base/model` |
| Cache | `/sys/devices/system/cpu/cpu0/cache/` |
| OS | `/etc/os-release` |
| Kernel | `uname -r` |
| Memory | sysinfo crate |
| Uptime | sysinfo crate |

---

## Version

- Spec version: 1.0
- Last updated: 2024-12
