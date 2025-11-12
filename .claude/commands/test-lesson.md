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

---

## Step 1: Setup - Parse Arguments and Detect Hardware

Execute this single comprehensive setup script:

```bash
cat > /tmp/test_setup.sh << 'SCRIPT'
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

# Find project root and lessons directory
PROJECT_ROOT=$(git rev-parse --show-toplevel 2>/dev/null || pwd)
LESSONS_DIR="$PROJECT_ROOT/lessons"

if [ ! -d "$LESSONS_DIR" ]; then
    echo "✗ ERROR: Lessons directory not found at $LESSONS_DIR"
    echo "Are you running this from the project root?"
    exit 1
fi

# Find lesson directory
LESSON_DIR=$(find "$LESSONS_DIR" -maxdepth 1 -type d -name "${LESSON_NUM}-*" | head -1)

if [ -z "$LESSON_DIR" ]; then
    echo "✗ ERROR: Lesson $LESSON_NUM not found"
    echo "Available lessons:"
    ls -1 "$LESSONS_DIR" | grep "^[0-9]"
    exit 1
fi

LESSON_NAME=$(basename "$LESSON_DIR")

echo "=== Test Lesson $LESSON_NUM: $LESSON_NAME ==="
echo "Mode: $MODE"
echo "Directory: $LESSON_DIR"
echo ""

# Save lesson metadata to files (file-based state management)
echo "$LESSON_DIR" > /tmp/test_lesson_dir.txt
echo "$LESSON_NUM" > /tmp/test_lesson_num.txt
echo "$MODE" > /tmp/test_mode.txt
echo "$LESSON_NAME" > /tmp/test_lesson_name.txt

# Detect and save hardware configuration
echo "=== Detecting Hardware ==="

# Detect USB CDC port (for flashing)
USB_CDC_PORT=$(ls /dev/cu.usbmodem* 2>/dev/null | head -1)
if [ -z "$USB_CDC_PORT" ]; then
    echo "✗ ERROR: USB CDC port not found"
    echo "Please connect ESP32-C6 via USB"
    exit 1
fi
echo "✓ USB CDC: $USB_CDC_PORT"
echo "$USB_CDC_PORT" > /tmp/usb_cdc_port.txt

# Detect ESP JTAG probe (if available)
ESP_PROBE=$(probe-rs list 2>&1 | grep -i "esp.*jtag" | grep -oE '[0-9a-fA-F]{4}:[0-9a-fA-F]{4}(:[0-9A-F:]+)?' | head -1)
if [ -n "$ESP_PROBE" ]; then
    echo "✓ ESP Probe: $ESP_PROBE"
    echo "$ESP_PROBE" > /tmp/esp_probe.txt
    echo "--probe $ESP_PROBE" > /tmp/probe_arg.txt
else
    echo "⚠ ESP Probe: not detected (some tests may be skipped)"
    echo "" > /tmp/esp_probe.txt
    echo "" > /tmp/probe_arg.txt
fi

# Detect UART port (if available)
UART_PORT=$(ls /dev/cu.usbserial* 2>/dev/null | head -1)
if [ -n "$UART_PORT" ]; then
    echo "✓ UART: $UART_PORT"
    echo "$UART_PORT" > /tmp/uart_port.txt
else
    echo "⚠ UART: not detected (optional)"
    echo "" > /tmp/uart_port.txt
fi

# Detect binary name from Cargo.toml
cd "$LESSON_DIR"
BINARY_NAME=$(grep -A1 '\[\[bin\]\]' Cargo.toml 2>/dev/null | grep 'name' | cut -d'"' -f2 | head -1)
if [ -z "$BINARY_NAME" ]; then
    BINARY_NAME="main"  # Fallback to default
fi
echo "Binary name: $BINARY_NAME"
echo "$BINARY_NAME" > /tmp/binary_name.txt

# Define standard paths
TARGET_DIR="target/riscv32imac-unknown-none-elf/release"
echo "$TARGET_DIR" > /tmp/target_dir.txt

# Check for TEST.md
if [ ! -f "$LESSON_DIR/TEST.md" ]; then
    echo ""
    echo "⚠ WARNING: No TEST.md found for this lesson"
    echo "Will perform generic infrastructure tests"
    echo "generic" > /tmp/test_spec_type.txt
else
    echo ""
    echo "✓ Found TEST.md specification"
    echo "custom" > /tmp/test_spec_type.txt
fi

SCRIPT

chmod +x /tmp/test_setup.sh
/tmp/test_setup.sh
```

---

## Step 2: Read TEST.md (if available)

**If TEST.md exists (test_spec_type.txt contains "custom"):**

Use the Read tool to read the TEST.md file:
```
Read: $(cat /tmp/test_lesson_dir.txt)/TEST.md
```

**If TEST.md doesn't exist (test_spec_type.txt contains "generic"):**

Skip to Step 3 and use the generic test specification below.

---

## Step 3: Execute Tests

### Cleanup Previous Debug Sessions

```bash
echo "=== Cleanup ==="
pkill -f "probe-rs" || true
pkill -f "openocd" || true
sleep 1
echo "✓ Cleanup complete"
```

### Build Firmware

```bash
cat > /tmp/test_build.sh << 'SCRIPT'
#!/bin/bash
set -e

LESSON_DIR=$(cat /tmp/test_lesson_dir.txt)
cd "$LESSON_DIR"

echo "=== Build Firmware ==="
echo "Building in: $LESSON_DIR"

START_TIME=$(date +%s)
cargo build --release 2>&1 | tail -20
BUILD_EXIT=$?
END_TIME=$(date +%s)
BUILD_TIME=$((END_TIME - START_TIME))

if [ $BUILD_EXIT -eq 0 ]; then
    BINARY_NAME=$(cat /tmp/binary_name.txt)
    TARGET_DIR=$(cat /tmp/target_dir.txt)
    BINARY_PATH="$TARGET_DIR/$BINARY_NAME"

    if [ -f "$BINARY_PATH" ]; then
        BINARY_SIZE=$(ls -lh "$BINARY_PATH" | awk '{print $5}')
        echo ""
        echo "✓ Build successful"
        echo "✓ Binary: $BINARY_NAME"
        echo "✓ Size: $BINARY_SIZE"
        echo "✓ Build time: ${BUILD_TIME}s"
    else
        echo "✗ Binary not found at $BINARY_PATH"
        exit 1
    fi
else
    echo "✗ Build failed (exit code: $BUILD_EXIT)"
    exit $BUILD_EXIT
fi

SCRIPT

chmod +x /tmp/test_build.sh
/tmp/test_build.sh
```

### Flash Firmware

```bash
cat > /tmp/test_flash.sh << 'SCRIPT'
#!/bin/bash
set -e

LESSON_DIR=$(cat /tmp/test_lesson_dir.txt)
USB_CDC_PORT=$(cat /tmp/usb_cdc_port.txt)
BINARY_NAME=$(cat /tmp/binary_name.txt)
TARGET_DIR=$(cat /tmp/target_dir.txt)

cd "$LESSON_DIR"
BINARY_PATH="$TARGET_DIR/$BINARY_NAME"

if [ ! -f "$BINARY_PATH" ]; then
    echo "✗ ERROR: Binary not found at $BINARY_PATH"
    exit 1
fi

echo "=== Flash Firmware ==="
echo "Port: $USB_CDC_PORT"
echo "Binary: $BINARY_NAME"

espflash flash --port "$USB_CDC_PORT" "$BINARY_PATH" 2>&1 | tail -15

if [ $? -eq 0 ]; then
    echo ""
    echo "✓ Flash complete"
else
    echo "✗ Flash failed"
    exit 1
fi

SCRIPT

chmod +x /tmp/test_flash.sh
/tmp/test_flash.sh
```

### Execute Infrastructure Tests

Run these standard infrastructure tests for all lessons:

**Test: Debug Symbols Verification**
```bash
LESSON_DIR=$(cat /tmp/test_lesson_dir.txt)
BINARY_NAME=$(cat /tmp/binary_name.txt)
TARGET_DIR=$(cat /tmp/target_dir.txt)

cd "$LESSON_DIR"
echo "=== Test: Debug Symbols ==="
file "$TARGET_DIR/$BINARY_NAME" | grep "not stripped"
```

**Test: Source Code Structure**
```bash
LESSON_DIR=$(cat /tmp/test_lesson_dir.txt)
cd "$LESSON_DIR"

echo "=== Test: Source Code Structure ==="
echo "Checking required files..."

if [ -f src/bin/main.rs ]; then
    echo "✓ src/bin/main.rs exists ($(wc -l < src/bin/main.rs) lines)"
else
    echo "⚠ src/bin/main.rs missing"
fi

if [ -f src/lib.rs ]; then
    echo "✓ src/lib.rs exists ($(wc -l < src/lib.rs) lines)"
fi

if [ -f Cargo.toml ]; then
    echo "✓ Cargo.toml exists"
fi

if [ -f .cargo/config.toml ]; then
    echo "✓ .cargo/config.toml exists"
fi
```

**Test: Cargo.toml Configuration**
```bash
LESSON_DIR=$(cat /tmp/test_lesson_dir.txt)
cd "$LESSON_DIR"

echo "=== Test: Cargo.toml Configuration ==="
echo "Binary configuration:"
grep -A2 '\[\[bin\]\]' Cargo.toml

echo ""
echo "Debug configuration:"
grep -A2 '\[profile.release\]' Cargo.toml | grep "debug" || echo "⚠ No debug setting found"
```

### Execute Lesson-Specific Tests

**If TEST.md exists:** Follow the automated test procedures specified in TEST.md

**If TEST.md doesn't exist:** The generic tests above are sufficient

For tests with complex logic, conditionals, or loops, **always use temp scripts**:

```bash
cat > /tmp/test_custom.sh << 'SCRIPT'
#!/bin/bash
set -e

# Your test logic here
# Can use if/then/fi, loops, etc.

SCRIPT

chmod +x /tmp/test_custom.sh
/tmp/test_custom.sh
```

---

## Step 4: Generate Test Report

Create a comprehensive markdown report. Use this template:

```bash
cat > /tmp/generate_report.sh << 'SCRIPT'
#!/bin/bash

LESSON_NUM=$(cat /tmp/test_lesson_num.txt)
LESSON_NAME=$(cat /tmp/test_lesson_name.txt)
MODE=$(cat /tmp/test_mode.txt)
CURRENT_DATE=$(date '+%Y-%m-%d %H:%M')

cat > /tmp/test_report.md << EOF
# Lesson $LESSON_NUM Test Report

**Date:** $CURRENT_DATE
**Lesson:** $LESSON_NUM - $LESSON_NAME
**Mode:** $MODE
**Duration:** ~X minutes (update this)

## Summary
- Total Automated Tests: N (count your tests)
- Passed: Y
- Failed: Z
- Success Rate: Y/N (%)
- Manual Tests: M (if any)

## Environment
- ESP32-C6: Connected ✓
- USB CDC Port: $(cat /tmp/usb_cdc_port.txt) ✓
- JTAG Probe: $(cat /tmp/esp_probe.txt) $([ -s /tmp/esp_probe.txt ] && echo "✓" || echo "✗")
- UART: $(cat /tmp/uart_port.txt) $([ -s /tmp/uart_port.txt ] && echo "✓" || echo "⚠ Optional")
- Firmware: Built ✓

## Automated Test Results

### Test 1: Build Verification
- **Command:** \`cargo build --release\`
- **Expected:** Successful build with debug symbols
- **Actual:** [Fill in from test output]
- **Status:** ✓ PASS / ✗ FAIL

### Test 2: Flash Firmware
- **Command:** \`espflash flash --port ... \`
- **Expected:** Firmware flashes successfully
- **Actual:** [Fill in from test output]
- **Status:** ✓ PASS / ✗ FAIL

### Test 3: Debug Symbols
- **Command:** \`file ... | grep "not stripped"\`
- **Expected:** Binary contains debug symbols
- **Actual:** [Fill in from test output]
- **Status:** ✓ PASS / ✗ FAIL

### Test 4: Source Code Structure
- **Command:** Check for required files
- **Expected:** All required files present
- **Actual:** [Fill in from test output]
- **Status:** ✓ PASS / ✗ FAIL

[Add more tests as needed based on TEST.md]

## Manual Test Instructions

$([ "$(cat /tmp/test_spec_type.txt)" = "custom" ] && echo "⚠️ **See TEST.md for manual test procedures**" || echo "⚠️ **No TEST.md found - manual testing not specified**")

## Issues Found

[List any issues discovered during testing]

## Recommendations

$([ "$(cat /tmp/test_spec_type.txt)" = "generic" ] && echo "1. **Create TEST.md** - This lesson lacks a TEST.md specification. Consider adding one with hardware wiring, test procedures, and expected outputs.")

## Conclusion

**Test Status:** ✓ PASS / ✗ FAIL (update based on results)

[Write 2-3 sentence summary of test results]

**Hardware Status:**
- ✓ ESP32-C6 detected and functional
- ✓ Firmware builds and flashes successfully
- ✓ Ready for hardware testing

EOF

cat /tmp/test_report.md

SCRIPT

chmod +x /tmp/generate_report.sh
/tmp/generate_report.sh
```

**Important:** Fill in the test results based on actual output from your test executions. Present the final markdown report to the user.

---

## Best Practices for Test Execution

### 1. Shell Syntax Guidelines

**❌ Don't do this (causes parse errors):**
```bash
if [ $EXIT_CODE -eq 0 ]; then
    echo "success"
fi
```

**✅ Do this instead:**
```bash
cat > /tmp/test_step.sh << 'SCRIPT'
#!/bin/bash
if [ $EXIT_CODE -eq 0 ]; then
    echo "success"
fi
SCRIPT
chmod +x /tmp/test_step.sh
/tmp/test_step.sh
```

### 2. Variable Management

**File-based state is reliable:**
```bash
# Save values to files
echo "$VALUE" > /tmp/my_value.txt

# Read back later
VALUE=$(cat /tmp/my_value.txt)
```

**Don't rely on export/source across tool calls** - variables don't persist

### 3. Binary Path Construction

Always detect binary name from Cargo.toml:
```bash
BINARY_NAME=$(grep -A1 '\[\[bin\]\]' Cargo.toml | grep 'name' | cut -d'"' -f2)
```

### 4. Serial Communication

For reading serial output, use Python:
```bash
cat > /tmp/test_serial.sh << 'SCRIPT'
#!/bin/bash
python3 << 'PYCODE'
import serial
import time

port = "/dev/cu.usbmodem1101"
ser = serial.Serial(port, 115200, timeout=1)
# ... serial operations ...
PYCODE
SCRIPT
chmod +x /tmp/test_serial.sh
/tmp/test_serial.sh
```

### 5. Success Criteria

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

## Cleanup

After testing completes, clean up temp files:

```bash
echo "=== Cleanup Temp Files ==="
rm -f /tmp/test_*.txt /tmp/test_*.sh /tmp/test_*.md
rm -f /tmp/usb_*.txt /tmp/esp_*.txt /tmp/uart_*.txt
rm -f /tmp/binary_*.txt /tmp/target_*.txt /tmp/probe_*.txt
echo "✓ Cleanup complete"
```

---

**After testing, present the final test report to the user.**
