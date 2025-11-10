# Lesson 01: Blinky

Basic GPIO output and input with serial logging.

## Learning Objectives

- Configure GPIO pins as output and input
- Toggle GPIO output (blink LED)
- Read GPIO input state
- Use structured logging with `log` crate
- Understand basic embedded Rust project structure

## Hardware Requirements

- ESP32-C6 development board
- USB-C cable
- Optional: LED + resistor connected to GPIO13

### Pin Configuration

```
ESP32-C6
--------
GPIO13  -->  LED (or test with GPIO9 reading the state)
GPIO9   -->  Input (reads GPIO13 state for testing)
```

**Note**: No external hardware needed! GPIO9 can read GPIO13's state, allowing you to test without any wiring.

## What You'll Learn

This lesson demonstrates:
- Creating an ESP32-C6 project with `esp-generate`
- GPIO output control (HIGH/LOW)
- GPIO input reading
- Structured logging with `info!()` macro
- Basic timing with `Delay`
- State detection between pins

## Prerequisites

### Software Installation

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Add RISC-V target for ESP32-C6
rustup target add riscv32imac-unknown-none-elf

# Install esp-generate (project template generator)
cargo install esp-generate --locked

# Install espflash (flashing tool)
cargo install espflash --locked
```

## Creating Your First ESP32-C6 Project

### Step 1: Generate Project with esp-generate

```bash
# Generate a new ESP32-C6 project
esp-generate --chip esp32c6 lesson-01-blinky

cd lesson-01-blinky
```

This creates a properly configured project with:
- Correct `.cargo/config.toml` with espflash runner
- `build.rs` for linker configuration
- `rust-toolchain.toml` with correct Rust version
- `Cargo.toml` with base dependencies
- Skeleton `src/main.rs`

### Step 2: Update Cargo.toml

Replace the `[dependencies]` section with:

```toml
[dependencies]
# Hardware abstraction layer
esp-hal = { version = "1.0.0", features = ["esp32c6", "unstable"] }

# Panic handler with backtrace
esp-backtrace = { version = "0.15", features = ["esp32c6", "panic-handler", "println"] }

# Serial printing and logging
esp-println = { version = "0.13", features = ["esp32c6", "log"] }
log = "0.4"

# Bootloader app descriptor
esp-bootloader-esp-idf = { version = "0.4.0", features = ["esp32c6"] }

# Critical sections
critical-section = "1.2.0"
```

**Key dependencies explained**:
- `esp-hal` - Hardware abstraction layer with `unstable` for latest features
- `esp-backtrace` - Panic handler with println support for debugging
- `esp-println` - Serial printing with `log` integration
- `log` - Standard Rust logging framework (info!, debug!, warn! macros)
- `esp-bootloader-esp-idf` - Required by ESP bootloader
- `critical-section` - Safe concurrent access to shared resources

### Step 3: Add Cargo Aliases (Optional but Recommended)

Add to `.cargo/config.toml`:

```toml
[alias]
br = "build --release"        # br = build release (fast shortcut)
ck = "check"                  # ck = check syntax only (very fast)
ff = "run --release"          # ff = flash firmware (build + flash)
```

### Step 4: Write the Code

Replace `src/main.rs` with the Blinky code (see this lesson's `src/main.rs`).

### Step 5: Build & Flash

```bash
# Build
cargo build --release

# Flash to ESP32-C6
cargo run --release
```

### Using Cargo Aliases (Faster)

```bash
cargo br   # build release
cargo ck   # check syntax only
cargo ff   # flash firmware (build + flash + monitor)
```

## Expected Output

When you flash and run this lesson, you should see:

```
🚀 Starting Lesson 01: Blinky
✓ GPIO13 configured as output
✓ GPIO9 configured as input
Starting GPIO demonstration...

--- GPIO Output Test ---
Set GPIO13 HIGH
  GPIO9 reads: HIGH
Set GPIO13 LOW
  GPIO9 reads: LOW

--- Blinking Loop ---

🔴 LED ON  → GPIO9: HIGH
⚫ LED OFF → GPIO9: LOW
🔴 LED ON  → GPIO9: HIGH
⚫ LED OFF → GPIO9: LOW
  └─ 10 cycles completed
```

## Code Structure

- `src/main.rs` - Main firmware implementation
- `Cargo.toml` - Project dependencies
- `.cargo/config.toml` - Build configuration with espflash runner
- `rust-toolchain.toml` - Rust toolchain specification
- `build.rs` - Build script for linker configuration

## Key Concepts

### GPIO Output

```rust
let mut led = Output::new(
    peripherals.GPIO13,
    Level::Low,           // Start with LED off
    OutputConfig::default(),
);
```

Control a pin's state (HIGH or LOW) to drive LEDs, relays, or other digital outputs.

### GPIO Input

```rust
let input = Input::new(peripherals.GPIO9, InputConfig::default());
let state = input.is_high();  // Returns true if HIGH, false if LOW
```

Read the state of a pin without needing external buttons or sensors.

### Structured Logging

```rust
info!("🚀 Starting Lesson 01: Blinky");     // Major milestones
info!("✓ GPIO{} configured as output", LED_PIN);  // Configuration steps
```

Using the `log` crate provides consistent, filterable logging across your firmware.

### Delays

```rust
delay.delay_millis(500);  // Wait 500 milliseconds
```

Simple blocking delays using CPU cycle counter. Good for basic timing, but blocks execution.

## Experiments

### Easy
1. Change `BLINK_DELAY_MS` to `250` for faster blinking
2. Add a counter to show how many blinks have occurred

### Medium
3. Blink 5 times, then pause for 2 seconds
4. Create an SOS pattern (morse code: · · · − − − · · ·)

### Advanced
5. Read GPIO9 state and change blink speed based on it
6. Add a third GPIO pin with different blink pattern

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Build fails | Ensure you're in `lessons/01-blinky/` directory |
| Can't find device | Check USB connection: `ls /dev/cu.*` or `ls /dev/ttyUSB*` |
| No serial output | Serial port may be different, check connection |
| LED doesn't blink | Verify GPIO13 wiring (or check GPIO9 reads state changes) |
| Permission denied | On Linux: `sudo usermod -a -G dialout $USER` (then logout/login) |

## Next Steps

- **Lesson 02**: Simple task scheduler - Run multiple tasks at different rates
- Experiment with different GPIO pins
- Try connecting an external LED to GPIO13

## References

- [esp-hal GPIO Module](https://docs.esp-rs.org/esp-hal/esp-hal/0.20.1/esp32c6/esp_hal/gpio/index.html)
- [ESP32-C6 Technical Reference](https://www.espressif.com/sites/default/files/documentation/esp32-c6_technical_reference_manual_en.pdf)
- [Rust Embedded Book](https://rust-embedded.github.io/book/)

---

*Your first ESP32-C6 embedded Rust firmware!* 🚀
