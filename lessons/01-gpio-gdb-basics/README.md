# Lesson 01: GPIO Basics + GDB Fundamentals

**Duration**: 90-120 minutes
**Complexity**: ⭐⭐☆☆☆
**Hardware**: ESP32-C6-DevKit-C, LED + 220Ω resistor, breadboard

## Learning Objectives

### esp-hal 1.0.0 APIs
- GPIO input configuration (button with pull-up resistor)
- GPIO output control (LED on/off)
- Basic polling and debouncing techniques
- Using `esp_hal::gpio::Input` and `Output` types

### Claude Code + GDB Debugging
- **Memory inspection**: `x/` command for register dumps
- **Variable inspection**: `print` command for firmware variables
- **Variable modification**: `set` command to change values at runtime
- **Function calls**: `call` command to execute firmware functions from GDB
- **Breakpoints and stepping**: Control execution flow interactively

### Embedded Concepts
- Memory-mapped I/O (GPIO registers)
- Pull-up resistors for button inputs
- Debouncing hardware inputs
- Hardware-based unit testing pattern

---

## Hardware Setup

### Components Required
- ESP32-C6-DevKit-C board
- LED (any color)
- 220Ω resistor
- Breadboard
- 2x jumper wires

### Wiring Diagram

```
ESP32-C6        LED               Button
--------        ---               ------
GPIO12     -->  Anode (long leg)
               Cathode (short)
                    |
                  220Ω
                    |
                   GND

GPIO9      -->  Onboard BOOT button (no wiring needed)
               (Active LOW: pressed = 0V, released = 3.3V via pull-up)
```

**Pin Details**:
- **GPIO12** (LED): Output, active HIGH (LED on when GPIO = 3.3V)
- **GPIO9** (Button): Input with internal pull-up, active LOW

### Physical Setup
1. Insert LED into breadboard (note polarity: long leg = anode = positive)
2. Connect GPIO12 to LED anode via jumper wire
3. Connect LED cathode to 220Ω resistor
4. Connect resistor to GND rail
5. Connect GND rail to ESP32 GND pin
6. Use onboard BOOT button (GPIO9) - no wiring needed!

---

## Progressive Debugging Workflow

This lesson teaches GDB debugging through **intentional bugs**. Each commit represents a stage of development and debugging:

### Commit 1: Broken GPIO Init (Bug: Missing GPIO Enable)

**Symptom**: LED initialization code runs, button reads correctly, but LED doesn't light up.

**Code State** (simplified):
```rust
// GPIO peripheral not enabled!
let mut led = Output::new(peripherals.GPIO12, Level::Low);
```

**GDB Debugging Session**:

```gdb
# Build and flash broken version
$ cargo build --release
$ espflash flash target/riscv32imac-unknown-none-elf/release/main

# In another terminal, start GDB
$ riscv32-esp-elf-gdb target/riscv32imac-unknown-none-elf/release/main
(gdb) target remote :3333  # Connect to OpenOCD/probe-rs

# Inspect GPIO registers
(gdb) x/16x 0x60004000
0x60004000:  0x00000000  0x00000000  0x00000000  0x00000000  # GPIO_OUT, GPIO_OUT_W1TS, etc.
0x60004010:  0x00000000  0x00000000  0x00000000  0x00000000
...

# Check GPIO_ENABLE_REG (offset 0x20)
(gdb) x/1xw 0x60004020
0x60004020:  0x00000000  # Bit 12 should be 1, but it's 0!

# This reveals: GPIO12 is not enabled!
```

**Root Cause**: esp-hal 1.0.0 `Output::new()` should automatically enable the GPIO, but in the broken version, the enable step is missing (simulated bug).

**Fix**: Add explicit GPIO enable call (or fix `Output::new()` implementation).

**Claude's Role**: Claude uses GDB to inspect registers, compares to ESP32-C6 Technical Reference Manual, identifies missing enable bit, and fixes the code.

---

### Commit 2: Working Polling (Bug: No Debounce)

**Symptom**: Button toggles LED, but single press triggers multiple toggles (bouncing).

**Code State**:
```rust
// Detects button press but no debounce!
if button_last_state && !button_current {
    led_state = !led_state;
    // Toggle LED immediately (BUG: bounces cause multiple toggles)
}
```

**GDB Debugging Session**:

```gdb
# Set breakpoint on button press detection
(gdb) break main.rs:68

# Press button once
Breakpoint 1, main () at src/bin/main.rs:68
68              if button_last_state && !button_current {

(gdb) continue
Breakpoint 1, main () at src/bin/main.rs:68  # Hit again! (bounce)

(gdb) continue
Breakpoint 1, main () at src/bin/main.rs:68  # Hit again! (bounce)

(gdb) continue
Breakpoint 1, main () at src/bin/main.rs:68  # Hit again! (bounce)
```

**Observation**: Single button press triggers breakpoint 3-5 times (mechanical bouncing).

**Fix**: Add debounce delay:
```rust
if button_last_state && !button_current {
    delay.delay_millis(50);  // Debounce delay
    if button.is_low() {     // Re-check after delay
        led_state = !led_state;
    }
}
```

**Claude's Role**: Claude sets breakpoint, observes multiple rapid hits, adds debounce logic, validates with GDB.

---

### Commit 3: LED Control Functions

**Feature**: Add functions callable from GDB for interactive LED control.

**Code**:
```rust
#[no_mangle]
pub extern "C" fn led_on(gpio_out_reg: *mut u32) {
    unsafe {
        let current = gpio_out_reg.read_volatile();
        gpio_out_reg.write_volatile(current | (1 << 12));
    }
}

#[no_mangle]
pub extern "C" fn led_off(gpio_out_reg: *mut u32) {
    unsafe {
        let current = gpio_out_reg.read_volatile();
        gpio_out_reg.write_volatile(current & !(1 << 12));
    }
}

#[no_mangle]
pub extern "C" fn led_toggle(gpio_out_reg: *mut u32) {
    unsafe {
        let current = gpio_out_reg.read_volatile();
        gpio_out_reg.write_volatile(current ^ (1 << 12));
    }
}
```

**GDB Usage**:

```gdb
# Call functions from GDB to control LED remotely
(gdb) call led_on(0x60004004 as *mut u32)
[GDB] led_on() called - GPIO12 = HIGH
# LED turns on!

(gdb) call led_off(0x60004004 as *mut u32)
[GDB] led_off() called - GPIO12 = LOW
# LED turns off!

(gdb) call led_toggle(0x60004004 as *mut u32)
[GDB] led_toggle() called - GPIO12 toggled
# LED toggles state!
```

**Benefit**: Control hardware without modifying/reflashing firmware. **Live firmware interaction**.

**Claude's Role**: Demonstrates calling firmware functions from GDB, shows how to control hardware interactively.

---

### Commit 4: GDB-Based Register Validation

**Feature**: Hardware-based unit testing pattern.

**Concept**: After each LED operation, validate hardware state matches expectations using GDB.

**Test Workflow**:

```gdb
# 1. Turn LED on
(gdb) call led_on(0x60004004 as *mut u32)

# 2. Validate GPIO_OUT_REG bit 12 = 1
(gdb) x/1xw 0x60004004
0x60004004:  0x00001000  # Binary: ...0001 0000 0000 0000 (bit 12 set) ✓

# 3. Turn LED off
(gdb) call led_off(0x60004004 as *mut u32)

# 4. Validate GPIO_OUT_REG bit 12 = 0
(gdb) x/1xw 0x60004004
0x60004004:  0x00000000  # Binary: ...0000 0000 0000 0000 (bit 12 clear) ✓

# 5. Toggle LED
(gdb) call led_toggle(0x60004004 as *mut u32)

# 6. Validate bit 12 toggled
(gdb) x/1xw 0x60004004
0x60004004:  0x00001000  # Bit 12 set again ✓
```

**Automation with GDB Scripts**:

Create `validate_gpio.gdb`:
```gdb
# Automated register validation script
define test_led_on
    call led_on(0x60004004 as *mut u32)
    x/1xw 0x60004004
    # Expected: 0x00001000 (bit 12 set)
end

define test_led_off
    call led_off(0x60004004 as *mut u32)
    x/1xw 0x60004004
    # Expected: 0x00000000 (bit 12 clear)
end

# Run tests
test_led_on
test_led_off
test_led_on
```

Run script: `(gdb) source validate_gpio.gdb`

**Claude's Role**: Creates GDB validation scripts, automates hardware testing, validates register state after each operation.

---

## ESP32-C6 GPIO Register Map

Key registers for this lesson (base address: `0x60004000`):

| Register | Offset | Description |
|----------|--------|-------------|
| `GPIO_OUT_REG` | 0x0004 | GPIO output value (bit N = GPION) |
| `GPIO_OUT_W1TS_REG` | 0x0008 | Write 1 to set bit (atomic set) |
| `GPIO_OUT_W1TC_REG` | 0x000C | Write 1 to clear bit (atomic clear) |
| `GPIO_ENABLE_REG` | 0x0020 | GPIO output enable (bit N = GPION enabled) |
| `GPIO_IN_REG` | 0x003C | GPIO input value (read button state) |
| `GPIO_STATUS_REG` | 0x0044 | Interrupt status register |
| `GPIO_FUNC_OUT_SEL_CFG[N]` | 0x0554 + N*4 | Function select for GPION output |

**Example: Reading button state**:
```gdb
(gdb) x/1xw 0x6000403C  # GPIO_IN_REG
0x6000403C:  0x00000200  # Binary: bit 9 = 1 (button released, pull-up)
                          # Press button...
0x6000403C:  0x00000000  # Binary: bit 9 = 0 (button pressed, active LOW)
```

---

## GDB Command Reference

### Essential Commands for This Lesson

**Connect to target**:
```gdb
$ riscv32-esp-elf-gdb target/riscv32imac-unknown-none-elf/release/main
(gdb) target remote :3333  # OpenOCD default port
```

**Memory inspection**:
```gdb
# Examine memory (x = examine)
(gdb) x/16x 0x60004000        # 16 hex words starting at address
(gdb) x/1xw 0x60004004        # 1 hex word (32-bit)
(gdb) x/1xb 0x60004004        # 1 hex byte (8-bit)
(gdb) x/s 0x3C000000          # String at address

# Format specifiers:
# x = hexadecimal, d = decimal, u = unsigned decimal
# b = byte (8-bit), h = halfword (16-bit), w = word (32-bit)
```

**Variable inspection**:
```gdb
(gdb) print led_state         # Print variable value
(gdb) print/x led_state       # Print in hexadecimal
(gdb) print &led_state        # Print address of variable
```

**Variable modification**:
```gdb
(gdb) set led_state = true    # Change variable at runtime
(gdb) set led_state = 1       # Same as true for bool
```

**Function calls**:
```gdb
(gdb) call led_on(0x60004004 as *mut u32)
(gdb) call led_toggle(0x60004004 as *mut u32)
```

**Breakpoints**:
```gdb
(gdb) break main.rs:68        # Break at line 68
(gdb) break led_on            # Break at function entry
(gdb) info breakpoints        # List all breakpoints
(gdb) delete 1                # Delete breakpoint #1
(gdb) disable 2               # Disable breakpoint #2
```

**Execution control**:
```gdb
(gdb) continue                # Resume execution
(gdb) step                    # Step into function
(gdb) next                    # Step over function
(gdb) finish                  # Run until function returns
(gdb) until 75                # Run until line 75
```

**Backtrace**:
```gdb
(gdb) backtrace               # Show call stack
(gdb) frame 0                 # Switch to stack frame 0
(gdb) info locals             # Show local variables
```

---

## Building and Running

### Build Firmware
```bash
# Debug build (faster compile, larger binary)
cargo build

# Release build (optimized, smaller binary, retains debug symbols)
cargo build --release
```

### Flash to ESP32-C6
```bash
# Auto-detect port and flash
cargo run --release

# Or specify port manually
espflash flash --port /dev/cu.usbmodem* target/riscv32imac-unknown-none-elf/release/main
```

### Expected Output
```
=== Lesson 01: GPIO Basics + GDB Fundamentals ===

[INIT] Configuring GPIO12 as output for LED...
[INIT] GPIO12 configured successfully
[INIT] Configuring GPIO9 as input for button (pull-up)...
[INIT] GPIO9 configured successfully

Ready! Press button to toggle LED.
(Use GDB to inspect registers and call functions)

[BUTTON] Press #1 detected!
[LED] Turned ON (GPIO12 = HIGH)

[BUTTON] Press #2 detected!
[LED] Turned OFF (GPIO12 = LOW)
```

---

## Debugging Setup (GDB)

### Option 1: Using OpenOCD (Traditional)

**Terminal 1: Start OpenOCD**
```bash
# ESP32-C6 built-in USB-JTAG
openocd -f board/esp32c6-builtin.cfg
```

**Terminal 2: Start GDB**
```bash
riscv32-esp-elf-gdb target/riscv32imac-unknown-none-elf/release/main
(gdb) target remote :3333
(gdb) monitor reset halt
(gdb) load  # Flash firmware via GDB (optional, if not already flashed)
(gdb) continue
```

### Option 2: Using probe-rs (Modern, Recommended)

```bash
# Flash and attach GDB server in one command
probe-rs gdb target/riscv32imac-unknown-none-elf/release/main --chip esp32c6

# In another terminal
riscv32-esp-elf-gdb target/riscv32imac-unknown-none-elf/release/main
(gdb) target remote :1337  # probe-rs default port
```

---

## Success Criteria

By the end of this lesson, you should be able to:

- [ ] Build and flash Lesson 01 firmware to ESP32-C6
- [ ] Press button to toggle LED (works with debouncing)
- [ ] Connect GDB to running firmware
- [ ] Inspect GPIO registers with `x/` command
- [ ] Validate GPIO_OUT_REG bit 12 matches LED state
- [ ] Call `led_on()`, `led_off()`, `led_toggle()` from GDB
- [ ] Set breakpoint on button press detection
- [ ] Modify `led_state` variable from GDB and observe LED change
- [ ] Understand memory-mapped I/O and GPIO register layout

**GDB Skills Acquired**:
- ✅ Memory inspection (`x/`)
- ✅ Variable printing (`print`)
- ✅ Variable modification (`set`)
- ✅ Function calls (`call`)
- ✅ Breakpoints (`break`, `continue`)
- ✅ Hardware register validation

---

## Troubleshooting

### LED Doesn't Light Up
1. **Check wiring**: Confirm GPIO12 → LED anode, LED cathode → 220Ω → GND
2. **Check polarity**: LED long leg = anode (positive)
3. **Use GDB to test**:
   ```gdb
   (gdb) call led_on(0x60004004 as *mut u32)
   (gdb) x/1xw 0x60004004  # Should show bit 12 set (0x1000)
   ```
4. **Measure voltage**: GPIO12 should be 3.3V when `led_on()` called

### Button Doesn't Work
1. **Check that you're using GPIO9** (onboard BOOT button)
2. **Verify pull-up**: Button released = HIGH, pressed = LOW
3. **Use GDB to read GPIO_IN_REG**:
   ```gdb
   (gdb) x/1xw 0x6000403C  # GPIO_IN_REG
   # Press button and read again, bit 9 should change
   ```

### GDB Won't Connect
1. **Check OpenOCD/probe-rs is running** in separate terminal
2. **Verify USB connection**: `ls /dev/cu.usbmodem*` (macOS) or `ls /dev/ttyACM*` (Linux)
3. **Try reset**: `(gdb) monitor reset halt`
4. **Check firewall**: Ensure port 3333 (OpenOCD) or 1337 (probe-rs) is open

### Multiple Button Toggles per Press
- This is **expected before Commit 2** (no debounce)
- After adding debounce delay, single press = single toggle
- Use GDB breakpoint to observe bouncing behavior

---

## Next Steps

**Lesson 02: UART CLI + Streaming Infrastructure** will build on these GPIO skills by adding:
- UART communication for CLI commands
- Command parser: `gpio.init`, `gpio.on`, `gpio.off`
- Streaming telemetry mode
- DMA for high-throughput data
- **Hardware unit testing via CLI + GDB**

The GPIO functions from Lesson 01 become the foundation for CLI-controlled hardware testing!

---

## References

- [esp-hal 1.0.0 Documentation](https://docs.esp-rs.org/esp-hal/)
- [ESP32-C6 Technical Reference Manual](https://www.espressif.com/sites/default/files/documentation/esp32-c6_technical_reference_manual_en.pdf) (Chapter 7: GPIO)
- [GDB Manual](https://sourceware.org/gdb/current/onlinedocs/gdb/)
- [RISC-V GDB Cheat Sheet](https://risc-v-getting-started-guide.readthedocs.io/en/latest/gdb.html)

---

**Lesson 01 Complete!** ✅
You now have foundational GPIO + GDB skills for embedded debugging. Ready for Lesson 02!
