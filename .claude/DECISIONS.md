# riscfetch - Architecture Decision Records

This document records important architectural and design decisions made during the development of riscfetch.

## ADR-001: Use Rust as Implementation Language

**Date**: 2024-11-17
**Status**: Accepted

### Context
Need to choose an implementation language for a system information tool.

### Options Considered
1. Shell script (like neofetch)
2. Python
3. C/C++
4. Rust
5. Go

### Decision
Use Rust

### Rationale
- **Performance**: Comparable to C/C++, much faster than shell/Python
- **Safety**: Memory safety without garbage collection
- **Ecosystem**: Excellent crates (clap, colored, sysinfo)
- **Modern**: Fits the "modern fastfetch-inspired" vision
- **Single binary**: Easy distribution
- **RISC-V support**: First-class Rust support for RISC-V

### Consequences
- Positive: Fast, safe, modern
- Negative: Requires Rust toolchain for building from source
- Neutral: Steeper learning curve for contributors unfamiliar with Rust

---

## ADR-002: Complement Rather Than Compete with fastfetch

**Date**: 2024-11-17
**Status**: Accepted

### Context
Define positioning relative to existing fetch tools (neofetch, fastfetch, etc.)

### Decision
Position as **complementary** to fastfetch, not a replacement

### Rationale
- Avoid duplicating effort on generic Linux info
- Focus on RISC-V specific value
- Encourage users to run both tools
- Smaller scope = easier maintenance
- Differentiation through specialization

### Consequences
- Positive: Clear value proposition, smaller codebase
- Negative: May confuse users expecting all info in one tool
- Mitigation: Clear documentation about complementary nature

---

## ADR-003: Modular Code Structure

**Date**: 2024-11-17
**Status**: Accepted

### Context
Initial implementation was monolithic (~500 lines in main.rs)

### Decision
Split into modules: main, info, display, benchmark

### Rationale
- **Maintainability**: Easier to understand and modify
- **Testing**: Unit tests for specific modules
- **Reusability**: Functions can be reused
- **Separation of concerns**: Clear responsibilities

### Module Breakdown
- `main.rs`: CLI and orchestration (130 lines)
- `info.rs`: System information (330 lines)
- `display.rs`: Visual output (180 lines)
- `benchmark.rs`: Performance tests (130 lines)

### Consequences
- Positive: Better organization, easier testing
- Negative: Slight increase in complexity (module boundaries)
- Neutral: More files to navigate

---

## ADR-004: Use cargo-husky for Pre-commit Hooks

**Date**: 2024-11-17
**Status**: Accepted

### Context
Need automated code quality checks before commits

### Options Considered
1. Manual git hooks
2. pre-commit framework (Python-based)
3. cargo-husky
4. lefthook
5. No pre-commit hooks

### Decision
Use cargo-husky

### Rationale
- **Native Rust**: Integrates seamlessly with Cargo
- **Simple setup**: Just add as dev-dependency
- **User hooks**: Custom hook scripts in `.cargo-husky/hooks/`
- **Automatic installation**: Hooks install on `cargo build`
- **No external dependencies**: Pure Rust solution

### Hook Configuration
```bash
#!/bin/sh
cargo fmt -- --check
cargo clippy -- -D warnings
cargo test
```

### Consequences
- Positive: Automated quality checks, prevents bad commits
- Negative: Slower commit process (~1s overhead)
- Mitigation: Hooks can be skipped with `git commit --no-verify` if needed

---

## ADR-005: GitHub Actions for CI/CD

**Date**: 2024-11-17
**Status**: Accepted

### Context
Need continuous integration and release automation

### Decision
Use GitHub Actions with two workflows:
1. `ci.yml` - Test/lint on push
2. `release.yml` - Build binaries on tag push

### Rationale
- **Native to GitHub**: No external service needed
- **Free for public repos**: No cost
- **Matrix builds**: Easy multi-platform builds
- **Artifact storage**: GitHub Releases integration
- **Community familiarity**: Most contributors know GH Actions

### Platform Coverage
- Linux: x86_64 (glibc, musl)
- macOS: x86_64, ARM64
- Windows: x86_64

### Consequences
- Positive: Automated releases, multi-platform support
- Negative: Tied to GitHub platform
- Neutral: ~5-10 minutes build time per release

---

## ADR-006: Emoji + Color for Field Labels

**Date**: 2024-11-17
**Status**: Accepted

### Context
How to visually distinguish different information fields

### Options Considered
1. Plain text labels
2. Colors only
3. Emoji only
4. Emoji + Colors (chosen)

### Decision
Use emoji + terminal colors

### Rationale
- **Visual hierarchy**: Easy to scan at a glance
- **Modern aesthetic**: Matches fastfetch style
- **Memorable**: Emoji aids recognition
- **Accessibility**: Colors provide redundancy for emoji
- **Fun**: Fits the "show off your setup" use case

### Emoji Guidelines
- One emoji per field
- Choose semantically meaningful emoji
- Fallback: If terminal doesn't support emoji, text still readable

### Consequences
- Positive: Beautiful output, easy to read
- Negative: May not work in all terminals (rare)
- Mitigation: Future `--plain` mode for ASCII-only output

---

## ADR-007: Splash Animation as Opt-in Feature

**Date**: 2024-11-17
**Status**: Accepted

### Context
Whether to show animation by default or require flag

### Decision
Animation is **opt-in** via `--splash` flag

### Rationale
- **Performance**: Adds ~1 second to runtime
- **Scripting**: Scripts shouldn't have unexpected delays
- **Surprise factor**: Users who want it will discover it
- **Default behavior**: Fast info display for typical use

### Implementation
- 6 frames × 2 rotations = 12 frames total
- 80ms per frame = ~960ms total animation
- Rainbow color cycling
- Clear screen before and after

### Consequences
- Positive: Fast default behavior, cool feature for demos
- Negative: Hidden feature may be undiscovered
- Mitigation: Document in README, mention in help text

---

## ADR-008: Benchmark Mode as Optional Feature

**Date**: 2024-11-17
**Status**: Accepted

### Context
Whether to always run benchmarks or make them optional

### Decision
Benchmarks are **opt-in** via `--benchmark` flag

### Rationale
- **Performance impact**: Benchmarks take several seconds
- **Main use case**: Quick info display, not benchmarking
- **Separate concern**: System info ≠ performance testing
- **Battery/heat**: Running benchmarks consumes power

### Benchmark Suite
1. Integer ops (10M iterations)
2. Floating-point ops (5M iterations)
3. Memory bandwidth (10MB × 2)

Total time: ~2-5 seconds depending on hardware

### Consequences
- Positive: Fast default operation, benchmarks when needed
- Negative: Feature may be overlooked
- Mitigation: Clear documentation, help text

---

## ADR-009: MIT License with Single Author

**Date**: 2024-11-17
**Status**: Accepted

### Context
Choose license and copyright holder

### Decision
MIT License, Copyright (c) 2024 kako-jun

### Rationale
- **Permissive**: Allows commercial use
- **Simple**: Easy to understand
- **Popular**: Widely accepted in Rust community
- **Attribution**: Single author for initial version
- **Future contributions**: Contributors retain copyright, grant license

### License Text
Standard MIT License template with kako-jun as copyright holder

### Consequences
- Positive: Maximum freedom for users and contributors
- Negative: No copyleft protection
- Neutral: Contributors need to agree to license

---

## ADR-010: Support Only RISC-V, Fail on Other Architectures

**Date**: 2024-11-17
**Status**: Accepted

### Context
Whether to provide degraded functionality on non-RISC-V systems

### Decision
**Hard fail** with friendly message: "Sorry, not RISC-V 😢"

### Rationale
- **Clear identity**: RISC-V exclusive tool
- **Simplicity**: No fallback logic needed
- **Marketing**: Exclusivity is a feature
- **Code clarity**: No architecture-specific code paths
- **Differentiation**: Other tools cover x86/ARM

### Detection Method
```rust
1. Check uname -m for "riscv"
2. Check /proc/cpuinfo for "riscv" or "RISC-V"
3. Return false if neither match
```

### Consequences
- Positive: Crystal clear tool purpose
- Negative: Can't test on x86 development machines
- Mitigation: Emulator support (QEMU), CI in future

---

## ADR-011: Board Detection via Device Tree

**Date**: 2024-11-17
**Status**: Accepted

### Context
How to identify specific RISC-V boards

### Decision
Use Linux device tree (`/proc/device-tree/`)

### Rationale
- **Standard location**: All Linux systems on ARM/RISC-V
- **Reliable**: Kernel-provided, official board info
- **Detailed**: Model name and compatible strings
- **Future-proof**: New boards will have device tree

### Detection Strategy
```
Priority:
1. /proc/device-tree/model (clean name)
2. /proc/device-tree/compatible (parse for known patterns)
3. Empty string (unknown board)
```

### Board Database
Hardcoded patterns for known boards:
- StarFive VisionFive 2
- SiFive HiFive Unmatched/Unleashed
- Milk-V Mars/Pioneer
- T-Head boards

### Consequences
- Positive: Accurate board detection
- Negative: Need to maintain board database
- Future: Externalize to JSON/TOML config

---

## ADR-012: Use std::hint::black_box for Benchmark Optimization Prevention

**Date**: 2024-11-17
**Status**: Accepted

### Context
Prevent compiler from optimizing away benchmark code

### Options Considered
1. Empty `println!("")` - Clippy warning
2. Volatile writes
3. `std::hint::black_box()`
4. `test::black_box()` (unstable)

### Decision
Use `std::hint::black_box()`

### Rationale
- **Stable Rust**: Available in stable since 1.66
- **Clippy compliant**: No warnings
- **Clear intent**: Explicit optimization barrier
- **Standard**: Recommended approach in Rust community

### Usage
```rust
let result = compute_something();
std::hint::black_box(result);  // Prevent optimization
```

### Consequences
- Positive: Clean code, no warnings
- Negative: Requires Rust 1.66+
- Mitigation: Reasonable minimum version

---

## Template for Future ADRs

```markdown
## ADR-XXX: Title

**Date**: YYYY-MM-DD
**Status**: Proposed | Accepted | Deprecated | Superseded

### Context
Background and problem statement

### Options Considered
1. Option A
2. Option B
3. Option C

### Decision
Chosen option

### Rationale
Why this decision was made

### Consequences
- Positive: Benefits
- Negative: Drawbacks
- Mitigation: How to address drawbacks
```

---

**Last Updated**: 2024-11-17
**Total ADRs**: 12
**Next ADR Number**: 013
