# riscfetch - TODO List

## 🚀 v0.1.0 - Initial Release (COMPLETED ✅)

### Core Features
- [x] RISC-V architecture detection
- [x] Hart count display
- [x] ISA extension detection (M, A, F, D, C, V)
- [x] Z extension detection (Zicsr, Zifencei, Zba, Zbb)
- [x] Board recognition (VisionFive 2, SiFive, Milk-V)
- [x] Vector extension detection
- [x] Cache information (L1D, L1I, L2)
- [x] Memory usage display
- [x] Kernel version
- [x] OS detection
- [x] Uptime display

### UI/UX
- [x] Multiple logo styles (default, SiFive, Kendryte)
- [x] Animated splash screen (rotating RISC-V logo)
- [x] Color-coded output
- [x] Emoji icons for each field

### Performance
- [x] Benchmark mode (--benchmark)
- [x] Integer ops benchmark (M extension)
- [x] Float ops benchmark (F/D extension)
- [x] Memory bandwidth benchmark

### Development
- [x] Modular code structure
- [x] Unit tests
- [x] Integration tests
- [x] cargo fmt/clippy compliance
- [x] Pre-commit hooks (cargo-husky)
- [x] GitHub Actions CI
- [x] GitHub Actions release workflow
- [x] Professional README
- [x] MIT License (kako-jun)

---

## 🎯 v0.2.0 - Community & Detection (PLANNED)

### Enhanced Board Detection
- [ ] BeagleV Starlight detection
- [ ] Allwinner D1/D1s detection
- [ ] Canaan Kendryte K510 detection
- [ ] SiFive FU540 (HiFive Unleashed) better detection
- [ ] Pine64 Ox64 detection
- [ ] Add board logo variants for detected boards

### Extended ISA Information
- [ ] VLEN (Vector length) detection
- [ ] ELEN (Element length) detection
- [ ] Zb* extension family detection (Zbc, Zbs)
- [ ] Hypervisor extension (H) detection
- [ ] Privileged architecture version display

### Output Formats
- [ ] JSON output mode (`--json`)
- [ ] Plain text mode (`--plain`, no colors)
- [ ] Compact mode (`--compact`, single line)
- [ ] Custom format strings

### Documentation
- [ ] Man page
- [ ] Contribution guidelines
- [ ] Board detection guide for contributors
- [ ] Architecture decision records

---

## 🌟 v0.3.0 - Advanced Features (FUTURE)

### Screenshot & Sharing
- [ ] `--screenshot` - Generate PNG/SVG for SNS
- [ ] Template system for screenshot layouts
- [ ] Twitter/Mastodon optimized output
- [ ] QR code generation for sharing

### Comparison & Benchmark
- [ ] `--compare` - Compare with x86_64/ARM64
- [ ] Historical benchmark tracking
- [ ] Performance regression detection
- [ ] Community benchmark leaderboard

### System Integration
- [ ] Privilege level detection (M/S/U modes)
- [ ] Hardware performance counter integration
- [ ] RISC-V specific CPU features (mvendorid, marchid)
- [ ] Boot time analysis
- [ ] Power consumption estimation

### Customization
- [ ] Custom color schemes (config file)
- [ ] User-defined info fields
- [ ] Plugin system architecture
- [ ] Theme gallery

---

## 🏆 v1.0.0 - Production Ready (VISION)

### Stability & Polish
- [ ] Comprehensive error handling
- [ ] Fallback mechanisms for missing info
- [ ] i18n support (Japanese, Chinese, etc.)
- [ ] Accessibility improvements

### Community Features
- [ ] Logo design contest integration
- [ ] Community board database
- [ ] User-submitted configurations
- [ ] Official RISC-V International endorsement

### Ecosystem Integration
- [ ] Package in major Linux distributions
  - [ ] Debian RISC-V
  - [ ] Fedora RISC-V
  - [ ] Ubuntu RISC-V
  - [ ] Arch Linux RISC-V
- [ ] Integration with RISC-V SDKs
- [ ] Board vendor partnerships

### Advanced Detection
- [ ] Emulator detection (QEMU, RVVM)
- [ ] Container/virtualization detection
- [ ] Cloud RISC-V instance detection
- [ ] FPGA implementation detection

---

## 🐛 Known Issues

### Current
- None reported yet

### To Investigate
- [ ] Performance on low-end RISC-V boards
- [ ] Compatibility with RISC-V emulators
- [ ] Cache detection on different kernel versions
- [ ] Board detection accuracy on custom boards

---

## 📋 Backlog (Not Prioritized)

### Nice to Have
- [ ] ASCII art alternatives (for terminals without Unicode)
- [ ] Audio notification on completion
- [ ] Integration with system monitoring tools
- [ ] Docker/Podman container support
- [ ] Remote RISC-V system query
- [ ] Browser-based demo (WebAssembly?)

### Community Requests
- (Waiting for GitHub issues)

---

## 🎨 Design Decisions to Make

### Open Questions
- [ ] Should we support RV32 boards? (Currently focused on RV64)
- [ ] Default behavior: Show all info or minimal info?
- [ ] Config file format: TOML, YAML, or JSON?
- [ ] Should benchmarks be opt-in or opt-out?

### Naming Considerations
- [ ] Current name "riscfetch" - is it clear enough?
- [ ] Alternative: "rvfetch", "riscv-fetch", "hartfetch"?

---

**Last Updated**: 2024-11-17
**Current Version**: v0.1.0
**Next Milestone**: v0.2.0 (Community & Detection)
