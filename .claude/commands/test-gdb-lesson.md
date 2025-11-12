---
description: Comprehensive test suite for GDB debugging lesson (Lesson 07) using probe-rs
argument-hint: [mode: quick|full] (default: quick)
---

# Test GDB Debugging Lesson (Lesson 07)

You are testing the GDB debugging capabilities documented in Lesson 07 **using probe-rs** as the debugger.

## Test Mode

{{argument}} <!-- "quick" or "full" - defaults to "quick" if not specified -->

- **quick**: Core capabilities only (~5-10 minutes)
- **full**: All capabilities including advanced scenarios (~15-20 minutes)

## Your Task

Execute a systematic test of debugging capabilities on real ESP32-C6 hardware using **probe-rs**.

### Prerequisites Check

Before starting tests, verify:

1. **Hardware connected:**
   - ESP32-C6 board connected via USB
   - JTAG debugger connected (TMS→GPIO4, TDI→GPIO5, TDO→GPIO6, TCK→GPIO7, GND)
   - MPU9250 IMU connected (SDA→GPIO2, SCL→GPIO11)
   - Button on GPIO9
   - NeoPixel on GPIO8
   - UART adapter on GPIO15 (TX) and GPIO23 (RX)

2. **Software ready:**
   - `probe-rs` installed (check with `which probe-rs`)
   - `espflash` installed (for flashing firmware)
   - Lesson 07 firmware built (`cargo build --release`)
   - Python 3 available for verifying firmware boot messages

### Test Structure

Your tests should be **deterministic and reproducible**. Follow this structure:

```
1. Environment Setup
   ├─ Clean up existing debug sessions
   ├─ Detect USB ports dynamically
   ├─ Auto-detect ESP JTAG probe
   ├─ Build firmware with debug symbols
   └─ Flash to ESP32-C6 using espflash

2. Core probe-rs Tests (ALWAYS run these)
   ├─ Test 1: Firmware boots successfully
   ├─ Test 2: probe-rs can attach to running firmware
   ├─ Test 3: Peripheral register inspection (I2C, GPIO)
   ├─ Test 4: Memory inspection (stack, heap, buffers)
   ├─ Test 5: Breakpoint at main()
   ├─ Test 6: Function breakpoint (handle_command)
   └─ Test 7: Call stack analysis (backtrace)

3. Infrastructure Tests (ALWAYS run these)
   ├─ Test 8: Debug symbols present in binary
   ├─ Test 9: Source code structure verified
   ├─ Test 10: .gdbinit configuration exists (for future GDB use)
   └─ Test 11: Python helper scripts syntax valid

4. Advanced Tests (ONLY in "full" mode)
   ├─ Test 12: Reset and re-attach
   ├─ Test 13: Multi-breakpoint workflow
   ├─ Test 14: Complex scenario - Debug "button not responding"
   └─ Test 15: Complex scenario - Debug "I2C timeout"

Note: GDB-specific features (Python scripts, custom commands, watchpoints) are
documented but not tested with probe-rs. They require riscv32-esp-elf-gdb.
```

### Execution Guidelines

All tests use **probe-rs** as the debugger. Follow these steps in order:

#### Step 0: Setup Environment Variables and Timeout Function

**CRITICAL: Run this entire block in ONE bash call to preserve all variables for subsequent tests.**

```bash
cd /Users/shanemattner/Desktop/esp32-c6-agentic-firmware/lessons/07-gdb-debugging

echo "=== Step 0: Environment Setup ==="

# 1. Define cross-platform timeout command
# Check in order: gtimeout (Homebrew), timeout (Linux), then fallback to manual kill
if command -v gtimeout > /dev/null 2>&1; then
    export CMD_TIMEOUT="gtimeout"
    echo "✓ Timeout: gtimeout (Homebrew)"
elif command -v timeout > /dev/null 2>&1; then
    export CMD_TIMEOUT="timeout"
    echo "✓ Timeout: timeout (GNU)"
else
    export CMD_TIMEOUT=""
    echo "⚠ Timeout: Not available - will use background processes with manual kill"
fi

# 2. Detect USB CDC port (required)
export USB_CDC_PORT=$(ls /dev/cu.usbmodem* 2>/dev/null | head -1)
if [ -z "$USB_CDC_PORT" ]; then
    echo "✗ USB CDC: not found - cannot flash firmware"
    exit 1
else
    echo "✓ USB CDC: $USB_CDC_PORT"
fi

# 3. Detect UART port (optional)
export UART_PORT=$(ls /dev/cu.usbserial* 2>/dev/null | head -1)
if [ -n "$UART_PORT" ]; then
    echo "✓ UART: $UART_PORT"
else
    echo "⚠ UART: not found (optional - only needed for Test 6)"
fi

# 4. Detect ESP JTAG probe and extract VID:PID:Serial format
export ESP_PROBE=$(probe-rs list 2>&1 | grep -i "esp.*jtag" | grep -oE '[0-9a-fA-F]{4}:[0-9a-fA-F]{4}(:[0-9A-F:]+)?' | head -1)
if [ -n "$ESP_PROBE" ]; then
    export PROBE_ARG="--probe $ESP_PROBE"
    echo "✓ ESP Probe: $ESP_PROBE"
    echo "  Probe arg: $PROBE_ARG"
else
    export PROBE_ARG=""
    echo "⚠ ESP Probe: Auto-detection failed - will try without --probe flag"
fi

# 5. Summary
echo ""
echo "Hardware detected: ESP32-C6 on $USB_CDC_PORT, probe: ${ESP_PROBE:-auto}"
echo ""
```

**IMPORTANT:** If variables don't persist across bash calls in your environment, you have two options:
1. Re-run the detection commands at the start of each test
2. Hardcode the detected values (e.g., `USB_CDC_PORT="/dev/cu.usbmodem2101"`)


#### Step 1: Cleanup and Verify Tools

```bash
# Clean up any existing debug sessions (do this at START, not END)
# Reason: Previous test runs may have crashed, leaving orphaned processes
# Better to clean up proactively than to fail with "exclusive access" errors
echo "=== Step 1: Cleanup ==="
pkill -f "probe-rs" || true
pkill -f "openocd" || true
sleep 1

# Verify cleanup succeeded
REMAINING=$(ps aux | grep -E "(probe-rs|openocd)" | grep -v grep | wc -l)
if [ "$REMAINING" -gt 0 ]; then
    echo "⚠ Warning: $REMAINING debug processes still running - may cause 'exclusive access' errors"
    ps aux | grep -E "(probe-rs|openocd)" | grep -v grep
else
    echo "✓ Cleanup successful - no orphaned debug processes"
fi

# Verify probe-rs is available
if ! which probe-rs > /dev/null 2>&1; then
    echo "✗ ERROR: probe-rs not found - cannot run tests"
    exit 1
fi
echo "✓ probe-rs found: $(which probe-rs)"

# Show detected probes
echo ""
echo "Detecting probes..."
probe-rs list
```

#### Step 2: Build firmware with debug symbols
```bash
cargo build --release  # Has debug=2 in Cargo.toml
```

#### Step 3: Flash firmware using espflash

**IMPORTANT STRATEGY:** Use `espflash` for flashing, then `probe-rs attach` for debugging.

**Why not `probe-rs run`?**
- `probe-rs run` flashes AND attaches in one command
- This creates an exclusive lock that blocks subsequent `probe-rs attach` commands
- Separate flashing (espflash) and debugging (probe-rs attach) avoids this issue

```bash
# Use the auto-detected USB CDC port from Step 1 (ALWAYS use variable, not hardcoded path)
espflash flash --port ${USB_CDC_PORT} target/riscv32imac-unknown-none-elf/release/main

# Verify firmware is running by capturing boot messages
# IMPORTANT: If this fails (device stuck in download mode), SKIP and continue to Test 2
# Reason: probe-rs attach will reset the device anyway, so boot state doesn't matter
sleep 1

python3 << 'EOF'
import serial
import time

try:
    ser = serial.Serial('${USB_CDC_PORT}', 115200, timeout=3)

    # Strategy 1: RTS+DTR reset (ESP32 standard reset sequence)
    ser.setRTS(True)   # IO0 = HIGH (normal boot, not download mode)
    ser.setDTR(False)  # EN = HIGH (not in reset)
    time.sleep(0.05)
    ser.setDTR(True)   # EN = LOW (enter reset)
    time.sleep(0.05)
    ser.setDTR(False)  # EN = HIGH (exit reset, should boot firmware)
    time.sleep(1.5)

    output = ser.read(ser.in_waiting).decode('utf-8', errors='replace')

    # Check if we got actual boot messages (not just bootloader in download mode)
    if output and "waiting for download" not in output:
        print("✓ Firmware boot verified")
        if output.strip():
            print(output[:500])  # Print first 500 chars
    elif output:
        # Device in download mode - try closing/reopening serial port
        ser.close()
        time.sleep(0.3)
        ser = serial.Serial('${USB_CDC_PORT}', 115200, timeout=2)
        time.sleep(1.0)
        output2 = ser.read(ser.in_waiting).decode('utf-8', errors='replace')
        if output2 and "waiting for download" not in output2:
            print("✓ Firmware boot verified (after port reset)")
            print(output2[:500])
        else:
            print("⚠ Device in download mode - SKIPPING boot verification")
            print("  → Test 2 will reset device via probe-rs")
    else:
        print("⚠ No output - SKIPPING boot verification")
        print("  → Test 2 will reset device via probe-rs")

    ser.close()
except Exception as e:
    print(f"⚠ Boot verification failed: {e}")
    print("  → SKIPPING - Test 2 will reset device via probe-rs")
EOF
```

**Note:** Boot verification may fail (device stuck in download mode). This is non-blocking - continue to Test 2.

#### Step 4: Run Core Tests with probe-rs

**Test 1: Firmware Boot Verification**
```bash
# Already done in Step 3 - firmware should show successful peripheral initialization
```

**Test 2: probe-rs Attach**
```bash
# Attach to running firmware (use $PROBE_ARG from Step 1)
probe-rs attach --chip esp32c6 $PROBE_ARG target/riscv32imac-unknown-none-elf/release/main
# Should enter interactive mode without errors
# Type 'quit' to exit
```

**Test 3: Peripheral Register Reads**
```bash
probe-rs attach --chip esp32c6 $PROBE_ARG target/riscv32imac-unknown-none-elf/release/main
# In probe-rs interactive mode:
> read32 0x60013004   # I2C STATUS register
> read32 0x6000403C   # GPIO IN register
> quit
```

**Test 4: Memory Inspection**
```bash
probe-rs attach --chip esp32c6 $PROBE_ARG target/riscv32imac-unknown-none-elf/release/main
> read 0x3FC88000 64   # Read 64 bytes from RAM base
> quit
```

**Test 5: Breakpoint at main()**
```bash
probe-rs attach --chip esp32c6 $PROBE_ARG target/riscv32imac-unknown-none-elf/release/main
> break main
> reset
# Should stop at main() entry point
> continue
> quit
```

**Test 6: Function Breakpoint**
```bash
probe-rs attach --chip esp32c6 $PROBE_ARG target/riscv32imac-unknown-none-elf/release/main
> break handle_command
> continue
# Send UART command via separate terminal to trigger breakpoint
# (This test may be skipped if interactive UART is not available)
> quit
```

**Test 7: Call Stack Analysis**
```bash
probe-rs attach --chip esp32c6 $PROBE_ARG target/riscv32imac-unknown-none-elf/release/main
> break main
> reset
> backtrace
# Should show call stack
> quit
```

#### Step 5: Infrastructure Tests (Static Analysis)

**Test 8: Debug Symbols**
```bash
file target/riscv32imac-unknown-none-elf/release/main | grep "not stripped"
# Should show "not stripped"
```

**Test 9: Source Code Structure**
```bash
test -f src/bin/main.rs && test -f src/lib.rs && test -f src/mpu9250.rs && test -f src/cli.rs
echo $?  # Should be 0 (success)
```

**Test 10: GDB Configuration Files (for future GDB testing)**
```bash
# NOTE: This lesson supports both probe-rs and GDB workflows. These files are for GDB users.
test -f .gdbinit && test -f gdb_helpers.py && test -f openocd.cfg
# These files enable GDB debugging with OpenOCD (alternative to probe-rs)
echo $?  # Should be 0
```

**Test 11: Python Helper Script Syntax**
```bash
python3 -m py_compile gdb_helpers.py
echo $?  # Should be 0 (no syntax errors)
```

### Expected Outputs

**Test 1: Firmware Boot Verification**
- Expected: Boot messages show successful firmware boot and ESP-IDF initialization
- Success criteria (at least one of):
  - See "boot:" or "ESP-ROM:" (bootloader messages)
  - See "I (XX) boot:" (ESP-IDF boot logs)
  - See partition table information
  - Firmware-specific peripheral init messages (if added by lesson code)
- Failure criteria:
  - No output at all (likely USB CDC not working)
  - Repeated panic/reset loops

**Test 2: probe-rs Attach**
- Expected: probe-rs enters interactive mode without errors
- Success criteria: No "exclusive access" errors, interactive prompt appears

**Test 3: Peripheral Register Reads**
- Expected: Can read I2C STATUS (0x60013004) and GPIO IN (0x6000403C)
- Success criteria: Returns valid 32-bit values (not error messages)

**Test 4: Memory Inspection**
- Expected: Can read RAM contents without errors
- Success criteria: Returns hex dump of memory contents

**Test 5: Breakpoint at main()**
- Expected: probe-rs stops at main() entry after reset
- Success criteria: Execution pauses, shows source location

**Test 6: Function Breakpoint**
- Expected: Stops when handle_command is called (if UART command sent)
- Success criteria: Break in handle_command function
- Note: May skip if interactive UART not available during test

**Test 7: Call Stack Analysis**
- Expected: Shows function call hierarchy
- Success criteria: `backtrace` displays multiple stack frames

**Test 8: Debug Symbols**
- Expected: Binary contains debug information
- Success criteria: `file` command shows "not stripped"

**Test 9: Source Code Structure**
- Expected: All required source files present
- Success criteria: All test commands return 0 (success)

**Test 10: GDB Configuration Files (for future GDB testing)**
- Expected: .gdbinit, gdb_helpers.py, openocd.cfg exist
- Success criteria: All files present
- Note: This lesson supports both probe-rs (tested here) and GDB workflows. These files enable GDB debugging with OpenOCD (alternative to probe-rs)

**Test 11: Python Helper Script Syntax**
- Expected: gdb_helpers.py has valid Python syntax
- Success criteria: py_compile succeeds with no errors

### Report Format

Generate a markdown report with this structure:

```markdown
# GDB Lesson 07 Test Report (probe-rs)

**Date:** YYYY-MM-DD HH:MM
**Mode:** quick/full
**Debugger:** probe-rs
**Duration:** X minutes

## Summary
- Total Tests: 11
- Passed: Y
- Failed: Z
- Skipped: N
- Success Rate: Y/11 (%)

## Environment
- ESP32-C6: Connected ✓/✗ (USB port: /dev/cu.usbmodemXXXX)
- JTAG Probe: Connected ✓/✗ (probe #X)
- probe-rs: Available ✓/✗
- Firmware: Built ✓/✗

## Test Results

### Core Tests (probe-rs)

#### ✓/✗/⊘ Test 1: Firmware Boot Verification
- Command: Captured boot messages via USB CDC
- Expected: Peripheral initialization messages
- Actual: [describe what happened]
- Status: PASS/FAIL/SKIP
- Notes: [any observations]

[... repeat for Tests 2-7 ...]

### Infrastructure Tests (Static Analysis)

#### ✓/✗ Test 8: Debug Symbols
- Command: `file target/.../main | grep "not stripped"`
- Expected: Binary has debug symbols
- Actual: [result]
- Status: PASS/FAIL

[... repeat for Tests 9-11 ...]

## Issues Found

1. [Issue description]
   - Test: Test X
   - Severity: High/Medium/Low
   - Root cause: [analysis]
   - Suggested fix: [recommendation]

## Recommendations

- [List any improvements to lesson, docs, or scripts]

## Next Steps

- [What should be done to address failures]
```

### Error Handling

If you encounter errors:

1. **probe-rs not found:**
   - Install with: `cargo install probe-rs --features cli`
   - Verify installation: `which probe-rs`

2. **No probes detected (`probe-rs list` empty):**
   - Verify JTAG debugger USB is connected
   - Check USB permissions (may need `sudo`)
   - Try unplugging/replugging JTAG debugger

3. **"Exclusive access" error when attaching:**
   - Another process is using the probe
   - Check: `ps aux | grep probe-rs`
   - Kill existing sessions: `pkill -f probe-rs`

4. **espflash cannot find port:**
   - USB CDC port changed after replug
   - Re-detect: `ls /dev/cu.usbmodem*`
   - Update `$USB_CDC_PORT` variable

5. **Firmware doesn't boot (no boot messages):**
   - Check USB cable supports data (not just power)
   - Verify baud rate is 115200
   - Try different USB port on computer

6. **Breakpoint not hit:**
   - Verify firmware has debug symbols: `file target/.../main | grep "not stripped"`
   - Check function name exists: Try simpler breakpoint like `break main`
   - Reset device after setting breakpoint

7. **Peripheral register reads return unexpected values:**
   - Verify peripheral is initialized (check boot messages)
   - Check address is correct (refer to ESP32-C6 TRM)
   - Try reading right after firmware boots

### Important Notes

- **DO NOT guess or assume** - If a test cannot run due to missing hardware, mark as SKIPPED with reason
- **Capture actual output** - Include real probe-rs output in report, not generic examples
- **Be thorough** - Even if a test passes, note any unexpected behavior
- **Time-box tests** - If a test hangs, wait max 30 seconds then FAIL and move on
- **Clean up processes** - probe-rs auto-cleans on exit, but check for orphaned processes

### Best Practices: Shell Commands and Scripts

**For Complex Tests with Conditionals:**

If a test requires multi-line logic with if/then/fi or complex conditionals, write to a temporary script to avoid shell parsing issues:

```bash
cat > /tmp/test_script.sh << 'SCRIPT'
#!/bin/bash

# Your test logic here with proper if/then/fi
if [ -f some_file ]; then
    echo "File exists"
else
    echo "File missing"
fi
SCRIPT

chmod +x /tmp/test_script.sh
/tmp/test_script.sh
```

**Why?** The bash tool executes commands in an eval context that can have issues with complex syntax. Temp scripts avoid parse errors like `(eval):1: parse error near 'then'`.

**Variable Persistence:**

If environment variables from Step 0 don't persist across bash calls:
- Option 1: Re-detect at start of each test (e.g., `USB_CDC_PORT=$(ls /dev/cu.usbmodem* | head -1)`)
- Option 2: Hardcode the values you detected in Step 0
- Option 3: Consolidate related tests into one large bash call

**Quoting probe arguments:**

Always quote the probe argument when it contains colons:
```bash
probe-rs attach --chip esp32c6 --probe "$ESP_PROBE" target/.../main
# or with literal value:
probe-rs attach --chip esp32c6 --probe "303a:1001:F0:F5:BD:01:88:2C" target/.../main
```

### Success Criteria

**Quick mode passes if:**
- At least 9/11 tests pass (81%+)
- Firmware boots successfully
- probe-rs can attach and read registers
- No critical issues found

**Full mode passes if:**
- At least 12/15 tests pass (80%+)
- At least one breakpoint test succeeds
- Report identifies any lesson documentation gaps

---

**After testing, report your findings to the user with the markdown report.**
