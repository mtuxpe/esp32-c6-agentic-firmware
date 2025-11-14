---
description: Generate all 7 GDB lessons sequentially using the /gen-lesson workflow
---

# /gen-all-lessons - Complete GDB Curriculum Generator

**Purpose**: Orchestrate creation of all 7 GDB lessons (01-07) sequentially, following the curriculum in `GDB_LESSON_PLANS.md`. Each lesson uses the `/gen-lesson` workflow with discovery-based pedagogy.

**Target Audience**: Project maintainers creating the complete ESP32-C6 + GDB curriculum.

**Time Estimate**: 14-28 hours total (2-4 hours per lesson × 7 lessons)

**Prerequisites**:
- Read `GDB_LESSON_PLANS.md` completely
- Hardware available: ESP32-C6, LED, FTDI UART, I2C sensor, SPI OLED, servo
- On `main` branch with clean working directory
- All dependencies installed (esp-hal, espflash, probe-rs, GDB)

---

## Workflow Overview

```
/gen-all-lessons
  ├─ Lesson 01: GPIO + GDB Fundamentals (⭐⭐☆☆☆, 60-90 min)
  ├─ Lesson 02: UART + DMA (⭐⭐⭐☆☆, 90-120 min)
  ├─ Lesson 03: I2C + GDB (⭐⭐⭐☆☆, 90-120 min)
  ├─ Lesson 04: SPI + OLED (⭐⭐⭐⭐☆, 120-150 min)
  ├─ Lesson 05: PWM + Servo (⭐⭐⭐☆☆, 90-120 min)
  ├─ Lesson 06: Multi-peripheral (⭐⭐⭐⭐☆, 120-180 min)
  └─ Lesson 07: Production Debug (⭐⭐⭐⭐⭐, 150-240 min)
```

**Each lesson follows**:
1. Review lesson plan from `GDB_LESSON_PLANS.md`
2. Proactive hardware testing
3. Progressive commit development (3-5 commits)
4. Documentation with commit walkthrough
5. Hardware validation
6. PR creation

---

## Pre-Flight Checklist

Before starting, verify:

### Repository State
```bash
# Must be on main branch
git branch --show-current  # Should show "main"

# Must have clean working directory
git status  # Should show "nothing to commit, working tree clean"

# Must have latest GDB documentation
ls -la GDB_*.md  # Should show 3 files
```

### Hardware Available
- [ ] ESP32-C6 development board (USB-C cable)
- [ ] LED + 220Ω resistor (Lesson 01)
- [ ] FTDI UART adapter (Lesson 02)
- [ ] I2C sensor (BME280, MPU6050, or similar) (Lesson 03)
- [ ] SPI OLED display (SSD1306 or similar) (Lesson 04)
- [ ] Servo motor (Lesson 05)
- [ ] All components for multi-peripheral lesson (Lesson 06)
- [ ] Breadboard, jumper wires

### Software Installed
```bash
# Verify tooling
cargo --version          # Rust installed
espflash --version       # ESP flashing tool
probe-rs --version       # Debugging tool
riscv32-esp-elf-gdb --version  # GDB for RISC-V

# Verify esp-hal version
cargo search esp-hal     # Should show 1.0.0 or newer
```

**If any checklist item fails**: Stop and fix before proceeding.

---

## Lesson 01: GPIO + GDB Fundamentals

**Duration**: 60-90 minutes
**Complexity**: ⭐⭐☆☆☆
**Hardware**: ESP32-C6 + LED + 220Ω resistor

### Lesson Specification

From `GDB_LESSON_PLANS.md`:

**GDB Techniques (3)**:
1. Memory inspection/writes - Read/write GPIO registers
2. GDB variables - Bit math calculator
3. Function calls - Call Rust functions from GDB

**Commit Structure (3 commits)**:
- Commit 1: Broken firmware (missing GPIO enable)
- Commit 2: GDB register control (bit math)
- Commit 3: Function calls (remote control)

**Wow Moment**: Calling `led_toggle()` from GDB while firmware runs

### Hardware Setup

**Wiring**:
```
ESP32-C6        LED
--------        ---
GPIO12     -->  Anode (long leg)
               Cathode (short leg) --> 220Ω resistor --> GND
GND        -->  GND
```

### Execution

**Step 1**: Review lesson plan
```bash
# Read Lesson 01 section in GDB_LESSON_PLANS.md (lines 1-80)
```

**Step 2**: Test hardware
```bash
/test-hardware gpio 12
```

**Expected output**: LED blinks, confirming GPIO12 works

**Step 3**: Generate lesson
```bash
/gen-lesson "Create Lesson 01: GPIO + GDB Fundamentals"
```

**Agent will**:
1. Create branch `lesson-01-gpio-gdb`
2. Copy structure from template
3. Implement 3 progressive commits:
   - Commit 1: Broken LED code (missing GPIO enable)
   - Commit 2: GDB-controlled LED (bit math with `set $mask`)
   - Commit 3: Add `led_on()`, `led_off()`, `led_toggle()` functions
4. Create README with commit walkthrough
5. Request hardware validation

**Step 4**: Validate hardware (USER ACTION REQUIRED)

Test each commit:
```bash
cd lessons/01-gpio-gdb

# Test Commit 1: Broken firmware
git checkout <commit-1-hash>
cargo run --release
# Expected: LED doesn't blink (broken)

# Test Commit 2: GDB control
git checkout <commit-2-hash>
cargo run --release
# In GDB: set $gpio = 12; set $mask = 1 << $gpio; set *(uint32_t*)0x60091008 = $mask
# Expected: LED turns on via GDB

# Test Commit 3: Function calls
git checkout <commit-3-hash>
cargo run --release
# In GDB: call led_toggle()
# Expected: LED toggles via GDB
```

**Step 5**: Confirm validation
> Type **"yes"** to proceed to PR creation

**Step 6**: Review PR

Agent creates PR for `lesson-01-gpio-gdb` branch.

### Success Criteria

- [ ] All 3 commits build successfully
- [ ] Commit 1: LED doesn't blink (broken as expected)
- [ ] Commit 2: LED controllable via GDB memory writes
- [ ] Commit 3: LED controllable via GDB function calls
- [ ] README has commit-by-commit walkthrough
- [ ] PR created and ready for review

---

## Lesson 02: UART + DMA

**Duration**: 90-120 minutes
**Complexity**: ⭐⭐⭐☆☆
**Hardware**: ESP32-C6 + FTDI UART adapter

### Lesson Specification

From `GDB_LESSON_PLANS.md`:

**GDB Techniques (3)**:
1. Watchpoints - Break when UART FIFO overflows
2. Conditional breakpoints - Only break on errors
3. Call stack - Debug panic in ISR

**Commit Structure (5 commits)**:
- Commit 1: UART init (minimal)
- Commit 2: Add data streaming
- Commit 3: Introduce DMA
- Commit 4: Watchpoints for buffer overflow
- Commit 5: Conditional breakpoints for errors

**Wow Moment**: Watchpoint catches buffer overflow in real-time

### Hardware Setup

**Wiring**:
```
ESP32-C6        FTDI UART
--------        ---------
GPIO16 (TX) --> RX
GPIO17 (RX) --> TX
GND         --> GND
```

### Execution

**Step 1**: Review lesson plan
```bash
# Read Lesson 02 section in GDB_LESSON_PLANS.md
```

**Step 2**: Test hardware
```bash
/test-hardware uart 16 17
```

**Expected output**: UART data visible on serial monitor

**Step 3**: Generate lesson
```bash
/gen-lesson "Create Lesson 02: UART + DMA"
```

**Step 4**: Validate hardware (USER ACTION REQUIRED)

Test each of 5 commits with GDB commands from README.

**Step 5**: Confirm and create PR

### Success Criteria

- [ ] All 5 commits build successfully
- [ ] UART streams data correctly
- [ ] DMA reduces CPU usage
- [ ] Watchpoint catches buffer overflow
- [ ] Conditional breakpoint only triggers on errors
- [ ] README documents all GDB techniques
- [ ] PR created

---

## Lesson 03: I2C + GDB

**Duration**: 90-120 minutes
**Complexity**: ⭐⭐⭐☆☆
**Hardware**: ESP32-C6 + I2C sensor (BME280 or MPU6050)

### Lesson Specification

From `GDB_LESSON_PLANS.md`:

**GDB Techniques (3)**:
1. Reverse continue - Rewind to find I2C error cause
2. Register diff - Compare before/after I2C transactions
3. Tracepoints - Non-intrusive logging

**Commit Structure (4 commits)**:
- Commit 1: I2C init (minimal)
- Commit 2: Read sensor data
- Commit 3: Add reverse debugging
- Commit 4: Tracepoints for performance analysis

**Wow Moment**: Reverse debugging to find I2C error root cause

### Hardware Setup

**Wiring** (BME280 example):
```
ESP32-C6        BME280
--------        ------
GPIO6 (SDA) --> SDA
GPIO7 (SCL) --> SCL
3.3V        --> VCC
GND         --> GND
```

### Execution

**Step 1**: Review lesson plan
```bash
# Read Lesson 03 section in GDB_LESSON_PLANS.md
```

**Step 2**: Test hardware
```bash
/test-hardware i2c 6 7
```

**Expected output**: I2C device detected at address (0x76 for BME280)

**Step 3**: Generate lesson
```bash
/gen-lesson "Create Lesson 03: I2C + GDB"
```

**Step 4**: Validate hardware (USER ACTION REQUIRED)

Test each of 4 commits.

**Step 5**: Confirm and create PR

### Success Criteria

- [ ] All 4 commits build successfully
- [ ] I2C sensor detected and read
- [ ] Reverse debugging works (requires probe-rs with record/replay)
- [ ] Register diff shows I2C state changes
- [ ] Tracepoints log without stopping execution
- [ ] README documents techniques
- [ ] PR created

---

## Lesson 04: SPI + OLED

**Duration**: 120-150 minutes
**Complexity**: ⭐⭐⭐⭐☆
**Hardware**: ESP32-C6 + SPI OLED display (SSD1306)

### Lesson Specification

From `GDB_LESSON_PLANS.md`:

**GDB Techniques (3)**:
1. Python scripting - Automate register inspection
2. Macro debugger - Custom GDB commands
3. Memory compare - Verify framebuffer updates

**Commit Structure (5 commits)**:
- Commit 1: SPI init (minimal)
- Commit 2: OLED initialization sequence
- Commit 3: Framebuffer rendering
- Commit 4: Python GDB script for auto-inspection
- Commit 5: Custom GDB macros

**Wow Moment**: Python script auto-inspects SPI transactions

### Hardware Setup

**Wiring** (SSD1306 SPI example):
```
ESP32-C6        SSD1306 OLED
--------        ------------
GPIO18 (MOSI)-->  SDA (MOSI)
GPIO19 (SCK) -->  SCK
GPIO20 (CS)  -->  CS
GPIO21 (DC)  -->  DC
GPIO22 (RST) -->  RES
3.3V         -->  VCC
GND          -->  GND
```

### Execution

**Step 1**: Review lesson plan

**Step 2**: Test hardware
```bash
/test-hardware spi 18 19 20
```

**Step 3**: Generate lesson
```bash
/gen-lesson "Create Lesson 04: SPI + OLED"
```

**Step 4**: Validate hardware (USER ACTION REQUIRED)

**Step 5**: Confirm and create PR

### Success Criteria

- [ ] All 5 commits build successfully
- [ ] OLED displays graphics
- [ ] Python GDB script works
- [ ] Custom GDB macros simplify debugging
- [ ] Memory compare verifies framebuffer
- [ ] README includes Python scripts
- [ ] PR created

---

## Lesson 05: PWM + Servo

**Duration**: 90-120 minutes
**Complexity**: ⭐⭐⭐☆☆
**Hardware**: ESP32-C6 + servo motor

### Lesson Specification

From `GDB_LESSON_PLANS.md`:

**GDB Techniques (3)**:
1. Disassembly - Inspect PWM timer assembly
2. Instruction stepping - Step through ISR
3. Performance analysis - Measure ISR timing

**Commit Structure (4 commits)**:
- Commit 1: PWM init (minimal)
- Commit 2: Servo control (angle setting)
- Commit 3: Disassembly inspection
- Commit 4: Performance profiling

**Wow Moment**: Disassembly reveals PWM timer optimization

### Hardware Setup

**Wiring**:
```
ESP32-C6        Servo
--------        -----
GPIO23 (PWM)-->  Signal (orange/yellow)
5V          -->  VCC (red)
GND         -->  GND (brown/black)
```

### Execution

**Step 1**: Review lesson plan

**Step 2**: Test hardware
```bash
/test-hardware pwm 23
```

**Step 3**: Generate lesson
```bash
/gen-lesson "Create Lesson 05: PWM + Servo"
```

**Step 4**: Validate hardware (USER ACTION REQUIRED)

**Step 5**: Confirm and create PR

### Success Criteria

- [ ] All 4 commits build successfully
- [ ] Servo moves to commanded angles
- [ ] Disassembly shows PWM timer code
- [ ] Instruction stepping works in ISR
- [ ] Performance analysis measures timing
- [ ] README documents techniques
- [ ] PR created

---

## Lesson 06: Multi-Peripheral Integration

**Duration**: 120-180 minutes
**Complexity**: ⭐⭐⭐⭐☆
**Hardware**: ESP32-C6 + LED + UART + I2C sensor + OLED

### Lesson Specification

From `GDB_LESSON_PLANS.md`:

**GDB Techniques (3)**:
1. Core dumps - Save state for post-mortem analysis
2. Remote memory - Inspect while running
3. Checkpoint/restore - Save/resume execution state

**Commit Structure (5 commits)**:
- Commit 1: Multi-peripheral init (GPIO + UART + I2C + SPI)
- Commit 2: Sensor data pipeline (I2C → processing → OLED)
- Commit 3: Core dump on panic
- Commit 4: Remote memory inspection
- Commit 5: Checkpoint/restore workflow

**Wow Moment**: Core dump captures state at crash, analyze offline

### Hardware Setup

**Wiring**: Combines all previous lessons
- GPIO12: LED
- GPIO16/17: UART
- GPIO6/7: I2C sensor
- GPIO18-22: SPI OLED

### Execution

**Step 1**: Review lesson plan

**Step 2**: Test hardware (multi-peripheral)
```bash
# Test each peripheral individually first
/test-hardware gpio 12
/test-hardware uart 16 17
/test-hardware i2c 6 7
/test-hardware spi 18 19 20
```

**Step 3**: Generate lesson
```bash
/gen-lesson "Create Lesson 06: Multi-Peripheral Integration"
```

**Step 4**: Validate hardware (USER ACTION REQUIRED)

**Step 5**: Confirm and create PR

### Success Criteria

- [ ] All 5 commits build successfully
- [ ] All peripherals work together
- [ ] Core dump generated on panic
- [ ] Remote memory inspection works while running
- [ ] Checkpoint/restore saves and resumes state
- [ ] README documents workflow
- [ ] PR created

---

## Lesson 07: Production Debugging

**Duration**: 150-240 minutes
**Complexity**: ⭐⭐⭐⭐⭐
**Hardware**: ESP32-C6 + all peripherals

### Lesson Specification

From `GDB_LESSON_PLANS.md`:

**GDB Techniques (3)**:
1. Automated test harness - GDB scripts for regression testing
2. Trace analysis - Record and replay execution
3. Historical debugging - Time-travel debugging

**Commit Structure (6 commits)**:
- Commit 1: Production firmware with intentional bugs
- Commit 2: GDB test harness setup
- Commit 3: Automated regression tests
- Commit 4: Trace recording
- Commit 5: Historical debugging workflow
- Commit 6: Full debugging toolkit

**Wow Moment**: Time-travel debugging finds race condition

### Hardware Setup

**Wiring**: Full system (all peripherals from Lessons 01-06)

### Execution

**Step 1**: Review lesson plan

**Step 2**: Test hardware (full system)

**Step 3**: Generate lesson
```bash
/gen-lesson "Create Lesson 07: Production Debugging"
```

**Step 4**: Validate hardware (USER ACTION REQUIRED)

**Step 5**: Confirm and create PR

### Success Criteria

- [ ] All 6 commits build successfully
- [ ] Test harness automates debugging
- [ ] Trace recording captures execution
- [ ] Historical debugging works
- [ ] Time-travel debugging finds bugs
- [ ] README documents production workflow
- [ ] PR created

---

## Post-Lesson Workflow

After each lesson is complete:

### 1. Merge PR to Main

```bash
# Review PR
gh pr view <PR-number>

# Merge PR
gh pr merge <PR-number> --squash

# Update local main
git checkout main
git pull origin main
```

### 2. Update Progress Tracker

**Track completion** in project README or tracking document:

```markdown
## GDB Lesson Curriculum Progress

- [x] Lesson 01: GPIO + GDB Fundamentals (⭐⭐☆☆☆)
- [ ] Lesson 02: UART + DMA (⭐⭐⭐☆☆)
- [ ] Lesson 03: I2C + GDB (⭐⭐⭐☆☆)
- [ ] Lesson 04: SPI + OLED (⭐⭐⭐⭐☆)
- [ ] Lesson 05: PWM + Servo (⭐⭐⭐☆☆)
- [ ] Lesson 06: Multi-Peripheral (⭐⭐⭐⭐☆)
- [ ] Lesson 07: Production Debug (⭐⭐⭐⭐⭐)
```

### 3. Clean Up

```bash
# Delete merged branch (optional)
git branch -d lesson-{NN}-{name}
git push origin --delete lesson-{NN}-{name}
```

### 4. Move to Next Lesson

Return to main branch and proceed:
```bash
git checkout main
git pull origin main
# Ready for next lesson
```

---

## Checkpoint Strategy

**After each lesson completes**:
1. Merge PR to main
2. Update progress tracker
3. Take a break (15-30 minutes)
4. Review next lesson specification in `GDB_LESSON_PLANS.md`
5. Proceed to next lesson

**If blocked**:
- Hardware not working → Use `/test-hardware` to troubleshoot
- Code not building → Review esp-hal 1.0.0 docs
- GDB issues → Consult `GDB_REFERENCE.md`
- Unclear requirements → Review `GDB_LESSON_PLANS.md`

---

## Key Principles

### 1. Sequential Execution

**Must complete lessons in order**:
- Lesson 02 builds on Lesson 01 concepts
- Lesson 03 assumes GDB basics from Lessons 01-02
- Later lessons combine techniques from earlier ones

**Cannot skip lessons** - each is prerequisite for next.

### 2. Hardware Validation Required

**Every lesson must be tested on real hardware**:
- Use `/test-hardware` before implementation
- Validate all commits work
- Don't proceed with broken hardware

### 3. Progressive Commits

**Each lesson has 3-6 commits**:
- Commit 1: Broken or minimal (discovery phase)
- Commit 2+: Progressive GDB techniques
- Final commit: Complete implementation

**Test each commit** before moving to next lesson.

### 4. Documentation Quality

**README for each lesson must include**:
- Commit-by-commit walkthrough
- GDB commands for each commit
- Hardware wiring diagrams
- Troubleshooting section
- References to GDB_LESSON_PLANS.md

### 5. Collaborative Process

**This is a collaborative workflow**:
- Agent implements lessons following `/gen-lesson`
- User validates hardware at checkpoints
- User confirms before PRs are created
- User decides when to merge and proceed

**Not autonomous** - requires user confirmation at validation points.

---

## Expected Timeline

| Lesson | Duration | Cumulative Time |
|--------|----------|----------------|
| Lesson 01 | 60-90 min | 1.5 hrs |
| Lesson 02 | 90-120 min | 3.5 hrs |
| Lesson 03 | 90-120 min | 5.5 hrs |
| Lesson 04 | 120-150 min | 8 hrs |
| Lesson 05 | 90-120 min | 10 hrs |
| Lesson 06 | 120-180 min | 13 hrs |
| Lesson 07 | 150-240 min | 17 hrs |

**Total**: 14-28 hours (spread over multiple sessions)

**Recommended schedule**:
- Day 1: Lessons 01-02 (4-6 hours)
- Day 2: Lessons 03-04 (4-6 hours)
- Day 3: Lessons 05-06 (4-6 hours)
- Day 4: Lesson 07 (3-4 hours)

---

## Completion Criteria

All 7 lessons complete when:

- [ ] All lesson branches merged to main
- [ ] All PRs closed
- [ ] All READMEs have commit walkthroughs
- [ ] All lessons tested on hardware
- [ ] Progress tracker shows 7/7 complete
- [ ] Repository has `lessons/01-gpio-gdb/` through `lessons/07-production-debug/`

---

## Tips for Success

**Prepare hardware ahead of time**:
- Have all components ready before starting
- Test each peripheral individually
- Document working pin configurations
- Keep breadboard organized

**Take breaks**:
- Don't try to do all 7 lessons in one session
- Review lesson plan between lessons
- Test thoroughly at each checkpoint

**Reference materials**:
- Keep `GDB_LESSON_PLANS.md` open
- Consult `GDB_REFERENCE.md` for GDB commands
- Review `GDB_EXECUTIVE_SUMMARY.md` for quick lookup

**Ask for help**:
- If hardware doesn't work, troubleshoot before proceeding
- If GDB technique unclear, review reference docs
- If stuck, take a break and come back fresh

---

**This workflow orchestrates creation of all 7 GDB lessons sequentially, ensuring a complete, tested, and documented curriculum for ESP32-C6 + esp-hal 1.0.0 + GDB discovery-based learning.**
