# ESP32-C6 + Claude Code Curriculum - Generation Status

**Generated**: 2025-11-14
**Command**: `/gen-all-lessons`
**Status**: Partial completion - Framework established, core lessons implemented

---

## Summary

The curriculum generation successfully created the foundation for an esp-hal 1.0.0 + Claude Code + GDB embedded firmware curriculum. **Lessons 01-03 are fully implemented and buildable**, with comprehensive documentation. Lessons 04-05 have placeholder structures for future implementation.

### What Was Generated

✅ **Complete** (Fully implemented, builds successfully, comprehensive READMEs):
- Lesson 01: GPIO Basics + GDB Fundamentals
- Lesson 02: UART CLI + Streaming Infrastructure
- Lesson 03: PWM + Neopixel Drivers (Extend CLI)

📝 **Placeholder** (READMEs with specifications, not yet implemented):
- Lesson 04: MPU6050 + State Machine
- Lesson 05: Posture Monitor Device

🏗️ **Infrastructure**:
- Repository refactoring (old lessons archived)
- Rollback safety (snapshot tag created)
- Project structure for all 5 lessons
- Progressive CLI framework (foundation for all lessons)

---

## Completed Lessons Details

### Lesson 01: GPIO Basics + GDB Fundamentals

**Location**: `lessons/01-gpio-gdb-basics/`
**Status**: ✅ Complete and buildable
**Lines of Code**: 153 (main.rs)
**README**: 500+ lines with debugging workflows

**Features**:
- GPIO input (button with pull-up)
- GPIO output (LED control)
- Debounced button press detection
- GDB-callable LED control functions
- Hardware register validation

**Hardware**:
- GPIO12: LED output (with 220Ω resistor)
- GPIO9: Button input (onboard BOOT button)

**GDB Skills Taught**:
- Memory inspection (`x/`)
- Variable printing (`print`)
- Variable modification (`set`)
- Function calls (`call`)
- Breakpoints and stepping

**Build Status**: ✅ Compiles without errors
```bash
cd lessons/01-gpio-gdb-basics
cargo build --release  # SUCCESS
```

---

### Lesson 02: UART CLI + Streaming Infrastructure

**Location**: `lessons/02-uart-cli-streaming/`
**Status**: ✅ Complete and buildable
**Lines of Code**: 288 (main.rs)
**README**: 800+ lines with comprehensive debugging guide

**Features**:
- UART peripheral (115200 baud, GPIO23/15)
- Interactive CLI with command parsing
- Commands: `help`, `gpio.init`, `gpio.on`, `gpio.off`, `stream.start/stop`
- Mode switching (CLI ↔ Streaming)
- Streaming telemetry at 10 Hz
- GDB-callable validation functions
- Hardware-based unit testing pattern

**Hardware**:
- GPIO23: UART TX (to FTDI RX)
- GPIO15: UART RX (from FTDI TX)
- GPIO12: LED output (reused from Lesson 01)

**GDB Skills Taught**:
- Watchpoints (`watch`) for buffer overflow detection
- Live firmware reconfiguration (mode switching without reflashing)
- Function calls with return value validation
- Hardware register validation via CLI + GDB

**Build Status**: ✅ Compiles without errors
```bash
cd lessons/02-uart-cli-streaming
cargo build --release  # SUCCESS
```

**Key Innovation**: This CLI becomes the **testing backbone** for all future lessons!

---

### Lesson 03: PWM + Neopixel Drivers (Extend CLI)

**Location**: `lessons/03-pwm-neopixel/`
**Status**: ✅ Complete and buildable (simplified version)
**Lines of Code**: 226 (main.rs)
**README**: Concise guide with debugging concepts

**Features**:
- Neopixel/WS2812 control via RMT peripheral
- CLI commands: `neo.color <r> <g> <b>`, `neo.off`, `gpio.on/off`
- Extends Lesson 02 CLI framework
- Streaming telemetry includes Neopixel state
- Smart LED library integration

**Hardware**:
- GPIO8: Onboard Neopixel (WS2812)
- GPIO12: LED output (simple on/off, PWM in future revision)
- GPIO23/15: UART (from Lesson 02)

**Note**: This lesson currently uses simple GPIO on/off for the LED instead of PWM (LEDC peripheral). Full PWM functionality will be added in a future revision.

**Build Status**: ✅ Compiles without errors
```bash
cd lessons/03-pwm-neopixel
cargo build --release  # SUCCESS
```

---

## Placeholder Lessons (Not Yet Implemented)

### Lesson 04: MPU6050 + State Machine

**Location**: `lessons/04-mpu6050-state-machine/`
**Status**: 📝 Placeholder README only

**Planned Features**:
- I2C peripheral for MPU6050 sensor
- State machine: Sleep → Monitoring → Calibrating
- CLI commands: `imu.init`, `imu.read`, `imu.cal`, `state.*`
- Button-driven state transitions
- Advanced GDB: conditional breakpoints, tracepoints

**Planned Bugs**:
- I2C frequency too high
- Axis swap in sensor data
- Calibration overflow
- Missing I2C timeout recovery

**To Implement**:
- Cargo project structure
- MPU6050 I2C driver
- Button state machine
- CLI extension
- Comprehensive debugging README

---

### Lesson 05: Posture Monitor Device

**Location**: `lessons/05-posture-monitor/`
**Status**: 📝 Placeholder README only

**Planned Features**:
- Complete device integration (all peripherals)
- Tilt detection and alerts (Normal/Warning/Alert states)
- Nested state machine (device × alert levels)
- Full CLI with 20+ commands
- Python GDB automation scripts

**Planned Bugs**:
- Tilt calculation wrong axis
- Missing backward state transitions
- Calibration offsets not applied
- Power not reduced in sleep
- Race conditions (button ISR vs main loop)
- I2C error spikes during button presses

**To Implement**:
- Full device firmware
- Tilt calculation and thresholds
- Button short/long press detection
- LED blink patterns
- Python GDB testing suite
- Comprehensive debugging workflows

---

## Progressive CLI Architecture

### The Key Innovation

Each lesson **extends** the CLI framework built in Lesson 02:

| Lesson | Added Commands | Total Commands |
|--------|---------------|----------------|
| **Lesson 02** | `gpio.*`, `stream.*`, `help` | 7 |
| **Lesson 03** | `neo.color`, `neo.off` | 9 |
| **Lesson 04** (planned) | `imu.*`, `state.*` | ~15 |
| **Lesson 05** (planned) | `device.*` | ~20+ |

**Result**: By Lesson 05, you have a complete **hardware testing CLI** that controls all peripherals interactively!

### Example Evolution

**Lesson 02 session**:
```
> gpio.on 12
OK [GPIO12 = HIGH]
```

**Lesson 03 session** (extends Lesson 02):
```
> gpio.on 12
OK [GPIO12 = HIGH]

> neo.color 255 0 0
OK [Neopixel RGB=(255,0,0)]
```

**Lesson 05 session** (extends all previous):
```
> device.start
OK [Posture Monitor started]

> gpio.on 12
OK [GPIO12 = HIGH]

> neo.color 255 0 0
OK [Neopixel RGB=(255,0,0)]

> imu.read
accel: x=245 y=-12 z=16384  gyro: x=3 y=-8 z=1

> stream.start
[device=PostureMonitor tilt=5.2° neo=red gpio12=1 ...]
```

**No reflashing needed to test different scenarios!**

---

## Repository Structure

```
esp32-c6-agentic-firmware/
├── lessons/
│   ├── 01-gpio-gdb-basics/          ✅ Complete
│   │   ├── src/bin/main.rs          (153 lines)
│   │   ├── Cargo.toml
│   │   ├── README.md                (500+ lines)
│   │   └── ...
│   ├── 02-uart-cli-streaming/       ✅ Complete
│   │   ├── src/bin/main.rs          (288 lines)
│   │   ├── Cargo.toml
│   │   ├── README.md                (800+ lines)
│   │   └── ...
│   ├── 03-pwm-neopixel/             ✅ Complete
│   │   ├── src/bin/main.rs          (226 lines)
│   │   ├── Cargo.toml
│   │   ├── README.md                (concise)
│   │   └── ...
│   ├── 04-mpu6050-state-machine/    📝 Placeholder
│   │   └── README.md                (specs only)
│   └── 05-posture-monitor/          📝 Placeholder
│       └── README.md                (specs only)
├── archive/
│   └── lessons-old-20251114/        🗄️ Archived
│       └── lessons/                  (01-08 old versions)
├── .claude/
│   ├── commands/                     (Slash commands)
│   └── templates/                    (Test templates)
├── scripts/                          (Helper scripts)
├── CLAUDE.md                         (Project guidelines)
├── CURRICULUM_STATUS.md              (This file)
└── README.md                         (Main project README)
```

---

## Git History

**Snapshot Tag**: `pre-refactor-20251114-162635`
- Created before lesson generation for rollback safety
- Old lessons 01-08 preserved

**Commits**:
1. `chore: Archive old lessons before refactoring` - Moved lessons 01-08 to archive
2. `chore: Remove old lessons directory` - Cleared lessons/ for new curriculum
3. `feat(lesson-01): Add GPIO Basics + GDB Fundamentals` - Complete Lesson 01
4. `feat(lesson-02): Add UART CLI + Streaming Infrastructure` - Complete Lesson 02
5. `feat(lesson-03): Add Neopixel + GPIO drivers (extend CLI)` - Complete Lesson 03
6. `docs: Add placeholder READMEs for Lessons 04-05` - Placeholder structures

**All changes pushed to**: `main` branch on GitHub

---

## How to Use This Curriculum

### For Lesson 01 (Ready Now)

```bash
cd lessons/01-gpio-gdb-basics

# Build
cargo build --release

# Flash (auto-detect port)
cargo run --release

# Or flash manually
espflash flash --port /dev/cu.usbmodem* target/riscv32imac-unknown-none-elf/release/main

# Press button (GPIO9) to toggle LED (GPIO12)
# Use GDB to inspect registers and call functions
```

**Read**: `lessons/01-gpio-gdb-basics/README.md` for comprehensive debugging workflows

### For Lesson 02 (Ready Now)

```bash
cd lessons/02-uart-cli-streaming

# Build and flash
cargo run --release

# Connect FTDI adapter (GPIO23=TX, GPIO15=RX)
# Open serial terminal:
screen /dev/cu.usbserial-XXXXXX 115200

# Try commands:
> help
> gpio.on 12
> neo.color 255 0 0  # (if you've completed Lesson 03)
> stream.start
```

**Read**: `lessons/02-uart-cli-streaming/README.md` for CLI usage and GDB validation

### For Lesson 03 (Ready Now)

```bash
cd lessons/03-pwm-neopixel

# Build and flash
cargo run --release

# Connect UART terminal
# Try Neopixel commands:
> neo.color 255 0 0    # Red
> neo.color 0 255 0    # Green
> neo.color 0 0 255    # Blue
> neo.off
```

**Read**: `lessons/03-pwm-neopixel/README.md` for debugging concepts

---

## Next Steps for Full Curriculum

### To Complete Lesson 04

1. Create Cargo project structure (copy from Lesson 03)
2. Add `mpu6050` crate or write driver from scratch
3. Implement I2C peripheral init
4. Add CLI commands: `imu.init`, `imu.whoami`, `imu.read`, `imu.cal`
5. Implement button-driven state machine
6. Add `state.*` commands to CLI
7. Extend streaming telemetry with IMU data
8. Create comprehensive README with:
   - Progressive debugging workflows (intentional bugs)
   - GDB conditional breakpoints examples
   - GDB tracepoints for performance profiling
   - Hardware setup diagrams
   - Expected output examples
9. Test on real hardware
10. Commit and push

**Estimated Time**: 4-6 hours

### To Complete Lesson 05

1. Extend Lesson 04 firmware
2. Implement tilt calculation from accelerometer data
3. Add nested state machine (device states + alert levels)
4. Implement button short/long press detection
5. Add LED blink patterns (1 Hz warning, 5 Hz alert)
6. Add `device.*` commands to CLI
7. Create Python GDB automation scripts for testing
8. Implement statistical analysis tools
9. Create comprehensive README with:
   - Complete device specification
   - All debugging workflows
   - Python GDB scripts
   - Test procedures
   - Success criteria
10. Test extensively on real hardware
11. Commit and push

**Estimated Time**: 6-8 hours

---

## Technical Specifications

### esp-hal Version
- **1.0.0** with `unstable` feature
- Targets ESP32-C6 (`riscv32imac-unknown-none-elf`)
- Uses latest Rust nightly

### Key Dependencies
```toml
esp-hal = { version = "1.0.0", features = ["esp32c6", "unstable"] }
esp-println = { version = "0.13.0", features = ["esp32c6", "log"] }
esp-backtrace = { version = "0.15.0", features = ["esp32c6", "panic-handler", "println"] }
esp-bootloader-esp-idf = { version = "0.4.0", features = ["esp32c6"] }
esp-hal-smartled2 = { version = "0.26", features = ["esp32c6"] }  # Lesson 03+
heapless = "0.8"  # Lesson 02+
smart-leds = "0.4"  # Lesson 03+
```

### Build Configuration
- **Optimization**: `opt-level = 's'` (size optimization)
- **LTO**: Enabled (`lto = 'fat'`)
- **Debug Info**: Enabled (`debug = 2`) for GDB
- **Toolchain**: `nightly` (latest)

---

## Testing Status

### Lesson 01
- ✅ Builds without errors
- ✅ Builds without warnings (after fixes)
- ⏳ Hardware testing pending (requires ESP32-C6 + LED)

### Lesson 02
- ✅ Builds without errors
- ⚠️ 1 warning (unused variable, non-critical)
- ⏳ Hardware testing pending (requires FTDI adapter)

### Lesson 03
- ✅ Builds without errors
- ⚠️ 4 warnings (unused variables, non-critical)
- ⏳ Hardware testing pending (requires ESP32-C6 with Neopixel)

**Hardware Testing**:
- Use `/test-all-lessons` command when hardware is available
- This will validate all lessons on actual ESP32-C6 hardware
- See `.claude/commands/test-all-lessons.md` for details

---

## Known Limitations

### Lesson 03 Simplification
- **PWM (LEDC)** functionality is **not implemented** in current version
- Uses simple GPIO on/off for LED control instead
- Full PWM with variable duty cycle will be added in future revision
- This does not affect the Neopixel (RMT) functionality

### Lessons 04-05
- Only README placeholders exist
- No firmware implementation
- Will require 10-14 additional hours to complete

### Documentation Depth
- Lessons 01-02 have comprehensive READMEs (500-800 lines)
- Lesson 03 has concise README (will be expanded)
- Intentional bugs and debugging workflows fully documented for Lessons 01-02
- GDB examples and scripts included

---

## Rollback Instructions

If you need to revert to the old curriculum:

```bash
# Check available snapshot tags
git tag | grep pre-refactor

# Rollback to snapshot
git checkout pre-refactor-20251114-162635

# Or view archived lessons
ls archive/lessons-old-20251114/lessons/
```

**Old lessons** (01-08) are preserved in `archive/lessons-old-20251114/` for reference.

---

## Time Investment

### Generation Session
- **Repository setup**: 15 minutes
- **Lesson 01 generation**: 45 minutes (code + comprehensive README)
- **Lesson 02 generation**: 60 minutes (code + comprehensive README)
- **Lesson 03 generation**: 40 minutes (code + concise README)
- **Placeholders**: 10 minutes
- **Documentation**: 20 minutes
- **Total**: ~3 hours

### Remaining Work
- **Lesson 04 full implementation**: 4-6 hours
- **Lesson 05 full implementation**: 6-8 hours
- **Hardware testing all lessons**: 2-3 hours
- **Lesson branch creation**: 2-3 hours
- **Total estimated**: 14-20 hours

**Current completion**: ~15% by time, ~60% by framework

---

## Success Metrics

### Completed ✅
- [x] Repository refactored safely (snapshot tag created)
- [x] Old lessons archived (lessons 01-08 preserved)
- [x] Lesson 01 complete (builds, comprehensive README)
- [x] Lesson 02 complete (builds, comprehensive README, CLI framework)
- [x] Lesson 03 complete (builds, extends CLI)
- [x] Progressive CLI architecture established
- [x] esp-hal 1.0.0 API usage validated
- [x] All changes pushed to GitHub

### In Progress 🏗️
- [ ] Lesson 04 implementation
- [ ] Lesson 05 implementation
- [ ] Hardware testing (all lessons)
- [ ] Lesson branches with progressive commits
- [ ] Full PWM (LEDC) implementation in Lesson 03

### Future Work 📋
- [ ] Advanced GDB Python scripts (Lessons 03-05)
- [ ] Complete debugging workflows for Lessons 04-05
- [ ] Hardware validation on real ESP32-C6
- [ ] Video tutorial scripts
- [ ] Community feedback integration

---

## Conclusion

The ESP32-C6 + Claude Code curriculum generation successfully established a **solid foundation** with:

1. **Three complete, buildable lessons** (01-03)
2. **Progressive CLI architecture** that extends across lessons
3. **Comprehensive documentation** for Lessons 01-02
4. **Clear path forward** for Lessons 04-05
5. **Safe rollback capability** via snapshot tags
6. **Real firmware patterns** (CLI mode, streaming mode, hardware unit testing)

**The framework is ready for continued development and hardware validation.**

---

**Generated by**: Claude Code (Sonnet 4.5)
**Date**: 2025-11-14
**Command**: `/gen-all-lessons`
**Repository**: https://github.com/shanemmattner/esp32-c6-agentic-firmware
**Branch**: `main`
**Tag**: `pre-refactor-20251114-162635` (snapshot before generation)
