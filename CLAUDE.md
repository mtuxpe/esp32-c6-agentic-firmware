# CLAUDE.md - Guidelines for Claude Code Assistant

## Model Selection

**DEFAULT: Use Haiku Model**
- Unless explicitly told otherwise, use Claude Haiku (fastest, most cost-effective)
- Only use Sonnet or Opus if the task requires complex reasoning
- Current model: claude-haiku-4-5-20251001

**How to specify model:**
```
/model sonnet    # Switch to Sonnet
/model opus      # Switch to Opus
/model haiku     # Back to Haiku (default)
```

---

## File Operations

### ❌ Task() CANNOT Create Files
- `Task()` launches a subprocess agent for complex work
- **Agents cannot create files** - they can only read and report back
- **Don't use Task()** for file generation

### ✅ Use These Tools Instead
- `Write()` - Create new files or overwrite existing
- `Edit()` - Modify specific parts of existing files
- `Bash` - Create files via shell commands
- `Read()` - Read files before editing

### Rule of Thumb
**If you need to create/modify files → Use Write/Edit/Bash directly, NOT Task()**

---

## When to Use Task()

Task() is useful for:
- ✅ Complex research/exploration (general-purpose agent)
- ✅ Finding patterns in large codebases (Explore agent)
- ✅ Multi-step analysis and reporting
- ❌ **NOT for file creation/modification**

---

## Lean Lessons Approach

These lessons should be **simple and practical**:
- Focus on working code, not massive documentation
- Minimal READMEs (just basics)
- One main .rs file per lesson (~100-150 lines)
- Test on hardware immediately
- Keep it type-able for YouTube videos

---

## Project Conventions

### Directory Structure
```
lessons/{NN}-{name}/
├── src/
│   ├── bin/
│   │   └── main.rs          # Main firmware
│   └── lib.rs               # (optional library code)
├── .cargo/
│   └── config.toml          # Build config
├── Cargo.toml               # Dependencies
├── rust-toolchain.toml      # Toolchain
├── build.rs                 # Build script
└── README.md                # Simple docs (keep short!)
```

### Cargo.toml
- Always include `[[bin]]` section pointing to `src/bin/main.rs`
- Keep dependencies minimal
- Use esp-hal 1.0.0

### Documentation
- README.md: Keep it short (< 300 lines)
- Focus on: wiring, expected output, troubleshooting
- Skip lengthy theory - put that in code comments

---

## Testing Approach

1. **Build:** `cargo build --release`
2. **Flash:** `cargo run --release`
3. **Test:** Manual hardware validation
4. **Iterate:** Fix issues, re-test

No massive test plans until code works on hardware.

---

## Git Workflow

- Commit after each working lesson
- Keep commit messages clear and concise
- Format: `feat(lesson-{NN}): {brief description}`

---

## Bash Execution Best Practices

### Shell Limitations in LLM Context

The LLM's bash execution environment has important limitations:

**❌ Complex conditionals fail inline:**
```bash
# This will cause parse errors:
if [ $EXIT_CODE -eq 0 ]; then
    echo "success"
fi
```

**✅ Use temp scripts for complex logic:**
```bash
# This works reliably:
cat > /tmp/script.sh << 'SCRIPT'
#!/bin/bash
if [ $EXIT_CODE -eq 0 ]; then
    echo "success"
fi
SCRIPT
chmod +x /tmp/script.sh
/tmp/script.sh
```

### Variable Persistence

**Variables DON'T persist across Bash() tool calls:**
```bash
# Step 1:
export MY_VAR="value"

# Step 2 (different Bash call):
echo $MY_VAR  # Empty! Variable is gone
```

This applies to **all** variables, including those from `source`:

```bash
# Step 1:
source scripts/find-esp32-ports.sh  # Exports $USB_CDC_PORT

# Step 2 (different Bash call):
espflash flash --port $USB_CDC_PORT  # Variable is empty!
```

### Solutions for Variable Persistence

#### Option 1: Consolidate into Single Bash Call (Recommended)

```bash
# GOOD: All in one call
USB_PORT=$(ls /dev/cu.usbmodem* | head -1) && \
espflash flash --port $USB_PORT target/.../main && \
python3 read_uart.py $(ls /dev/cu.usbserial* | head -1) 5
```

#### Option 2: Re-detect in Each Step

```bash
# Step 1: Flash (detect port inline)
espflash flash --port $(ls /dev/cu.usbmodem* | head -1) target/.../main

# Step 2: Monitor (re-detect port)
python3 read_uart.py $(ls /dev/cu.usbserial* | head -1) 5
```

#### Option 3: Use Files for State (Last Resort)

```bash
# Step 1: Save to file
ls /dev/cu.usbmodem* | head -1 > /tmp/usb_port.txt

# Step 2: Read from file
python3 read_uart.py $(cat /tmp/usb_port.txt) 5
```

**Don't rely on `export` or `source`** - they don't work across tool invocations.

### When to Use Temp Scripts

Use temp scripts (`/tmp/*.sh`) for:
- Commands with if/then/fi conditionals
- Loops (for, while)
- Complex variable manipulation
- Multi-step operations with error checking

Use inline bash for:
- Simple single commands
- Command chains with `&&` or `||`
- Quick reads/writes without conditionals

---

## Common Mistakes to Avoid

1. ❌ Using Task() to generate files
2. ❌ Over-engineering lessons (keep them simple!)
3. ❌ Massive documentation before working code
4. ❌ Not testing on hardware
5. ❌ Using expensive models (Sonnet/Opus) by default
6. ❌ Using complex conditionals inline in Bash (use temp scripts!)
7. ❌ Expecting variables to persist across Bash() calls (use files!)

---

## Embedded Debugging Philosophy: Virtual Debug Ears and Eyes

**The core insight:** Instead of being blind while firmware runs, instrument the entire system with RTT telemetry to get **real-time visibility into register values, state changes, and hardware behavior without stopping execution**.

### Traditional vs Data-Driven Debugging

**Traditional embedded debugging:**
- Breakpoints freeze execution (destroy timing)
- UART logging blocks the firmware (14 KB/s max)
- You guess what's happening based on symptoms
- Hypothesis test each subsystem (slow, repetitive)

**Data-driven debugging with RTT:**
- **Eyes:** See register values, ADC outputs, GPIO states, memory contents
- **Ears:** Listen to I2C transactions, state transitions, error flags, event counters
- Everything runs live (firmware never stops, timing stays accurate)
- Patterns jump out immediately (no guessing, just observe)

### RTT as Virtual Instrumentation

Think of RTT logging as placing sensors throughout your firmware:

```
Physical Hardware              RTT Virtual Instrumentation
────────────────────          ──────────────────────────

I2C Bus           ────────→   "i2c: wr=5/5 rd=5/5 errs=0"
Config Register   ────────→   "ads_cfg: mux=0 pga=1 mode=0 dr=7"
ADC Output        ────────→   "ads_adc: raw=0x0ABC volts=1.234"
State FSM         ────────→   "ads_fsm: state=Reading time_ms=45"
Error Flags       ────────→   "i2c: timeouts=0 acks=0"
```

Instead of stopping to inspect a value (breakpoint), you let the firmware run and **stream all hardware state to your terminal in real-time**.

## Embedded Debugging Philosophy: Data-Driven Analysis

**The core insight:** In complex embedded systems, you don't debug by hypothesis testing - you debug by **collecting all data and finding patterns**.

### Why Traditional Debugging Fails in Embedded

```
Old approach: "Button doesn't work → check button pin → check interrupt → check state machine"
Problem: You're guessing at what's wrong. What if it's actually an I2C timeout that cascades?
         Or a race condition between ISR and main loop? Or corrupted state from previous operation?
```

### Data-Driven Debugging with RTT

```
New approach: "Button doesn't work → log ALL variables (GPIO, I2C, ISR state, FSM, timers, etc.)
              at 100 Hz → analyze patterns → see: 'button press → i2c_errors spike → sensor stops
              responding → LED never updates'"
Result: Root cause visible instantly. Fix is obvious: add I2C timeout recovery.
```

### Why This Works with Claude Code

1. **Humans are pattern-matchers** - Claude excels at analyzing massive datasets
2. **Correlations reveal causality** - When variables spike together, something connects them
3. **No hypothesis needed** - Just collect data and analyze. The relationships appear naturally
4. **RTT is non-blocking** - Unlike UART, timing stays accurate. Bugs don't hide
5. **Structured defmt logs** - Machine-parseable format enables automated pattern detection

### Variable Bandwidth Budget Approach

Instead of thinking "add minimal debug code," think in **data throughput budgets**:

```
Available RTT bandwidth: 1-10 MB/s depending on JTAG clock

Typical variable sizes @ 100 Hz:
- 4-byte integer: ~4 bytes per log
- defmt overhead: ~10-20 bytes per message
- Total per variable: ~15-25 bytes

Example: 100 variables × 25 bytes × 100 Hz = 250 KB/s
         This is 0.25% of RTT capacity on a 100 MB/s system
         Plenty of headroom for multiple channels and variable data
```

### When to Use This Strategy

✅ **Use maximum observability when:**
- System behavior is complex or non-obvious
- Multiple subsystems interact (I2C + GPIO + ISR + main loop)
- You're unfamiliar with the code
- Timing-sensitive bugs (RTT's non-blocking nature is critical)
- Quick iteration needed (Claude analyzing logs is fast)

❌ **Minimize logging only when:**
- Memory severely constrained (< 50 KB available for debug infrastructure)
- Production deployment (then use minimal counters for telemetry)
- Proven simple bugs (single-subsystem issues)

### The shift: From "minimal overhead" to "maximum insight"

Traditional embedded development: "We need to log carefully to avoid overhead"
RTT-driven development: "We have 1-10 MB/s available, let's use it all"

## Embedded Debugging Strategies for RTT

When using RTT (Real-Time Transfer) for autonomous firmware development and debugging, apply these battle-tested patterns:

### Event Counters for High-Frequency Debugging

Track events without blocking the main firmware loop using atomic operations:

```rust
use core::sync::atomic::{AtomicU32, Ordering};

// Global event counters
static I2C_ERRORS: AtomicU32 = AtomicU32::new(0);
static GPIO_INTERRUPTS: AtomicU32 = AtomicU32::new(0);
static SENSOR_READS: AtomicU32 = AtomicU32::new(0);

// In interrupt handler or hot path:
I2C_ERRORS.fetch_add(1, Ordering::Relaxed);  // 5-10 CPU cycles, non-blocking

// Log periodically (e.g., every 100ms):
defmt::info!("i2c_errors={}, interrupts={}, reads={}",
    I2C_ERRORS.load(Ordering::Relaxed),
    GPIO_INTERRUPTS.load(Ordering::Relaxed),
    SENSOR_READS.load(Ordering::Relaxed)
);
```

**Why this works:**
- Atomic operations use hardware compare-and-swap, not locks
- `Relaxed` ordering = no synchronization overhead
- Periodic logging prevents RTT saturation
- Counters survive firmware resets

### Bit Array State Tracking

For tracking many boolean states (e.g., 10K+ GPIO pin states), use bit arrays instead of byte arrays:

```rust
// Instead of: let mut states: [bool; 10000] = [false; 10000];  (10 KB)
// Use a bit array (1.25 KB):

let mut state_bits = [0u32; 312];  // 312 * 32 = 10,000 bits = 1,250 bytes

// Set bit: state_bits[pin_id / 32] |= 1 << (pin_id % 32);
// Clear bit: state_bits[pin_id / 32] &= !(1 << (pin_id % 32));
// Read bit: (state_bits[pin_id / 32] >> (pin_id % 32)) & 1
```

**RTT streaming:**
```rust
// Stream as 32-bit words for efficient transfer
for word in &state_bits {
    defmt::info!("state_word: bits={:032b}", word);
}
// 10,000 bits → 312 defmt messages → ~2-3 KB RTT bandwidth
```

### Memory Budget Guidelines

Allocate debug infrastructure based on available ESP32-C6 SRAM (512 KB total, ~400-450 KB available to user code):

| Debug Level | Allocation | Use Cases |
|-------------|-----------|-----------|
| **Minimal** | 10-20 KB | Single driver, basic counters, 5-10 debug variables |
| **Standard** | 50-80 KB | Multi-driver system, state tracking, event buffers |
| **Extensive** | 100-150 KB | Full system observability, large ring buffers, state arrays |
| **Available for App** | 250-400 KB | Remaining SRAM for actual firmware logic |

**Allocation strategy:**
```rust
// Track actual usage
const DEBUG_BUFFER_SIZE: usize = 64 * 1024;  // 64 KB for RTT ring buffers
const STATE_ARRAY_SIZE: usize = 16 * 1024;   // 16 KB for state tracking
const COUNTER_SIZE: usize = 4 * 1024;        // 4 KB for atomic counters
const AVAILABLE_FOR_APP: usize = 512_000 - DEBUG_BUFFER_SIZE - STATE_ARRAY_SIZE - COUNTER_SIZE;
// Available for app: ~428 KB
```

### RTT Bandwidth Planning

RTT throughput depends on JTAG clock frequency. Plan logging accordingly:

| JTAG Clock | Throughput | Recommended Load |
|-----------|-----------|-----------------|
| **1 MHz** | 250-500 KB/s | 5 variables @ 100 Hz |
| **4 MHz** | 1-2 MB/s | 10-15 variables @ 100 Hz |
| **10 MHz** | 3-5 MB/s | 20-30 variables @ 100 Hz |

**Saturation limits:**
- **Safe zone:** 1-2 MB/s (leaves headroom, won't drop frames)
- **Good zone:** 2-4 MB/s (acceptable, occasional frame loss tolerable)
- **Saturation:** 5+ MB/s (frame loss likely, debugging degrades)

**Rule of thumb:** `throughput ≈ (variables × bytes_per_msg × sample_rate_hz) / 1_000_000`

```rust
// Example: 15 sensor readings, 8 bytes each, 100 Hz sample rate
// Throughput = (15 × 8 × 100) / 1_000_000 = 0.012 MB/s (very safe)

// Bad example: 50 variables, 32 bytes each, 1000 Hz
// Throughput = (50 × 32 × 1000) / 1_000_000 = 1.6 MB/s (saturating)
```

### UART vs RTT Decision Matrix

Choose based on development phase and requirements:

| Factor | UART | RTT |
|--------|------|-----|
| **Throughput** | 14-250 KB/s | 1-10 MB/s |
| **Blocking** | Yes (blocking write) | No (ring buffer) |
| **GPIO Overhead** | Uses pins | Built-in JTAG |
| **Hardware Needed** | USB-Serial | JTAG probe |
| **Best For** | Production logging, simple debugging | Development, autonomous testing, high-speed capture |
| **Cost** | Cheap (~$5) | Moderate (~$30-50) |

**Recommendation:**
- **Development (L08-L09):** RTT + defmt for non-blocking, structured logging
- **Production (L10+):** UART + esp-println for power efficiency, external logging

### Arbitrary Memory/Register Access

Use probe-rs or GDB to inspect and modify memory at runtime without adding debug code:

```bash
# With probe-rs, you can query any variable from ELF symbols:
probe-rs run --chip esp32c6 --probe <probe-id> target/*/debug/firmware

# While running, attach GDB:
gdb target/*/debug/firmware
(gdb) target remote :3333  # OpenOCD port
(gdb) print my_global_var
(gdb) set my_global_var = 42
(gdb) continue
```

**Best practices:**
1. **Use ELF map file** to find variable addresses:
   ```bash
   cargo build && nm -n target/riscv32imac-unknown-none-elf/debug/firmware | grep my_var
   ```

2. **Read peripheral registers directly:**
   ```bash
   # Query GPIO state without adding logging code:
   (gdb) x/1xw 0x60004000  # Read GPIO register
   ```

3. **Set conditional breakpoints on hardware state:**
   ```bash
   (gdb) break main.rs:42 if sensor_value > 1000
   ```

### Practical Debugging Workflow for Autonomous Development

**Maximum Observability Strategy:**
Start with comprehensive logging of all relevant variables. RTT's non-blocking nature and 1-10 MB/s throughput enable logging 50-500+ variables @ 100 Hz without affecting timing.

**Step-by-step:**
1. **Log everything relevant** - All peripheral state, sensor data, FSM state, error flags
2. **Structured defmt format** - Machine-parseable logs enable instant pattern detection
3. **Sample at 100 Hz** - Fast enough to catch all behavior, slow enough to avoid saturation
4. **Claude Code analyzes patterns** - Correlations reveal root cause immediately
5. **Minimal iterations** - Usually fixed in 1-2 debug cycles vs many with minimal logging

**Example: Debugging I2C driver autonomously**
```rust
// Log all I2C and system state every 10ms
defmt::info!("i2c: status=0x{:04x} writes={} reads={} errors={} scl={} sda={} fifo={} state={}",
    i2c_status, i2c_writes, i2c_reads, i2c_errors, scl_pin, sda_pin, fifo_level, fsm_state
);

// Also log related sensor data
defmt::info!("sensors: accel_x={} accel_y={} accel_z={} temp={} ready={}",
    accel_x, accel_y, accel_z, temperature, sensor_ready
);
```

**Why this works for autonomous debugging:**
- Claude sees all state changes instantly
- Correlations appear naturally (button → i2c_error → sensor_fail)
- No guessing which variable to inspect next
- RTT non-blocking means timing is accurate (not masked by UART waits)
- defmt structure lets Claude write regex parsers to extract patterns

**Bottleneck considerations:**
- JTAG bandwidth: 10+ MB/s (rarely the limit)
- probe-rs parsing: ~1-10 MB/s (likely bottleneck)
- USB 2.0: 12 Mbps = 1.5 MB/s (may limit USB-based probes)
- Test actual limits with your hardware to find max sustainable variables

---

## Quick Reference

| Task | Tool | Time |
|------|------|------|
| Create lesson code | Write() + Bash | 5-10 min |
| Modify file | Edit() | 2-5 min |
| Create README | Write() | 3-5 min |
| Test on hardware | Manual | 10-20 min |
| **Avoid: Massive planning** | ~~Task()~~ | ⏱️ Don't |

---

## Slash Commands & Tools

Custom slash commands are stored in `.claude/commands/`:

### Lesson Testing
- **`/test-lesson <number> [mode]`** - Unified hardware testing for any lesson
  - Examples: `/test-lesson 07`, `/test-lesson 08 full`
  - Modes: `quick` (default, 3-5 min) or `full` (10-20 min)
  - Auto-detects hardware (USB ports, JTAG probes)
  - Reads lesson-specific `TEST.md` for test procedures
  - Generates comprehensive test reports

Each lesson has a `TEST.md` specification that documents:
- Hardware setup and wiring
- Automated tests (build, flash, infrastructure)
- Interactive tests (manual verification)
- Expected outputs and troubleshooting

### RTT Debugging
- **`/rtt [subcommand]`** - RTT (Real-Time Transfer) debugging and validation tools
  - `tutorial [topic]` - Learn RTT best practices interactively
  - `sweep [options]` - Performance characterization for your device
  - `validate [file]` - Automated firmware testing on hardware
  - `analyze [log]` - Log analysis and parsing
  - `tools` - Reference and system diagnostics
  - `guide` - Open full RTT Mastery reference

See `.claude/rtt-guide.md` for complete RTT reference documentation.

---

## Hardware Lesson Development

**CRITICAL: Always test on actual hardware as you develop. Don't write untested code.**

### Incremental Development Workflow

When creating hardware-interacting firmware, ALWAYS follow this sequence:

#### Phase 1: Minimal Viable Test (5-10 minutes)
1. **Start with template:** Copy `.claude/templates/uart_test_minimal.rs` to `src/bin/`
2. **Configure pins:** Update GPIO pin numbers for your hardware
3. **Build and flash:** `cargo build --bin uart_test_minimal`
4. **Verify output:** Use `python3 read_uart.py <port> 5` to confirm UART works
5. **Document working config:** Save pin numbers in uart-config.toml or README

#### Phase 2: Core Functionality (15-30 minutes)
1. Build on proven minimal test
2. Add one feature at a time
3. Test after each addition
4. Don't proceed if tests fail

#### Phase 3: Error Handling & Polish (10-15 minutes)
1. Add bounds checking, safety features
2. Improve error messages
3. Write comprehensive tests

### Why This Matters

❌ **BAD (Don't do this):**
```
1. Write 300-line complex firmware with DMA, slots, safety checks
2. Try to compile → API errors
3. Fix API errors
4. Flash → No output
5. Debug for 30+ minutes to find GPIO pins were wrong
```

✅ **GOOD (Do this):**
```
1. Copy uart_test_minimal.rs (47 lines)
2. Update GPIO pins
3. Flash and verify output → 5 minutes
4. Add one feature (e.g., single variable streaming) → 10 minutes
5. Add multi-variable support → 10 minutes
6. Add safety checks → 10 minutes
Total: ~35 minutes with validated checkpoints
```

### Hardware Verification Checklist

Before writing any complex firmware:

- [ ] Know which GPIO pins are connected (ask user if unsure)
- [ ] Have minimal test binary ready
- [ ] Can see output on serial port
- [ ] Verified correct TX/RX orientation (ESP TX → Adapter RX)
- [ ] Documented working configuration

### Pin Discovery Process

If you don't know which pins to use:

1. **Check documentation:**
   - Lesson-specific README
   - Previous working configurations
   - ESP32-C6 datasheet

2. **Ask user:**
   - "Which GPIO pins are connected to your UART adapter?"
   - "Is this the same setup as Lesson X?"

3. **Systematic testing:**
   - Try common pairs: GPIO16/17, GPIO23/15, GPIO4/5
   - Test both orientations (TX/RX swapped)
   - Use `scripts/test-uart-pins.sh` helper

---

## esp-hal 1.0.0 API Patterns

**IMPORTANT: esp-hal 1.0.0 has breaking changes from pre-1.0 versions.**

### UART Initialization

❌ **OLD (pre-1.0) - Don't use:**
```rust
use esp_hal::{gpio::Io, uart::{Config, Uart}};

let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
let uart = Uart::new_with_config(
    peripherals.UART1,
    Config::default().with_baudrate(115200),
    io.pins.gpio23,
    io.pins.gpio15,
).unwrap();
```

✅ **NEW (1.0.0+) - Use this:**
```rust
use esp_hal::{
    delay::Delay,
    main,
    uart::{Config as UartConfig, Uart},
};

let mut uart = Uart::new(peripherals.UART1, UartConfig::default())
    .expect("Failed to init UART")
    .with_tx(peripherals.GPIO23)
    .with_rx(peripherals.GPIO15);

// Writing to UART
uart.write(b"Hello\n").ok();
```

### DMA Support

**Status:** DMA APIs exist but are complex and not yet documented in simple lessons.

**Recommendation:** Start with blocking UART (shown above) for initial lessons. Add DMA as advanced topic later.

**Where to find examples:**
- Official esp-hal repo: https://github.com/esp-rs/esp-hal/tree/main/examples
- Check existing lessons: `grep -r "with_dma" lessons/*/src/`

---

## Serial Port Operations

**CRITICAL: Never use blocking serial operations that can freeze the conversation.**

### ✅ Recommended: Python Reader Script

Always use the provided Python script for serial monitoring:

```bash
# Auto-terminates after 5 seconds
python3 .claude/templates/read_uart.py /dev/cu.usbserial-FT58PFX4 5

# Or use device discovery
source scripts/find-esp32-ports.sh
python3 .claude/templates/read_uart.py $FTDI_PORT 5
```

**Why this works:**
- Guaranteed termination (no hanging)
- Cross-platform (macOS, Linux, Windows)
- Proper error handling
- Clean resource cleanup

### ❌ Anti-Patterns (Don't use these)

```bash
# BAD: Blocks forever, no timeout
cat /dev/cu.usbmodem1101

# BAD: Background process doesn't auto-terminate
cat /dev/cu.usbmodem1101 &

# BAD: Requires interactive TTY, fails in automation
espflash monitor /dev/cu.usbmodem1101

# BAD: macOS doesn't have GNU timeout by default
timeout 3 cat /dev/cu.usbserial-FT58PFX4
```

### Serial Port Cleanup

If you need to clean up stuck processes:

```bash
# Kill any hanging cat/espflash processes
pkill -f "cat /dev/cu\." || true
pkill -f "espflash monitor" || true
pkill -f "screen /dev/cu\." || true
```

---

## Device Discovery

**Don't hardcode serial port paths** - they change when devices are unplugged/replugged.

### Automatic Port Detection

```bash
# Use the discovery script
source scripts/find-esp32-ports.sh

# Variables are now exported:
# $USB_CDC_PORT - ESP32 USB-JTAG (for flashing/debugging)
# $FTDI_PORT    - FTDI UART (for data streaming)

# Use in commands:
espflash flash --port $USB_CDC_PORT target/.../main
python3 read_uart.py $FTDI_PORT 5
```

### Manual Detection

If discovery script doesn't work:

```bash
# macOS:
ls /dev/cu.usbmodem*    # ESP32 USB-JTAG
ls /dev/cu.usbserial*   # FTDI UART

# Linux:
ls /dev/ttyACM*         # ESP32 USB-JTAG
ls /dev/ttyUSB*         # FTDI UART
```

---

**Last Updated:** 2025-11-14
**Current Work:** Lesson 08 (UART + GDB Tandem Debugging)
**Next:** Hardware debugging infrastructure improvements
