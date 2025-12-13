#!/bin/bash
# RISC-V Hardware Test Script for riscfetch
# Run this script on Orange Pi RV2 or other RISC-V hardware

set -e

echo "=== riscfetch RISC-V Hardware Test ==="
echo ""

# Check if we're on RISC-V
ARCH=$(uname -m)
echo "Architecture: $ARCH"

if [[ "$ARCH" != "riscv64" && "$ARCH" != "riscv32" ]]; then
    echo "WARNING: This doesn't appear to be RISC-V hardware!"
    echo "Expected riscv64 or riscv32, got $ARCH"
    exit 1
fi

echo "Confirmed: Running on RISC-V hardware"
echo ""

# Show relevant system files
echo "=== /proc/cpuinfo (first 50 lines) ==="
head -50 /proc/cpuinfo
echo ""

echo "=== ISA string from cpuinfo ==="
grep -i "isa" /proc/cpuinfo | head -1 || echo "Not found"
echo ""

echo "=== Hardware IDs from cpuinfo ==="
grep -E "(mvendorid|marchid|mimpid)" /proc/cpuinfo | head -3 || echo "Not found"
echo ""

echo "=== Device Tree Model ==="
cat /proc/device-tree/model 2>/dev/null || echo "Not found"
echo ""

echo "=== Running cargo test ==="
cd "$(dirname "$0")"
cargo test --workspace 2>&1

echo ""
echo "=== Running riscfetch (normal mode) ==="
cargo run --package riscfetch -- 2>&1 || true

echo ""
echo "=== Running riscfetch --explain ==="
cargo run --package riscfetch -- --explain 2>&1 || true

echo ""
echo "=== Running riscfetch --json ==="
cargo run --package riscfetch -- --json 2>&1 || true

echo ""
echo "=== Test completed ==="
