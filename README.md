# ESP32-C6 Agentic Firmware Development

![ESP32-C6](https://img.shields.io/badge/ESP32--C6-Rust-orange)
![esp-hal](https://img.shields.io/badge/esp--hal-1.0.0-blue)
![License](https://img.shields.io/badge/license-MIT-green)

## Overview

Embedded Rust firmware development for ESP32-C6 using **esp-hal 1.0.0** with practical, lesson-based tutorials.

**About esp-hal 1.0.0:**
- Pure Rust (no C dependencies, no ESP-IDF)
- Bare-metal HAL with direct hardware access
- Implements embedded-hal 1.0 traits

## Key Features

✅ **Hardware-Validated** - All 5 lessons tested on ESP32-C6 hardware
✅ **Progressive CLI Architecture** - Each lesson extends a unified command-line interface
✅ **GDB-Integrated Debugging** - Real-time variable inspection + hardware debugging workflows
✅ **Claude Code Optimized** - Custom slash commands for lesson generation and automated testing
✅ **Production-Ready Build Times** - All lessons compile in <2 seconds
✅ **Comprehensive Documentation** - 2000+ lines of debugging guides, hardware test reports, and curriculum planning

## Lessons

Progressive tutorials from basic GPIO to advanced debugging with integrated CLI architecture:

- **[01-gpio-gdb-basics](./lessons/01-gpio-gdb-basics/)** - GPIO input/output + GDB fundamentals (breakpoints, variable inspection, function calls)
- **[02-uart-cli-streaming](./lessons/02-uart-cli-streaming/)** - Interactive CLI + real-time telemetry streaming (foundation for all subsequent lessons)
- **[03-pwm-neopixel](./lessons/03-pwm-neopixel/)** - PWM control + WS2812 NeoPixel drivers (extends CLI with color commands)
- **[04-mpu6050-state-machine](./lessons/04-mpu6050-state-machine/)** - I2C communication + MPU6050 IMU + state machine (extends CLI with sensor commands)
- **[05-posture-monitor](./lessons/05-posture-monitor/)** - Complete integration: tilt detection, visual alerts, automatic posture monitoring

**Status:** ✅ All 5 lessons are **hardware-tested and verified** (see [HARDWARE_TEST_REPORT.md](./HARDWARE_TEST_REPORT.md))

Each lesson builds on the previous CLI framework progressively. Start with Lesson 01 and work sequentially through the curriculum.

## Quick Start

### Prerequisites

```bash
# Install Rust and RISC-V target
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add riscv32imac-unknown-none-elf

# Install tools
cargo install espflash esp-generate --locked
```

### Build & Flash

```bash
cd lessons/01-gpio-gdb-basics
cargo build --release
cargo run --release  # Flash to ESP32-C6
```

See [QUICKSTART.md](./QUICKSTART.md) for detailed instructions.

## Rust Toolchain

This project uses Rust nightly. Each lesson includes a `rust-toolchain.toml` file that specifies the exact nightly version tested with that lesson.

If you encounter build errors:
1. Check the lesson's `rust-toolchain.toml` for the expected toolchain version
2. Run `rustup show` to verify your active toolchain
3. The toolchain will be automatically selected when you build from within a lesson directory

**Note:** esp-hal and its dependencies evolve rapidly. If a lesson fails to build with the latest nightly, the pinned version in `rust-toolchain.toml` ensures reproducible builds.

## Debugging

This project demonstrates multiple debugging approaches:

- **GDB Fundamentals** (Lesson 01) - Hardware debugging with breakpoints, watchpoints, variable inspection, and function calls
- **UART CLI + Streaming** (Lesson 02-05) - Interactive command interface + real-time telemetry streaming
- **Progressive Integration** - Each lesson extends the CLI with new peripheral commands (GPIO → UART → PWM → I2C)

## Documentation

### Getting Started
- **[QUICKSTART.md](./QUICKSTART.md)** - Quick start guide for hardware setup and first build
- **[CURRICULUM_STATUS.md](./CURRICULUM_STATUS.md)** - Complete lesson generation status and feature summary

### GDB & Debugging
- **[GDB_EXECUTIVE_SUMMARY.md](./GDB_EXECUTIVE_SUMMARY.md)** - High-level overview of GDB debugging approach
- **[GDB_LESSON_PLANS.md](./GDB_LESSON_PLANS.md)** - Detailed lesson planning and progression
- **[GDB_REFERENCE.md](./GDB_REFERENCE.md)** - GDB command reference and debugging workflows

### Development & Testing
- **[CLAUDE.md](./CLAUDE.md)** - Guidelines for Claude Code development (model selection, debugging philosophy, hardware testing)
- **[LESSON_GENERATION_GUIDE.md](./LESSON_GENERATION_GUIDE.md)** - Guide for generating new lessons
- **[HARDWARE_TEST_REPORT.md](./HARDWARE_TEST_REPORT.md)** - Comprehensive hardware validation results for all 5 lessons

### External Resources
- **[Official esp-hal Docs](https://docs.esp-rs.org/esp-hal/)** - HAL reference
- **[esp-hal Examples](https://github.com/esp-rs/esp-hal/tree/main/examples)** - Code examples

## esp-hal 1.0.0 Features

**Core Features:**
- Stable API with backward compatibility guarantees
- embedded-hal 1.0 standard traits
- Embassy async/await support
- DMA support for all peripherals
- Type-safe GPIO validation

**Example:**
```rust
#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut led = Output::new(peripherals.GPIO13, Level::Low, OutputConfig::default());
    let delay = Delay::new();

    loop {
        led.toggle();
        delay.delay_millis(500);
    }
}
```

No ESP-IDF or C dependencies required.

## Project Structure

```
lessons/
├── 01-gpio-gdb-basics/           # GPIO + GDB fundamentals
│   ├── src/bin/main.rs           # LED + button with GDB-callable functions
│   ├── Cargo.toml
│   ├── .cargo/config.toml
│   └── README.md                 # 500+ line debugging guide
├── 02-uart-cli-streaming/        # Interactive CLI + streaming telemetry
│   ├── src/bin/main.rs           # CLI parser + mode switching
│   └── README.md                 # 800+ line comprehensive guide
├── 03-pwm-neopixel/              # PWM + WS2812 NeoPixel (extends CLI)
├── 04-mpu6050-state-machine/     # I2C + MPU6050 + FSM (extends CLI)
└── 05-posture-monitor/           # Complete integration project

.claude/
├── commands/                     # Custom slash commands
│   ├── gen-lesson.md             # Generate new lesson with GDB workflow
│   ├── gen-all-lessons.md        # Generate complete curriculum
│   ├── test-all-lessons.md       # Hardware test all lessons
│   ├── test-uart-pins.md         # Test specific UART GPIO pins
│   ├── review-repo.md            # Comprehensive repo review
│   └── ...                       # 9 total slash commands
├── templates/
│   ├── uart_test_minimal.rs      # Minimal UART test firmware (73 lines)
│   ├── read_uart.py              # Time-bounded UART reader
│   ├── test-all-lessons-reliable.sh
│   └── ...                       # Testing scripts and templates
└── TEST.md.template              # Lesson test procedure template

scripts/
├── find-esp32-ports.sh           # Auto-detect ESP32 USB-JTAG and UART ports
├── test-uart-pins.sh             # Automated GPIO pin verification
├── monitor.py                    # UART monitoring with timeout
├── clean-targets.sh              # Clean all lesson build artifacts
└── ...                           # 7 total automation scripts
```

## Claude Code Integration

This repository includes extensive Claude Code automation:

### Custom Slash Commands

- `/gen-lesson <number> <name>` - Generate new lesson with complete GDB workflow
- `/gen-all-lessons` - Generate entire curriculum (used to create all 5 lessons)
- `/test-all-lessons [mode]` - Hardware test all lessons (quick/full modes)
- `/test-uart-pins <tx> <rx> [duration]` - Test specific UART GPIO configuration
- `/review-repo` - Comprehensive repository review and validation
- `/check-manufacturing` - Real-time component availability and DFM validation
- `/estimate-cost [quantity]` - BOM cost analysis with quantity breaks

### Testing Infrastructure

- **Automated hardware testing** - Probe-rs based testing without TTY requirements
- **Progressive commit workflow** - Hardware-validated commits with GDB checkpoints
- **Time-bounded serial operations** - No conversation-freezing operations
- **Port auto-detection** - Automatic ESP32 USB-JTAG and UART discovery

See [CLAUDE.md](./CLAUDE.md) for complete development guidelines.

## License

MIT OR Apache-2.0

## Acknowledgments

- [esp-rs Team](https://github.com/esp-rs) - esp-hal development
- [Espressif](https://www.espressif.com/) - ESP32-C6 hardware and tooling
- [Rust Embedded](https://github.com/rust-embedded) - embedded-hal standards
