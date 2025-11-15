# Lesson 02: UART CLI + Streaming Infrastructure

**Duration**: 180-240 minutes
**Complexity**: ⭐⭐⭐☆☆
**Hardware**: ESP32-C6-DevKit-C, FTDI UART adapter, LED + 220Ω resistor

## Learning Objectives

### esp-hal 1.0.0 APIs
- UART peripheral configuration and initialization
- UART read/write operations
- Command parsing with bounded buffers (`heapless::String`)
- Mode switching (CLI ↔ Streaming)

### Claude Code + GDB Debugging
- **Watchpoints**: Catch buffer overflow bugs
- **Mode switching via GDB**: Change firmware behavior without reflashing
- **Live parameter tuning**: Adjust variables at runtime
- **Hardware validation functions**: GDB-callable register checkers

### Real Firmware Patterns
- **CLI Mode**: Interactive command interface for testing/debugging
- **Streaming Mode**: High-speed telemetry output for monitoring
- **Progressive infrastructure**: This CLI becomes the testing backbone for all future lessons
- **Hardware unit testing**: CLI commands → GDB validates registers

---

## Hardware Setup

### Components Required
- ESP32-C6-DevKit-C board
- FTDI USB-to-UART adapter (or similar)
- LED + 220Ω resistor (from Lesson 01)
- Breadboard
- 4x jumper wires

### Wiring Diagram

```
ESP32-C6        FTDI UART       LED
--------        ---------       ---
GPIO23 (TX) --> RX
GPIO15 (RX) --> TX
GND         --> GND

GPIO12     -->  Anode (long leg)
               Cathode (short leg) --> 220Ω --> GND
```

**Pin Details**:
- **GPIO23**: UART TX (ESP32 transmits to FTDI)
- **GPIO15**: UART RX (ESP32 receives from FTDI)
- **GPIO12**: LED output (reused from Lesson 01)
- **GND**: Common ground between ESP32 and FTDI

### Physical Setup
1. Connect FTDI adapter:
   - ESP32 GPIO23 → FTDI RX
   - ESP32 GPIO15 → FTDI TX
   - ESP32 GND → FTDI GND
2. Connect LED (same as Lesson 01):
   - ESP32 GPIO12 → LED anode → 220Ω → GND
3. Plug FTDI adapter into computer via USB
4. Identify serial port: `ls /dev/cu.usbserial*` (macOS) or `ls /dev/ttyUSB*` (Linux)

---

## CLI Commands

This lesson implements a complete command-line interface over UART:

```
=== ESP32-C6 UART CLI ===
Type 'help' for commands

> help
Commands:
  gpio.init <pin>     - Initialize GPIO as output
  gpio.on <pin>       - Set GPIO high
  gpio.off <pin>      - Set GPIO low
  gpio.deinit <pin>   - Deinitialize GPIO
  stream.start        - Start streaming mode
  stream.stop         - Stop streaming (back to CLI)
  help                - Show this help

> gpio.init 12
OK [GPIO12 initialized as output]

> gpio.on 12
OK [GPIO12 = HIGH]

> gpio.off 12
OK [GPIO12 = LOW]

> stream.start
[Switching to streaming mode...]
[gpio12=0 counter=1 uptime_ms=1234]
[gpio12=0 counter=2 uptime_ms=1334]
[gpio12=0 counter=3 uptime_ms=1434]
...
```

### Hardware Validation via CLI + GDB

After each CLI command, use GDB to validate hardware state:

```gdb
# User types: gpio.on 12
# Firmware sets GPIO12 high

# Claude uses GDB:
(gdb) call validate_gpio_out_reg(true)
$1 = true  # ✓ Bit 12 is set as expected

# User types: gpio.off 12
(gdb) call validate_gpio_out_reg(false)
$2 = true  # ✓ Bit 12 is clear as expected
```

**This is hardware-based unit testing!**

---

## Progressive Debugging Workflow

This lesson uses **intentional bugs** to teach debugging techniques. Here's the commit-by-commit progression:

### Commit 1: Basic UART TX (Bug: Blocking Writes)

**Feature**: Send "Hello World" over UART

**Code State**:
```rust
// Blocking UART write
uart.write_str("Hello World\r\n").unwrap();
// Blocks until transmission completes
```

**Problem**: Blocking writes cause firmware to pause during transmission. At 115200 baud, each character takes ~87 µs. A 100-character message takes ~8.7 ms to transmit, blocking all other operations.

**GDB Debugging**:

```gdb
# Measure time spent in uart.write_str()
(gdb) break uart.write_str
(gdb) continue
Breakpoint 1, write_str () at ...

# Record cycle counter or time
(gdb) print UPTIME_MS
$1 = 1000

# Step over function
(gdb) finish

# Check time again
(gdb) print UPTIME_MS
$2 = 1009  # 9 ms spent in write! Too long for 10 Hz main loop
```

**Observation**: Blocking writes interfere with timing-sensitive operations.

**Fix** (for later commits): Use non-blocking writes or buffering.

---

### Commit 2: CLI Parser (Bug: Buffer Overflow)

**Feature**: Parse commands like `gpio.init`, `gpio.on`, `gpio.off`

**Code State**:
```rust
const CMD_BUFFER_SIZE: usize = 64;
let mut cmd_buffer: String<CMD_BUFFER_SIZE> = String::new();

// Read characters from UART
if uart.read(&mut rx_byte).is_ok() {
    let ch = rx_byte[0] as char;
    cmd_buffer.push(ch);  // BUG: No bounds checking!
}
```

**Problem**: User types >64 characters → buffer overflow (though `heapless::String` silently truncates, the logic doesn't handle this).

**GDB Debugging with Watchpoint**:

```gdb
# Set watchpoint on buffer capacity boundary
(gdb) watch *(char*)(&cmd_buffer + 64)

# Type a long command (>64 chars)
Hardware watchpoint 1: *(char*)(&cmd_buffer + 64)
Old value = 0
New value = 97  # 'a' character overwrote boundary!

# Backtrace shows overflow location
(gdb) backtrace
#0  process_uart_rx () at src/bin/main.rs:123
#1  main () at src/bin/main.rs:87
```

**Fix**: Check buffer length before pushing:
```rust
if ch.is_ascii_graphic() || ch == ' ' {
    if cmd_buffer.len() < CMD_BUFFER_SIZE - 1 {
        let _ = cmd_buffer.push(ch);
    } else {
        // Buffer full, ignore character
    }
}
```

**Claude's Role**: Use GDB watchpoint to catch exact moment of overflow, identify root cause.

---

### Commit 3: GPIO Control via CLI

**Feature**: Implement `gpio.init`, `gpio.on`, `gpio.off`, `gpio.deinit` commands

**Code**:
```rust
match parts[0] {
    "gpio.on" => {
        if let Ok(pin) = parts[1].parse::<u8>() {
            if pin == 12 {
                led.set_high();
                led_state = true;
                uart.write_str("OK [GPIO12 = HIGH]\r\n");
            }
        }
    }
    // ... other commands
}
```

**Hardware Validation Workflow**:

```gdb
# User types: gpio.on 12
# Firmware executes led.set_high()

# GDB validates GPIO_OUT_REG
(gdb) x/1xw 0x60004004
0x60004004:  0x00001000  # Bit 12 = 1 ✓

# Or use validation function
(gdb) call validate_gpio_out_reg(true)
$1 = true  # ✓ Hardware state matches expectation
```

**Claude demonstrates**:
1. User sends CLI command
2. Firmware updates GPIO
3. GDB inspects GPIO registers
4. Validation confirms hardware state matches software state
5. **No mocks, no simulation - real hardware testing!**

---

### Commit 4: Streaming Mode (Bug: DMA Misconfiguration)

**Note**: This lesson uses basic UART without DMA to keep it simple. The "DMA bug" is conceptual - in a DMA-enabled version, alignment issues would occur.

**Feature**: Add `stream.start` command → switch to streaming telemetry

**Streaming Output** (10 Hz):
```
[gpio12=0 counter=1 uptime_ms=1000]
[gpio12=0 counter=2 uptime_ms=1100]
[gpio12=1 counter=3 uptime_ms=1200]
[gpio12=1 counter=4 uptime_ms=1300]
```

**Format**: Parseable for automated testing and Claude Code analysis.

**Mode Switching**:
```rust
enum Mode {
    CLI = 0,
    Streaming = 1,
}

static mut MODE: Mode = Mode::CLI;

// In main loop:
match MODE {
    Mode::CLI => { /* process commands */ }
    Mode::Streaming => { /* output telemetry */ }
}
```

---

### Commit 5: Mode Switching via GDB

**Feature**: Change firmware mode from GDB without reflashing

**Scenario**: Firmware is in streaming mode (data flood). You want to test a CLI command without reflashing.

**GDB Workflow**:

```gdb
# Firmware currently streaming:
[gpio12=0 counter=45 uptime_ms=4500]
[gpio12=0 counter=46 uptime_ms=4600]
...

# Stop streaming via GDB
(gdb) call set_mode(0)  # 0 = CLI mode

# Firmware switches to CLI, shows prompt:
>

# Test CLI command
# (Type via UART terminal): gpio.on 12
OK [GPIO12 = HIGH]

# Validate with GDB
(gdb) call validate_gpio_out_reg(true)
$1 = true  # ✓ Works!

# Switch back to streaming
(gdb) call set_mode(1)  # 1 = Streaming mode

# Streaming resumes:
[gpio12=1 counter=47 uptime_ms=4700]
[gpio12=1 counter=48 uptime_ms=4800]
```

**Key Innovation**: **Live firmware reconfiguration** without code changes or reflashing!

---

### Commit 6: Buffer Overflow Protection + Error Handling

**Feature**: Comprehensive error handling

**Additions**:
1. Command buffer length checking (from Commit 2 fix)
2. Graceful handling of invalid commands
3. Parameter validation (e.g., pin number range)
4. UART error recovery

**Example Error Handling**:
```
> gpio.on 99
ERROR: Only GPIO12 supported

> invalid_command
ERROR: Unknown command. Type 'help' for commands.

> gpio.on
ERROR: Usage: gpio.on <pin>
```

**GDB Validation**:

Check that `GPIO_CHANGES` counter only increments on successful commands:

```gdb
(gdb) print GPIO_CHANGES
$1 = 0

# User: gpio.on 12
OK [GPIO12 = HIGH]

(gdb) print GPIO_CHANGES
$2 = 1  # ✓ Incremented

# User: gpio.on 99
ERROR: Only GPIO12 supported

(gdb) print GPIO_CHANGES
$3 = 1  # ✓ Not incremented (error case)
```

---

## Testing the CLI

### Using a Serial Terminal

**macOS/Linux**:
```bash
# Find FTDI port
ls /dev/cu.usbserial*  # macOS
ls /dev/ttyUSB*        # Linux

# Connect with screen
screen /dev/cu.usbserial-XXXXXX 115200

# Or use minicom
minicom -D /dev/cu.usbserial-XXXXXX -b 115200

# Or use Python script
python3 .claude/templates/read_uart.py /dev/cu.usbserial-XXXXXX 30
```

**Expected Interaction**:
```
=== ESP32-C6 UART CLI ===
Type 'help' for commands

> help
Commands:
  gpio.init <pin>     - Initialize GPIO as output
  gpio.on <pin>       - Set GPIO high
  gpio.off <pin>      - Set GPIO low
  ...

> gpio.on 12
OK [GPIO12 = HIGH]

> stream.start
[Switching to streaming mode...]
[gpio12=1 counter=1 uptime_ms=1234]
[gpio12=1 counter=2 uptime_ms=1334]
...
```

---

## GDB-Based Hardware Unit Testing

### Pattern: CLI Command → GDB Validation

**Workflow**:
1. Send CLI command via UART terminal
2. Firmware executes command
3. Use GDB to validate hardware state
4. Confirm registers match expected values

**Example Test Script** (`test_gpio_cli.gdb`):

```gdb
# Automated GPIO CLI testing script

define test_gpio_on
    # Assume user typed: gpio.on 12
    # Wait 100ms for command to execute
    continue 100

    # Validate GPIO_OUT_REG bit 12 = 1
    call validate_gpio_out_reg(true)

    if $1 == true
        printf "PASS: GPIO12 set high\n"
    else
        printf "FAIL: GPIO12 not high!\n"
    end
end

define test_gpio_off
    # Assume user typed: gpio.off 12
    continue 100

    call validate_gpio_out_reg(false)

    if $1 == true
        printf "PASS: GPIO12 set low\n"
    else
        printf "FAIL: GPIO12 not low!\n"
    end
end

# Run tests
test_gpio_on
test_gpio_off
test_gpio_on
```

**Usage**:
```bash
# In GDB session:
(gdb) source test_gpio_cli.gdb

# In UART terminal, type commands:
> gpio.on 12
> gpio.off 12
> gpio.on 12

# GDB output:
PASS: GPIO12 set high
PASS: GPIO12 set low
PASS: GPIO12 set high
```

**This is the foundation for automated hardware testing in all future lessons!**

---

## ESP32-C6 UART Register Map

Key UART registers (UART1 base: `0x60011000`):

| Register | Offset | Description |
|----------|--------|-------------|
| `UART_FIFO_REG` | 0x0000 | FIFO data register (read/write) |
| `UART_INT_RAW_REG` | 0x0004 | Interrupt raw status |
| `UART_INT_ENA_REG` | 0x000C | Interrupt enable |
| `UART_STATUS_REG` | 0x001C | UART status (TX/RX FIFO count) |
| `UART_CONF0_REG` | 0x0020 | Configuration (parity, stop bits) |
| `UART_CONF1_REG` | 0x0024 | Configuration (RX timeout, FIFO thresholds) |
| `UART_CLKDIV_REG` | 0x0028 | Clock divider for baud rate |

**Example: Reading UART status**:
```gdb
(gdb) x/1xw 0x6001101C  # UART_STATUS_REG
0x6001101C:  0x00000005  # TX FIFO has 5 bytes pending
```

---

## GDB Command Reference

### Essential Commands for This Lesson

**Watchpoints** (catch memory changes):
```gdb
# Watch a variable
(gdb) watch MODE
Hardware watchpoint 1: MODE

# Watch a memory address
(gdb) watch *(u32*)0x60004004  # GPIO_OUT_REG

# Watch with condition
(gdb) watch cmd_buffer.len if cmd_buffer.len > 60
```

**Function calls**:
```gdb
# Call validation functions
(gdb) call validate_gpio_out_reg(true)
$1 = true

# Switch modes
(gdb) call set_mode(1)  # Enter streaming
(gdb) call get_mode()
$2 = 1  # Confirmed

# Check statistics
(gdb) print GPIO_CHANGES
$3 = 5
```

**Conditional breakpoints**:
```gdb
# Break only on specific commands
(gdb) break process_command if parts[0] == "gpio.on"

# Break on buffer full
(gdb) break main.rs:95 if cmd_buffer.len() >= 63
```

**Continue for time** (useful for streaming):
```gdb
# Continue for 5 seconds
(gdb) continue 5000

# Or use timeout command
(gdb) timeout 5 continue
```

---

## Building and Running

### Build Firmware
```bash
cd lessons/02-uart-cli-streaming
cargo build --release
```

### Flash to ESP32-C6
```bash
# Flash via USB-JTAG
cargo run --release

# Or manually
espflash flash --port /dev/cu.usbmodem* target/riscv32imac-unknown-none-elf/release/main
```

### Expected USB CDC Output (via espflash monitor)
```
=== Lesson 02: UART CLI + Streaming Infrastructure ===

[INIT] Configuring GPIO12 as output for LED...
[INIT] LED ready
[INIT] Configuring UART on GPIO23 (TX), GPIO15 (RX) @ 115200 baud...
[INIT] UART ready
```

### UART Output (via FTDI, screen/minicom)
```
=== ESP32-C6 UART CLI ===
Type 'help' for commands

> help
Commands:
  gpio.init <pin>     - Initialize GPIO as output
  gpio.on <pin>       - Set GPIO high
  gpio.off <pin>      - Set GPIO low
  gpio.deinit <pin>   - Deinitialize GPIO
  stream.start        - Start streaming mode
  stream.stop         - Stop streaming (back to CLI)
  help                - Show this help

> gpio.on 12
OK [GPIO12 = HIGH]

> stream.start
[Switching to streaming mode...]
[gpio12=1 counter=1 uptime_ms=1234]
[gpio12=1 counter=2 uptime_ms=1334]
[gpio12=1 counter=3 uptime_ms=1434]
```

---

## Success Criteria

By the end of this lesson, you should be able to:

- [ ] Build and flash Lesson 02 firmware
- [ ] Connect FTDI adapter and establish UART communication @ 115200 baud
- [ ] Use CLI commands: `help`, `gpio.init`, `gpio.on`, `gpio.off`, `stream.start`, `stream.stop`
- [ ] Control LED via CLI commands
- [ ] Switch between CLI and streaming modes
- [ ] Use GDB to validate GPIO registers after CLI commands
- [ ] Call `validate_gpio_out_reg()` from GDB to check hardware state
- [ ] Switch modes via GDB without reflashing firmware
- [ ] Set watchpoint on buffer overflow
- [ ] Parse streaming telemetry output

**GDB Skills Acquired**:
- ✅ Watchpoints (`watch`)
- ✅ Function calls with return values
- ✅ Live firmware reconfiguration
- ✅ Hardware validation functions
- ✅ Mode switching via GDB

---

## Troubleshooting

### UART Not Working
1. **Check wiring**: ESP32 TX → FTDI RX, ESP32 RX → FTDI TX, GND → GND
2. **Verify baud rate**: 115200 in both firmware and terminal
3. **Find correct port**:
   ```bash
   ls /dev/cu.usbserial*  # macOS
   ls /dev/ttyUSB*        # Linux
   ```
4. **Test loopback**: Connect ESP32 TX to RX (bypass FTDI), should echo characters

### No CLI Prompt
1. **Power cycle ESP32** (unplug/replug USB)
2. **Check USB CDC output** with `espflash monitor` - should show init messages
3. **Send newline** to UART (press Enter) to trigger prompt

### Commands Not Working
1. **Check spelling**: Commands are case-sensitive
2. **Use help**: `help` command shows all available commands
3. **Use GDB to debug**: Set breakpoint in `process_command()` and step through

### Streaming Mode Stuck
- **Switch to CLI via GDB**: `(gdb) call set_mode(0)`
- **Or power cycle ESP32**

---

## Next Steps

**Lesson 03: PWM + Neopixel Drivers** will extend this CLI with:
- `pwm.init <pin> <freq_hz>` - Initialize PWM for LED brightness
- `pwm.duty <pin> <percent>` - Set duty cycle
- `neo.init <pin>` - Initialize Neopixel
- `neo.color <r> <g> <b>` - Set Neopixel color
- Streaming telemetry includes: `pwm12=50% pwm_freq=1000 neo_r=255 neo_g=0 neo_b=0`

The CLI built in Lesson 02 becomes the **testing backbone** for all future peripheral drivers!

---

## References

- [esp-hal 1.0.0 UART Documentation](https://docs.esp-rs.org/esp-hal/esp-hal/uart/index.html)
- [ESP32-C6 Technical Reference Manual - Chapter 28: UART](https://www.espressif.com/sites/default/files/documentation/esp32-c6_technical_reference_manual_en.pdf)
- [heapless Documentation](https://docs.rs/heapless/)
- [GDB Watchpoints](https://sourceware.org/gdb/current/onlinedocs/gdb/Set-Watchpoints.html)

---

**Lesson 02 Complete!** ✅
You now have a complete UART CLI + streaming infrastructure that will be extended in all future lessons!
