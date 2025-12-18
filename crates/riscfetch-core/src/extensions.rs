//! RISC-V extension definitions
//!
//! This module contains the constant definitions for all supported RISC-V extensions.
//! Based on RISC-V ISA specification (2025-11-26) and LLVM 22.0 support.

/// Standard extension definitions
/// Format: (char, name, description)
pub const STANDARD_EXTENSIONS: &[(char, &str, &str)] = &[
    ('i', "I", "Base Integer Instructions"),
    ('e', "E", "Embedded (16 registers)"),
    ('m', "M", "Integer Multiply/Divide"),
    ('a', "A", "Atomic Instructions"),
    ('f', "F", "Single-Precision Float"),
    ('d', "D", "Double-Precision Float"),
    ('q', "Q", "Quad-Precision Float"),
    ('c', "C", "Compressed (16-bit)"),
    ('b', "B", "Bit Manipulation"),
    ('v', "V", "Vector (SIMD)"),
    ('h', "H", "Hypervisor"),
];

/// Z-extension definitions (Unprivileged)
/// Format: (pattern, name, description, category)
pub const Z_EXTENSIONS: &[(&str, &str, &str, &str)] = &[
    // Base/CSR
    ("zicsr", "Zicsr", "CSR Instructions", "base"),
    ("zifencei", "Zifencei", "Instruction-Fetch Fence", "base"),
    ("zicntr", "Zicntr", "Base Counters/Timers", "base"),
    ("zihpm", "Zihpm", "Hardware Perf Counters", "base"),
    // Hints
    ("zihintpause", "Zihintpause", "Pause Hint", "hint"),
    ("zihintntl", "Zihintntl", "Non-Temporal Hints", "hint"),
    // Cache
    ("zicbom", "Zicbom", "Cache-Block Management", "cache"),
    ("zicboz", "Zicboz", "Cache-Block Zero", "cache"),
    ("zicbop", "Zicbop", "Cache-Block Prefetch", "cache"),
    // Conditional
    ("zicond", "Zicond", "Conditional Operations", "cond"),
    // Bit Manipulation
    ("zba", "Zba", "Address Generation", "bit"),
    ("zbb", "Zbb", "Basic Bit Manipulation", "bit"),
    ("zbc", "Zbc", "Carry-less Multiply", "bit"),
    ("zbs", "Zbs", "Single-bit Operations", "bit"),
    // Scalar Cryptography
    ("zbkb", "Zbkb", "Bit Manip for Crypto", "crypto"),
    ("zbkc", "Zbkc", "Carry-less for Crypto", "crypto"),
    ("zbkx", "Zbkx", "Crossbar for Crypto", "crypto"),
    ("zk", "Zk", "Scalar Crypto (All)", "crypto"),
    ("zkn", "Zkn", "NIST Algorithm Suite", "crypto"),
    ("zknd", "Zknd", "AES Decryption", "crypto"),
    ("zkne", "Zkne", "AES Encryption", "crypto"),
    ("zknh", "Zknh", "SHA-2 Hash", "crypto"),
    ("zks", "Zks", "ShangMi Suite", "crypto"),
    ("zksed", "Zksed", "SM4 Block Cipher", "crypto"),
    ("zksh", "Zksh", "SM3 Hash", "crypto"),
    ("zkr", "Zkr", "Entropy Source", "crypto"),
    ("zkt", "Zkt", "Data-Indep Timing", "crypto"),
    // Floating Point
    ("zfh", "Zfh", "Half-Precision Float", "fp"),
    ("zfhmin", "Zfhmin", "Minimal Half-Precision", "fp"),
    ("zfa", "Zfa", "Additional FP Instrs", "fp"),
    ("zfinx", "Zfinx", "Float in Int Regs", "fp"),
    ("zdinx", "Zdinx", "Double in Int Regs", "fp"),
    ("zhinx", "Zhinx", "Half in Int Regs", "fp"),
    ("zhinxmin", "Zhinxmin", "Min Half in Int Regs", "fp"),
    ("zfbfmin", "Zfbfmin", "Scalar BFloat16", "fp"),
    // Compressed
    ("zca", "Zca", "Compressed Base", "comp"),
    ("zcb", "Zcb", "Compressed Basic Ops", "comp"),
    ("zcd", "Zcd", "Compressed Double FP", "comp"),
    ("zcf", "Zcf", "Compressed Single FP", "comp"),
    ("zcmp", "Zcmp", "Compressed Push/Pop", "comp"),
    ("zcmt", "Zcmt", "Compressed Table Jump", "comp"),
    ("zcmop", "Zcmop", "Compressed May-Be-Ops", "comp"),
    ("zclsd", "Zclsd", "Compressed LD/SD Pair", "comp"),
    // Atomics
    ("zacas", "Zacas", "Atomic Compare-and-Swap", "atomic"),
    ("zabha", "Zabha", "Atomic Byte/Halfword", "atomic"),
    ("zaamo", "Zaamo", "Atomic AMO Subset", "atomic"),
    ("zalrsc", "Zalrsc", "Atomic LR/SC Subset", "atomic"),
    ("zawrs", "Zawrs", "Wait-on-Reservation-Set", "atomic"),
    // Memory Model
    ("za64rs", "Za64rs", "Reservation Set 64B", "mem"),
    ("za128rs", "Za128rs", "Reservation Set 128B", "mem"),
    ("zama16b", "Zama16b", "Misaligned Atomics 16B", "mem"),
    ("zic64b", "Zic64b", "64-byte Cache Block", "mem"),
    ("ziccamoa", "Ziccamoa", "Main Mem Atomics AMO", "mem"),
    ("ziccamoc", "Ziccamoc", "Main Mem Atomics CAS", "mem"),
    ("ziccif", "Ziccif", "Inst Fetch Coherence", "mem"),
    ("zicclsm", "Zicclsm", "Load/Store Misaligned", "mem"),
    ("ziccrse", "Ziccrse", "Reservation Set Size", "mem"),
    ("ztso", "Ztso", "Total Store Ordering", "mem"),
    // Multiply
    ("zmmul", "Zmmul", "Multiply Only (no Div)", "mul"),
    // Other
    ("zimop", "Zimop", "May-Be-Operations", "other"),
    ("zilsd", "Zilsd", "Load/Store Pair", "other"),
    // Vector
    ("zve32f", "Zve32f", "Vector 32-bit Float", "vec"),
    ("zve32x", "Zve32x", "Vector 32-bit Int", "vec"),
    ("zve64d", "Zve64d", "Vector 64-bit Double", "vec"),
    ("zve64f", "Zve64f", "Vector 64-bit Float", "vec"),
    ("zve64x", "Zve64x", "Vector 64-bit Int", "vec"),
    ("zvfh", "Zvfh", "Vector Half-Precision", "vec"),
    ("zvfhmin", "Zvfhmin", "Min Vector Half-Prec", "vec"),
    ("zvfbfmin", "Zvfbfmin", "Vector BFloat16 Conv", "vec"),
    ("zvfbfwma", "Zvfbfwma", "Vector BF16 Widen MA", "vec"),
    ("zvl32b", "Zvl32b", "VLEN >= 32 bits", "vec"),
    ("zvl64b", "Zvl64b", "VLEN >= 64 bits", "vec"),
    ("zvl128b", "Zvl128b", "VLEN >= 128 bits", "vec"),
    ("zvl256b", "Zvl256b", "VLEN >= 256 bits", "vec"),
    ("zvl512b", "Zvl512b", "VLEN >= 512 bits", "vec"),
    ("zvl1024b", "Zvl1024b", "VLEN >= 1024 bits", "vec"),
    ("zvl2048b", "Zvl2048b", "VLEN >= 2048 bits", "vec"),
    ("zvl4096b", "Zvl4096b", "VLEN >= 4096 bits", "vec"),
    ("zvl8192b", "Zvl8192b", "VLEN >= 8192 bits", "vec"),
    ("zvl16384b", "Zvl16384b", "VLEN >= 16384 bits", "vec"),
    ("zvl32768b", "Zvl32768b", "VLEN >= 32768 bits", "vec"),
    ("zvl65536b", "Zvl65536b", "VLEN >= 65536 bits", "vec"),
    // Vector Cryptography
    ("zvbb", "Zvbb", "Vector Bit Manipulation", "vcrypto"),
    ("zvbc", "Zvbc", "Vector Carry-less Mul", "vcrypto"),
    ("zvkb", "Zvkb", "Vector Crypto Bit Manip", "vcrypto"),
    ("zvkg", "Zvkg", "Vector GCM/GMAC", "vcrypto"),
    ("zvkn", "Zvkn", "Vector NIST (All)", "vcrypto"),
    ("zvknc", "Zvknc", "Vector NIST+Carryless", "vcrypto"),
    ("zvkned", "Zvkned", "Vector AES", "vcrypto"),
    ("zvkng", "Zvkng", "Vector NIST+GCM", "vcrypto"),
    ("zvknha", "Zvknha", "Vector SHA-2 (256)", "vcrypto"),
    ("zvknhb", "Zvknhb", "Vector SHA-2 (512)", "vcrypto"),
    ("zvks", "Zvks", "Vector ShangMi (All)", "vcrypto"),
    ("zvksc", "Zvksc", "Vector SM+Carryless", "vcrypto"),
    ("zvksed", "Zvksed", "Vector SM4", "vcrypto"),
    ("zvksg", "Zvksg", "Vector SM+GCM", "vcrypto"),
    ("zvksh", "Zvksh", "Vector SM3", "vcrypto"),
    ("zvkt", "Zvkt", "Vector Data-Indep Time", "vcrypto"),
];

/// S-extension definitions (Privileged/Supervisor)
/// Format: (pattern, name, description, category)
pub const S_EXTENSIONS: &[(&str, &str, &str, &str)] = &[
    // Virtual Memory (Sv*)
    ("svinval", "Svinval", "Fine-Grained TLB Inv", "vm"),
    ("svnapot", "Svnapot", "NAPOT Translation", "vm"),
    ("svpbmt", "Svpbmt", "Page-Based Mem Types", "vm"),
    ("svade", "Svade", "A/D Update on Fault", "vm"),
    ("svadu", "Svadu", "A/D Hardware Update", "vm"),
    ("svbare", "Svbare", "Bare Translation Mode", "vm"),
    ("svvptc", "Svvptc", "VPTC Invalidation", "vm"),
    // Supervisor (Ss*)
    ("ssaia", "Ssaia", "Adv Interrupt Arch", "sup"),
    ("ssccfg", "Ssccfg", "Counter Config", "sup"),
    ("ssccptr", "Ssccptr", "Common Ptr Convention", "sup"),
    ("sscofpmf", "Sscofpmf", "Count Overflow/Filter", "sup"),
    ("sscounterenw", "Sscounterenw", "Counter Enables", "sup"),
    ("sscsrind", "Sscsrind", "Indirect CSR Access", "sup"),
    ("ssctr", "Ssctr", "Control Transfer Rec", "sup"),
    ("ssdbltrp", "Ssdbltrp", "Double Trap", "sup"),
    ("ssnpm", "Ssnpm", "Pointer Masking", "sup"),
    ("sspm", "Sspm", "Pointer Masking", "sup"),
    ("ssqosid", "Ssqosid", "QoS Identifiers", "sup"),
    ("ssstateen", "Ssstateen", "State Enable", "sup"),
    ("ssstrict", "Ssstrict", "No Non-Conforming Ext", "sup"),
    ("sstc", "Sstc", "Supervisor Timer", "sup"),
    ("sstvala", "Sstvala", "Trap Value Address", "sup"),
    ("sstvecd", "Sstvecd", "Trap Vector Mode", "sup"),
    ("ssu64xl", "Ssu64xl", "U-mode 64-bit", "sup"),
    // Machine (Sm*)
    ("smaia", "Smaia", "Adv Interrupt Arch", "mach"),
    ("smcdeleg", "Smcdeleg", "Counter Delegation", "mach"),
    ("smcntrpmf", "Smcntrpmf", "Counter PMF", "mach"),
    ("smcsrind", "Smcsrind", "Indirect CSR Access", "mach"),
    ("smctr", "Smctr", "Control Transfer Rec", "mach"),
    ("smdbltrp", "Smdbltrp", "Double Trap", "mach"),
    ("smepmp", "Smepmp", "Enhanced PMP", "mach"),
    ("smmpm", "Smmpm", "M-mode Ptr Masking", "mach"),
    ("smnpm", "Smnpm", "Nesting Ptr Masking", "mach"),
    ("smrnmi", "Smrnmi", "Resumable NMI", "mach"),
    ("smstateen", "Smstateen", "State Enable", "mach"),
    // Hypervisor (Sh*)
    ("sha", "Sha", "H-mode Ext Subset", "hyp"),
    ("shcounterenw", "Shcounterenw", "Counter Enables", "hyp"),
    ("shgatpa", "Shgatpa", "Guest Addr Translation", "hyp"),
    (
        "shlcofideleg",
        "Shlcofideleg",
        "Lcof Interrupt Deleg",
        "hyp",
    ),
    ("shtvala", "Shtvala", "H-mode Trap Value", "hyp"),
    ("shvsatpa", "Shvsatpa", "VS-mode Saturation", "hyp"),
    ("shvstvala", "Shvstvala", "VS-mode Trap Value", "hyp"),
    ("shvstvecd", "Shvstvecd", "VS-mode Trap Vector", "hyp"),
    // Debug (Sd*)
    ("sdext", "Sdext", "External Debug", "debug"),
    ("sdtrig", "Sdtrig", "Debug Triggers", "debug"),
    // User (Su*)
    ("supm", "Supm", "U-mode Ptr Masking", "user"),
];

/// Category display names for Z-extensions
pub const Z_CATEGORY_NAMES: &[(&str, &str)] = &[
    ("base", "Base"),
    ("hint", "Hints"),
    ("cache", "Cache"),
    ("cond", "Conditional"),
    ("bit", "Bit Manipulation"),
    ("crypto", "Cryptography"),
    ("fp", "Floating Point"),
    ("comp", "Compressed"),
    ("atomic", "Atomics"),
    ("mem", "Memory Model"),
    ("mul", "Multiply"),
    ("vec", "Vector"),
    ("vcrypto", "Vector Crypto"),
    ("other", "Other"),
];

/// Category display names for S-extensions
pub const S_CATEGORY_NAMES: &[(&str, &str)] = &[
    ("vm", "Virtual Memory"),
    ("sup", "Supervisor"),
    ("mach", "Machine"),
    ("hyp", "Hypervisor"),
    ("debug", "Debug"),
    ("user", "User"),
];
