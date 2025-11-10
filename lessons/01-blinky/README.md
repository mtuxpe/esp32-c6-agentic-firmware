# Lesson 01: Blinky
## Basic GPIO Output & Input

**Duration:** 20 minutes
**Goal:** Understand GPIO control and see logs on serial output

---

## 📋 Prerequisites

### Hardware
- ESP32-C6 development board
- USB-C cable
- Optional: LED + resistor connected to GPIO13

### Software
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add RISC-V target
rustup target add riscv32imac-unknown-none-elf

# Install flashing tool
cargo install espflash
```

---

## 🚀 Creating Your First Embedded Rust Project

### Step 1: Create a New Project

```bash
# Create a new binary project
cargo new --name blinky blinky

cd blinky
```

### Step 2: Update `Cargo.toml`

Replace the entire `Cargo.toml` with:

```toml
[package]
name = "blinky"
version = "0.1.0"
edition = "2021"
resolver = "2"

[dependencies]
esp-hal = { version = "1.0.0", features = ["esp32c6"] }
esp-backtrace = { version = "0.14", features = ["esp32c6", "panic-handler", "println"] }
esp-println = { version = "0.13", features = ["esp32c6"] }
log = { version = "0.4" }

[[example]]
name = "blinky"
path = "src/main.rs"

[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
```

### Step 3: Create `rust-toolchain.toml`

In the project root, create `rust-toolchain.toml`:

```toml
[toolchain]
channel = "stable"
components = ["rustfmt", "clippy"]
targets = ["riscv32imac-unknown-none-elf"]
```

### Step 4: Write the Code

Replace `src/main.rs` with the code in this lesson.

### Step 5: Build

```bash
cargo build --release
```

### Step 6: Flash to ESP32-C6

```bash
cargo run --release
```

Or manually with espflash:

```bash
espflash flash --port /dev/cu.usbserial-10 target/riscv32imac-unknown-none-elf/release/blinky
```

### Step 7: Monitor Serial Output

```bash
python3 ../../scripts/monitor.py --port /dev/cu.usbserial-10
```

**Expected output:**
```
✓ Connected to /dev/cu.usbserial-10 at 115200 baud
======================================================================
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
(Check GPIO9 input state as GPIO13 toggles)

🔴 LED ON  → GPIO9: HIGH
⚫ LED OFF → GPIO9: LOW
🔴 LED ON  → GPIO9: HIGH
⚫ LED OFF → GPIO9: LOW
  └─ 10 cycles completed
...
```

---

## 💡 Code Explanation

### Pin Configuration (Top of File)

```rust
const LED_PIN: u8 = 13;        // GPIO13 - LED output
const INPUT_PIN: u8 = 9;       // GPIO9 - Input (detects LED state)
const BLINK_DELAY_MS: u32 = 500;
```

**Why constants?**
- Easy to change pins in one place
- Clear what each pin does
- Good programming practice

### GPIO Output

```rust
let mut led = Output::new(
    peripherals.GPIO13,
    Level::Low,
    OutputConfig::default(),
);
```

- `GPIO13` - The pin to control
- `Level::Low` - Start with LED off
- `OutputConfig::default()` - Standard mode

### GPIO Input

```rust
let input = Input::new(peripherals.GPIO9, InputConfig::default());
```

- Reads the state without needing an external button
- Perfect for learning - no hardware required!
- Can detect changes made by GPIO13 output

### Reading GPIO State

```rust
if input.is_high() { "HIGH" } else { "LOW" }
```

Simple way to read a digital input pin.

---

## 🔬 What This Demonstrates

1. **GPIO Output** - Controlling a pin (blinking)
2. **GPIO Input** - Reading a pin state
3. **Logging** - Sending debug messages to serial
4. **Timing** - Using delays for synchronization
5. **State Detection** - Input reading the output's state

**No external hardware needed!** GPIO9 reads GPIO13's state automatically.

---

## ✅ Expected Behavior

- ✅ Logs appear on serial monitor
- ✅ GPIO13 switches HIGH/LOW every 500ms
- ✅ GPIO9 correctly reflects GPIO13 state
- ✅ Cycle counter increments every 10 cycles
- ✅ No compile errors or panics

---

## 🧪 Try This

### Easy
1. Change `BLINK_DELAY_MS` to `250` for faster blinking
2. Add a log message inside the loop showing the cycle number

### Medium
3. Make GPIO13 blink 5 times, then stay off for 2 seconds
4. Create a pattern (like SOS in morse code)

### Advanced
5. Read from GPIO9 and do something different based on state
6. Add a third pin and have multiple outputs

---

## 🐛 Troubleshooting

| Problem | Solution |
|---------|----------|
| Build fails | Run `rustup target add riscv32imac-unknown-none-elf` |
| Can't flash | Check port: `ls /dev/cu.* \| grep serial` |
| No serial output | Try `python3 scripts/monitor.py --port /dev/cu.usbserial-10` |
| Port in use | Check: `lsof /dev/cu.usbserial-10` and kill the process |
| LED doesn't blink | Verify wiring to GPIO13, check power |

---

## 📚 Next Steps

- **Lesson 02:** Button input and state changes
- **Lesson 03:** Multiple GPIO pins (traffic light)
- **Lesson 04:** Async/await with Embassy

---

## 🎯 Key Takeaways

1. GPIO output controls pins (HIGH/LOW)
2. GPIO input reads pin states
3. Logging helps you understand what's happening
4. Simple constants make code maintainable
5. You don't need external hardware to test GPIO!

---

*That's it! You've just built your first ESP32-C6 firmware from scratch.* 🚀
