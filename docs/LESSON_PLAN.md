# Lesson Plan: ESP32-C6 Professional Embedded Rust Development

**Philosophy**: Build practical, testable firmware using modern embedded Rust patterns.

**Stack**:
- `esp-hal 1.0.0` - Pure Rust HAL (no C dependencies)
- `Embassy` - Async/await for concurrent tasks
- `Enum-based state machines` - Type-safe state management
- CLI for interactive testing - Every lesson builds on it

---

## 🎯 Core Principle: Test as You Go

Every lesson includes:
1. **Functional tests** - Verify hardware behavior (GPIO state changes, etc.)
2. **Unit tests** - Test logic in isolation
3. **Interactive CLI** - Test without recompiling

This mirrors professional embedded development where you can't just `print` to debug.

---

## 📚 Complete Lesson Sequence

### Phase 1: Foundation & CLI (Weeks 1-2)

#### ✅ Lesson 01: Blinky (Complete)
**Status**: Finished
**Duration**: 15 min
**Concepts**: GPIO output, blocking code, serial logging

**What you learn:**
- esp-hal 1.0.0 initialization
- GPIO output configuration
- Blocking delays
- Serial logging patterns

**Hardware needed**: LED + resistor on GPIO13

---

#### ✅ Lesson 02: Embassy + Async Tasks (REPLACES old Lesson 02)
**Status**: Next
**Duration**: 30 min
**Concepts**: Async/await, task spawning, channel communication
**Hardware**: LED on GPIO13, Input on GPIO9

**What you learn:**
- Embassy executor and task spawning
- Async task coordination
- Channel-based communication between tasks
- Non-blocking code patterns
- Task testing with functional tests

**Key patterns:**
```rust
// Task 1: Blinks LED
#[embassy_executor::task]
async fn blink_task(mut led: Output<'static, GPIO13>) {
    loop {
        led.set_high();
        Timer::after(Duration::from_millis(500)).await;
        led.set_low();
        Timer::after(Duration::from_millis(500)).await;
    }
}

// Task 2: Reads input
#[embassy_executor::task]
async fn read_input_task(input: Input<'static, GPIO9>) {
    loop {
        if input.is_high().await {
            println!("GPIO9: HIGH");
        }
        Timer::after(Duration::from_millis(100)).await;
    }
}
```

**Tests included:**
- Toggle GPIO13, verify GPIO9 detects the change
- Task doesn't panic under load
- Channel communication works correctly

---

#### 🔜 Lesson 03: Serial CLI Framework (NEW - Core Lesson)
**Duration**: 45 min
**Concepts**: REPL, command parsing, protocol design
**Hardware**: Serial connection only

**What you learn:**
- Design a command protocol (simple text-based)
- Async serial reading with Embassy
- Command dispatch and parsing
- Extensible command system

**Implemented commands (and how to add more):**
```
gpio list          # List GPIO states
gpio write <pin> <high|low>
gpio read <pin>
echo <message>     # Echo test
help              # Show available commands
```

**Architecture:**
- Generic `Command` trait - Easy to extend in future lessons
- Command registry - Add new commands without rewriting the parser
- Clean error handling - Unknown commands don't crash

**Tests included:**
- Parse valid commands correctly
- Reject malformed input gracefully
- Command dispatch works as expected
- Easy to test without hardware

**Why CLI first?**
- No need to recompile to test different behaviors
- Interactive feedback loop matches professional development
- Foundation for all future lessons
- Tests the serial communication you'll use in every lesson

---

### Phase 2: Concurrency & State (Weeks 3-4)

#### 🔜 Lesson 04: Testing Framework for Embedded
**Duration**: 30 min
**Concepts**: Unit tests, integration tests, hardware verification

**What you learn:**
- `#[cfg(test)]` patterns for embedded
- Mock GPIO for unit tests
- Hardware integration tests (actual GPIO toggle + verify)
- Test utilities and helpers

**Example test:**
```rust
#[tokio::test]
async fn test_gpio_state_change() {
    // Arrange
    let led = create_test_gpio(13);
    let input = create_test_gpio(9);

    // Act
    led.set_high();

    // Assert - Verify input sees the change
    assert!(input.is_high());
}
```

**Patterns covered:**
- Property-based testing for state machines
- Fuzzing GPIO sequences
- Black-box testing of async code

---

#### 🔜 Lesson 05: State Machines with CLI Control
**Duration**: 40 min
**Concepts**: Enum-based FSM, state transitions, testing states
**Hardware**: LED on GPIO13, Button on GPIO9

**What you learn:**
- Design state machines with enums
- Type-safe state transitions
- CLI for state inspection and control

**Example FSM:**
```rust
enum LedMode {
    Off,
    On,
    SlowBlink(u32),   // Cycle counter
    FastBlink(u32),
}

enum Event {
    ButtonPressed,
    Timer,
}
```

**CLI commands for this lesson:**
```
led status         # Show current state
led toggle         # Send ButtonPressed event
led set <mode>     # Direct state change
```

**Tests included:**
- All state transitions are valid
- Invalid transitions panic with clear message
- CLI commands produce correct state changes
- State persists correctly

---

#### 🔜 Lesson 06: Interrupt-Driven GPIO with CLI Logging
**Duration**: 40 min
**Concepts**: Interrupts, interrupt handlers, event logging
**Hardware**: Button on GPIO9, LED on GPIO13

**What you learn:**
- Configure GPIO interrupts
- Handle interrupts safely in Embassy
- Log events with timestamps
- View event history via CLI

**CLI commands:**
```
events list        # Show last N events with timestamps
events clear       # Clear event log
gpio <pin> watch   # Monitor a pin for changes
```

---

### Phase 3: Peripherals (Weeks 5-7)

Each peripheral lesson follows the same pattern:
1. **Basic control** - Read/write via CLI commands
2. **Async driver** - Non-blocking access
3. **Tests** - Unit tests + integration tests
4. **Extended CLI** - Rich control commands

#### 🔜 Lesson 07: UART Serial Communication
**Duration**: 35 min
**Concepts**: UART setup, baud rates, serial protocols
**Hardware**: UART TX/RX pins

**CLI commands:**
```
uart write <data>     # Send data
uart read [bytes]     # Read N bytes (blocking)
uart config baud <rate>
uart status
```

**Tests:**
- Echo test (write, read back)
- Baud rate changes
- Error handling

---

#### 🔜 Lesson 08: I2C Sensor Reading
**Duration**: 45 min
**Concepts**: I2C protocol, addressing, sensor drivers
**Hardware**: I2C sensor (BME280 or similar - uses only power/GND/SCL/SDA)

**What you learn:**
- I2C master communication
- Sensor driver pattern
- Reading multi-byte values
- Error handling (NACK, timeout)

**CLI commands:**
```
i2c scan           # Find devices on bus
i2c read <addr>    # Read from address
sensor temp        # Read temperature
sensor humidity    # Read humidity
```

**Tests:**
- Scan finds expected devices
- Read/write timing
- CRC validation if sensor includes it

---

#### 🔜 Lesson 09: SPI Display Control
**Duration**: 50 min
**Concepts**: SPI protocol, display controllers, framebuffers
**Hardware**: SPI display (small LCD)

**What you learn:**
- SPI master setup (MOSI, MISO, CLK, CS)
- Display controller commands
- Framebuffer management
- Async drawing

**CLI commands:**
```
display clear      # Clear screen
display text <row> <col> <text>
display pixel <x> <y> <color>
display show       # Render framebuffer
```

---

#### 🔜 Lesson 10: Combined Peripherals Project
**Duration**: 60 min
**Concepts**: Integrating multiple peripherals, system design
**Hardware**: All previous peripherals + UART

**Project**: Build a sensor monitoring dashboard
- Read I2C sensor every second
- Display on SPI LCD
- Log to UART
- Control via serial CLI
- Tests for each component

---

### Phase 4: Advanced Patterns (Weeks 8-9)

#### 🔜 Lesson 11: Software Design Patterns
**Duration**: 40 min
**Concepts**: Builder pattern, traits, dependency injection

**What you learn:**
- Device abstractions
- Testable driver design
- Composition over inheritance

---

#### 🔜 Lesson 12: Error Handling & Recovery
**Duration**: 35 min
**Concepts**: Result types, custom errors, watchdog timers

**What you learn:**
- Proper error types for embedded
- Recovery patterns
- Watchdog configuration

---

#### 🔜 Lesson 13: Power Management
**Duration**: 40 min
**Concepts**: Sleep modes, wake sources, power profiling

---

#### 🔜 Lesson 14: Connectivity (WiFi)
**Duration**: 60 min
**Concepts**: WiFi setup, TCP/IP basics, HTTP server

---

#### 🔜 Lesson 15: Complete System: IoT Data Logger
**Duration**: 90 min
**Concepts**: Integrating all previous lessons into production system

---

## 🛠️ Development Workflow for Each Lesson

### Step 1: Read the lesson README
- Understand concepts
- See expected behavior
- Review hardware setup

### Step 2: Build and run
```bash
cd lessons/NN-topic
cargo ff              # Flash firmware
```

### Step 3: Test with CLI (if applicable)
```bash
# In another terminal:
python3 ../../scripts/monitor.py --port /dev/cu.usbserial-10 --baud 115200

# Try commands:
> help
> gpio list
> gpio write 13 high
> gpio read 9
```

### Step 4: Run tests
```bash
cargo test --release
```

### Step 5: Understand and modify
- Add new CLI commands
- Change LED timing
- Add validation logic
- Write new tests

---

## 📊 Skill Progression

| Lesson | GPIO | Async | State | UART | I2C | SPI | Testing | CLI |
|--------|------|-------|-------|------|-----|-----|---------|-----|
| 01 | ✅ | — | — | — | — | — | — | — |
| 02 | ✅ | ✅ | — | — | — | — | ✅ | — |
| 03 | ✅ | ✅ | — | ✅ | — | — | ✅ | ✅ |
| 04 | ✅ | ✅ | — | — | — | — | ✅ | ✅ |
| 05 | ✅ | ✅ | ✅ | — | — | — | ✅ | ✅ |
| 06 | ✅ | ✅ | ✅ | — | — | — | ✅ | ✅ |
| 07 | ✅ | ✅ | ✅ | ✅ | — | — | ✅ | ✅ |
| 08 | ✅ | ✅ | ✅ | ✅ | ✅ | — | ✅ | ✅ |
| 09 | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| 10+ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |

---

## 🧪 Testing Strategy

### Unit Tests (Per lesson)
- Test logic in isolation
- No hardware needed
- Fast execution
- Example:
  ```rust
  #[test]
  fn test_command_parsing() {
      let cmd = parse_command("gpio write 13 high");
      assert_eq!(cmd.pin, 13);
      assert_eq!(cmd.level, Level::High);
  }
  ```

### Integration Tests
- Test with real hardware
- Verify GPIO state changes
- Example:
  ```rust
  #[tokio::test]
  async fn test_gpio_output_affects_input() {
      // Requires GPIO13 connected to GPIO9
      let led = Output::new(peripherals.GPIO13, ...);
      let input = Input::new(peripherals.GPIO9, ...);

      led.set_high();
      assert!(input.is_high());
  }
  ```

### Property-Based Tests
- Fuzz command parsing
- Test state machine transitions
- Example:
  ```rust
  proptest! {
      #[test]
      fn parse_any_command_safely(s in ".*") {
          let _ = parse_command(&s);  // Should never panic
      }
  }
  ```

---

## 🎓 Professional Practices Covered

✅ **Testing**: Unit, integration, property-based
✅ **Error handling**: Custom error types
✅ **Async patterns**: Channels, tasks, synchronization
✅ **Design patterns**: Builder, trait-based design
✅ **Code organization**: Modules, separation of concerns
✅ **Documentation**: README, code comments, examples
✅ **Debugging**: CLI logging, state inspection
✅ **Performance**: Non-blocking I/O, efficient timing
✅ **Hardware abstraction**: Mock GPIO for testing
✅ **Real-world integration**: Multiple peripherals working together

---

## 🚀 When You're Done

You'll have:
- **Production-ready code structure** - Used in real embedded products
- **Comprehensive test suite** - Confidence in your code
- **Interactive control** - CLI for testing and debugging
- **Reusable patterns** - Apply to any embedded system
- **Professional workflow** - Build → Test → Debug → Deploy

The code from these lessons is **directly applicable** to commercial embedded systems.

---

**Last Updated**: Nov 2024
**Status**: Planning phase - Ready for implementation
**Next**: Create Lesson 02 (Embassy + Async) and Lesson 03 (CLI Framework)
