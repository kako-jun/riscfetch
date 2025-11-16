# 🚀 riscfetch

**RISC-V Architecture Information Display Tool**

Show off your RISC-V setup with style! `riscfetch` is a specialized fetch tool designed exclusively for RISC-V architectures, inspired by neofetch but tailored for the RISC-V community.

## ✨ Features

- 🎯 **RISC-V Exclusive**: Only displays information on RISC-V systems
- 🎨 **Beautiful ASCII Art**: Multiple logo styles (default, SiFive, Kendryte)
- ✨ **Animated Splash Screen**: Eye-catching rotating block animation (like modern AI coding agents!)
- 📊 **Detailed ISA Information**: Shows base ISA and extensions (M, A, F, D, C, V, Zicsr, etc.)
- 💻 **System Info**: CPU, SoC, OS, and uptime information
- 🌈 **Colorful Output**: Terminal-friendly colored display
- ⚡ **Benchmarks**: Optional performance testing for ISA extensions

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

🧠 CPU: RV64IMAFDC
🏗️  SoC: sifive,fu740
🧪 Extensions: M (Multiply), A (Atomic), F (Float), D (Double), C (Compressed)
🕹️  OS: Debian GNU/Linux 12 (bookworm) - 6.5.0-riscv64
🚀 Uptime: 3h 42m
```

### On Non-RISC-V Systems

```bash
$ riscfetch

Sorry, not RISC-V 😢
```

## 🔧 Technical Details

`riscfetch` detects RISC-V architecture and gathers information from:

- `/proc/cpuinfo` - CPU and ISA information
- `/proc/device-tree/compatible` - SoC information
- `/etc/os-release` - OS distribution details
- `uname` - Kernel and architecture details
- System uptime

### Supported ISA Extensions

- **Base**: RV32I, RV64I
- **Standard Extensions**:
  - M: Integer Multiplication and Division
  - A: Atomic Instructions
  - F: Single-Precision Floating-Point
  - D: Double-Precision Floating-Point
  - C: Compressed Instructions
  - V: Vector Operations
- **Z Extensions**: Zicsr, Zifencei, Zba, Zbb, etc.

## 🌟 Why riscfetch?

- **Community Pride**: RISC-V users are still a minority - show off your setup!
- **SNS-Friendly**: Perfect for sharing your RISC-V environment on social media
- **Educational**: Learn about ISA extensions and their meanings
- **Lightweight**: Single binary, fast execution
- **Fun**: Because every architecture deserves its own fetch tool!

## 🛣️ Roadmap

- [ ] `--screenshot` - Generate image for social media sharing
- [ ] `--benchmark` - Simple ISA-specific benchmarks
- [ ] `--compare` - Compare with other architectures
- [ ] Custom color schemes
- [ ] JSON output for scripting
- [ ] More SoC-specific logos

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📝 License

MIT License - See LICENSE file for details

## 🙏 Acknowledgments

Inspired by [neofetch](https://github.com/dylanaraps/neofetch) and created for the awesome RISC-V community!

---

**Made with ❤️ for RISC-V enthusiasts**
