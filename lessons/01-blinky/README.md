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
- `.cargo/config.toml` with espflash runner
- `build.rs` for linker configuration and helpful error messages
- `rust-toolchain.toml` with correct Rust version and target
- `Cargo.toml` with base dependencies
- Skeleton code in `src/bin/main.rs` and `src/lib.rs`

**Note**: `esp-generate` creates a more complex structure with `src/bin/main.rs` and a `[[bin]]` section in `Cargo.toml`. This is useful for projects with multiple binaries, but for learning we'll simplify to the standard `src/main.rs` structure.

### Step 2: Simplify Project Structure

```bash
# Move main.rs to standard location
mv src/bin/main.rs src/main.rs
rmdir src/bin
rm src/lib.rs
```

### Step 3: Update Cargo.toml

Edit `Cargo.toml` and make these changes:

**a) Remove the `[[bin]]` section** (lines that look like this):
```toml
[[bin]]
name = "lesson-01-blinky"
path = "./src/bin/main.rs"
```

**b) Change edition** from `"2024"` to `"2021"`:
```toml
edition = "2021"  # was "2024"
```

**c) Add logging dependencies**. Your `[dependencies]` section should look like:

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

**What we added**:
- `esp-backtrace` - Better panic messages with stack traces
- `esp-println` - Serial output with `log` crate integration
- `log` - Standard Rust logging (`info!()`, `debug!()`, `warn!()` macros)
- `"unstable"` feature to `esp-hal` - Enables latest drivers

**What was already there**:
- `esp-hal` - Hardware abstraction layer for ESP32-C6
- `esp-bootloader-esp-idf` - Required by ESP bootloader
- `critical-section` - Thread-safe critical sections

### Step 4: Update .cargo/config.toml

Edit `.cargo/config.toml`:

**a) Remove `--monitor` flag** from the runner (line 2):
```toml
# Change from:
runner = "espflash flash --monitor --chip esp32c6"

# To:
runner = "espflash flash --chip esp32c6"
```

This separates flashing from monitoring, giving you more control.

**b) Add cargo aliases** at the end of the file:

```toml
[alias]
br = "build --release"        # br = build release (fast shortcut)
ck = "check"                  # ck = check syntax only (very fast)
ff = "run --release"          # ff = flash firmware (build + flash)
```

### Step 5: Write the Code

Replace the generated skeleton in `src/main.rs` with the Blinky code.

You can copy from this lesson's `src/main.rs` or type it out yourself (recommended for learning!).

Key changes from the generated code:
- Add GPIO output and input configuration
- Add structured logging with `info!()` macro
- Use `Delay::new()` for timing (simpler than `Instant::now()`)
- Implement blink loop with state monitoring

### Step 6: Build & Flash

```bash
# Build
cargo build --release

# Flash to ESP32-C6
cargo run --release
```

### Summary: What We Changed from esp-generate Defaults

| What | esp-generate Default | Our Simplified Version | Why? |
|------|---------------------|----------------------|------|
| Project structure | `src/bin/main.rs` + `src/lib.rs` | `src/main.rs` | Simpler, standard Rust structure |
| Cargo.toml | Has `[[bin]]` section | No `[[bin]]` section | Cargo auto-finds `src/main.rs` |
| Edition | `"2024"` | `"2021"` | More stable, widely used |
| Dependencies | Minimal (no logging) | Added logging crates | Essential for debugging |
| Runner | `espflash flash --monitor` | `espflash flash` | Separate flash from monitor |
| Aliases | None | Added `br`, `ck`, `ff` | Speed up development |

**Why simplify?** For YouTube tutorials and learning, simpler is better. You can always add complexity later when needed.

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
