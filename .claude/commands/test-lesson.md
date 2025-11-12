# Test Lesson - Unified Hardware Testing Command

Execute comprehensive hardware tests for any ESP32-C6 lesson.

**Usage:**
```bash
/test-lesson <lesson_number> [mode]
```

**Arguments:**
- `lesson_number`: Lesson to test (e.g., "07", "08", "03")
- `mode` (optional): "quick" or "full" (defaults to "quick")

**Examples:**
```bash
/test-lesson 07           # Quick test of lesson 07
/test-lesson 08 full      # Full test of lesson 08
/test-lesson 03 quick     # Quick test of lesson 03
```

---

## Your Task

{{argument}} <!-- e.g., "07", "08 full", "03" -->

Execute hardware validation tests for the specified lesson according to its TEST.md specification.

### Step 1: Parse Arguments and Locate Lesson

```bash
cat > /tmp/parse_test_args.sh << 'SCRIPT'
#!/bin/bash
set -e

# Parse arguments
ARGS="{{argument}}"

# Extract lesson number (first argument)
LESSON_NUM=$(echo "$ARGS" | awk '{print $1}')
if [ -z "$LESSON_NUM" ]; then
    echo "✗ ERROR: No lesson number provided"
    echo "Usage: /test-lesson <lesson_number> [mode]"
    echo "Example: /test-lesson 07"
    exit 1
fi

# Normalize lesson number (07 -> 07, 7 -> 07)
LESSON_NUM=$(printf "%02d" "$LESSON_NUM" 2>/dev/null || echo "$LESSON_NUM")

# Extract mode (second argument, defaults to "quick")
MODE=$(echo "$ARGS" | awk '{print $2}')
MODE=${MODE:-quick}

# Find lesson directory
LESSON_DIR=$(find /Users/shanemattner/Desktop/esp32-c6-agentic-firmware/lessons -maxdepth 1 -type d -name "${LESSON_NUM}-*" | head -1)

if [ -z "$LESSON_DIR" ]; then
    echo "✗ ERROR: Lesson $LESSON_NUM not found"
    echo "Available lessons:"
    ls -1 /Users/shanemattner/Desktop/esp32-c6-agentic-firmware/lessons | grep "^[0-9]"
    exit 1
fi

LESSON_NAME=$(basename "$LESSON_DIR")

echo "=== Test Lesson $LESSON_NUM: $LESSON_NAME ==="
echo "Mode: $MODE"
echo "Directory: $LESSON_DIR"
echo ""

# Export for next steps
echo "$LESSON_DIR" > /tmp/test_lesson_dir.txt
echo "$LESSON_NUM" > /tmp/test_lesson_num.txt
echo "$MODE" > /tmp/test_mode.txt
echo "$LESSON_NAME" > /tmp/test_lesson_name.txt
SCRIPT

chmod +x /tmp/parse_test_args.sh
/tmp/parse_test_args.sh
```

### Step 2: Load and Parse TEST.md Specification

```bash
LESSON_DIR=$(cat /tmp/test_lesson_dir.txt)
LESSON_NUM=$(cat /tmp/test_lesson_num.txt)

# Check if TEST.md exists
if [ ! -f "$LESSON_DIR/TEST.md" ]; then
    echo "⚠ WARNING: No TEST.md found for this lesson"
    echo "Creating default test specification..."

    # Use generic fallback testing
    echo "Will perform basic tests: build, flash, infrastructure checks"
    echo "generic" > /tmp/test_spec_type.txt
else
    echo "✓ Found TEST.md specification"
    echo "custom" > /tmp/test_spec_type.txt
fi
```

### Step 3: Read TEST.md Using Read Tool

**IMPORTANT:** Use the Read tool to read the TEST.md file. This provides you with the full test specification.

```
Read the file at: $LESSON_DIR/TEST.md
```

After reading TEST.md, you will see the lesson-specific test specification including:
- Hardware setup requirements
- Automated test procedures
- Interactive test procedures
- Expected outputs
- Troubleshooting tips

### Step 4: Execute Tests According to TEST.md

Follow the test specification from TEST.md. Standard pattern:

**A. Environment Setup**
```bash
cat > /tmp/test_env_setup.sh << 'SCRIPT'
#!/bin/bash
set -e

LESSON_DIR=$(cat /tmp/test_lesson_dir.txt)
cd "$LESSON_DIR"

cat > /tmp/test_env.sh << 'ENV_SCRIPT'
#!/bin/bash
export LESSON_DIR=$(cat /tmp/test_lesson_dir.txt)
export LESSON_NUM=$(cat /tmp/test_lesson_num.txt)
export MODE=$(cat /tmp/test_mode.txt)

# Detect USB CDC port (for flashing)
export USB_CDC_PORT=$(ls /dev/cu.usbmodem* 2>/dev/null | head -1)
if [ -z "$USB_CDC_PORT" ]; then
    echo "✗ ERROR: USB CDC port not found"
    exit 1
fi
echo "✓ USB CDC: $USB_CDC_PORT"

# Detect ESP JTAG probe (if available)
export ESP_PROBE=$(probe-rs list 2>&1 | grep -i "esp.*jtag" | grep -oE '[0-9a-fA-F]{4}:[0-9a-fA-F]{4}(:[0-9A-F:]+)?' | head -1)
if [ -n "$ESP_PROBE" ]; then
    export PROBE_ARG="--probe $ESP_PROBE"
    echo "✓ ESP Probe: $ESP_PROBE"
else
    export PROBE_ARG=""
    echo "⚠ ESP Probe: not detected (some tests may be skipped)"
fi

# Detect UART port (if available)
export UART_PORT=$(ls /dev/cu.usbserial* 2>/dev/null | head -1)
if [ -n "$UART_PORT" ]; then
    echo "✓ UART: $UART_PORT"
else
    echo "⚠ UART: not detected (optional)"
fi
ENV_SCRIPT

chmod +x /tmp/test_env.sh
/tmp/test_env.sh
SCRIPT

chmod +x /tmp/test_env_setup.sh
/tmp/test_env_setup.sh
```

**B. Cleanup**
```bash
echo "=== Cleanup ==="
pkill -f "probe-rs" || true
pkill -f "openocd" || true
sleep 1
echo "✓ Cleanup complete"
```

**C. Build Firmware**
```bash
source /tmp/test_env.sh
cd "$LESSON_DIR"

echo "=== Build Firmware ==="
cargo build --release
echo "✓ Build complete"
```

**D. Flash Firmware**
```bash
source /tmp/test_env.sh
cd "$LESSON_DIR"

echo "=== Flash Firmware ==="
espflash flash --port "$USB_CDC_PORT" target/riscv32imac-unknown-none-elf/release/main
echo "✓ Flash complete"
```

**E. Execute Lesson-Specific Tests**

Follow the automated tests section from TEST.md. Common patterns:

**Infrastructure tests:**
- Debug symbols verification
- Source code structure
- Configuration files
- Build artifacts

**Hardware tests (if automated):**
- Boot message verification
- Peripheral initialization checks
- Basic functionality smoke tests

**Interactive tests (document for manual execution):**
- Detailed hardware interaction
- Register inspection
- Memory debugging
- Protocol verification

### Step 5: Generate Test Report

Create a comprehensive markdown report following this structure:

```markdown
# Lesson XX Test Report

**Date:** YYYY-MM-DD HH:MM
**Lesson:** {lesson_number} - {lesson_name}
**Mode:** {quick/full}
**Duration:** X minutes

## Summary
- Total Automated Tests: N
- Passed: Y
- Failed: Z
- Success Rate: Y/N (%)
- Manual Tests: M (see Manual Test Instructions)

## Environment
- ESP32-C6: Connected ✓/✗
- JTAG Probe: Detected ✓/✗
- UART: Detected ✓/✗
- Firmware: Built ✓/✗

## Automated Test Results

### Test 1: {Name}
- **Command:** `...`
- **Expected:** ...
- **Actual:** ...
- **Status:** ✓ PASS / ✗ FAIL

[... repeat for all automated tests ...]

## Manual Test Instructions

⚠️ **The following tests require manual execution**

```bash
# Commands for manual testing
...
```

**Expected Results:**
- ...

## Issues Found
[List any issues]

## Recommendations
[List any improvements]

## Conclusion
**Test Status:** ✓ PASS / ✗ FAIL
[Summary paragraph]
```

### Error Handling

If TEST.md is missing or incomplete:
1. Fall back to generic infrastructure tests:
   - Build verification
   - Flash verification
   - Debug symbols check
   - Source code structure
2. Document in report that custom tests were not available
3. Recommend creating TEST.md for this lesson

If hardware is not connected:
1. Skip hardware-dependent tests
2. Run all infrastructure tests that don't require hardware
3. Clearly mark skipped tests in report

### Success Criteria

**Quick mode passes if:**
- Firmware builds successfully
- Firmware flashes successfully
- At least 70% of automated tests pass
- No critical configuration issues

**Full mode passes if:**
- All automated tests pass (100%)
- Manual test instructions are clear
- No unresolved issues

---

**After testing, present the markdown report to the user.**
