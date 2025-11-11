# Lesson 05: Unit and Integration Testing

Testing embedded Rust code: host-based unit tests for pure functions.

## Learning Objectives

- Understand the difference between unit tests (host) and integration tests (device)
- Write host-based unit tests for pure functions
- Use `#[cfg_attr(not(test), no_std)]` to conditionally enable std for testing
- Separate testable logic from hardware-dependent code
- Run tests on your development machine without hardware

## What You'll Learn

This lesson demonstrates **host-based unit testing** for embedded code:
- Pure functions (HSV color conversion, rotation calculation) tested on host
- Fast test execution without flashing to hardware
- Standard Rust testing workflow with `cargo test`
- Architectural patterns for testable embedded code

## Project Structure

```
lessons/05-unit-and-integration-testing/
├── src/
│   ├── lib.rs             # Library entry point
│   ├── color.rs           # HSV→RGB conversion with unit tests
│   ├── rotation.rs        # Rotation calculation with unit tests
│   └── state_machine.rs   # State machine with unit tests
├── Cargo.toml             # Dependencies: statig for state machines
└── README.md
```

## Running Tests

### Host-Based Unit Tests (Fast!)

```bash
cd lessons/05-unit-and-integration-testing

# Temporarily remove device configuration
mv .cargo .cargo.device
mv rust-toolchain.toml rust-toolchain.toml.device

# Run tests on your development machine
cargo test --lib

# Restore device configuration
mv .cargo.device .cargo
mv rust-toolchain.toml.device rust-toolchain.toml
```

**Expected Output:**
```
running 28 tests
test color::tests::test_black ... ok
test color::tests::test_cyan ... ok
test rotation::tests::test_90_degrees ... ok
test rotation::tests::test_quadrant_1 ... ok
test state_machine::tests::test_initial_state ... ok
test state_machine::tests::test_toggle_on ... ok
... (22 more tests)

test result: ok. 28 passed; 0 failed; 0 ignored
```

## Code Examples

### 1. Pure Function with Host Tests (color.rs)

```rust
//! HSV to RGB color conversion - testable pure function

#![cfg_attr(not(test), no_std)]  // Use no_std for embedded, std for tests

pub fn hsv_to_rgb(hsv: HsvColor) -> (u8, u8, u8) {
    // Pure function - no hardware dependencies
    // Integer-only math, works on both host and device
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
}
```

### 2. Algorithm Testing (rotation.rs)

```rust
//! Rotation angle calculation - integer-only atan2 approximation

pub fn calculate_rotation_angle(accel_x: i16, accel_y: i16) -> u32 {
    // Pure function - maps accelerometer X/Y to 0-360°
    // No floating point, no hardware dependencies
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_quadrant_1() {
        let angle = calculate_rotation_angle(1000, 1000);
        assert!((40..=50).contains(&angle), "45° ≈ {angle}°");
    }

    #[test]
    fn test_angle_range() {
        // Test edge cases
        for x in [-16000, -8000, 0, 8000, 16000] {
            for y in [-16000, -8000, 0, 8000, 16000] {
                let angle = calculate_rotation_angle(x, y);
                assert!(angle < 360, "Angle {angle} out of range");
            }
        }
    }
}
```

### 3. State Machine Testing (state_machine.rs)

```rust
//! Simple toggle state machine using statig

use statig::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    ButtonPressed,
}

#[derive(Default)]
pub struct SimpleMachine;

#[state_machine(
    initial = "State::off()",
    state(derive(Debug, Clone, PartialEq))
)]
impl SimpleMachine {
    #[state]
    fn off(&mut self, event: &Event) -> Response<State> {
        match event {
            Event::ButtonPressed => Transition(State::on()),
        }
    }

    #[state]
    fn on(&mut self, event: &Event) -> Response<State> {
        match event {
            Event::ButtonPressed => Transition(State::off()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_state() {
        let sm = SimpleMachine::default().state_machine();
        assert_eq!(sm.state(), &State::off());
    }

    #[test]
    fn test_toggle_on() {
        let mut sm = SimpleMachine::default().state_machine();
        sm.handle(&Event::ButtonPressed);
        assert_eq!(sm.state(), &State::on());
    }

    #[test]
    fn test_toggle_off() {
        let mut sm = SimpleMachine::default().state_machine();
        sm.handle(&Event::ButtonPressed); // Off → On
        sm.handle(&Event::ButtonPressed); // On → Off
        assert_eq!(sm.state(), &State::off());
    }
}
```

## Key Concepts

### What Makes a Function "Pure" and Testable?

✅ **Pure Functions (Host Testable):**
- No hardware dependencies (GPIO, I2C, SPI, etc.)
- Deterministic (same input → same output)
- No side effects
- Examples: math calculations, data transformations, algorithms

❌ **Hardware Functions (Require Device):**
- Use GPIO, I2C, SPI, RMT peripherals
- Timing-dependent operations
- Interrupt handling
- Examples: button debouncing, sensor reading, LED control

### Testing Strategy

| Code Type | Test Method | Speed | Hardware Required |
|-----------|-------------|-------|-------------------|
| Pure functions | Host unit tests (`cargo test`) | Fast (milliseconds) | No |
| Algorithms | Host unit tests | Fast | No |
| Hardware I/O | Device testing | Slow (flash + run) | Yes |
| Integration | Device testing | Slow | Yes |

### Pattern: Separate Pure Logic from Hardware

```rust
// ✅ Good: Testable architecture
pub fn calculate_color(angle: u32) -> (u8, u8, u8) {
    // Pure function - host testable
}

pub fn read_sensor_and_update_led(i2c: &mut I2c, led: &mut Led) {
    let angle = read_angle_from_sensor(i2c);  // Hardware
    let (r, g, b) = calculate_color(angle);   // Pure (tested!)
    led.set_color(r, g, b);                   // Hardware
}

// ❌ Bad: Mixed concerns
pub fn update_led_from_sensor(i2c: &mut I2c, led: &mut Led) {
    // Everything mixed together - hard to test
}
```

## Test Coverage

This lesson includes **28 unit tests**:

**Color Module (12 tests):**
- Primary colors (red, green, blue)
- Secondary colors (cyan, magenta, yellow)
- Grayscale (black, white, gray)
- Edge cases (hue wrapping, saturation clamping)

**Rotation Module (12 tests):**
- Cardinal directions (0°, 90°, 180°, 270°)
- All four quadrants
- Edge cases (zero input, large values, full range)

**State Machine Module (4 tests):**
- Initial state verification
- State transitions (Off → On, On → Off)
- Multiple toggle cycles
- Event handling

## Benefits of Host Testing

1. **⚡ Fast Feedback**: Tests run in milliseconds, not seconds
2. **🐛 Easy Debugging**: Use standard debuggers and tools
3. **🤖 CI/CD Friendly**: Automated testing without hardware
4. **💰 Cost Effective**: No hardware required for development
5. **🔁 Rapid Iteration**: Test-driven development workflow

## Limitations

**What you CAN'T test on host:**
- Hardware timing and delays
- GPIO electrical characteristics
- I2C/SPI communication protocols
- Interrupt handling
- Real sensor data
- Multi-peripheral coordination

**Solution:** Manual hardware validation or advanced frameworks like `embedded-test` (covered in future lessons)

## Integration with Previous Lessons

This lesson extracts testable code from Lesson 04:
- `color.rs`: HSV→RGB conversion (originally in Lesson 04)
- `rotation.rs`: Angle calculation (originally in Lesson 04 state machine)

These pure functions now have comprehensive test coverage!

## Test-Driven Development (TDD) Going Forward

Starting from Lesson 06, we'll adopt **test-driven development**:

### Why Test "Obvious" Logic?

You might think: *"Why test that a toggle state machine goes Off → On? That's obvious!"*

**Three critical reasons:**

1. **Regression Prevention** 🛡️
   - You write logic today that works perfectly
   - Six months later, you refactor or add features
   - Your "obvious" test fails → **You just caught a bug before it shipped**
   - Without the test, you'd discover the bug in production

2. **TDD Forces Better Design** 🏗️
   - Writing tests first makes you think: *"How do I test this?"*
   - That question naturally leads to:
     - **Isolation** - Separate pure logic from hardware
     - **Pure functions** - No side effects, deterministic
     - **Clear interfaces** - Testable APIs are good APIs
     - **Loose coupling** - Easy to mock and test independently
   - **Result**: Better architecture without trying

3. **Living Documentation** 📖
   - Tests show how code is *supposed* to work
   - New developers read tests to understand behavior
   - Tests are always up-to-date (or they fail)

**Bottom line**: Tests aren't just about catching bugs *now*. They're about confidence when you change code *later*.

### TDD Workflow

1. **Think about tests first** - What behavior do we want?
2. **Write the test** - Define expected behavior in code
3. **Watch it fail** - Confirm the test catches missing functionality
4. **Implement** - Write minimum code to pass the test
5. **Verify** - Run tests and confirm they pass
6. **Refactor** - Improve code while keeping tests green

### When to Use TDD

✅ **Write tests first for:**
- Pure functions (color conversion, calculations, parsing)
- State machines (transitions, event handling)
- Data transformations
- Algorithms

⚠️ **Write tests after for:**
- Hardware setup (I2C init, GPIO config)
- Exploratory code (not sure what API we want yet)
- Quick prototypes

### Keep Tests Simple

- **3-5 tests per module** (not 10-20!)
- Test main use cases, not every edge case
- Code must be type-able for live videos
- Focus on readability over exhaustive coverage

## Next Steps

- **Lesson 06+**: Use TDD workflow for new features
- Practice writing tests before implementation
- Separate pure logic from hardware for easier testing

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Tests won't compile | Ensure `.cargo/` and `rust-toolchain.toml` are renamed |
| `can't find crate for std` | Check `#![cfg_attr(not(test), no_std)]` in lib.rs |
| Import errors | Modules need to be public (`pub mod color`) |
| Test failures | Check test assertions match expected values |

## References

- [The Rust Book: Writing Tests](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Rust Embedded Book: Testing](https://docs.rust-embedded.org/book/start/qemu.html)
- [Testing no_std code](https://blog.dbrgn.ch/2019/12/24/testing-for-no-std-compatibility/)

---

*Fast, reliable testing - the foundation of quality embedded code!* ✅
