#!/usr/bin/env bash
# Clean all Cargo target directories to free up disk space
# These directories contain build artifacts and can be safely removed
# They will be regenerated on next build

set -e

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo "=== Cleaning Cargo target directories ==="
echo "Repository: $REPO_ROOT"
echo

# Find all target directories in lessons/
TARGET_DIRS=$(find "$REPO_ROOT/lessons" -type d -name "target" 2>/dev/null)

if [ -z "$TARGET_DIRS" ]; then
    echo "No target directories found."
    exit 0
fi

# Calculate total size before cleaning
echo "Calculating disk space usage..."
TOTAL_SIZE=$(du -ch $TARGET_DIRS 2>/dev/null | grep total | awk '{print $1}')
echo "Total size of target directories: $TOTAL_SIZE"
echo

# Confirm deletion
read -p "Remove all target directories? (y/N): " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Cancelled."
    exit 0
fi

# Remove target directories
echo "Removing target directories..."
for dir in $TARGET_DIRS; do
    echo "  Removing: $dir"
    rm -rf "$dir"
done

echo
echo "Done! Freed approximately $TOTAL_SIZE of disk space."
echo "Run 'cargo build' in any lesson to regenerate build artifacts."
