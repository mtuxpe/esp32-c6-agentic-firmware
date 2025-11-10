# Lesson 02: Debugger with probe-rs
## Hardware Debugging Essentials for ESP32-C6

**Duration:** 30 minutes
**Goal:** Learn to debug embedded firmware using probe-rs and ESP32-C6's built-in USB-JTAG

---

## 📋 Prerequisites

### Hardware
- ESP32-C6 development board
- USB-C cable (provides both power and JTAG debugging)
- Optional: LED + resistor on GPIO13 (from Lesson 01)

**Important:** GPIO 12 (D-) and GPIO 13 (D+) are reserved for USB-JTAG. Do NOT use them for general I/O in this lesson.

### Software (Same as Lesson 01)
```bash
# Install Rust and RISC-V target
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add riscv32imac-unknown-none-elf

# Install probe-rs for debugging
cargo install probe-rs --locked

# Install espflash for building
cargo install espflash --locked

# Install esp-generate for project templates
cargo install esp-generate --locked
```

### Verify probe-rs Installation
```bash
probe-rs list
# Should show: ESP32-C6 (riscv) @ [port]
```

---

## 🚀 Running This Lesson

### Step 1: Build the Project
```bash
cd lessons/02-debugger
cargo build --release
```

### Step 2: Flash to ESP32-C6
```bash
cargo run --release
# Or use cargo alias:
cargo ff
```

You should see log output:
```
🚀 Starting Lesson 02: Debugger with probe-rs
📍 Set breakpoints below to inspect GPIO state
✓ GPIO13 configured as output
✓ GPIO9 configured as input
...
```

### Step 3: Set a Breakpoint and Debug

**Option A: Using CLI (probe-rs)**

```bash
# Build with debug info
cargo build --release

# Connect debugger (press Enter to continue after connecting)
probe-rs run target/riscv32imac-unknown-none-elf/release/lesson-02-debugger
```

When paused at a breakpoint:
```
monitor read 0x600a4008  # Read GPIO_OUT register
monitor read 0x600a401c  # Read GPIO_IN register
monitor print cycle      # Watch the cycle variable
continue                 # Resume execution
```

**Option B: Using VSCode + probe-rs Extension**

1. Install probe-rs extension for VSCode
2. Set breakpoint by clicking line number (left margin)
3. Press F5 to start debugging
4. Use Debug console to read registers:
   ```
   monitor read 0x600a4008
   ```

---

## 💡 What You'll Learn

### Breakpoints
Set breakpoints at specific lines to pause execution and inspect state:

```rust
loop {
    led.set_high();  // 📍 BREAKPOINT #1
    info!("🔴 LED ON");
    delay.delay_millis(500);

    led.set_low();   // 📍 BREAKPOINT #2
    info!("⚫ LED OFF");
    delay.delay_millis(500);

    cycle += 1;      // 📍 BREAKPOINT #3: Watch cycle increment
}
```

### GPIO Register Inspection

When paused after `led.set_high()`:
```
GPIO_OUT register (0x600a4008):
  - Bit 13 should be SET (1) → LED is ON
GPIO_IN register (0x600a401c):
  - Bit 9 should be SET (1) → GPIO9 reads HIGH (same as GPIO13)
```

When paused after `led.set_low()`:
```
GPIO_OUT register (0x600a4008):
  - Bit 13 should be CLEAR (0) → LED is OFF
GPIO_IN register (0x600a401c):
  - Bit 9 should be CLEAR (0) → GPIO9 reads LOW (same as GPIO13)
```

### Local Variable Inspection

At breakpoint on `cycle += 1`:
- Read `cycle` variable: increments 0 → 1 → 2 ... → 10
- When `cycle % 10 == 0`, special log appears
- Use debugger to verify this logic works

### Call Stack

At any breakpoint, view the call stack:
```
loop (line 102)
  └─ main (line 53)
    └─ _start (entry point)
```

Shows the function call chain.

---

## 🔬 Hands-On Exercises

### Exercise 1: Set Your First Breakpoint

**Goal:** Pause execution and see the code is running

1. Set breakpoint at line 102 (`led.set_high();`)
2. Run `cargo ff` (flash and monitor)
3. Wait for breakpoint to hit
4. Resume execution with `continue` command
5. Observe logs on serial output

**Result:** You've paused the CPU mid-execution!

### Exercise 2: Inspect GPIO Registers

**Goal:** Verify GPIO register values match code intent

1. Set breakpoint at line 103 (right after `led.set_high();`)
2. Hit breakpoint
3. Read GPIO peripheral registers:
   ```
   monitor read 0x600a4008  # GPIO_OUT
   monitor read 0x600a401c  # GPIO_IN
   ```
4. Verify:
   - Bit 13 of GPIO_OUT is SET (binary: ...1...)
   - Bit 9 of GPIO_IN is SET (reads same as GPIO13)
5. Continue
6. Immediately set another breakpoint at line 107 (after `led.set_low();`)
7. Hit breakpoint
8. Read same registers
9. Verify:
   - Bit 13 of GPIO_OUT is CLEAR (binary: ...0...)
   - Bit 9 of GPIO_IN is CLEAR (reads same as GPIO13)

**Result:** You've proven the hardware registers match what the code is doing!

### Exercise 3: Watch Variables Change

**Goal:** Use debugger to see local variable values

1. Set breakpoint at line 110 (`cycle += 1;`)
2. Hit breakpoint multiple times (press `continue` each time)
3. Each time paused, check the `cycle` variable value:
   ```
   monitor print cycle
   ```
4. Watch it increment: 0 → 1 → 2 → 3 ...
5. When you see `  └─ 10 cycles completed` in logs, `cycle` should be 10
6. Continue and watch it go 10 → 11 → 12 ...
7. When you see `  └─ 20 cycles completed`, `cycle` should be 20

**Result:** Debugger shows you live variable values without needing logs!

### Exercise 4: Understand Call Stack

**Goal:** See function calls in the execution stack

1. Set breakpoint anywhere in the loop
2. Hit breakpoint
3. Ask debugger for stack trace:
   ```
   backtrace
   ```
4. You should see:
   - Current function: `loop` (or the main loop code)
   - Caller: `main`
   - Caller's caller: `_start` (entry point)

**Result:** You understand the call chain!

---

## 🧪 Common Debugging Scenarios

### Scenario 1: "Is my GPIO code actually running?"

**Approach:**
1. Set breakpoint at `led.set_high()`
2. If breakpoint hits → code is running ✓
3. If breakpoint doesn't hit → something else is wrong (check loops, panics)

### Scenario 2: "Is my GPIO actually changing the pin?"

**Approach:**
1. Set breakpoint after `led.set_high()`
2. Read GPIO_OUT register
3. Check if bit 13 is set
4. If yes → hardware is responding ✓
5. If no → something is wrong with pin configuration

### Scenario 3: "Why is my loop not acting right?"

**Approach:**
1. Set breakpoint in loop
2. Watch the counter variable increment
3. Check the modulo logic: `if cycle % 10 == 0`
4. At breakpoint when cycle=10, the condition should be true
5. Verify with debugger

---

## 🐛 Troubleshooting

| Problem | Solution |
|---------|----------|
| `probe-rs list` shows nothing | Check USB cable, power up ESP32-C6 |
| Breakpoint doesn't hit | Build with `cargo build --release`, check code location |
| Can't read registers | Verify chip is paused at breakpoint first |
| Need to inspect memory address | Use `monitor read 0x<address>` |
| Debugger freezes | Disconnect USB, reconnect, rebuild |
| GPIO13/12 seem broken | These are reserved for JTAG—don't use them! |

---

## 🎯 Key Concepts

### USB-JTAG

ESP32-C6 has a built-in JTAG controller connected directly to USB:
- No external debugging hardware needed
- GPIO 12/13 are the D-/D+ lines
- Simultaneous with serial communication (different endpoints)
- Allows pausing CPU, reading memory, setting breakpoints

### probe-rs

Rust-native debugging tool:
- Works with ARM and RISC-V processors
- Full breakpoint/watchpoint support
- Real-time memory and register inspection
- No external OpenOCD server required

### Debugging vs Logging

| Aspect | Debugging (JTAG) | Logging (Serial) |
|--------|------------------|-----------------|
| **When to use** | Understanding code flow | Performance monitoring |
| **State inspection** | Automatic at breakpoint | Must be explicitly logged |
| **Overhead** | None when running | Always active |
| **Best combined** | Yes! Use both together | Yes! Use both together |

---

## 📚 Next Steps

- **Lesson 03:** Async/await with Embassy (use debugger for async debugging)
- **Lesson 04:** I2C Sensor Driver (debug I2C protocol)
- **Lesson 05:** SPI Display (debug display output)

---

## 🎯 Key Takeaways

1. ✅ **ESP32-C6 has free debugging** - USB-JTAG is built-in
2. ✅ **probe-rs is Rust-native** - No ESP-IDF or OpenOCD needed
3. ✅ **Breakpoints are powerful** - Pause and inspect anytime
4. ✅ **Registers show ground truth** - Hardware state matters most
5. ✅ **Logging + Debugging** - Use both together for best results
6. ✅ **GPIO12/13 are reserved** - For JTAG; don't reconfigure them

---

## 📖 References

- [probe-rs Docs](https://probe.rs/docs/)
- [ESP32-C6 USB-JTAG](https://docs.espressif.com/projects/esp-idf/en/stable/esp32c6/api-guides/usb-serial-jtag-console.html)
- [ESP32-C6 JTAG Debugging](https://docs.espressif.com/projects/esp-idf/en/stable/esp32c6/api-guides/jtag-debugging/)
- [esp-hal Examples](https://github.com/esp-rs/esp-hal/tree/main/examples)

---

*Lesson 02: Learning to debug like a professional.* 🔍
