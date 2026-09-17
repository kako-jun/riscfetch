//! Extension implication and composition tables
//!
//! `/proc/cpuinfo` only lists the extensions the kernel bothered to enumerate. It does
//! not compute the extensions that are *implied* by those (e.g. `M` implies `Zmmul`),
//! nor does it collapse a set of extensions into the shorthand name the ISA manual
//! defines for their union (e.g. `Zba`+`Zbb`+`Zbs` is, by definition, `B`). This module
//! computes both directions so riscfetch can show extensions that are genuinely present
//! even when the kernel didn't spell them out (issue #10).
//!
//! ## Sources
//!
//! - **Implication** entries are taken from the `Implies` field of each
//!   `RISCVExtension`/`RISCVExperimentalExtension` definition in LLVM mainline
//!   (`llvm/lib/Target/RISCV/RISCVFeatures.td`, fetched 2026-09-17). Each entry below is
//!   commented with the corresponding `FeatureStdExt*` definition it was read from.
//! - **Composition** entries mark extension names that the RISC-V ISA manual defines as
//!   pure shorthand/union notation for a fixed set of other extensions (no additional
//!   behavior of their own), so the presence of every member of the set means the
//!   shorthand name is present too, even if the ISA string never spells it out. The
//!   member sets are the same sets LLVM uses for the forward `Implies` direction of
//!   those shorthand extensions.
//! - Entries that could not be confirmed against the LLVM table, or whose implication
//!   is conditional on rv32 vs rv64 in a way this table can't express (e.g. `Zcf` is
//!   only legal on rv32), are intentionally left out. Missing coverage is preferred over
//!   a false positive.

/// Forward implication: if the extension named by `.0` is present, every extension
/// named in `.1` is implied to be present too, regardless of whether the ISA string
/// mentions it directly.
#[allow(clippy::type_complexity)]
pub const IMPLICATIONS: &[(&str, &[&str])] = &[
    // --- Integer / CSR ---
    ("M", &["Zmmul"]),       // FeatureStdExtM -> Zmmul
    ("Zicntr", &["Zicsr"]),  // FeatureStdExtZicntr -> Zicsr
    ("Ziccid", &["Ziccif"]), // FeatureStdExtZiccid -> Ziccif
    // --- Floating point ---
    ("F", &["Zicsr"]),                // FeatureStdExtF -> Zicsr
    ("D", &["F"]),                    // FeatureStdExtD -> F
    ("Q", &["D"]),                    // FeatureStdExtQ -> D
    ("Zfh", &["Zfhmin"]),             // FeatureStdExtZfh -> Zfhmin
    ("Zfhmin", &["F"]),               // FeatureStdExtZfhmin -> F
    ("Zvfh", &["Zvfhmin", "Zfhmin"]), // FeatureStdExtZvfh -> Zvfhmin, Zfhmin
    ("Zvfhmin", &["Zve32f"]),         // FeatureStdExtZvfhmin -> Zve32f
    ("Zfinx", &["Zicsr"]),            // FeatureStdExtZfinx -> Zicsr
    ("Zdinx", &["Zfinx"]),            // FeatureStdExtZdinx -> Zfinx
    ("Zhinx", &["Zhinxmin"]),         // FeatureStdExtZhinx -> Zhinxmin
    // --- Atomics ---
    ("A", &["Zaamo", "Zalrsc"]), // FeatureStdExtA -> Zaamo, Zalrsc
    ("Zabha", &["Zaamo"]),       // FeatureStdExtZabha -> Zaamo
    ("Zacas", &["Zaamo"]),       // FeatureStdExtZacas -> Zaamo
    // --- Compressed ---
    ("C", &["Zca"]),                          // FeatureStdExtC -> Zca
    ("Zcb", &["Zca"]),                        // FeatureStdExtZcb -> Zca
    ("Zcf", &["F", "Zca"]),                   // FeatureStdExtZcf -> F, Zca
    ("Zcd", &["D", "Zca"]),                   // FeatureStdExtZcd -> D, Zca
    ("Zce", &["Zca", "Zcb", "Zcmp", "Zcmt"]), // FeatureStdExtZce -> Zca, Zcb, Zcmp, Zcmt
    // --- Bit manipulation / scalar crypto ---
    ("B", &["Zba", "Zbb", "Zbs"]),  // FeatureStdExtB -> Zba, Zbb, Zbs
    ("Zbc", &["Zbkc"]),             // FeatureStdExtZbc -> Zbkc
    ("Zk", &["Zkn", "Zkr", "Zkt"]), // FeatureStdExtZk -> Zkn, Zkr, Zkt
    ("Zkn", &["Zbkb", "Zbkc", "Zbkx", "Zkne", "Zknd", "Zknh"]), // FeatureStdExtZkn -> Zbkb, Zbkc, Zbkx, Zkne, Zknd, Zknh
    ("Zks", &["Zbkb", "Zbkc", "Zbkx", "Zksed", "Zksh"]), // FeatureStdExtZks -> Zbkb, Zbkc, Zbkx, Zksed, Zksh
    // --- Vector ---
    ("V", &["Zve64d", "Zvl128b"]), // FeatureStdExtV -> Zve64d, Zvl128b
    ("Zve64d", &["Zve64f", "D"]),  // FeatureStdExtZve64d -> Zve64f, D
    ("Zve64f", &["Zve32f", "Zve64x"]), // FeatureStdExtZve64f -> Zve32f, Zve64x
    ("Zve64x", &["Zve32x", "Zvl64b"]), // FeatureStdExtZve64x -> Zve32x, Zvl64b
    ("Zve32f", &["Zve32x", "F"]),  // FeatureStdExtZve32f -> Zve32x, F
    ("Zvbb", &["Zvkb"]),           // FeatureStdExtZvbb -> Zvkb
    ("Zvbc", &["Zve64x"]),         // FeatureStdExtZvbc -> Zve64x
    ("Zvkb", &["Zve32x"]),         // FeatureStdExtZvkb -> Zve32x
    ("Zvkned", &["Zve32x"]),       // FeatureStdExtZvkned -> Zve32x
    ("Zvknha", &["Zve32x"]),       // FeatureStdExtZvknha -> Zve32x
    ("Zvknhb", &["Zve64x", "Zvknha"]), // FeatureStdExtZvknhb -> Zve64x, Zvknha
    ("Zvkn", &["Zvkned", "Zvknhb", "Zvkb", "Zvkt"]), // FeatureStdExtZvkn -> Zvkned, Zvknhb, Zvkb, Zvkt
    ("Zvks", &["Zvksed", "Zvksh", "Zvkb", "Zvkt"]), // FeatureStdExtZvks -> Zvksed, Zvksh, Zvkb, Zvkt
    // Zvl chain: a larger guaranteed minimum VLEN implies every smaller minimum too.
    ("Zvl65536b", &["Zvl32768b"]),
    ("Zvl32768b", &["Zvl16384b"]),
    ("Zvl16384b", &["Zvl8192b"]),
    ("Zvl8192b", &["Zvl4096b"]),
    ("Zvl4096b", &["Zvl2048b"]),
    ("Zvl2048b", &["Zvl1024b"]),
    ("Zvl1024b", &["Zvl512b"]),
    ("Zvl512b", &["Zvl256b"]),
    ("Zvl256b", &["Zvl128b"]),
    ("Zvl128b", &["Zvl64b"]),
    ("Zvl64b", &["Zvl32b"]),
];

/// Composition (converse of a shorthand extension): if every extension in `.1` is
/// present, the ISA manual defines `.0` as pure shorthand for that exact set, so it is
/// considered present too.
pub const COMPOSITIONS: &[(&str, &[&str])] = &[
    ("A", &["Zaamo", "Zalrsc"]),
    ("B", &["Zba", "Zbb", "Zbs"]),
    ("Zce", &["Zca", "Zcb", "Zcmp", "Zcmt"]),
    ("Zk", &["Zkn", "Zkr", "Zkt"]),
    ("Zkn", &["Zbkb", "Zbkc", "Zbkx", "Zkne", "Zknd", "Zknh"]),
    ("Zks", &["Zbkb", "Zbkc", "Zbkx", "Zksed", "Zksh"]),
    ("Zvkn", &["Zvkned", "Zvknhb", "Zvkb", "Zvkt"]),
    ("Zvks", &["Zvksed", "Zvksh", "Zvkb", "Zvkt"]),
];

use std::collections::BTreeSet;

/// Given the set of extension names already known to be present (canonical casing,
/// e.g. `"M"`, `"Zba"`, `"Sstc"`), compute the set of additional extension names that
/// become present through implication or composition.
///
/// This resolves the full transitive closure with a worklist (`while changed`), so
/// implication chains (`V` -> `Zve64d` -> ... -> `Zve32x`) and composition chains are
/// fully followed. The worklist only ever inserts names into a set, so it terminates
/// even if the tables ever contained a cycle.
#[must_use]
pub fn compute_derived(explicit: &BTreeSet<String>) -> BTreeSet<String> {
    let mut known: BTreeSet<String> = explicit.clone();
    let mut derived: BTreeSet<String> = BTreeSet::new();

    loop {
        let mut changed = false;

        for &(name, implies) in IMPLICATIONS {
            if known.contains(name) {
                for &imp in implies {
                    if known.insert(imp.to_string()) {
                        derived.insert(imp.to_string());
                        changed = true;
                    }
                }
            }
        }

        for &(name, requires) in COMPOSITIONS {
            if !known.contains(name) && requires.iter().all(|r| known.contains(*r)) {
                known.insert(name.to_string());
                derived.insert(name.to_string());
                changed = true;
            }
        }

        if !changed {
            break;
        }
    }

    derived
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set(names: &[&str]) -> BTreeSet<String> {
        names.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn m_implies_zmmul() {
        let derived = compute_derived(&set(&["M"]));
        assert!(derived.contains("Zmmul"));
    }

    #[test]
    fn zba_zbb_zbs_compose_into_b() {
        let derived = compute_derived(&set(&["Zba", "Zbb", "Zbs"]));
        assert!(derived.contains("B"));
    }

    #[test]
    fn zba_alone_does_not_compose_into_b() {
        let derived = compute_derived(&set(&["Zba"]));
        assert!(!derived.contains("B"));
    }

    #[test]
    fn v_transitively_implies_zve32x() {
        let derived = compute_derived(&set(&["V"]));
        assert!(derived.contains("Zve64d"));
        assert!(derived.contains("Zve64f"));
        assert!(derived.contains("Zve32f"));
        assert!(derived.contains("Zve32x"));
        assert!(derived.contains("Zve64x"));
        assert!(derived.contains("D"));
        assert!(derived.contains("F"));
        assert!(derived.contains("Zicsr"));
    }

    #[test]
    fn explicit_extensions_are_not_marked_derived() {
        // If B is already explicit, it must not appear in the derived set even though
        // its components are also present.
        let derived = compute_derived(&set(&["B", "Zba", "Zbb", "Zbs"]));
        assert!(!derived.contains("B"));
        assert!(!derived.contains("Zba"));
    }

    #[test]
    fn no_infinite_loop_on_empty_input() {
        let derived = compute_derived(&BTreeSet::new());
        assert!(derived.is_empty());
    }
}
