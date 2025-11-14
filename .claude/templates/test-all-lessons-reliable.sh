#!/bin/bash
# test-all-lessons-reliable.sh - Comprehensive lesson testing with detailed reporting
# Usage: bash test-all-lessons-reliable.sh

set -euo pipefail

REPO_ROOT="/Users/shanemattner/Desktop/esp32-c6-agentic-firmware"
LOG_DIR="/tmp/lesson-test-logs"
SUMMARY_FILE="$LOG_DIR/test-summary.txt"
TIMESTAMP=$(date '+%Y-%m-%d_%H-%M-%S')

# === Environment Setup ===
cd "$REPO_ROOT"
mkdir -p "$LOG_DIR"

echo "=========================================="
echo "Comprehensive Lesson Test Suite"
echo "=========================================="
echo ""
echo "Timestamp: $(date '+%Y-%m-%d %H:%M:%S')"
echo "Repository: $REPO_ROOT"
echo "Log directory: $LOG_DIR"
echo ""

# === Pre-flight Checks ===
echo "=== Pre-flight Checks ==="
echo ""

# Verify environment
if ! bash .claude/templates/verify-test-environment.sh 2>&1 | tail -5; then
    echo ""
    echo "❌ Pre-flight checks failed"
    echo "Run: bash .claude/templates/verify-test-environment.sh"
    exit 1
fi

echo ""
echo "=== Collecting Lessons ==="
echo ""

# Find all lessons
LESSONS=($(find lessons -maxdepth 1 -type d -name "[0-9]*" | sort))
LESSON_COUNT=${#LESSONS[@]}

echo "Found $LESSON_COUNT lessons:"
for lesson in "${LESSONS[@]}"; do
    echo "  - $(basename "$lesson")"
done
echo ""

# === Initialize Summary ===
{
    echo "=========================================="
    echo "Lesson Test Summary"
    echo "=========================================="
    echo ""
    echo "Timestamp: $(date '+%Y-%m-%d %H:%M:%S')"
    echo "Total lessons: $LESSON_COUNT"
    echo ""
} > "$SUMMARY_FILE"

# === Test Each Lesson ===
echo "=== Testing Lessons ==="
echo ""

PASS_COUNT=0
FAIL_COUNT=0
FAILED_LESSONS=()

for lesson in "${LESSONS[@]}"; do
    LESSON_NAME=$(basename "$lesson")

    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "Testing: $LESSON_NAME"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""

    # Run test and capture result
    if bash .claude/templates/test-single-lesson-reliable.sh "$lesson"; then
        PASS_COUNT=$((PASS_COUNT + 1))

        # Log to summary
        {
            echo "✅ $LESSON_NAME - PASS"

            # Extract binary size if available
            BINARY=$(find "$lesson/target/riscv32imac-unknown-none-elf/release/" \
                     -type f -perm +111 -name main 2>/dev/null | head -1)
            if [ -n "$BINARY" ]; then
                SIZE=$(ls -lh "$BINARY" | awk '{print $5}')
                echo "   Binary size: $SIZE"
            fi
            echo ""
        } >> "$SUMMARY_FILE"
    else
        EXIT_CODE=$?
        FAIL_COUNT=$((FAIL_COUNT + 1))
        FAILED_LESSONS+=("$LESSON_NAME")

        # Log to summary
        {
            echo "❌ $LESSON_NAME - FAIL (exit code: $EXIT_CODE)"
            echo "   Log: $LOG_DIR/${LESSON_NAME}.log"
            echo "   First error:"
            grep -E 'error(\[E[0-9]+\])?:' "$LOG_DIR/${LESSON_NAME}.log" 2>/dev/null | head -1 | sed 's/^/   /' || echo "   (no error details captured)"
            echo ""
        } >> "$SUMMARY_FILE"
    fi

    echo ""
    echo ""
done

# === Final Summary ===
{
    echo "=========================================="
    echo "Final Results"
    echo "=========================================="
    echo ""
    echo "Total tested: $LESSON_COUNT"
    echo "Passed: $PASS_COUNT"
    echo "Failed: $FAIL_COUNT"
    echo ""

    if [ $FAIL_COUNT -gt 0 ]; then
        echo "Failed lessons:"
        for failed in "${FAILED_LESSONS[@]}"; do
            echo "  - $failed"
        done
        echo ""
    fi

    echo "=========================================="
} >> "$SUMMARY_FILE"

# === Display Summary ===
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
cat "$SUMMARY_FILE"
echo ""
echo "Detailed logs: $LOG_DIR/"
echo "Summary file: $SUMMARY_FILE"
echo ""

if [ $FAIL_COUNT -eq 0 ]; then
    echo "🎉 All lessons passed! ✅"
    exit 0
else
    echo "⚠️  $FAIL_COUNT lesson(s) failed ❌"
    echo ""
    echo "Next steps:"
    echo "  1. Review failed lesson logs in $LOG_DIR/"
    echo "  2. Fix issues in failed lessons"
    echo "  3. Re-run: bash .claude/templates/test-all-lessons-reliable.sh"
    exit 1
fi
