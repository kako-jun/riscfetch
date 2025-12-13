# riscfetch

[![CI](https://github.com/kako-jun/riscfetch/actions/workflows/ci.yml/badge.svg)](https://github.com/kako-jun/riscfetch/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/riscfetch.svg)](https://crates.io/crates/riscfetch)
[![docs.rs](https://docs.rs/riscfetch-core/badge.svg)](https://docs.rs/riscfetch-core)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[English](README.md) | [中文](README.zh-CN.md) | [日本語](README.ja.md)

System information tool for RISC-V. Displays ISA extensions, hart count, hardware IDs, and more.

**RISC-V only.** Exits on other architectures.

## Why RISC-V?

I enjoy anime, movies, and food from all over the world. Just a fan of cool tech and the future. RISC-V is fun to tinker with, and I wanted a neofetch-style tool for it.

## Install

### From crates.io

```bash
cargo install riscfetch
```

### From GitHub Releases

```bash
# Download the latest release
curl -LO https://github.com/kako-jun/riscfetch/releases/latest/download/riscfetch-linux-riscv64

# Make it executable
chmod +x riscfetch-linux-riscv64

# Move to PATH
sudo mv riscfetch-linux-riscv64 /usr/local/bin/riscfetch
```

## Usage

```bash
riscfetch              # standard output
riscfetch -e           # explain each ISA extension
riscfetch -j           # JSON output
riscfetch -s           # animated splash
riscfetch -b           # run benchmarks
```

## Output

```
ISA:    rv64imafdcv_zicsr_zifencei_zba_zbb_zbs
Ext:    I M A F D C V
Z-Ext:  zicsr zifencei zba zbb zbs
Vector: Enabled, VLEN>=128
Harts:  4 harts
HW IDs: vendor:0x489 arch:0x8000000000000007 impl:0x0
Cache:  L1D:32K L1I:32K L2:2048K

--------------------------------

Board:  StarFive VisionFive 2
OS:     Ubuntu 24.04 LTS
Kernel: 6.8.0-riscv64
Memory: 3.45 GiB / 8.00 GiB
Uptime: 3h 42m
User:   user@visionfive2
```

## Options

| Flag | Description |
|------|-------------|
| `-e, --explain` | Show meaning of each extension |
| `-j, --json` | Machine-readable JSON output |
| `-s, --splash` | Animated startup |
| `-b, --benchmark` | ISA-specific benchmarks |
| `-l, --logo <VENDOR>` | Vendor logo (see below) |
| `--style <STYLE>` | Logo style: normal, small, none |

### Supported Vendors

| Vendor | Description |
|--------|-------------|
| `default` | Generic RISC-V logo |
| `sifive` | SiFive (HiFive Unmatched, Unleashed) |
| `starfive` | StarFive (VisionFive 2) |
| `kendryte` | Kendryte/Canaan (K210, K510) |
| `allwinner` | Allwinner (D1) |
| `espressif` | Espressif (ESP32-C3, C6) |
| `spacemit` | SpacemiT (K1, Orange Pi RV2) |
| `thead` | T-Head/Alibaba (XuanTie C906, C910) |
| `milkv` | Milk-V (Duo, Mars, Pioneer) |
| `sipeed` | Sipeed (Lichee, Maix series) |
| `sophgo` | Sophgo (CV1800B, SG2000) |

## Complements fastfetch

riscfetch shows RISC-V specific info. Use with fastfetch for full system details.

## Contributing

Issues and pull requests are welcome!

- Bug reports
- Feature requests
- Support for new RISC-V boards
- Documentation improvements

### Testing Help Wanted

We have limited hardware for testing. If you can test on any of the following, please report your results (working or not):

- **RV32E** (embedded with 16 registers) - e.g., ESP32-C3, CH32V003
- **Non-Vector CPUs** - e.g., VisionFive 2, Allwinner D1
- **Different VLEN values** - VLEN=128, 512, 1024, etc.
- **Exotic Z-extensions** - Zk (crypto), Zcmp, Zacas, etc.

Even "it works" reports are valuable! Please open an issue with your `/proc/cpuinfo` and riscfetch output.

## License

MIT
