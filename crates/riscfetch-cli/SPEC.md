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
| `-a` | `--all` | Show all extensions with checkmarks for supported ones |
| `-s` | `--splash` | Show animated splash screen |
| `-b` | `--benchmark` | Run ISA-specific benchmarks |
| `-l` | `--logo <VENDOR>` | Vendor logo (see Supported Vendors below) |
| | `--style <STYLE>` | Logo style: normal, small, none |
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

Extensions are grouped by category for better readability:

```
ISA:        rv64imafdcv_zicbom_zicboz_zicntr_zicsr_zifencei_...
Ext:        I M A F D C V
Z-Base:     Zicsr Zifencei Zicntr Zihpm
Z-Bit:      Zba Zbb Zbc Zbs
Z-Cache:    Zicbom Zicboz
Z-Vector:   Zvl128b Zvl256b
S-Sup:      Sstc
Vector:     Enabled, VLEN>=256
Harts:      8 harts
HW IDs:     vendor:0x710 arch:0x8000000000000007 impl:0x0
Cache:      L1D:32K L1I:32K L2:512K

--------------------------------

Board:      SpacemiT K1
OS:         Ubuntu 24.04 LTS
Kernel:     6.1.15-riscv64
Memory:     3.45 GiB / 8.00 GiB
Uptime:     3h 42m
User:       user@spacemit
```

### Field Definitions

| Field | Description | Example |
|-------|-------------|---------|
| ISA | Full ISA string from /proc/cpuinfo | `rv64imafdc_zicsr_zifencei` |
| Ext | Standard extensions (space-separated) | `I M A F D C V` |
| Z-{Category}: | Z-extensions grouped by category | `Z-Bit: Zba Zbb Zbc Zbs` |
| S-{Category}: | S-extensions (privileged) by category | `S-Sup: Sstc` |
| Vector | Vector extension status and VLEN | `Enabled, VLEN>=256` or empty |
| Harts | Number of hardware threads | `4 harts` |
| HW IDs | Hardware identifiers | `vendor:0x489 arch:0x... impl:0x...` |
| Cache | Cache sizes | `L1D:32K L1I:32K L2:2048K` |
| Board | Device tree model name | `SpacemiT K1` |
| OS | Operating system name and version | `Ubuntu 24.04 LTS` |
| Kernel | Kernel version | `6.8.0-riscv64` |
| Memory | Used / Total memory | `3.45 GiB / 8.00 GiB` |
| Uptime | System uptime | `3h 42m` |
| User | Username and hostname | `user@hostname` |

### Extension Categories

**Z-Extension Categories:**
| Category | Display Name | Examples |
|----------|-------------|----------|
| base | Base | Zicsr, Zifencei, Zicntr, Zihpm |
| hint | Hints | Zihintpause, Zihintntl |
| cache | Cache | Zicbom, Zicboz, Zicbop |
| bit | Bit Manipulation | Zba, Zbb, Zbc, Zbs |
| crypto | Cryptography | Zk, Zkn, Zknd, Zkne, ... |
| fp | Floating Point | Zfh, Zfhmin, Zfa, Zfinx, ... |
| comp | Compressed | Zca, Zcb, Zcd, Zcf, ... |
| atomic | Atomics | Zacas, Zabha, Zaamo, ... |
| mem | Memory Model | Za64rs, Za128rs, Ztso, ... |
| vec | Vector | Zve32x, Zvl128b, Zvl256b, ... |
| vcrypto | Vector Crypto | Zvbb, Zvbc, Zvkg, ... |

**S-Extension Categories:**
| Category | Display Name | Examples |
|----------|-------------|----------|
| vm | Virtual Memory | Svinval, Svnapot, Svpbmt, ... |
| sup | Supervisor | Ssaia, Sstc, Ssstateen, ... |
| mach | Machine | Smaia, Smepmp, Smstateen, ... |
| hyp | Hypervisor | Sha, Shgatpa, ... |
| debug | Debug | Sdext, Sdtrig |
| user | User | Supm |

### Separator

A line of dashes (`--------------------------------`) separates RISC-V specific info (above) from general system info (below).

---

## Output Format (--explain Mode)

Shows each extension with its description, grouped by category with aligned columns:

```
Extensions:
  I          Base Integer Instructions
  M          Integer Multiply/Divide
  A          Atomic Instructions
  F          Single-Precision Float
  D          Double-Precision Float
  C          Compressed (16-bit)
  V          Vector (SIMD)

Z-Extensions (Base):
  Zicsr      CSR Instructions
  Zifencei   Instruction-Fetch Fence
  Zicntr     Base Counters/Timers
  Zihpm      Hardware Perf Counters

Z-Extensions (Bit Manipulation):
  Zba        Address Generation
  Zbb        Basic Bit Manipulation
  Zbc        Carry-less Multiply
  Zbs        Single-bit Operations

Z-Extensions (Cryptography):
  Zkt        Data-Indep Timing

Z-Extensions (Vector):
  Zvl128b    VLEN >= 128 bits
  Zvl256b    VLEN >= 256 bits
  Zvkt       Vector Data-Indep Time

S-Extensions (Supervisor):
  Sstc       Supervisor Timer
```

---

## Output Format (--all Mode)

Shows ALL defined extensions (144 total) with checkmarks indicating support status:

- `✓` (green, bold) = Supported by this CPU
- `✗` (gray/dim) = Not supported

### Compact mode (`-a`)

```
Ext:        ✓I ✗E ✓M ✓A ✓F ✓D ✗Q ✓C ✗B ✗V ✗H
Z-Base:     ✓Zicsr ✓Zifencei ✗Zicntr ✗Zihpm
Z-Bit:      ✓Zba ✓Zbb ✗Zbc ✓Zbs
Z-Cache:    ✗Zicbom ✗Zicboz ✗Zicbop
...
```

### Explained mode (`-a -e`)

```
Extensions:
 ✓ I          Base Integer Instructions
 ✗ E          Embedded (16 registers)
 ✓ M          Integer Multiply/Divide
 ...

Z-Extensions (Base):
 ✓ Zicsr      CSR Instructions
 ✓ Zifencei   Instruction-Fetch Fence
 ✗ Zicntr     Base Counters/Timers
 ...
```

The `--all` flag can be combined with other options (`-e`, `-l`, `-j`, `-r`, etc.).

---

## Output Format (--json Mode)

### Success (on RISC-V)

```json
{
  "isa": "rv64imafdcv_zicsr_zifencei_zba_zbb_sstc",
  "extensions": ["I", "M", "A", "F", "D", "C", "V"],
  "z_extensions": ["Zicsr", "Zifencei", "Zba", "Zbb"],
  "s_extensions": ["Sstc"],
  "vector": {
    "enabled": true,
    "vlen": null,
    "elen": null
  },
  "hart_count": 8,
  "hardware_ids": {
    "mvendorid": "0x710",
    "marchid": "0x8000000000000007",
    "mimpid": "0x0"
  },
  "cache": {
    "l1d": "32K",
    "l1i": "32K",
    "l2": "512K",
    "l3": null
  },
  "board": "SpacemiT K1",
  "memory_used_bytes": 3707764736,
  "memory_total_bytes": 8589934592,
  "kernel": "6.1.15-riscv64",
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

## Output Format (--all --json Mode)

When `--all` is combined with `--json`, extensions include ALL defined extensions with a `supported` field:

```json
{
  "isa": "rv64imafdc_zicsr_zifencei_zba_zbb",
  "extensions": [
    {"name": "I", "description": "Base Integer Instructions", "supported": true},
    {"name": "E", "description": "Embedded (16 registers)", "supported": false},
    {"name": "M", "description": "Integer Multiply/Divide", "supported": true},
    ...
  ],
  "z_extensions": [
    {"name": "Zicsr", "description": "CSR Instructions", "category": "base", "supported": true},
    {"name": "Zifencei", "description": "Instruction-Fetch Fence", "category": "base", "supported": true},
    {"name": "Zicntr", "description": "Base Counters/Timers", "category": "base", "supported": false},
    ...
  ],
  "s_extensions": [
    {"name": "Sstc", "description": "Supervisor Timer", "category": "sup", "supported": true},
    {"name": "Svinval", "description": "Fine-Grained Invalidation", "category": "vm", "supported": false},
    ...
  ],
  "vector": "Enabled, VLEN>=256",
  "hart_count": 8,
  "hardware_ids": {...},
  "cache": "L1D:32K L1I:32K L2:512K",
  "board": "SpacemiT K1",
  "memory_used_bytes": 3707764736,
  "memory_total_bytes": 8589934592,
  "kernel": "6.1.15-riscv64",
  "os": "Ubuntu 24.04 LTS",
  "uptime_seconds": 13320
}
```

With `--riscv-only` (`-a -r -j`), system fields (board, memory, kernel, os, uptime) are omitted.

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

## Vendor Logos (--logo)

| Vendor | Description |
|--------|-------------|
| default | Generic RISC-V logo |
| sifive | SiFive (HiFive Unmatched, Unleashed) |
| starfive | StarFive (VisionFive 2) |
| thead | T-Head/Alibaba (XuanTie C906, C910) |
| milkv | Milk-V (Duo, Mars, Pioneer) |
| sipeed | Sipeed (Lichee, Maix series) |
| pine64 | Pine64 (Star64, Oz64) |
| kendryte | Kendryte/Canaan (K210, K510) |
| allwinner | Allwinner (D1) |
| espressif | Espressif (ESP32-C3, C6) |
| spacemit | SpacemiT (K1, Orange Pi RV2) |
| sophgo | Sophgo (CV1800B, SG2000) |
| wch | WCH (CH32V003, CH32V103) |

## Logo Styles (--style)

| Style | Description |
|-------|-------------|
| normal | Full ASCII art logo (default) |
| small | Compact one-line logo |
| none | No logo, data only |

---

## Help Output (--help)

Must include:
- Program description: "RISC-V architecture information display tool"
- All options with short and long forms
- Brief description of each option

```
RISC-V architecture information display tool

Usage: riscfetch [OPTIONS]

Options:
  -l, --logo <VENDOR>   Vendor logo (default, sifive, starfive, thead, milkv, sipeed, pine64, kendryte, allwinner, espressif, spacemit, sophgo, wch)
      --style <STYLE>   Logo style (normal, small, none)
  -b, --benchmark       Run simple benchmarks
  -s, --splash          Show animated splash screen on startup
  -e, --explain         Show detailed explanation of each ISA extension
  -j, --json            Output in JSON format (machine-readable)
  -a, --all             Show all extensions with checkmarks for supported ones
  -h, --help            Print help
  -V, --version         Print version
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

- Spec version: 2.0
- Last updated: 2025-12
- Based on RISC-V ISA spec version: 2025-11-26
- Supports 98 Z-extensions and 46 S-extensions (144 total)
