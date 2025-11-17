# riscfetch - Technical Design

## 🏗️ Architecture Overview

### High-Level Design

```
┌─────────────────────────────────────────┐
│         CLI Interface (clap)            │
│  --logo, --benchmark, --splash, --help  │
└─────────────────┬───────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────┐
│          main.rs (Orchestration)        │
│  • Parse arguments                      │
│  • Check RISC-V architecture            │
│  • Coordinate display flow              │
└─────────────────┬───────────────────────┘
                  │
        ┌─────────┼─────────┐
        ▼         ▼         ▼
   ┌────────┐ ┌────────┐ ┌────────┐
   │ info   │ │display │ │benchmark│
   │ module │ │ module │ │ module  │
   └────────┘ └────────┘ └────────┘
        │         │         │
        ▼         ▼         ▼
   System APIs  ANSI     Performance
   /proc, /sys  Escape   Timing
   sysinfo      Sequences
```

### Module Responsibilities

#### `main.rs` - Entry Point
**Responsibility**: Orchestration and flow control
- CLI argument parsing (clap)
- Architecture verification
- Module coordination
- Exit code management

**Key Functions**:
- `main()`: Entry point
- `display_riscv_info()`: Coordinate information display

#### `info.rs` - Information Gathering
**Responsibility**: System information collection
- Read from `/proc/cpuinfo`, `/sys/`, `/etc/`
- Parse device tree information
- Execute system commands (uname)
- Use sysinfo crate for cross-platform info

**Key Functions**:
- `is_riscv()`: Detect RISC-V architecture
- `get_board_info()`: Board model detection
- `get_cpu_info()`: CPU and ISA base
- `get_hart_count()`: Hardware thread count
- `get_soc_info()`: SoC identification
- `get_extensions()`: ISA extension parsing
- `get_vector_info()`: Vector extension details
- `get_cache_info()`: Cache hierarchy
- `get_memory_info()`: Memory usage
- `get_kernel_info()`: Kernel version
- `get_os_info()`: OS distribution
- `get_uptime()`: System uptime

**Tests**: Unit tests for parsing logic

#### `display.rs` - Visual Output
**Responsibility**: User interface and animation
- ASCII art logo generation
- ANSI color sequences
- Terminal animation
- Screen clearing and cursor control

**Key Functions**:
- `display_logo()`: Show selected logo
- `show_splash_animation()`: Rotating RISC-V animation
- `get_default_logo()`: Default RISC-V logo
- `get_sifive_logo()`: SiFive branded logo
- `get_kendryte_logo()`: Kendryte branded logo
- `get_animation_frames()`: Animation frame data

**No Tests**: Visual output is hard to unit test

#### `benchmark.rs` - Performance Testing
**Responsibility**: ISA-specific benchmarks
- Integer operations (M extension)
- Floating-point operations (F/D extensions)
- Memory bandwidth
- Timing and scoring

**Key Functions**:
- `run_benchmarks()`: Execute all benchmarks
- `benchmark_integer_ops()`: Test M extension
- `benchmark_float_ops()`: Test F/D extensions
- `benchmark_memory()`: Memory bandwidth

**Tests**: Verify benchmarks return valid scores

## 🔍 Information Detection Strategy

### RISC-V Architecture Detection

```rust
Priority order:
1. uname -m → Check for "riscv" in architecture
2. /proc/cpuinfo → Check for "riscv" or "RISC-V" in content
3. Fail → Not RISC-V, exit with message
```

### Board Model Detection

```rust
Priority order:
1. /proc/device-tree/model → Clean model string
2. /proc/device-tree/compatible → Parse and match known boards
   - StarFive: "starfive" + "visionfive2"
   - SiFive: "sifive" + "unmatched"/"unleashed"
   - Milk-V: "milkv"/"milk-v" + "mars"/"pioneer"
   - T-Head: "thead"
3. Empty string → No board info available
```

**Known Board Patterns**:
| Vendor | Compatible String | Display Name |
|--------|-------------------|--------------|
| StarFive | starfive,visionfive2 | StarFive VisionFive 2 |
| SiFive | sifive,hifive-unmatched | SiFive HiFive Unmatched |
| SiFive | sifive,hifive-unleashed | SiFive HiFive Unleashed |
| Milk-V | milkv,mars | Milk-V Mars |
| Milk-V | milkv,pioneer | Milk-V Pioneer |
| T-Head | thead,* | T-Head Board |

### ISA Extension Parsing

```rust
Input: "rv64imafdcv_zicsr_zifencei_zba_zbb"

Parse strategy:
1. Extract base: "rv64" or "rv32"
2. Extract standard letters: i, m, a, f, d, c, v
3. Split on '_' for Z extensions
4. Map letters to descriptions:
   - M: "M (Multiply)"
   - A: "A (Atomic)"
   - F: "F (Float)"
   - D: "D (Double)"
   - C: "C (Compressed)"
   - V: "V (Vector)"
```

### Cache Information Detection

```rust
Path structure: /sys/devices/system/cpu/cpu0/cache/
- index0/size → L1D (Data)
- index1/size → L1I (Instruction)
- index2/size → L2 (Unified)

Output format: "L1D: 32K, L1I: 32K, L2: 2048K"
```

## 🎨 Display Design

### Color Scheme

| Element | Color | Purpose |
|---------|-------|---------|
| Board | Bright Blue | Hardware identification |
| CPU | Bright Cyan | Core information |
| Harts | Bright White | Thread count |
| SoC | Bright Green | System-on-chip |
| ISA | Bright Yellow | Architecture features |
| Vector | Bright Magenta | Advanced features |
| Cache | Bright Cyan | Memory hierarchy |
| Memory | Bright Red | Resource usage |
| Kernel | Bright Green | System software |
| OS | Bright Magenta | Distribution |
| Uptime | Bright Blue | System state |

### Emoji Usage

| Field | Emoji | Reason |
|-------|-------|--------|
| Board | 🖥️ | Desktop computer |
| CPU | 🧠 | Brain/intelligence |
| Harts | ⚙️ | Mechanical/threads |
| SoC | 🏗️ | Building/architecture |
| ISA | 🧪 | Science/experiment |
| Vector | 📐 | Math/geometry |
| Cache | 💾 | Storage/memory |
| Memory | 🧮 | Calculation/resources |
| Kernel | 🐧 | Linux penguin |
| OS | 🕹️ | Gaming/control |
| Uptime | 🚀 | Speed/running |

### Animation Design

6-frame rotation animation:
1. Front view (Frame 1)
2. Slight rotation right (Frame 2)
3. More rotation (Frame 3)
4. Side view (Frame 4)
5. Rotation back (Frame 5)
6. Nearly front (Frame 6)

Color cycling: Cyan → Blue → Magenta → Red → Yellow → Green

## 🔧 Dependencies

### Runtime Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| clap | 4.5 | CLI argument parsing |
| colored | 2.1 | Terminal color output |
| sysinfo | 0.31 | Cross-platform system info |

### Development Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| cargo-husky | 1.0 | Git pre-commit hooks |

### Build Configuration

```toml
[package]
edition = "2021"  # Rust 2021 edition
rust-version = "1.70"  # Minimum Rust version (implied)

[profile.release]
# Future optimization options
opt-level = 3
lto = true
codegen-units = 1
strip = true
```

## 🧪 Testing Strategy

### Unit Tests
- `info::extract_extensions()`: ISA parsing
- `info::get_uptime()`: Uptime formatting
- `benchmark::*()`: Benchmark validity

### Integration Tests
- CLI help command
- CLI version command
- RISC-V detection (architecture-dependent)

### Test Coverage Goals
- Line coverage: >80%
- Function coverage: >90%
- Critical paths: 100%

## 🚀 CI/CD Pipeline

### On Push (ci.yml)
```yaml
Triggers: Push to main/master/develop, PRs
Jobs:
  1. Test (Ubuntu)
  2. Format check (rustfmt)
  3. Lint (clippy -D warnings)
Caching: cargo registry, index, build artifacts
```

### On Tag (release.yml)
```yaml
Triggers: Push tags matching 'v*'
Platforms:
  - Linux x86_64 (glibc)
  - Linux x86_64 (musl)
  - macOS x86_64
  - macOS ARM64
  - Windows x86_64
Post-processing: Strip binaries
Artifacts: GitHub Releases
```

## 🔐 Security Considerations

### Input Validation
- All file reads use safe Rust APIs
- No unsafe code blocks
- Path traversal prevention (use absolute paths)

### System Calls
- Minimize uname usage
- Sanitize command outputs
- No user-provided command execution

### Information Disclosure
- Only read publicly accessible system files
- No sensitive information collection
- No network communication

## 📊 Performance Targets

| Metric | Target | Current |
|--------|--------|---------|
| Cold start time | <100ms | ~50ms |
| Memory usage | <10MB | ~5MB |
| Binary size | <5MB | ~2MB |
| Lines of code | <2000 | ~1200 |

## 🔄 Future Architecture Changes

### v0.2.0
- Add configuration file support (TOML)
- Plugin system foundation
- Logging infrastructure

### v0.3.0
- Screenshot generation (image crate)
- Remote query capability
- Database of known boards

### v1.0.0
- Stable plugin API
- i18n framework
- Telemetry (opt-in)

---

**Last Updated**: 2024-11-17
**Architecture Version**: 1.0
**Review Date**: 2025-01-17
