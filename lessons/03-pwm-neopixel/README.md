# Lesson 03: PWM + Neopixel Drivers (Extend CLI)

**Duration**: 180-240 minutes
**Complexity**: ⭐⭐⭐☆☆
**Hardware**: ESP32-C6, LED (PWM), onboard Neopixel, FTDI UART

## Learning Objectives

### esp-hal 1.0.0 APIs
- LEDC peripheral (PWM for LED brightness control)
- RMT peripheral (WS2812 Neopixel timing)
- Clock configuration and dividers
- Smart LED library integration

### Claude Code + GDB (Advanced)
- Python GDB scripting for automated testing
- Disassembly inspection for timing validation
- Register-level peripheral debugging

## Hardware Setup

```
ESP32-C6        Components
--------        ----------
GPIO12     -->  LED + 220Ω (PWM-controlled brightness)
GPIO8      -->  Onboard Neopixel (no wiring)
GPIO23/15  -->  FTDI UART (from Lesson 02)
```

## Extended CLI Commands

**From Lesson 02** (still available):
```
gpio.*, stream.*
```

**New in Lesson 03**:
```
> pwm.duty <percent>        # Set PWM duty cycle (0-100)
> neo.color <r> <g> <b>     # Set Neopixel RGB (0-255)
> neo.off                   # Turn Neopixel off
> stream.start              # Stream PWM + Neo state
```

## Example Session

```
> pwm.duty 50
OK [PWM duty = 50%]

> neo.color 255 0 0
OK [Neopixel RGB=(255,0,0)]  # Red

> stream.start
[pwm12=50% neo_r=255 neo_g=0 neo_b=0 counter=1 uptime_ms=1234]
[pwm12=50% neo_r=255 neo_g=0 neo_b=0 counter=2 uptime_ms=1334]
```

## Intentional Bugs & GDB Debugging

### Bug 1: PWM Frequency Wrong
- Goal: 1 kHz PWM
- Actual: 80 kHz (forgot prescaler)
- **GDB Debug**: Inspect LEDC timer registers, calculate frequency
- **Fix**: Adjust clock divider

### Bug 2: Neopixel Wrong Colors
- Goal: `neo.color 255 0 0` → red
- Actual: Shows green
- **Root Cause**: RGB vs GRB byte order (WS2812 expects GRB)
- **GDB Debug**: Inspect RMT buffer, find byte sequence

### Bug 3: Neopixel Flickering
- **Root Cause**: RMT clock divider wrong → bit timing violates WS2812 spec
- **GDB Debug with Disassembly**:
  - `(gdb) disas rmt_send_pulse`
  - Inspect RMT clock config
  - Calculate bit timing from APB clock
  - Fix divider to meet WS2812 800 kHz requirement

## Progressive Infrastructure Pattern

**Key Innovation**: Each lesson **extends** the CLI from Lesson 02!

- Lesson 02: `gpio.*`, `stream.*`
- **Lesson 03 adds**: `pwm.*`, `neo.*`
- Lesson 04 will add: `imu.*`, `state.*`
- Lesson 05 will add: `device.*`

**Result**: By Lesson 05, you have a complete testing CLI with 20+ commands!

## Building and Running

```bash
cd lessons/03-pwm-neopixel
cargo build --release
cargo run --release
```

## Success Criteria

- [ ] PWM controls LED brightness (0-100%)
- [ ] Neopixel shows correct colors
- [ ] CLI commands work: `pwm.duty`, `neo.color`, `neo.off`
- [ ] Streaming telemetry includes PWM + Neopixel state
- [ ] GDB can inspect LEDC and RMT registers
- [ ] Understand timing bugs and fixes

## Next Steps

**Lesson 04** will add MPU6050 I2C sensor + state machine, continuing to extend this CLI framework!

---

**Complete detailed debugging workflows and GDB scripts will be added in future curriculum revisions.**
