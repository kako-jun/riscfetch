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

#### Z-Extension Categories

Z-extensions are grouped by functional category:

| Category | Description | Examples |
|----------|-------------|----------|
| Base | Core CSR/Fence operations | Zicsr, Zifencei, Zicntr, Zihpm |
| Hints | Hint instructions | Zihintpause, Zihintntl |
| Cache | Cache management | Zicbom, Zicboz, Zicbop |
| Conditional | Conditional operations | Zicond |
| Bit Manipulation | Bit operations | Zba, Zbb, Zbc, Zbs |
| Cryptography | Scalar crypto | Zk, Zkn, Zknd, Zkne, Zknh, Zks, Zksed, Zksh, Zkr, Zkt |
| Floating Point | FP extensions | Zfh, Zfhmin, Zfa, Zfinx, Zdinx, Zhinx |
| Compressed | Compressed instructions | Zca, Zcb, Zcd, Zcf, Zcmp, Zcmt |
| Atomics | Atomic operations | Zacas, Zabha, Zaamo, Zalrsc, Zawrs |
| Memory Model | Memory ordering | Za64rs, Za128rs, Zama16b, Ztso, Zic64b |
| Multiply | Multiply only | Zmmul |
| Vector | Vector operations | Zve32x, Zve64x, Zvl128b, Zvfh |
| Vector Crypto | Vector cryptography | Zvbb, Zvbc, Zvkg, Zvkn, Zvks |
| Other | Miscellaneous | Zimop, Zilsd |

---

### `parse_s_extensions(isa: &str) -> String`

Extracts S-extensions (privileged) from ISA string.

#### S-Extension Categories

| Category | Description | Examples |
|----------|-------------|----------|
| Virtual Memory | Address translation | Svinval, Svnapot, Svpbmt, Svade, Svadu |
| Supervisor | S-mode features | Ssaia, Sstc, Ssstateen, Sstvala |
| Machine | M-mode features | Smaia, Smstateen, Smepmp, Smrnmi |
| Hypervisor | H-mode features | Sha, Shgatpa, Shtvala, Shvstvala |
| Debug | Debug features | Sdext, Sdtrig |
| User | U-mode features | Supm |

---

### Full Extension List

#### Z-Extensions (98 total)

**Base (4)**
- Zicsr, Zifencei, Zicntr, Zihpm

**Hints (2)**
- Zihintpause, Zihintntl

**Cache (3)**
- Zicbom, Zicboz, Zicbop

**Conditional (1)**
- Zicond

**Bit Manipulation (4)**
- Zba, Zbb, Zbc, Zbs

**Cryptography (13)**
- Zbkb, Zbkc, Zbkx, Zk, Zkn, Zknd, Zkne, Zknh, Zks, Zksed, Zksh, Zkr, Zkt

**Floating Point (8)**
- Zfh, Zfhmin, Zfa, Zfinx, Zdinx, Zhinx, Zhinxmin, Zfbfmin

**Compressed (8)**
- Zca, Zcb, Zcd, Zcf, Zcmp, Zcmt, Zcmop, Zclsd

**Atomics (5)**
- Zacas, Zabha, Zaamo, Zalrsc, Zawrs

**Memory Model (10)**
- Za64rs, Za128rs, Zama16b, Zic64b, Ziccamoa, Ziccamoc, Ziccif, Zicclsm, Ziccrse, Ztso

**Multiply (1)**
- Zmmul

**Other (2)**
- Zimop, Zilsd

**Vector (21)**
- Zve32f, Zve32x, Zve64d, Zve64f, Zve64x, Zvfh, Zvfhmin, Zvfbfmin, Zvfbfwma
- Zvl32b, Zvl64b, Zvl128b, Zvl256b, Zvl512b, Zvl1024b, Zvl2048b, Zvl4096b, Zvl8192b, Zvl16384b, Zvl32768b, Zvl65536b

**Vector Crypto (16)**
- Zvbb, Zvbc, Zvkb, Zvkg, Zvkn, Zvknc, Zvkned, Zvkng, Zvknha, Zvknhb, Zvks, Zvksc, Zvksed, Zvksg, Zvksh, Zvkt

#### S-Extensions (46 total)

**Virtual Memory (7)**
- Svinval, Svnapot, Svpbmt, Svade, Svadu, Svbare, Svvptc

**Supervisor (17)**
- Ssaia, Ssccfg, Ssccptr, Sscofpmf, Sscounterenw, Sscsrind, Ssctr, Ssdbltrp, Ssnpm, Sspm, Ssqosid, Ssstateen, Ssstrict, Sstc, Sstvala, Sstvecd, Ssu64xl

**Machine (11)**
- Smaia, Smcdeleg, Smcntrpmf, Smcsrind, Smctr, Smdbltrp, Smepmp, Smmpm, Smnpm, Smrnmi, Smstateen

**Hypervisor (8)**
- Sha, Shcounterenw, Shgatpa, Shlcofideleg, Shtvala, Shvsatpa, Shvstvala, Shvstvecd

**Debug (2)**
- Sdext, Sdtrig

**User (1)**
- Supm

Unknown extensions are still parsed but may not have descriptions.

---

## Derived Extensions (Implication & Composition)

`/proc/cpuinfo` only lists the extensions the kernel bothered to enumerate. It does not
compute the extensions that are architecturally implied by those, so riscfetch computes
them itself (issue #10).

Two relations are combined:

- **Implication** (forward): if extension `X` is present, its dependencies are present
  too, even if the ISA string never spells them out. Examples: `M` implies `Zmmul`, `D`
  implies `F`, `F` implies `Zicsr`, `A` implies `Zalrsc`+`Zaamo`, `C` implies `Zca`, `V`
  implies a chain down to `Zve32x` (and, per the ISA manual's minimum vector
  requirements, also `Zve64d`/`Zve64f`/`Zve32f`/`Zve64x`/`D`/`F`/`Zicsr`), `Zfh` implies
  `Zfhmin`, `Zvfh` implies `Zvfhmin`/`Zfhmin`, `Zdinx` implies `Zfinx`, and the `Zvl*`
  family chains from the largest declared minimum VLEN down to `Zvl32b`.
- **Composition** (converse): some extension names are pure ISA-manual shorthand for the
  union of a fixed set of other extensions (no behavior of their own). If every member
  of the set is present, the shorthand name is considered present too. Examples:
  `Zba`+`Zbb`+`Zbs` composes into `B`; `Zaamo`+`Zalrsc` composes into `A`;
  `Zkn`+`Zkr`+`Zkt` composes into `Zk`; similarly for `Zkn`, `Zks`, `Zvkn`, `Zvks`, `Zce`.

Both relations are resolved to their full transitive closure with a worklist
(`while changed`), so implication chains and composition chains are fully followed and
the computation terminates even in the presence of a cycle.

### Source of truth

The implication table (`riscfetch_core::implications::IMPLICATIONS`) is sourced from the
`Implies` field of each `RISCVExtension` definition in LLVM mainline
(`llvm/lib/Target/RISCV/RISCVFeatures.td`, fetched 2026-09-17). The composition table
(`riscfetch_core::implications::COMPOSITIONS`) mirrors the same member sets, applied in
the converse direction per the RISC-V ISA manual's definition of those names as
shorthand. Entries that could not be confirmed against the LLVM table, or whose
implication depends on rv32 vs rv64 in a way the table can't express (e.g. `Zcf` is only
legal on rv32), are intentionally left out — missing coverage is preferred over a false
positive.

### `ExtensionInfo.derived`

`ExtensionInfo` (returned by `get_extensions_with_derived`,
`parse_z_extensions_with_category_and_derived`,
`parse_s_extensions_with_category_and_derived`, and the corresponding `--all` /
plain functions) has a `derived: bool` field:

| `derived` | Meaning |
|-----------|---------|
| `false` | The ISA string names this extension directly (including via the `G` shorthand) |
| `true`  | Not named directly; inferred via implication or composition |

`supported` keeps its existing meaning (the extension is present) regardless of
`derived` — a derived extension is still `supported: true`.

`ExtensionEntry` (the JSON-serializable type used by `collect_riscv_info` /
`collect_all_info`) carries the same `derived: bool` field.

Whether this is a breaking change depends on which surface you consume:

- **JSON output**: additive. `derived` is a new field alongside `name`/`description`;
  existing consumers that only read the fields they already knew about are unaffected.
- **Rust API**: breaking. Both `ExtensionEntry` and `ExtensionInfo` are public structs
  with all-public fields and no `#[non_exhaustive]`, so any caller that constructs one by
  struct literal (`ExtensionEntry { name, description }`) no longer compiles once
  `derived` is a required field. This is why `riscfetch-core` is versioned 3.0.0 even
  though the JSON schema change alone would only warrant a minor bump.

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

- Spec version: 2.1
- Last updated: 2026-09-17
- Based on RISC-V ISA spec version: 2026-09 (Unprivileged/Privileged)
- Reference: LLVM mainline RISC-V extension support
