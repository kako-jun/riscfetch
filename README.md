# riscfetch

System information tool for RISC-V. Displays ISA extensions, hart count, hardware IDs, and more.

**RISC-V only.** Exits on other architectures.

## Install

```bash
cargo install riscfetch
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
```

## Options

| Flag | Description |
|------|-------------|
| `-e, --explain` | Show meaning of each extension |
| `-j, --json` | Machine-readable JSON output |
| `-s, --splash` | Animated startup |
| `-b, --benchmark` | ISA-specific benchmarks |
| `-l, --logo <STYLE>` | Logo style: default, sifive, kendryte |

## Complements fastfetch

riscfetch shows RISC-V specific info. Use with fastfetch for full system details.

## License

MIT
