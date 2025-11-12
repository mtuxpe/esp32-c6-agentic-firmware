# Unified Lesson Testing Guide

## Overview

We've implemented Option 2: **One command with argument + TEST.md per lesson**

This approach provides:
- ✅ Single command to maintain (`/test-lesson`)
- ✅ Lesson-specific test specs (each lesson has `TEST.md`)
- ✅ Scalable to 12+ lessons
- ✅ Consistent test structure across all lessons

---

## Quick Start

### Test a lesson:
```bash
/test-lesson 07          # Quick test of lesson 07 (~3 min)
/test-lesson 08 full     # Full test of lesson 08 (~10 min)
/test-lesson 03          # Quick test of lesson 03
```

The command will:
1. Find the lesson directory
2. Read `lessons/{NN}-{name}/TEST.md`
3. Execute tests according to specification
4. Generate markdown report

---

## File Structure

```
esp32-c6-agentic-firmware/
├── .claude/
│   ├── commands/
│   │   └── test-lesson.md        # Unified test command
│   ├── TEST.md.template           # Template for new lessons
│   └── TESTING-GUIDE.md           # This file
│
└── lessons/
    ├── 07-gdb-debugging/
    │   ├── TEST.md                # Lesson 07 test spec
    │   ├── src/...
    │   └── Cargo.toml
    │
    └── 08-defmt-rtt-logging/
        ├── TEST.md                # Lesson 08 test spec
        ├── src/...
        └── Cargo.toml
```

---

## TEST.md Format

Each lesson's `TEST.md` follows this structure:

```markdown
# Lesson XX Test Specification

**Lesson:** {number} - {name}
**Hardware:** {components}
**Test Duration:** ~X minutes (quick), ~Y minutes (full)

## Hardware Setup
[Wiring diagram and verification checklist]

## Test Modes
[Quick vs Full mode descriptions]

## Automated Tests
[Tests that run automatically]
- Test 1: Build verification
- Test 2: Debug symbols
- Test 3: Source structure
- Test 4: Flash firmware
- Test 5: Boot verification
- Test 6+: Lesson-specific tests

## Interactive Tests (Manual)
[Tests requiring manual execution]

## Expected Outputs
[What you should see when it works]

## Troubleshooting
[Common issues and solutions]

## Performance Benchmarks (Full Mode)
[Expected performance metrics]

## Notes
[Any special considerations]
```

---

## Creating TEST.md for New Lessons

### Option 1: Copy Template
```bash
cp .claude/TEST.md.template lessons/{NN}-{name}/TEST.md
# Edit and customize for your lesson
```

### Option 2: Ask Claude Code
```
Create a TEST.md for lesson XX following the template
```

### Required Sections

**Minimum (for quick mode):**
- Hardware Setup
- Automated Tests (build, flash, infrastructure)
- Expected Outputs
- Troubleshooting

**Full Testing:**
- All above sections
- Interactive Tests
- Performance Benchmarks
- Comparison notes (if applicable)

---

## Test Command Details

### Arguments

```bash
/test-lesson <lesson_number> [mode]
```

**lesson_number:** Required
- Format: "07", "08", "03", etc.
- Automatically normalized (7 → 07)

**mode:** Optional (defaults to "quick")
- `quick` - Fast validation (3-5 min)
- `full` - Comprehensive testing (10-20 min)

### What It Does

1. **Parse Arguments**
   - Extracts lesson number
   - Finds lesson directory
   - Checks for TEST.md

2. **Environment Setup**
   - Auto-detects USB CDC port
   - Auto-detects JTAG probe
   - Auto-detects UART port (if present)
   - Cleans up orphaned debug processes

3. **Execute Tests**
   - Reads TEST.md specification
   - Runs automated tests
   - Documents manual tests
   - Captures results

4. **Generate Report**
   - Markdown format
   - Summary statistics
   - Detailed test results
   - Issues and recommendations

### Success Criteria

**Quick Mode:**
- 70%+ of automated tests pass
- Firmware builds and flashes
- No critical configuration issues

**Full Mode:**
- 100% of automated tests pass
- Manual test instructions clear
- All documented issues resolved

---

## Hardware Auto-Detection

The command automatically detects:

**USB CDC Port (for flashing):**
```bash
/dev/cu.usbmodem*    # macOS
/dev/ttyACM*         # Linux
```

**ESP JTAG Probe:**
```bash
probe-rs list | grep "esp.*jtag"
# Extracts: 303a:1001:F0:F5:BD:01:88:2C
```

**UART Port (optional):**
```bash
/dev/cu.usbserial*   # macOS
/dev/ttyUSB*         # Linux
```

This handles:
- Multiple probes connected
- USB port changes after replug
- Missing optional hardware

---

## Example: Testing Lesson 07

### Quick Test (3 minutes)

```bash
/test-lesson 07
```

**Output:**
```
=== Test Lesson 07: 07-gdb-debugging ===
Mode: quick
Directory: /path/to/lessons/07-gdb-debugging

✓ USB CDC: /dev/cu.usbmodem1101
✓ ESP Probe: 303a:1001:F0:F5:BD:01:88:2C
✓ Build complete
✓ Flash complete

=== Infrastructure Tests ===
✓ Test 8: Debug Symbols - PASS
✓ Test 9: Source Code Structure - PASS
✓ Test 10: GDB Configuration Files - PASS
✓ Test 11: Python Helper Script Syntax - PASS
✓ Test 12: Cargo.toml Debug Configuration - PASS

=== Summary ===
Total Tests: 5
Passed: 5/5 (100%)
Status: PASS
```

### Full Test (15 minutes)

```bash
/test-lesson 07 full
```

Includes all quick mode tests plus:
- Interactive debugging verification
- Register inspection tests
- Memory debugging tests
- Performance profiling

---

## Adding Tests to Existing Lessons

### Step 1: Create TEST.md

```bash
cd lessons/03-mpu9250
cp ../../.claude/TEST.md.template TEST.md
```

### Step 2: Customize for Lesson

Edit TEST.md:
- Update hardware connections
- List lesson-specific tests
- Document expected outputs
- Add troubleshooting tips

### Step 3: Test It

```bash
/test-lesson 03
```

### Step 4: Iterate

- Run the test
- Fix any issues in TEST.md
- Verify on hardware
- Commit when working

---

## Best Practices

### TEST.md Writing

**DO:**
- ✅ Be specific (exact pin numbers, commands)
- ✅ Include expected outputs (copy/paste actual)
- ✅ Document common issues
- ✅ Keep automated tests fast (<30 sec each)
- ✅ Make manual tests clear (step-by-step)

**DON'T:**
- ❌ Assume hardware state
- ❌ Use generic examples
- ❌ Skip troubleshooting section
- ❌ Make tests hardware-dependent without checks

### Test Organization

**Automated Tests Should:**
- Run without user interaction
- Complete in <5 minutes (quick mode)
- Check infrastructure (build, flash, configs)
- Verify basic functionality

**Manual Tests Should:**
- Require interactive terminal (probe-rs, GDB)
- Verify hardware behavior
- Test edge cases
- Validate performance

---

## Troubleshooting

### "Lesson XX not found"

**Cause:** Lesson directory doesn't match pattern
**Solution:**
```bash
ls lessons/ | grep "^[0-9]"  # List lessons
# Ensure directory is named: NN-name
```

### "No TEST.md found"

**Cause:** TEST.md missing for lesson
**Solution:**
```bash
cp .claude/TEST.md.template lessons/{NN}-{name}/TEST.md
# Edit and customize
```

### "USB CDC port not found"

**Cause:** ESP32-C6 not connected
**Solution:**
```bash
ls /dev/cu.usbmodem*    # macOS
ls /dev/ttyACM*         # Linux
# Replug USB cable
```

### "ESP Probe not detected"

**Cause:** JTAG debugger not connected
**Solution:**
```bash
probe-rs list
# Should show ESP JTAG probe
# Check JTAG wiring (GPIO 4,5,6,7)
```

---

## Migration Plan (Future Lessons)

### Current Status

- ✅ Lesson 07: TEST.md created
- ✅ Lesson 08: TEST.md created
- ⏳ Lessons 01-06: TBD
- ⏳ Lessons 09+: TBD

### To Add TEST.md to Existing Lessons

1. Pick a lesson (e.g., 03-mpu9250)
2. Copy template: `cp .claude/TEST.md.template lessons/03-mpu9250/TEST.md`
3. Wire up hardware
4. Run lesson manually and document:
   - Expected boot output
   - Hardware connections
   - Common issues
5. Fill in TEST.md sections
6. Test with `/test-lesson 03`
7. Commit when working

### For New Lessons

1. Create lesson code
2. Create TEST.md alongside (copy template)
3. Document as you develop
4. Test with `/test-lesson {NN}`
5. Iterate until passing

---

## Future Improvements

### Possible Enhancements

1. **Auto-generate TEST.md skeleton**
   - Parse Cargo.toml for dependencies
   - Scan code for GPIO pins
   - Generate boilerplate

2. **Test result database**
   - Track pass/fail over time
   - Regression detection
   - Performance trending

3. **Hardware profiles**
   - Save USB port mappings
   - Remember probe configurations
   - Support multiple setups

4. **CI/CD integration**
   - Automated testing on hardware
   - Nightly regression tests
   - Pull request validation

---

## Summary

**We now have:**
- ✅ One unified `/test-lesson` command
- ✅ Lesson-specific `TEST.md` files
- ✅ Reusable template (`.claude/TEST.md.template`)
- ✅ Auto-detecting hardware configuration
- ✅ Comprehensive test reports
- ✅ Documented in CLAUDE.md

**Next steps:**
1. Add TEST.md to remaining lessons (01-06, 09+)
2. Test on real hardware
3. Iterate and improve based on usage
4. Consider adding more lessons!

---

**Last Updated:** 2025-11-12
**Status:** Implemented and tested on Lessons 07-08
