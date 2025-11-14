#!/usr/bin/env bash
# Comprehensive repository cleanup script
# Removes build artifacts, temporary files, and caches

set -e

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo "=== Repository Cleanup ==="
echo "Root: $REPO_ROOT"
echo

# Function to calculate and display size
calc_size() {
    local paths="$1"
    if [ -n "$paths" ]; then
        du -ch $paths 2>/dev/null | grep total | awk '{print $1}'
    else
        echo "0B"
    fi
}

# 1. Clean Cargo target directories
echo "[1/4] Cargo target directories"
TARGET_DIRS=$(find "$REPO_ROOT/lessons" -type d -name "target" 2>/dev/null || true)
if [ -n "$TARGET_DIRS" ]; then
    SIZE=$(calc_size "$TARGET_DIRS")
    echo "  Found: $(echo "$TARGET_DIRS" | wc -l) directories ($SIZE total)"
    for dir in $TARGET_DIRS; do
        echo "  Removing: $dir"
        rm -rf "$dir"
    done
else
    echo "  None found"
fi
echo

# 2. Clean Cargo.lock files (not needed in git, regenerated on build)
echo "[2/4] Cargo.lock files"
LOCK_FILES=$(find "$REPO_ROOT/lessons" -type f -name "Cargo.lock" 2>/dev/null || true)
if [ -n "$LOCK_FILES" ]; then
    echo "  Found: $(echo "$LOCK_FILES" | wc -l) files"
    for file in $LOCK_FILES; do
        echo "  Removing: $file"
        rm -f "$file"
    done
else
    echo "  None found"
fi
echo

# 3. Clean macOS .DS_Store files
echo "[3/4] .DS_Store files"
DS_STORE=$(find "$REPO_ROOT" -name ".DS_Store" 2>/dev/null || true)
if [ -n "$DS_STORE" ]; then
    echo "  Found: $(echo "$DS_STORE" | wc -l) files"
    for file in $DS_STORE; do
        rm -f "$file"
    done
else
    echo "  None found"
fi
echo

# 4. Clean editor swap files
echo "[4/4] Editor temporary files"
SWAP_FILES=$(find "$REPO_ROOT" -type f \( -name "*.swp" -o -name "*.swo" -o -name "*~" \) 2>/dev/null || true)
if [ -n "$SWAP_FILES" ]; then
    echo "  Found: $(echo "$SWAP_FILES" | wc -l) files"
    for file in $SWAP_FILES; do
        rm -f "$file"
    done
else
    echo "  None found"
fi
echo

echo "=== Cleanup Complete ==="
