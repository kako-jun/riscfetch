# riscfetch-core Specification

This document defines the expected behavior of riscfetch-core functions. Tests must be written against this specification, not against implementation behavior.

## References

- [RISC-V ISA Specification](https://riscv.org/technical/specifications/)
- [RISC-V Unprivileged ISA (Volume 1)](https://github.com/riscv/riscv-isa-manual)

## ISA String Format

RISC-V ISA strings follow this format:
```
rv{32|64}[base extensions]_[z-extensions]_[s-extensions]
```

Examples:
- `rv64imafdc` - 64-bit with I, M, A, F, D, C extensions
- `rv64gc` - 64-bit with G (= IMAFD) + C extensions
- `rv64imafdc_zicsr_zba_zbb` - with Z-extensions

## Standard Extensions

Per RISC-V specification:

| Letter | Name | Description |
|--------|------|-------------|
| I | Base Integer | Base integer instruction set (required) |
| E | Embedded | Reduced 16-register variant (mutually exclusive with I) |
| M | Multiply | Integer multiplication and division |
| A | Atomic | Atomic memory operations |
| F | Float | Single-precision (32-bit) floating-point |
| D | Double | Double-precision (64-bit) floating-point (requires F) |
| Q | Quad | Quad-precision (128-bit) floating-point (requires D) |
| C | Compressed | 16-bit compressed instructions |
| B | Bit Manipulation | Bit manipulation operations |
| V | Vector | Vector operations (SIMD) |
| H | Hypervisor | Hypervisor extension |
| G | General | Shorthand for IMAFDZicsr_Zifencei (not a real extension) |

---

## Function Specifications

### `parse_extensions_compact(isa: &str) -> String`

Parses an ISA string and returns detected standard extensions.

#### Input
- `isa`: RISC-V ISA string (case-insensitive)

#### Output
- Space-separated extension letters in canonical order: `I E M A F D Q C B V H`
- Empty string if no valid extensions found

#### Behavior

| Input | Expected Output | Reason |
|-------|-----------------|--------|
| `"rv64imafdc"` | `"I M A F D C"` | Standard extensions |
| `"rv64gc"` | `"I M A F D C"` | G expands to IMAFD |
| `"rv32imac"` | `"I M A C"` | 32-bit variant |
| `"rv32e"` | `"E"` | Embedded variant |
| `"rv32ec"` | `"E C"` | Embedded with compressed |
| `"rv64imafdcv"` | `"I M A F D C V"` | With vector |
| `"RV64IMAFDC"` | `"I M A F D C"` | Case insensitive |
| `"rv64imafdc_zba_zbb"` | `"I M A F D C"` | Z-extensions ignored |
| `""` | `""` | Empty input |
| `"unknown"` | `""` | Invalid input |
| `"rv64"` | `""` | No extensions specified |

#### Important Notes
- The `v` in `rv64` prefix must NOT be detected as Vector extension
- `G` is a shorthand, not stored in output (expand to IMAFD)
- `E` and `I` are mutually exclusive per spec

---

### `parse_z_extensions(isa: &str) -> String`

Extracts Z-extensions and S-extensions from ISA string.

#### Input
- `isa`: RISC-V ISA string

#### Output
- Space-separated Z/S-extension names in order of appearance
- Empty string if none found

#### Behavior

| Input | Expected Output | Reason |
|-------|-----------------|--------|
| `"rv64i_zicsr_zifencei"` | `"zicsr zifencei"` | Z-extensions after underscore |
| `"rv64i_zba_zbb_zbc"` | `"zba zbb zbc"` | Order preserved |
| `"rv64i_sstc"` | `"sstc"` | S-extensions included |
| `"rv64imafdc"` | `""` | No Z-extensions |
| `"rv64gc"` | `"zicsr zifencei"` | G implies Zicsr_Zifencei |
| `"rv64i_Zicsr"` | `"zicsr"` | Case normalized to lowercase |
| `""` | `""` | Empty input |

---

### `parse_vector_from_isa(isa: &str) -> Option<String>`

Detects vector extension and infers VLEN from zvl* extensions.

#### Input
- `isa`: RISC-V ISA string

#### Output
- `Some(description)` if vector capability detected
- `None` if no vector support

#### Behavior

| Input | Expected Output | Reason |
|-------|-----------------|--------|
| `"rv64imafdcv"` | `Some("Enabled")` | V present, VLEN unknown |
| `"rv64imafdc"` | `None` | No V extension |
| `"rv64i_zve32x"` | `Some("Enabled, ...")` | Zve* implies vector |
| `"rv64imafdcv_zvl256b"` | `Some("Enabled, VLEN>=256")` | VLEN from zvl256b |
| `"rv64imafdcv_zvl512b"` | `Some("Enabled, VLEN>=512")` | VLEN from zvl512b |
| `"rv64i_zvl128b_zvl256b"` | `Some("..., VLEN>=256")` | Largest zvl* wins |

#### VLEN Detection
- `zvl32b` → VLEN >= 32
- `zvl64b` → VLEN >= 64
- `zvl128b` → VLEN >= 128
- `zvl256b` → VLEN >= 256
- `zvl512b` → VLEN >= 512
- `zvl1024b` → VLEN >= 1024
- If multiple present, use largest value
- If V present but no zvl* specified, VLEN is implementation-defined (do not display)

---

### `parse_extensions_explained(isa: &str) -> Vec<(String, String)>`

Returns detected extensions with human-readable descriptions.

#### Output Format
- Vector of `(extension_name, description)` tuples
- Same detection logic as `parse_extensions_compact`

#### Expected Descriptions

| Extension | Description |
|-----------|-------------|
| I | Base Integer Instructions |
| E | Embedded (16 registers) |
| M | Integer Multiply/Divide |
| A | Atomic Instructions |
| F | Single-Precision Float |
| D | Double-Precision Float |
| Q | Quad-Precision Float |
| C | Compressed (16-bit) |
| B | Bit Manipulation |
| V | Vector (SIMD) |
| H | Hypervisor |

---

### `parse_z_extensions_explained(isa: &str) -> Vec<(String, String)>`

Returns Z-extensions with descriptions.

#### Known Z-Extensions

| Extension | Display Name | Description |
|-----------|--------------|-------------|
| zicsr | Zicsr | CSR Instructions |
| zifencei | Zifencei | Instruction-Fetch Fence |
| zicntr | Zicntr | Base Counters/Timers |
| zihpm | Zihpm | Hardware Perf Counters |
| zba | Zba | Address Generation |
| zbb | Zbb | Basic Bit Manipulation |
| zbc | Zbc | Carry-less Multiply |
| zbs | Zbs | Single-bit Operations |
| zicbom | Zicbom | Cache-Block Management |
| zicboz | Zicboz | Cache-Block Zero |
| zihintpause | Zihintpause | Pause Hint |
| zkt | Zkt | Data-Independent Timing |
| zvkt | Zvkt | Vector Data-Independent Timing |

Unknown Z-extensions should return `(name, "Unknown")`.

---

## Edge Cases

### Case Sensitivity
- All input should be treated as case-insensitive
- Output extension letters are always uppercase (I, M, A, etc.)
- Output Z-extension names are always lowercase (zicsr, zba, etc.)

### Invalid Input
- Empty string → empty output (not error)
- Non-ISA strings → empty output (not error)
- Malformed ISA strings → best-effort parsing

### The "rv64" Prefix Problem
The `v` in `rv64` must NOT be detected as Vector extension.

```
WRONG: parse_extensions_compact("rv64imafdc") → "I M A F D C V"
RIGHT: parse_extensions_compact("rv64imafdc") → "I M A F D C"
```

### G Expansion
`G` is a shorthand for `IMAFD`, not a real extension.

```
parse_extensions_compact("rv64gc") → "I M A F D C"  (not "G C")
```

---

## Test Requirements

1. All tests must reference this specification
2. Tests must be written BEFORE fixing bugs
3. If a test fails, either:
   - The implementation is wrong (fix the code)
   - The specification is wrong (update SPEC.md first, then tests)
4. Never change tests just to make them pass

## Version

- Spec version: 1.0
- Last updated: 2024-12
- Based on RISC-V ISA spec version: 20240411
