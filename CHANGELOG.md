# Changelog

All notable changes to this project will be documented in this file.

## [2.0.0] - 2025-12-18

### Added
- Complete RISC-V extension support (144 extensions: 98 Z + 46 S)
- `--all` (`-a`) flag to show all extensions with checkmarks (✓/✗)
- `--all` works with `--json` for machine-readable output with `supported` field
- Pine64 and WCH vendors (now 13 vendors total)
- Extensions grouped by category (Base, Bit, Cache, Crypto, Vector, etc.)

### Changed
- Data-driven vendor definitions (vendors.rs)
- Data-driven extension definitions (extensions.rs)
- Updated to RISC-V ISA spec 2025-11-26

## [1.1.0] - 2025-12-17

### Added
- `--riscv-only` (`-r`) flag to show only RISC-V specific info
- `RiscvInfo` struct and `collect_riscv_info()` in core library
- Integration tests for the new flag

### Changed
- JSON output respects `--riscv-only` flag (excludes OS, memory, uptime fields)

## [1.0.0] - 2024-12-15

### Changed
- Stable release

## [0.2.0] - 2024-12-14

### Added
- `--explain` flag for detailed ISA extension explanations
- `--json` flag for machine-readable JSON output
- Shell completions (bash, zsh, fish, powershell)
- Man page generation
- Hardware ID display (mvendorid, marchid, mimpid)
- VLEN detection for vector extension
- 38 Z-extension patterns with descriptions

### Changed
- Display style: removed emojis, use neofetch-style labels
- Separated RISC-V specific info from general system info
- Combined hardware IDs into single line format
- README rewritten in concise technical style

### Removed
- CLAUDE.md (keep English-only documentation)

## [0.1.0] - 2024-11-28

### Added
- Initial release
- ISA string detection from /proc/cpuinfo
- Standard extension parsing (I, M, A, F, D, C, V)
- Z-extension detection
- Hart count display
- Board detection via device-tree
- Cache info from sysfs
- Memory, kernel, OS, uptime display
- Animated splash screen (`--splash`)
- Multiple logo styles (`--logo`)
- Simple benchmarks (`--benchmark`)
- Pre-commit hooks (fmt, clippy)
- CI/CD workflows
