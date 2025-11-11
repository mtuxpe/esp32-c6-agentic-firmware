# Testing Strategy for ESP32-C6 Embedded Rust

**Date:** 2025-11-10
**Status:** Active
**Framework:** embedded-test + probe-rs

---

## Overview

This document outlines our testing strategy for ESP32-C6 firmware using esp-hal 1.0.0 in Rust.

We use a **hybrid approach**:
1. **Host-based unit tests** for pure functions (fast, no hardware)
2. **On-device integration tests** for hardware peripherals (real validation)

---

## Test Categories

### 1. Host-Based Unit Tests

**What:** Pure functions without hardware dependencies

**Examples:**
- Mathematical calculations (HSV→RGB conversion)
- Algorithm implementation (rotation angle from accelerometer)
- Data transformations
- State machine logic (without GPIO)

**How:**
```rust
// src/color.rs
#![cfg_attr(not(test), no_std)]

pub fn hsv_to_rgb(h: u16, s: u8, v: u8) -> (u8, u8, u8) {
    // Pure function implementation
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

**Run:** `cargo test --lib`

**Benefits:**
- ⚡ Fast execution (milliseconds)
- 🐛 Easy debugging with standard tools
- 🤖 CI/CD friendly
- 💰 No hardware required

---

### 2. On-Device Integration Tests

**What:** Hardware-dependent functionality

**Examples:**
- GPIO input/output timing
- I2C communication with real sensors
- RMT peripheral (NeoPixel)
- Interrupt handling
- Delay accuracy
- Multi-peripheral coordination

**How:**
```rust
// src/bin/test_gpio.rs
#![no_std]
#![no_main]

use esp_hal::gpio::{Input, Output, Level};

struct Context {
    gpio: esp_hal::gpio::GpioPin<9>,
}

#[embedded_test::tests(default_timeout = 3)]
mod tests {
    use super::*;

    #[init]
    fn init() -> Context {
        let peripherals = esp_hal::init(esp_hal::Config::default());
        Context { gpio: peripherals.GPIO9 }
    }

    #[test]
    fn test_gpio_toggle(ctx: Context) {
        let mut pin = Output::new(ctx.gpio, Level::Low);
        pin.set_high();
        assert!(pin.is_set_high());
    }
}
```

**Run:** `cargo run --bin test_gpio`

**Benefits:**
- ✅ Validates real hardware behavior
- ⏱️ Catches timing issues
- 🔌 Tests actual peripheral interactions
- 🐛 Finds integration bugs

---

## Project Structure

```
lessons/{NN}-{name}/
├── src/
│   ├── bin/
│   │   ├── main.rs              # Main firmware
│   │   ├── test_gpio.rs         # GPIO integration test
│   │   ├── test_i2c.rs          # I2C integration test
│   │   └── test_neopixel.rs     # NeoPixel integration test
│   ├── lib.rs                   # Library with pure logic
│   ├── color.rs                 # Pure functions + unit tests
│   └── hardware.rs              # Hardware-dependent code
├── .cargo/
│   └── config.toml              # probe-rs runner config
├── Cargo.toml                   # Test targets + dependencies
├── build.rs                     # embedded-test linker script
└── README.md
```

---

## Configuration Files

### Cargo.toml

```toml
[dependencies]
embedded-test = "0.7.0"
esp-hal = { version = "1.0.0", features = ["esp32c6", "unstable"] }

# Binary test targets
[[bin]]
name = "test_gpio"
path = "src/bin/test_gpio.rs"
harness = false

[[bin]]
name = "test_i2c"
path = "src/bin/test_i2c.rs"
harness = false
```

### .cargo/config.toml

```toml
[target.riscv32imac-unknown-none-elf]
runner = "probe-rs run --chip esp32c6"

[build]
target = "riscv32imac-unknown-none-elf"
```

### build.rs

```rust
fn main() {
    println!("cargo::rustc-link-arg=-Tembedded-test.x");
}
```

---

## Test Workflow

### 1. Development Cycle

```
Write code → Add unit tests → cargo test --lib (fast!)
    ↓
Add integration test → cargo run --bin test_X (hardware)
    ↓
Both pass → Commit
```

### 2. Running Tests

```bash
# Host-based unit tests (no hardware)
cargo test --lib

# On-device integration tests (requires ESP32-C6)
cargo run --bin test_gpio
cargo run --bin test_i2c
cargo run --bin test_neopixel

# Run all integration tests
for test in test_gpio test_i2c test_neopixel; do
    cargo run --bin $test
done
```

### 3. CI/CD Integration

```yaml
# .github/workflows/test.yml
jobs:
  unit-tests:
    runs-on: ubuntu-latest
    steps:
      - run: cargo test --lib

  integration-tests:
    runs-on: self-hosted  # With ESP32-C6 hardware
    steps:
      - run: cargo run --bin test_gpio
      - run: cargo run --bin test_i2c
```

---

## Decision Tree: Which Test Type?

```
Is the function pure (no hardware)?
│
├─ YES → Host-based unit test (#[cfg(test)])
│         Examples: hsv_to_rgb(), calculate_angle()
│
└─ NO → On-device integration test (embedded-test)
          Examples: GPIO read/write, I2C communication
```

---

## What to Test

### ✅ Always Test

- Public API functions
- Pure mathematical calculations
- Hardware initialization sequences
- Error handling paths
- Edge cases and boundary conditions

### ⚠️ Consider Testing

- Internal helper functions
- State machine transitions
- Timing-sensitive operations
- Multi-peripheral coordination

### ❌ Don't Need to Test

- Simple getters/setters
- Trivial wrappers
- Framework/library code (already tested)

---

## embedded-test Features

### Test Attributes

```rust
#[test]                    // Standard test
#[should_panic]            // Expects panic
#[ignore]                  // Skip test
#[timeout(seconds)]        // Custom timeout (default 60s)
```

### Test Configuration

```rust
#[embedded_test::tests(
    default_timeout = 3,              // Seconds
    executor = hil_test::Executor::new()  // Optional async executor
)]
```

### Initialization Hook

```rust
#[init]
fn init() -> Context {
    // Setup code runs before each test
    let peripherals = esp_hal::init(esp_hal::Config::default());
    Context { /* ... */ }
}
```

---

## Best Practices

### 1. Separate Pure Logic from Hardware

```rust
// ✅ Good: Testable pure function
pub fn calculate_color(angle: u32) -> (u8, u8, u8) {
    // No hardware dependencies
}

// ❌ Bad: Mixed concerns
pub fn calculate_and_display_color(i2c: &mut I2c, led: &mut NeoPixel) {
    // Hard to test
}
```

### 2. Use Context Structs

```rust
struct Context {
    gpio: GpioPin<9>,
    i2c: I2c<'static, Blocking>,
    delay: Delay,
}

#[init]
fn init() -> Context {
    // Initialize once, use in all tests
}
```

### 3. Descriptive Test Names

```rust
#[test]
fn test_button_debounce_ignores_rapid_presses() { }

#[test]
fn test_i2c_returns_error_for_invalid_address() { }
```

### 4. Use Assertions with Messages

```rust
assert_eq!(
    data[0], EXPECTED_WHO_AM_I,
    "WHO_AM_I should be 0x71 for MPU9250, got 0x{:02x}",
    data[0]
);
```

---

## Troubleshooting

| Issue | Solution |
|-------|----------|
| `duplicate lang item in crate 'std'` | Add `#![cfg_attr(not(test), no_std)]` to library |
| `probe-rs: Device not found` | Check USB cable, verify `probe-rs info` detects device |
| Test timeout | Increase timeout: `#[timeout(10)]` |
| Test fails on hardware but not host | Hardware issue or timing dependency |
| Linker error with embedded-test | Ensure `build.rs` includes linker script |

---

## Tools Setup

### Install probe-rs

```bash
cargo install probe-rs-tools --locked
```

### Verify Detection

```bash
probe-rs info
# Output should show:
# Probes found (1):
#   ESP32-C6 (USB JTAG)
```

### Check Connection

```bash
probe-rs list
# Should list ESP32-C6
```

---

## Example Test Output

### Host Tests

```
$ cargo test --lib

running 8 tests
test color::tests::test_pure_red ... ok
test color::tests::test_pure_green ... ok
test color::tests::test_pure_blue ... ok
test rotation::tests::test_quadrant_1 ... ok

test result: ok. 8 passed; 0 failed
Finished in 0.03s
```

### Device Tests

```
$ cargo run --bin test_gpio

Flashing device... ✓
Running tests on ESP32-C6...

✓ test_gpio_output_levels (12ms)
✓ test_gpio_input_with_pullup (3ms)
✓ test_gpio_toggle_timing (18ms)

========================================
Test Summary: 3 passed, 0 failed
========================================
```

---

## Code Coverage

While traditional coverage tools don't work well for `no_std` embedded:

**Strategies:**
- Manual tracking of tested functions
- Integration test coverage via hardware validation
- Use `#[cfg(test)]` coverage in pure functions

**Future:** Explore `cargo-llvm-cov` for host tests

---

## References

- [embedded-test GitHub](https://github.com/probe-rs/embedded-test)
- [esp-hal HIL tests](https://github.com/esp-rs/esp-hal/tree/main/hil-test)
- [probe-rs Documentation](https://probe.rs/)
- [Rust Embedded Book: Testing](https://docs.rust-embedded.org/book/start/qemu.html)
- [Testing no_std code](https://blog.dbrgn.ch/2019/12/24/testing-for-no-std-compatibility/)

---

*Confidence through testing - embedded systems done right!* ✅
