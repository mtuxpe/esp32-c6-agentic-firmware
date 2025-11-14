#!/bin/bash
# verify-test-environment.sh - Verify testing prerequisites
# Usage: bash verify-test-environment.sh

set -euo pipefail

REPO_ROOT="/Users/shanemattner/Desktop/esp32-c6-agentic-firmware"

echo "=== Test Environment Verification ==="
echo ""

# Check 1: Working Directory
echo "Check 1: Working Directory"
CURRENT_DIR=$(pwd)
if [ "$CURRENT_DIR" = "$REPO_ROOT" ]; then
    echo "✅ Correct: $CURRENT_DIR"
else
    echo "❌ Wrong directory"
    echo "   Current:  $CURRENT_DIR"
    echo "   Expected: $REPO_ROOT"
    echo ""
    echo "Fix: cd $REPO_ROOT"
    exit 1
fi
echo ""

# Check 2: Repository Structure
echo "Check 2: Repository Structure"
REQUIRED_DIRS=("lessons" ".claude" "scripts")
REQUIRED_FILES=("CLAUDE.md" "README.md")

for dir in "${REQUIRED_DIRS[@]}"; do
    if [ -d "$dir" ]; then
        echo "✅ Directory exists: $dir/"
    else
        echo "❌ Missing directory: $dir/"
        exit 1
    fi
done

for file in "${REQUIRED_FILES[@]}"; do
    if [ -f "$file" ]; then
        echo "✅ File exists: $file"
    else
        echo "❌ Missing file: $file"
        exit 1
    fi
done
echo ""

# Check 3: Rust Toolchain
echo "Check 3: Rust Toolchain"
if command -v rustc &>/dev/null; then
    RUSTC_VERSION=$(rustc --version)
    echo "✅ rustc: $RUSTC_VERSION"
else
    echo "❌ rustc not found in PATH"
    exit 1
fi

if command -v cargo &>/dev/null; then
    CARGO_VERSION=$(cargo --version)
    echo "✅ cargo: $CARGO_VERSION"
else
    echo "❌ cargo not found in PATH"
    exit 1
fi
echo ""

# Check 4: Lesson Count
echo "Check 4: Lesson Count"
LESSON_COUNT=$(find lessons -maxdepth 1 -type d -name "[0-9]*" | wc -l | tr -d ' ')
echo "✅ Found $LESSON_COUNT lessons"
echo ""

# Check 5: Active Processes
echo "Check 5: Active Build Processes"
if pgrep -f "cargo build" >/dev/null; then
    echo "⚠️  Warning: cargo build process already running"
    echo "   PIDs: $(pgrep -f 'cargo build' | tr '\n' ' ')"
    echo "   Consider killing with: pkill -f 'cargo build'"
else
    echo "✅ No conflicting cargo processes"
fi
echo ""

# Check 6: Log Directory
echo "Check 6: Test Log Directory"
LOG_DIR="/tmp/lesson-test-logs"
mkdir -p "$LOG_DIR"
if [ -w "$LOG_DIR" ]; then
    echo "✅ Log directory: $LOG_DIR (writable)"
else
    echo "❌ Log directory not writable: $LOG_DIR"
    exit 1
fi
echo ""

# Check 7: RISCV Target
echo "Check 7: RISC-V Target"
if rustup target list --installed | grep -q "riscv32imac-unknown-none-elf"; then
    echo "✅ Target installed: riscv32imac-unknown-none-elf"
else
    echo "⚠️  Target not installed: riscv32imac-unknown-none-elf"
    echo "   Install with: rustup target add riscv32imac-unknown-none-elf"
fi
echo ""

echo "=== Environment Verification Complete ==="
echo ""
echo "Status: All checks passed ✅"
echo "Ready to run lesson tests."
