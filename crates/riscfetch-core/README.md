# riscfetch-core

RISC-V system information library. Query ISA extensions, hardware IDs, vector capabilities, and more.

## Usage

```rust
use riscfetch_core::*;

if is_riscv() {
    println!("ISA: {}", get_isa_string());
    println!("Extensions: {}", get_extensions_compact());
    println!("Harts: {}", get_hart_count());

    let hw = get_hardware_ids();
    println!("Vendor: {}", hw.mvendorid);
}
```

## Functions

| Function | Returns |
|----------|---------|
| `is_riscv()` | `bool` - architecture check |
| `get_isa_string()` | Full ISA string from /proc/cpuinfo |
| `get_extensions_compact()` | "I M A F D C V" format |
| `get_extensions_explained()` | Vec of (name, description) |
| `get_z_extensions()` | Z-extension string |
| `get_z_extensions_explained()` | Vec of (name, description) |
| `get_vector_detail()` | VLEN info if V extension present |
| `get_hardware_ids()` | HardwareIds struct |
| `get_hart_count()` | Formatted string "N harts" |
| `get_cache_info()` | L1D/L1I/L2/L3 sizes |
| `collect_riscv_info()` | RiscvInfo struct (RISC-V specific only) |
| `collect_all_info()` | SystemInfo struct (includes system info) |

## License

MIT
