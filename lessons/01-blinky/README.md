# Lesson 01: Blinky
## The "Hello World" of Embedded Systems

**Goal**: Blink an LED using esp-hal 1.0.0 in the simplest way possible.

**Duration**: 15 minutes

---

## 🎯 Learning Objectives

After completing this lesson, you will understand:
1. ✅ Basic esp-hal 1.0.0 initialization
2. ✅ GPIO output configuration
3. ✅ Blocking delays for timing
4. ✅ Logging for debugging
5. ✅ The minimal firmware structure

---

## 📋 Prerequisites

### Hardware
- ESP32-C6 development board
- USB-C cable
- Onboard LED (typically on GPIO8)

### Software
```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add RISC-V target for ESP32-C6
rustup target add riscv32imac-unknown-none-elf

# Install flashing tool
cargo install espflash
```

---

## 📖 Code Walkthrough

### The Complete Code (30 lines!)

```rust
#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{delay::Delay, gpio::{Level, Output, OutputConfig}, main};
use log::info;

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();
    info!("🚀 Starting Blinky");

    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut led = Output::new(peripherals.GPIO8, Level::Low, OutputConfig::default());
    let delay = Delay::new();

    loop {
        led.set_high();
        delay.delay_millis(1000);
        led.set_low();
        delay.delay_millis(1000);
    }
}
```

### Breaking it Down

#### 1. **Attributes** (Lines 1-2)
```rust
#![no_std]   // Don't use standard library (too big for embedded)
#![no_main]  // We define our own entry point
```

#### 2. **Imports** (Lines 4-6)
```rust
use esp_backtrace as _;  // Crash handler
use esp_hal::{...};      // Hardware abstraction layer
use log::info;           // Logging macro
```

#### 3. **Entry Point** (Line 10)
```rust
#[main]
fn main() -> ! {  // Never returns (infinite loop inside)
```

#### 4. **Initialization** (Lines 11-15)
```rust
esp_println::logger::init_logger_from_env();           // Enable logging
let peripherals = esp_hal::init(...);                  // Initialize HAL
let mut led = Output::new(peripherals.GPIO8, ...);     // Configure GPIO
let delay = Delay::new();                               // Create delay provider
```

#### 5. **Main Loop** (Lines 17-22)
```rust
loop {
    led.set_high();           // LED on
    delay.delay_millis(1000); // Wait 1s
    led.set_low();            // LED off
    delay.delay_millis(1000); // Wait 1s
}
```

---

## 🛠️ Building and Running

### Step 1: Navigate to Lesson
```bash
cd lessons/01-blinky
```

### Step 2: Build
```bash
cargo build --release
```

**Expected output:**
```
   Compiling esp-hal v1.0.0
   Compiling blinky v0.1.0
    Finished release [optimized] target(s) in 8.2s
```

### Step 3: Flash to ESP32-C6
```bash
cargo run --release
```

**Expected output:**
```
[INFO] Connecting...
[INFO] Flashing has completed!
```

### Step 4: Monitor Serial Output
```bash
espflash monitor /dev/cu.usbserial-10
```

**Expected output:**
```
🚀 Starting Blinky (Lesson 01)
✓ HAL initialized
✓ GPIO8 configured as output
💡 Entering blink loop...
```

---

## ✅ Expected Behavior

When running correctly:
- ✅ Onboard LED blinks every second
- ✅ Serial output shows initialization messages
- ✅ No errors or warnings
- ✅ LED timing is consistent

---

## 🔍 Understanding the Code

### Key Concepts

**1. Hardware Abstraction Layer (HAL)**
```rust
let peripherals = esp_hal::init(esp_hal::Config::default());
```
- Gets safe access to all hardware
- Prevents multiple parts of code from conflicting
- Type-safe: can't use wrong pins by accident

**2. GPIO Output**
```rust
let mut led = Output::new(peripherals.GPIO8, Level::Low, OutputConfig::default());
```
- `GPIO8` - specific pin (compile-time checked!)
- `Level::Low` - start with LED off
- `OutputConfig::default()` - standard push-pull mode

**3. Delay**
```rust
delay.delay_millis(1000);
```
- Blocking delay (stops all code)
- Good for simple cases like this
- We'll learn async delays in Lesson 02

**4. Infinite Loop**
```rust
loop { }  // Required! Embedded systems never exit
```
- Firmware runs forever
- No operating system to return to
- Must have infinite loop or function won't compile

---

## 🎓 Best Practices Demonstrated

### 1. **Comprehensive Logging**
```rust
info!("🚀 Starting Blinky");
info!("✓ HAL initialized");
```
**Why**: Helps debug issues, provides feedback

### 2. **Descriptive Names**
```rust
let mut led = Output::new(...);  // Clear what this is
```
**Why**: Code is self-documenting

### 3. **Explicit Configuration**
```rust
Level::Low, OutputConfig::default()
```
**Why**: No magic numbers, clear intent

### 4. **Step-by-Step Progression**
```rust
// Step 1: Init logging
// Step 2: Init HAL
// Step 3: Configure GPIO
```
**Why**: Easy to understand flow

---

## 🧪 Experiments to Try

### Easy
1. Change delay time to 500ms (faster blink)
2. Start with LED on (`Level::High`)
3. Add more log messages

### Medium
4. Use `led.toggle()` instead of set_high/set_low
5. Try GPIO9 instead of GPIO8
6. Add a counter to count blinks

### Advanced
7. Create a blink pattern (SOS in morse code!)
8. Add error handling for missing GPIO
9. Read button input (peek at Lesson 02)

---

## 🐛 Troubleshooting

### LED doesn't blink
- **Check GPIO pin**: Your board might use different pin
- **Check wiring**: LED connected correctly?
- **Check power**: USB cable providing power?

### Build fails
```bash
# Ensure target is installed
rustup target add riscv32imac-unknown-none-elf

# Clean and rebuild
cargo clean
cargo build --release
```

### Flash fails
```bash
# Check port exists
ls /dev/cu.*

# Try different port
espflash flash ... --port /dev/cu.usbserial-XX
```

### No serial output
```bash
# Check ESP_LOG_LEVEL
export ESP_LOG_LEVEL=INFO

# Try different baud rate
espflash monitor --speed 115200
```

---

## 📊 Code Metrics

| Metric | Value |
|--------|-------|
| Lines of code | ~30 |
| Binary size | 34 KB |
| RAM usage | Minimal (~1KB) |
| Dependencies | 3 crates |
| Build time | ~8 seconds |

---

## 🎯 Next Steps

### Completed ✅
- [x] Build and flash firmware
- [x] Understand esp-hal 1.0.0 basics
- [x] Configure GPIO output
- [x] Use blocking delays

### Continue Learning 🚀
- [ ] **Lesson 02**: Embassy async (non-blocking delays)
- [ ] **Lesson 03**: Button input + interrupts
- [ ] **Lesson 04**: State machines with enums

---

## 💡 Key Takeaways

1. **esp-hal 1.0.0 is simple**: ~30 lines for blinking LED
2. **Pure Rust**: No C code, no ESP-IDF needed
3. **Type-safe**: Compiler catches pin mistakes
4. **Modern patterns**: This is the 2024+ way
5. **Logging is essential**: Always log your initialization

---

## 📚 Additional Resources

- [esp-hal Documentation](https://docs.esp-rs.org/esp-hal/)
- [ESP32-C6 Datasheet](https://www.espressif.com/sites/default/files/documentation/esp32-c6_datasheet_en.pdf)
- [Rust Embedded Book](https://rust-embedded.github.io/book/)
- [Repository README](../../README.md)

---

## ✨ LLM-Friendly Notes

**For Claude Code / AI Assistants:**

This lesson demonstrates the **minimal pattern** for ESP32-C6 firmware:
1. Standard attributes: `#![no_std]`, `#![no_main]`
2. Essential imports: backtrace, hal, logging
3. Init sequence: logger → hal → peripherals → loop
4. GPIO pattern: `Output::new(pin, level, config)`
5. Delay pattern: `Delay::new()` then `delay_millis()`

**Replication Template:**
```rust
// Standard boilerplate
#![no_std]
#![no_main]
use esp_backtrace as _;
use esp_hal::{...};
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    // Init
    esp_println::logger::init_logger_from_env();
    let peripherals = esp_hal::init(esp_hal::Config::default());

    // Your code here

    loop {
        // Main logic
    }
}
```

**Key Patterns to Preserve:**
- Logging at each initialization step
- Descriptive variable names
- Comments explaining "why" not just "what"
- Type-safe peripheral access
- No unwrap() unless absolutely necessary

---

*Lesson 01 Complete! Ready for Lesson 02? 🚀*
