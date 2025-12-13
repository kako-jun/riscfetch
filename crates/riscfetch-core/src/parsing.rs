//! ISA string parsing functions (pure functions for testing)

/// Standard extension definitions
pub const STANDARD_EXTENSIONS: [(&char, &str, &str); 11] = [
    (&'i', "I", "Base Integer Instructions"),
    (&'e', "E", "Embedded (16 registers)"),
    (&'m', "M", "Integer Multiply/Divide"),
    (&'a', "A", "Atomic Instructions"),
    (&'f', "F", "Single-Precision Float"),
    (&'d', "D", "Double-Precision Float"),
    (&'q', "Q", "Quad-Precision Float"),
    (&'c', "C", "Compressed (16-bit)"),
    (&'b', "B", "Bit Manipulation"),
    (&'v', "V", "Vector (SIMD)"),
    (&'h', "H", "Hypervisor"),
];

/// Z-extension definitions
pub const Z_EXTENSIONS: [(&str, &str, &str); 38] = [
    ("zicsr", "Zicsr", "CSR Instructions"),
    ("zifencei", "Zifencei", "Instruction-Fetch Fence"),
    ("zicntr", "Zicntr", "Base Counters/Timers"),
    ("zihpm", "Zihpm", "Hardware Perf Counters"),
    ("zicbom", "Zicbom", "Cache-Block Management"),
    ("zicboz", "Zicboz", "Cache-Block Zero"),
    ("zicond", "Zicond", "Conditional Operations"),
    ("zihintpause", "Zihintpause", "Pause Hint"),
    ("zba", "Zba", "Address Generation"),
    ("zbb", "Zbb", "Basic Bit Manipulation"),
    ("zbc", "Zbc", "Carry-less Multiply"),
    ("zbs", "Zbs", "Single-bit Operations"),
    ("zbkb", "Zbkb", "Bit Manip for Crypto"),
    ("zbkc", "Zbkc", "Carry-less for Crypto"),
    ("zbkx", "Zbkx", "Crossbar for Crypto"),
    ("zfh", "Zfh", "Half-Precision Float"),
    ("zfhmin", "Zfhmin", "Minimal Half-Precision"),
    ("zkt", "Zkt", "Constant-Time Execution"),
    ("zca", "Zca", "Compressed Base"),
    ("zcb", "Zcb", "Compressed Basic Ops"),
    ("zcd", "Zcd", "Compressed Double FP"),
    ("zcf", "Zcf", "Compressed Single FP"),
    ("zve32f", "Zve32f", "Vector 32-bit Float"),
    ("zve32x", "Zve32x", "Vector 32-bit Int"),
    ("zve64d", "Zve64d", "Vector 64-bit Double"),
    ("zve64f", "Zve64f", "Vector 64-bit Float"),
    ("zve64x", "Zve64x", "Vector 64-bit Int"),
    ("zvfh", "Zvfh", "Vector Half-Precision"),
    ("zvfhmin", "Zvfhmin", "Min Vector Half-Prec"),
    ("zvkt", "Zvkt", "Vector Constant-Time"),
    ("zvl128b", "Zvl128b", "VLEN >= 128 bits"),
    ("zvl256b", "Zvl256b", "VLEN >= 256 bits"),
    ("zvl512b", "Zvl512b", "VLEN >= 512 bits"),
    ("svinval", "Svinval", "Fine-Grained TLB"),
    ("svnapot", "Svnapot", "NAPOT Translation"),
    ("svpbmt", "Svpbmt", "Page-Based Mem Types"),
    ("sscofpmf", "Sscofpmf", "Count Overflow/Filter"),
    ("sstc", "Sstc", "Supervisor Timer"),
];

/// Strip rv32/rv64 prefix from ISA base part to get extension letters only
pub fn strip_rv_prefix(base: &str) -> &str {
    base.strip_prefix("rv64")
        .or_else(|| base.strip_prefix("rv32"))
        .unwrap_or(base)
}

/// Parse extensions from ISA string (pure function for testing)
pub fn parse_extensions_compact(isa: &str) -> String {
    let isa = isa.to_lowercase();
    let mut exts = Vec::new();

    // Get the base part before any underscore
    let base = isa.split('_').next().unwrap_or(&isa);
    let ext_part = strip_rv_prefix(base);

    // G is shorthand for IMAFD (per RISC-V spec)
    let has_g = ext_part.contains('g');

    // Standard extensions in canonical order
    // Note: E and I are mutually exclusive
    let standard = [
        ('i', "I", false), // (char, name, implied_by_g)
        ('e', "E", false), // E = embedded (16 registers)
        ('m', "M", true),
        ('a', "A", true),
        ('f', "F", true),
        ('d', "D", true),
        ('q', "Q", false),
        ('c', "C", false),
        ('b', "B", false),
        ('v', "V", false),
        ('h', "H", false),
    ];

    for (ch, name, implied_by_g) in standard {
        if ext_part.contains(ch) || (has_g && implied_by_g) {
            exts.push(name);
        }
    }

    // If G is present but I wasn't explicitly added, add I (G implies IMAFD)
    if has_g && !exts.contains(&"I") && !exts.contains(&"E") {
        exts.insert(0, "I");
    }

    exts.join(" ")
}

/// Parse Z-extensions from ISA string (pure function for testing)
pub fn parse_z_extensions(isa: &str) -> String {
    let isa = isa.to_lowercase();
    let mut z_exts = Vec::new();

    // Check if G is present (G implies Zicsr_Zifencei per RISC-V spec)
    let base = isa.split('_').next().unwrap_or(&isa);
    let ext_part = strip_rv_prefix(base);
    let has_g = ext_part.contains('g');

    // Add implied Z-extensions from G
    if has_g {
        z_exts.push("zicsr".to_string());
        z_exts.push("zifencei".to_string());
    }

    // Add explicit Z-extensions and S-extensions
    for part in isa.split('_') {
        if (part.starts_with('z') || part.starts_with('s')) && !z_exts.contains(&part.to_string()) {
            z_exts.push(part.to_string());
        }
    }

    z_exts.join(" ")
}

/// Parse extensions with explanations (pure function for testing)
pub fn parse_extensions_explained(isa: &str) -> Vec<(String, String)> {
    let isa = isa.to_lowercase();
    let base = isa.split('_').next().unwrap_or(&isa);
    let ext_part = strip_rv_prefix(base);
    let mut exts = Vec::new();

    for (ch, name, desc) in STANDARD_EXTENSIONS {
        if ext_part.contains(*ch) {
            exts.push((name.to_string(), desc.to_string()));
        }
    }

    exts
}

/// Parse Z-extensions with explanations (pure function for testing)
pub fn parse_z_extensions_explained(isa: &str) -> Vec<(String, String)> {
    let isa = isa.to_lowercase();
    let mut z_exts = Vec::new();

    for (pattern, name, desc) in Z_EXTENSIONS {
        if isa.contains(pattern) {
            z_exts.push((name.to_string(), desc.to_string()));
        }
    }

    z_exts
}

/// Parse vector details from ISA string (pure function for testing)
/// Returns None if no vector extension, Some(details) otherwise
pub fn parse_vector_from_isa(isa: &str) -> Option<String> {
    let isa = isa.to_lowercase();
    let base = isa.split('_').next().unwrap_or(&isa);
    let ext_part = strip_rv_prefix(base);

    // Check for V extension in the extension part, or zve in Z-extensions
    if !ext_part.contains('v') && !isa.contains("zve") {
        return None;
    }

    let mut details = vec!["Enabled".to_string()];

    // Detect VLEN from zvl* extensions (use largest value)
    // If no zvl* specified, VLEN is implementation-defined (do not display)
    if isa.contains("zvl1024b") {
        details.push("VLEN>=1024".to_string());
    } else if isa.contains("zvl512b") {
        details.push("VLEN>=512".to_string());
    } else if isa.contains("zvl256b") {
        details.push("VLEN>=256".to_string());
    } else if isa.contains("zvl128b") {
        details.push("VLEN>=128".to_string());
    } else if isa.contains("zvl64b") {
        details.push("VLEN>=64".to_string());
    } else if isa.contains("zvl32b") {
        details.push("VLEN>=32".to_string());
    }
    // No default VLEN - it's implementation-defined per RISC-V spec

    Some(details.join(", "))
}
