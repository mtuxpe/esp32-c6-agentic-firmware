#!/bin/bash
# test-single-lesson-reliable.sh - Reliable single lesson build test
# Usage: bash test-single-lesson-reliable.sh <lesson_path>
# Example: bash test-single-lesson-reliable.sh lessons/01-button-neopixel

set -euo pipefail

LESSON_PATH="${1:-}"
REPO_ROOT="/Users/shanemattner/Desktop/esp32-c6-agentic-firmware"

# === Input Validation ===
if [ -z "$LESSON_PATH" ]; then
    echo "Usage: $0 <lesson_path>"
    echo ""
    echo "Examples:"
    echo "  $0 lessons/01-button-neopixel"
    echo "  $0 lessons/08-uart-gdb-tandem"
    exit 1
fi

# === Environment Verification ===
echo "=== Environment Check ==="

# Check we're in repo root (or navigate there)
if [ ! -f "$REPO_ROOT/CLAUDE.md" ]; then
    echo "❌ Repository not found at: $REPO_ROOT"
    echo "Current directory: $(pwd)"
    exit 1
fi

cd "$REPO_ROOT"
echo "✅ Repository root: $REPO_ROOT"

# Check lesson exists
if [ ! -f "$LESSON_PATH/Cargo.toml" ]; then
    echo "❌ Lesson not found: $LESSON_PATH"
    echo "Available lessons:"
    ls -1 lessons/ | head -10
    exit 1
fi
echo "✅ Lesson found: $LESSON_PATH"

# Check toolchain
if ! command -v cargo &>/dev/null; then
    echo "❌ cargo not in PATH"
    exit 1
fi
RUSTC_VERSION=$(rustc --version | head -1)
echo "✅ Toolchain: $RUSTC_VERSION"

echo ""

# === Test Execution ===
LESSON_NAME=$(basename "$LESSON_PATH")
LOG_DIR="/tmp/lesson-test-logs"
LOG_FILE="$LOG_DIR/${LESSON_NAME}.log"
TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S')

mkdir -p "$LOG_DIR"

echo "=== Testing: $LESSON_NAME ==="
echo "Started: $TIMESTAMP"
echo "Log file: $LOG_FILE"
echo ""

# Write test header to log
{
    echo "=============================================="
    echo "Lesson Test: $LESSON_NAME"
    echo "Timestamp: $TIMESTAMP"
    echo "=============================================="
    echo ""
    echo "Environment:"
    echo "  Working dir: $(pwd)"
    echo "  Rust: $RUSTC_VERSION"
    echo "  Lesson path: $LESSON_PATH"
    echo ""
} > "$LOG_FILE"

# Clean previous build (idempotent)
echo "[1/3] Cleaning previous build..."
cargo clean --manifest-path "$LESSON_PATH/Cargo.toml" &>>"$LOG_FILE"
echo "      ✅ Clean complete"

# Build with full output capture
echo "[2/3] Building release binary..."
echo ""

if cargo build --release --manifest-path "$LESSON_PATH/Cargo.toml" 2>&1 | tee -a "$LOG_FILE"; then
    BUILD_SUCCESS=1
    echo ""
    echo "[3/3] Build succeeded ✅"
    echo ""

    # Find and report binary
    BINARY=$(find "$LESSON_PATH/target/riscv32imac-unknown-none-elf/release/" \
             -type f -perm +111 -name main 2>/dev/null | head -1)

    if [ -n "$BINARY" ]; then
        SIZE=$(ls -lh "$BINARY" | awk '{print $5}')
        echo "📦 Binary Information:"
        echo "   Path: $BINARY"
        echo "   Size: $SIZE"

        # Detailed size info if available
        if command -v size &>/dev/null; then
            echo ""
            echo "   Memory usage:"
            size "$BINARY" | tail -1 | awk '{printf "   text: %s bytes\n   data: %s bytes\n   bss:  %s bytes\n", $1, $2, $3}'
        fi
    else
        echo "⚠️  Warning: Binary not found in target directory"
        echo "   Expected: $LESSON_PATH/target/riscv32imac-unknown-none-elf/release/main"
    fi

    echo ""
    echo "=== Test Result: SUCCESS ✅ ==="
    echo "Full log: $LOG_FILE"

    exit 0
else
    BUILD_SUCCESS=0
    EXIT_CODE=${PIPESTATUS[0]}

    echo ""
    echo "[3/3] Build failed ❌"
    echo ""
    echo "❌ Build Result: FAILED (exit code: $EXIT_CODE)"
    echo ""
    echo "Error summary (first 15 errors):"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    grep -E 'error(\[E[0-9]+\])?:' "$LOG_FILE" | head -15
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    echo "Troubleshooting:"
    echo "  1. View full log: cat $LOG_FILE"
    echo "  2. Try updating deps: cargo update --manifest-path $LESSON_PATH/Cargo.toml"
    echo "  3. Check Rust version: rustc --version"
    echo "  4. Verify Cargo.lock age: ls -lh $LESSON_PATH/Cargo.lock"
    echo ""
    echo "=== Test Result: FAILURE ❌ ==="
    echo "Full log: $LOG_FILE"

    exit $EXIT_CODE
fi
