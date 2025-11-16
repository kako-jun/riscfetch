# 🚀 riscfetch

**RISC-V Specialized System Information Tool**

A fast, beautiful system information tool exclusively for RISC-V architectures. Inspired by **fastfetch** but focused entirely on **RISC-V specific information** that general-purpose tools don't show.

## 💡 Why riscfetch?

Unlike neofetch/fastfetch which show generic Linux info, `riscfetch` is laser-focused on **RISC-V architecture details**:

- 🔍 **Hart Count** - RISC-V specific hardware thread information
- 🧬 **ISA Extensions** - Detailed breakdown (M, A, F, D, C, V, Zicsr, Zifencei, Zba, Zbb, etc.)
- 🖥️ **Board Detection** - Recognizes VisionFive 2, SiFive Unmatched, Milk-V boards
- 📐 **Vector Extension** - V extension detection and details
- 💾 **Cache Hierarchy** - L1/L2 cache information
- 🏗️ **SoC Details** - Device tree compatible strings

**Use riscfetch alongside fastfetch** - let fastfetch handle generic Linux info, use riscfetch to show off your RISC-V hardware!

## ✨ Features

- 🎯 **RISC-V Exclusive**: Only displays information on RISC-V systems
- 🎨 **Beautiful ASCII Art**: Multiple logo styles (default, SiFive, Kendryte)
- ✨ **Animated Splash Screen**: Eye-catching rotating block animation (like modern AI coding agents!)
- 📊 **Comprehensive RISC-V Info**: Everything specific to RISC-V architecture
- 🌈 **Colorful Output**: Terminal-friendly colored display inspired by fastfetch
- ⚡ **Benchmarks**: Optional ISA-specific performance testing
- 🚀 **Fast**: Written in Rust for minimal overhead

## 📦 Installation

### From Source

```bash
git clone https://github.com/kako-jun/riscfetch.git
cd riscfetch
cargo build --release
sudo cp target/release/riscfetch /usr/local/bin/
```

### Using Cargo

```bash
cargo install riscfetch
```

## 🎮 Usage

### Basic Usage

Simply run the command:

```bash
riscfetch
```

### With Animated Splash Screen

Show a cool rotating block animation (inspired by modern AI coding agents):

```bash
riscfetch --splash
```

The splash screen features a colorful RISC-V logo that rotates with smooth color transitions!

### With Logo Options

Choose different logo styles:

```bash
# Default RISC-V logo
riscfetch --logo default

# SiFive logo
riscfetch --logo sifive

# Kendryte logo
riscfetch --logo kendryte
```

### With Benchmarks

Run ISA-specific performance benchmarks:

```bash
riscfetch --benchmark
```

### Combine Options

```bash
# Full experience with splash, custom logo, and benchmarks
riscfetch --splash --logo sifive --benchmark
```

## 📸 Example Output

```
      ____  ____  ____   ____      __  __
     / __ \/_  _\/ ___\ / ___|    / / / /
    / /_/ / / /  \___ \/ /   ____/ / / /
   / _, _/ / /  /___/ / /___/___/ /_/ /
  /_/ |_| /_/  /_____/\____/    \____/

        RISC-V Architecture Info

🖥️  Board: StarFive VisionFive 2
🧠 CPU: RV64IMAFDC
⚙️  Harts: 4 harts
🏗️  SoC: starfive,jh7110
🧪 ISA: M (Multiply), A (Atomic), F (Float), D (Double), C (Compressed)
📐 Vector: Enabled (V extension)
💾 Cache: L1D: 32K, L1I: 32K, L2: 2048K
🧮 Memory: 3.45 GiB / 8.00 GiB
🐧 Kernel: 6.5.0-riscv64
🕹️  OS: Debian GNU/Linux 12 (bookworm)
🚀 Uptime: 3h 42m
```

### On Non-RISC-V Systems

```bash
$ riscfetch

Sorry, not RISC-V 😢
```

## 🔧 RISC-V Specific Information Detected

### Hardware Information
- **Hart Count** - `/proc/cpuinfo` processor entries (RISC-V hardware threads)
- **Board Model** - `/proc/device-tree/model` with automatic recognition for:
  - StarFive VisionFive 2
  - SiFive HiFive Unmatched/Unleashed
  - Milk-V Mars/Pioneer
  - T-Head boards
- **SoC Details** - Device tree compatible strings
- **Cache Hierarchy** - `/sys/devices/system/cpu/cpu0/cache/` (L1D, L1I, L2)

### ISA Information
- **Base ISA** - RV32I or RV64I detection
- **Standard Extensions**:
  - M: Integer Multiplication and Division
  - A: Atomic Instructions
  - F: Single-Precision Floating-Point
  - D: Double-Precision Floating-Point
  - C: Compressed Instructions
  - V: Vector Operations
- **Z Extensions**: Zicsr, Zifencei, Zba, Zbb, Zbc, Zbs, etc.
- **Vector Details** - V extension presence and configuration

### System Information
- **Memory** - Real-time usage via sysinfo (important for memory-constrained RISC-V boards)
- **Kernel** - RISC-V kernel version
- **OS** - Distribution name
- **Uptime** - System uptime

## 🆚 riscfetch vs fastfetch

| Feature | fastfetch | riscfetch |
|---------|-----------|-----------|
| Speed | ⚡ Blazing fast | 🚀 Fast (Rust) |
| Scope | All Linux systems | RISC-V only |
| Hart detection | Shows CPU count | Shows "harts" (RISC-V term) |
| ISA extensions | ❌ | ✅ Detailed breakdown |
| Board detection | Generic | RISC-V boards (VisionFive, etc.) |
| Vector info | ❌ | ✅ V extension details |
| Cache info | ✅ Generic | ✅ RISC-V specific |
| Use together? | ✅ YES! | ✅ Complementary tools |

**Recommendation**: Use both! Run `fastfetch` for general Linux info, then `riscfetch` to showcase RISC-V specific features.

## 🛣️ Roadmap

- [x] Hart count detection
- [x] ISA extension breakdown
- [x] Board recognition (VisionFive, Unmatched, Milk-V)
- [x] Vector extension detection
- [x] Cache information
- [x] Animated splash screen
- [x] ISA-specific benchmarks
- [ ] `--screenshot` - Generate image for social media sharing
- [ ] `--json` - JSON output for scripting
- [ ] More board-specific logos and detection
- [ ] VLEN/ELEN detection for vector extensions
- [ ] Privilege level detection
- [ ] Performance counter integration

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📝 License

MIT License - See LICENSE file for details

## 🙏 Acknowledgments

- Inspired by [fastfetch](https://github.com/fastfetch-cli/fastfetch) - The modern, actively maintained system info tool
- Thanks to [neofetch](https://github.com/dylanaraps/neofetch) for pioneering the fetch tool concept
- Created for the awesome RISC-V community!

## 🌐 Related Projects

- **[fastfetch](https://github.com/fastfetch-cli/fastfetch)** - Fast system info (use alongside riscfetch!)
- **[screenfetch](https://github.com/KittyKatt/screenFetch)** - Original fetch tool
- **[pfetch](https://github.com/dylanaraps/pfetch)** - Minimal fetch in POSIX shell

---

**Made with ❤️ for RISC-V enthusiasts**

*Show the world your RISC-V setup! 🚀*
