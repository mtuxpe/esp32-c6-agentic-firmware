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

## Lessons

Progressive tutorials from basic GPIO to advanced debugging:

- **[01-button-neopixel](./lessons/01-button-neopixel/)** - GPIO input/output with WS2812 NeoPixel control via RMT peripheral
- **[02-task-scheduler](./lessons/02-task-scheduler/)** - Cooperative task scheduling and periodic execution
- **[03-mpu9250](./lessons/03-mpu9250/)** - I2C communication with MPU9250 IMU sensor
- **[04-static-color-navigator](./lessons/04-static-color-navigator/)** - State machine-based UI navigation with button input
- **[05-unit-and-integration-testing](./lessons/05-unit-and-integration-testing/)** - Testing strategies for embedded firmware
- **[06-uart-terminal](./lessons/06-uart-terminal/)** - UART communication and interactive terminal interface
- **[07-gdb-debugging](./lessons/07-gdb-debugging/)** - Hardware debugging with GDB and OpenOCD
- **[08-uart-gdb-tandem](./lessons/08-uart-gdb-tandem/)** - Real-time variable streaming + GDB tandem debugging

**Status:** Lessons 01, 07, and 08 are fully tested and documented with hardware validation.

Each lesson builds on previous concepts progressively. Start with Lesson 01 and work sequentially through the curriculum.

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
cd lessons/01-button-neopixel
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

- **GDB + OpenOCD** (Lesson 07) - Hardware debugging with breakpoints, watchpoints, and variable inspection
- **UART Variable Streaming** (Lesson 08) - Real-time variable monitoring with GDB tandem debugging

## Documentation

- **[QUICKSTART.md](./QUICKSTART.md)** - Quick start guide
- **[CLAUDE.md](./CLAUDE.md)** - Guidelines for Claude Code development
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
├── 01-button-neopixel/       # Lesson 1: GPIO + NeoPixel
│   ├── src/bin/main.rs
│   ├── Cargo.toml
│   ├── .cargo/config.toml
│   └── README.md
├── 02-task-scheduler/        # Lesson 2: Task scheduling
├── 03-mpu9250/               # Lesson 3: I2C sensor
└── ...

.claude/
├── commands/                 # Custom slash commands
├── templates/                # Code templates
└── TESTING-GUIDE.md

scripts/
├── find-esp32-ports.sh       # Auto port detection
└── ...
```

## License

MIT OR Apache-2.0

## Acknowledgments

- [esp-rs Team](https://github.com/esp-rs) - esp-hal development
- [Espressif](https://www.espressif.com/) - ESP32-C6 hardware and tooling
- [Rust Embedded](https://github.com/rust-embedded) - embedded-hal standards
