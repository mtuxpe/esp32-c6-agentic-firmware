# PRD: Lesson 05 - Unit and Integration Testing for ESP32-C6

**Date:** 2025-11-10
**Status:** Planning
**Target:** ESP32-C6 (esp-hal 1.0.0 beta)

---

## Overview

This lesson demonstrates testing strategies for embedded Rust on ESP32-C6, combining:
- **Host-based unit tests** for pure functions (no hardware)
- **On-device integration tests** using embedded-test + probe-rs

We'll test code from previous lessons to demonstrate both testing approaches.

---

## Learning Objectives

- Understand difference between unit tests (host) and integration tests (device)
- Set up embedded-test framework with probe-rs runner
- Write host-based unit tests for pure functions (HSV conversion, rotation calculation)
- Write on-device integration tests for hardware peripherals (GPIO, I2C)
- Use `#[cfg(test)]` to conditionally enable std for testing
- Understand test organization: separate testable logic from hardware code

---

## Hardware Requirements

- ESP32-C6 development board
- USB-C cable (built-in USB-JTAG for probe-rs)
- No additional hardware needed (tests use onboard peripherals)

---

## Testing Strategy

### Approach 1: Host-Based Unit Tests (Pure Functions)

**What to test:**
- HSV to RGB color conversion (`color.rs` from Lesson 04)
- Rotation angle calculation (`state_machine.rs` from Lesson 04)
- State machine logic (without hardware)

**How it works:**
- Tests run on development machine (x86_64/aarch64)
- Use `#[cfg(test)]` to conditionally enable std
- Standard `cargo test` workflow
- Fast, no hardware required

**Example structure:**
```rust
// src/color.rs
#![cfg_attr(not(test), no_std)]

pub fn hsv_to_rgb(h: u16, s: u8, v: u8) -> (u8, u8, u8) {
    // Pure function - no hardware dependencies
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_red() {
        assert_eq!(hsv_to_rgb(0, 100, 100), (255, 0, 0));
    }
}
```

### Approach 2: On-Device Integration Tests (Hardware)

**What to test:**
- GPIO input/output (digital read/write)
- I2C communication (MPU9250 WHO_AM_I)
- NeoPixel LED control (RMT peripheral)
- Button debouncing with real timing

**How it works:**
- Tests run on ESP32-C6 via probe-rs
- Uses embedded-test framework
- Binary targets in `src/bin/` (like esp-hal hil-test)
- Flashed and executed on device
- Results reported back to host via probe-rs

**Example structure:**
```rust
// src/bin/test_gpio.rs
#![no_std]
#![no_main]

#[embedded_test::tests(default_timeout = 3)]
mod tests {
    #[init]
    fn init() -> Context {
        let peripherals = esp_hal::init(esp_hal::Config::default());
        Context { gpio: peripherals.GPIO9 }
    }

    #[test]
    fn test_gpio_output(ctx: Context) {
        let mut pin = Output::new(ctx.gpio, Level::Low);
        assert_eq!(pin.is_set_low(), true);
        pin.set_high();
        assert_eq!(pin.is_set_high(), true);
    }
}
```

---

## Project Structure

```
lessons/05-unit-and-integration-testing/
├── src/
│   ├── bin/
│   │   ├── test_gpio.rs          # GPIO integration test
│   │   ├── test_i2c.rs           # I2C with MPU9250
│   │   └── test_neopixel.rs      # RMT/LED test
│   ├── lib.rs                    # Main library
│   ├── color.rs                  # Pure functions with unit tests
│   └── rotation.rs               # Pure functions with unit tests
├── tests/
│   └── host_tests.rs             # Host-based unit tests (if needed)
├── .cargo/
│   └── config.toml               # probe-rs runner config
├── Cargo.toml                    # Dependencies
├── build.rs                      # Build script for embedded-test
└── README.md                     # Documentation
```

---

## Cargo.toml Configuration

```toml
[package]
name = "lesson-05-testing"
version = "0.1.0"
edition = "2021"

[dependencies]
esp-hal = { version = "1.0.0", features = ["esp32c6", "unstable"] }
esp-backtrace = { version = "0.15", features = ["esp32c6", "panic-handler", "println"] }
esp-println = { version = "0.13", features = ["esp32c6", "log"] }
log = "0.4"
critical-section = "1.2.0"

# Testing
embedded-test = "0.7.0"

# Optional: Enable std for host-based tests
[dev-dependencies]
# No special deps needed for host tests

# Binary test targets (on-device)
[[bin]]
name = "test_gpio"
path = "src/bin/test_gpio.rs"
harness = false

[[bin]]
name = "test_i2c"
path = "src/bin/test_i2c.rs"
harness = false

[[bin]]
name = "test_neopixel"
path = "src/bin/test_neopixel.rs"
harness = false

[profile.dev]
opt-level = "s"

[profile.release]
codegen-units = 1
debug = 2
opt-level = 's'
```

---

## .cargo/config.toml

```toml
[target.riscv32imac-unknown-none-elf]
runner = "probe-rs run --chip esp32c6"

[build]
target = "riscv32imac-unknown-none-elf"

[env]
ESP_LOG = "info"
```

---

## build.rs

```rust
fn main() {
    println!("cargo::rustc-link-arg=-Tembedded-test.x");
}
```

---

## Test Examples

### Host-Based Unit Test (color.rs)

```rust
#![cfg_attr(not(test), no_std)]

pub struct HsvColor {
    pub hue: u16,        // 0-360
    pub saturation: u8,  // 0-100
    pub value: u8,       // 0-100
}

pub fn hsv_to_rgb(hsv: HsvColor) -> (u8, u8, u8) {
    // Pure function - integer-only conversion
    // (Implementation from Lesson 04)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pure_red() {
        let hsv = HsvColor::new(0, 100, 100);
        assert_eq!(hsv_to_rgb(hsv), (255, 0, 0));
    }

    #[test]
    fn test_pure_green() {
        let hsv = HsvColor::new(120, 100, 100);
        assert_eq!(hsv_to_rgb(hsv), (0, 255, 0));
    }

    #[test]
    fn test_grayscale() {
        let hsv = HsvColor::new(0, 0, 50);
        assert_eq!(hsv_to_rgb(hsv), (127, 127, 127));
    }
}
```

Run with: `cargo test --lib`

### On-Device Integration Test (test_gpio.rs)

```rust
#![no_std]
#![no_main]

use esp_hal::{
    gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull},
    delay::Delay,
};

struct Context {
    gpio1: esp_hal::gpio::GpioPin<9>,
    gpio2: esp_hal::gpio::GpioPin<8>,
    delay: Delay,
}

#[embedded_test::tests(default_timeout = 3)]
mod tests {
    use super::*;

    #[init]
    fn init() -> Context {
        let peripherals = esp_hal::init(esp_hal::Config::default());
        Context {
            gpio1: peripherals.GPIO9,
            gpio2: peripherals.GPIO8,
            delay: Delay::new(),
        }
    }

    #[test]
    fn test_gpio_output_levels(ctx: Context) {
        let mut pin = Output::new(ctx.gpio1, Level::Low, OutputConfig::default());

        assert!(pin.is_set_low(), "Pin should start low");

        pin.set_high();
        assert!(pin.is_set_high(), "Pin should be high after set_high()");

        pin.set_low();
        assert!(pin.is_set_low(), "Pin should be low after set_low()");
    }

    #[test]
    fn test_gpio_input_with_pullup(ctx: Context) {
        let pin = Input::new(
            ctx.gpio1,
            InputConfig::default().with_pull(Pull::Up)
        );

        // With pull-up, pin should read high when floating
        assert!(pin.is_high(), "Pin with pull-up should read high");
    }

    #[test]
    fn test_gpio_toggle_timing(mut ctx: Context) {
        let mut pin = Output::new(ctx.gpio1, Level::Low, OutputConfig::default());

        for _ in 0..10 {
            pin.toggle();
            ctx.delay.delay_millis(1);
        }

        // After 10 toggles, should be high (started low)
        assert!(pin.is_set_high());
    }
}
```

Run with: `cargo run --bin test_gpio`

### On-Device I2C Test (test_i2c.rs)

```rust
#![no_std]
#![no_main]

use esp_hal::{
    i2c::master::{Config, I2c},
};

struct Context {
    i2c: I2c<'static, esp_hal::Blocking>,
}

const MPU9250_ADDR: u8 = 0x68;
const WHO_AM_I_REG: u8 = 0x75;
const EXPECTED_WHO_AM_I: u8 = 0x71;

#[embedded_test::tests(default_timeout = 3)]
mod tests {
    use super::*;

    #[init]
    fn init() -> Context {
        let peripherals = esp_hal::init(esp_hal::Config::default());

        let i2c = I2c::new(peripherals.I2C0, Config::default())
            .unwrap()
            .with_sda(peripherals.GPIO2)
            .with_scl(peripherals.GPIO11);

        Context { i2c }
    }

    #[test]
    fn test_mpu9250_who_am_i(mut ctx: Context) {
        let mut data = [0u8; 1];

        // Write WHO_AM_I register address, then read 1 byte
        ctx.i2c
            .write_read(MPU9250_ADDR, &[WHO_AM_I_REG], &mut data)
            .expect("I2C communication failed");

        assert_eq!(
            data[0], EXPECTED_WHO_AM_I,
            "WHO_AM_I should be 0x71 for MPU9250"
        );
    }

    #[test]
    fn test_i2c_invalid_address_fails(mut ctx: Context) {
        let mut data = [0u8; 1];

        // Non-existent device should return error
        let result = ctx.i2c.write_read(0x6B, &[0x00], &mut data);

        assert!(result.is_err(), "Invalid address should return error");
    }
}
```

Run with: `cargo run --bin test_i2c`

---

## Expected Output

### Host-Based Tests

```bash
$ cargo test --lib

running 8 tests
test color::tests::test_pure_red ... ok
test color::tests::test_pure_green ... ok
test color::tests::test_pure_blue ... ok
test color::tests::test_grayscale ... ok
test rotation::tests::test_quadrant_1 ... ok
test rotation::tests::test_quadrant_2 ... ok
test rotation::tests::test_quadrant_3 ... ok
test rotation::tests::test_quadrant_4 ... ok

test result: ok. 8 passed; 0 failed; 0 ignored
```

### On-Device Integration Tests

```bash
$ cargo run --bin test_gpio

Running test_gpio::tests::test_gpio_output_levels...
✓ test_gpio_output_levels (12ms)

Running test_gpio::tests::test_gpio_input_with_pullup...
✓ test_gpio_input_with_pullup (3ms)

Running test_gpio::tests::test_gpio_toggle_timing...
✓ test_gpio_toggle_timing (18ms)

========================================
Test Summary: 3 passed, 0 failed
========================================
```

```bash
$ cargo run --bin test_i2c

Running test_i2c::tests::test_mpu9250_who_am_i...
✓ test_mpu9250_who_am_i (45ms)

Running test_i2c::tests::test_i2c_invalid_address_fails...
✓ test_i2c_invalid_address_fails (8ms)

========================================
Test Summary: 2 passed, 0 failed
========================================
```

---

## Key Concepts

### 1. Pure Functions = Host Tests

Functions with **no hardware dependencies** can be tested on the host:
- Mathematical calculations (HSV→RGB, atan2 approximation)
- Data transformations
- State machine logic (without actual GPIO)
- Algorithm validation

**Benefits:**
- Fast execution (no flashing)
- Easy debugging (use regular debugger)
- CI/CD friendly

### 2. Hardware Interactions = Device Tests

Tests that require **real peripherals**:
- GPIO read/write timing
- I2C communication with real sensors
- RMT/NeoPixel control
- Interrupt handling
- Delay accuracy

**Benefits:**
- Validates actual hardware behavior
- Catches timing issues
- Tests real peripheral interactions

### 3. Test Organization Pattern

**Separate concerns:**
```
src/
├── lib.rs           # Hardware-agnostic logic
├── color.rs         # Pure functions with #[cfg(test)] tests
├── rotation.rs      # Pure functions with #[cfg(test)] tests
└── bin/
    ├── test_gpio.rs    # Hardware integration test
    └── test_i2c.rs     # Hardware integration test
```

This pattern allows:
- Pure functions in library crates can be unit tested
- Hardware tests in separate binaries
- Easy reuse of testable logic

---

## Setup Requirements

### 1. Install probe-rs

```bash
cargo install probe-rs-tools --locked
```

### 2. Verify ESP32-C6 Detection

```bash
probe-rs info
# Should detect ESP32-C6 via USB-JTAG
```

### 3. Run Tests

```bash
# Host-based unit tests (fast)
cargo test --lib

# On-device integration tests (requires hardware)
cargo run --bin test_gpio
cargo run --bin test_i2c
cargo run --bin test_neopixel
```

---

## Troubleshooting

| Issue | Cause | Solution |
|-------|-------|----------|
| `probe-rs: Device not found` | USB-JTAG not detected | Check USB-C cable, try different port |
| `duplicate lang item in crate 'std'` | Missing `#[cfg(test)]` on std usage | Add `#![cfg_attr(not(test), no_std)]` |
| Test timeout | Test takes too long | Increase `#[embedded_test::tests(default_timeout = 10)]` |
| I2C test fails | MPU9250 not connected | Check I2C wiring (GPIO2/11), verify sensor |
| Compilation error in tests | Wrong target | Ensure `.cargo/config.toml` sets correct target |

---

## Testing Workflow

### Development Cycle

1. **Write pure function** in `src/color.rs`
2. **Add unit tests** with `#[cfg(test)]`
3. **Test on host**: `cargo test --lib` (fast iteration)
4. **Write integration test** in `src/bin/test_color.rs`
5. **Test on device**: `cargo run --bin test_color`
6. **Commit** when both pass

### CI/CD Integration

```yaml
# .github/workflows/test.yml
- name: Run host tests
  run: cargo test --lib

- name: Run device tests (if runner has hardware)
  run: |
    cargo run --bin test_gpio
    cargo run --bin test_i2c
```

---

## What We'll Build

This lesson creates:
- ✅ Host-based tests for HSV conversion (from Lesson 04)
- ✅ Host-based tests for rotation calculation (from Lesson 04)
- ✅ On-device GPIO tests (from Lessons 01-04)
- ✅ On-device I2C tests (from Lesson 03-04)
- ✅ On-device NeoPixel tests (from Lesson 02-04)
- ✅ Documentation on testing strategies

**No new hardware features** - this lesson focuses on validating existing code.

---

## Next Steps

After completing this lesson:
- Add tests to all future lessons
- Explore mocking strategies for hardware
- Investigate defmt-test as alternative framework
- Set up continuous integration with hardware runners

---

## References

- [embedded-test GitHub](https://github.com/probe-rs/embedded-test)
- [esp-hal HIL tests](https://github.com/esp-rs/esp-hal/tree/main/hil-test)
- [probe-rs Documentation](https://probe.rs/)
- [Testing no_std code](https://blog.dbrgn.ch/2019/12/24/testing-for-no-std-compatibility/)
- [Ferrous Systems: Testing Embedded Apps](https://ferrous-systems.com/blog/test-embedded-app/)

---

*Test-driven embedded development - confidence through validation!* ✅
