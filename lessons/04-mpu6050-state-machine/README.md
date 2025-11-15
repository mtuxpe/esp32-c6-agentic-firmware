# Lesson 04: MPU6050 + State Machine (Extend CLI)

**Duration**: 240-300 minutes
**Complexity**: ⭐⭐⭐⭐☆
**Hardware**: ESP32-C6, MPU6050 IMU, Button, LED, Neopixel, UART

## Learning Objectives

### esp-hal 1.0.0 APIs
- I2C peripheral configuration and communication
- MPU6050/MPU9250 sensor register access
- State machine implementation in embedded systems
- Button-driven state transitions

### Claude Code + GDB (Advanced)
- Conditional breakpoints for I2C error detection
- Tracepoints for performance profiling
- State machine debugging and forced transitions
- Watchpoints on calibration progress

## Hardware Setup

```
ESP32-C6        MPU6050         Other
--------        -------         -----
GPIO2 (SDA) --> SDA
GPIO11 (SCL)--> SCL
3.3V        --> VCC
GND         --> GND

GPIO9       --> Onboard BOOT button
GPIO12      --> LED (from previous lessons)
GPIO8       --> Neopixel (from previous lessons)
GPIO23/15   --> FTDI UART (from previous lessons)
```

## State Machine

```
┌─────────┐  Button      ┌──────────┐  Button     ┌────────────┐
│  Sleep  │ ──────────> │Monitoring│ ─────────> │Calibrating │
└─────────┘              └──────────┘             └──────┬─────┘
     ▲                        ▲                          │
     │                        │                          │
     └────────────────────────┴──────────────────────────┘
              Button                    Auto (100 samples)
```

**States**:
- **Sleep**: Minimal activity, Neopixel off
- **Monitoring**: Read IMU at 10 Hz, Neopixel blue
- **Calibrating**: Collect 100 samples, Neopixel yellow, auto-return to Monitoring

**Transitions**:
- Button press cycles: Sleep → Monitoring → Calibrating → Sleep

## Extended CLI Commands

**From Lessons 02-03** (still available):
```
gpio.*, neo.*, stream.*
```

**New in Lesson 04**:
```
> imu.init              # Wake MPU6050 from sleep
> imu.whoami            # Read WHO_AM_I register (should be 0x68 or 0x71)
> imu.read              # Read accel/gyro data
> state.get             # Get current device state
> state.set <state>     # Force state (sleep/monitor/calib)
> stream.start          # Stream all data
```

## Example Session

```
> imu.init
OK [MPU6050 woken]

> imu.whoami
WHO_AM_I = 0x68

> imu.read
accel: x=245 y=-12 z=16384  gyro: x=3 y=-8 z=1

> state.set monitor
OK [State = Monitoring]

> stream.start
[state=Monitoring accel=(245,-12,16380) gyro=(3,-8,1) neo=(0,0,30) cal=0 cnt=1 t=1234]
[state=Monitoring accel=(246,-13,16381) gyro=(4,-7,2) neo=(0,0,30) cal=0 cnt=2 t=1334]
```

## GDB Debugging Workflows

### Conditional Breakpoint for I2C Errors

```gdb
# Break only when I2C fails
(gdb) break mpu::read_accel if $return.is_err()

# Or break in main when read fails
(gdb) break main.rs:XXX if i2c_result == Err
```

### State Machine Debugging

```gdb
# Watch state changes
(gdb) watch DEVICE_STATE

# Force state transition
(gdb) call set_device_state(1)  # 1 = Monitoring
(gdb) print DEVICE_STATE
$1 = Monitoring

# Observe state machine behavior
(gdb) continue
```

### Calibration Progress Monitoring

```gdb
# Watch calibration sample count
(gdb) watch CALIBRATION_SAMPLES

# Print all calibration variables
(gdb) print CALIBRATION_SAMPLES
(gdb) print cal_accel_x_sum
(gdb) print cal_accel_y_sum
(gdb) print cal_accel_z_sum
```

### I2C Register Inspection

```gdb
# Inspect I2C peripheral registers (base: 0x60013000)
(gdb) x/16x 0x60013000

# Check I2C status during communication
(gdb) break mpu::read_accel
(gdb) continue
(gdb) x/4xw 0x60013000  # I2C_CTR, I2C_STATUS, etc.
```

## Hardware Validation

After CLI commands, validate with GDB:

```
> imu.read
accel: x=245 y=-12 z=16384

# In GDB:
(gdb) print IMU_ACCEL_X
$1 = 245
(gdb) print IMU_ACCEL_Y
$2 = -12
(gdb) print IMU_ACCEL_Z
$3 = 16384
```

## Building and Running

```bash
cd lessons/04-mpu6050-state-machine
cargo build --release
cargo run --release
```

## Success Criteria

- [ ] I2C initializes at 100 kHz
- [ ] MPU6050 WHO_AM_I reads 0x68 or 0x71
- [ ] Accelerometer and gyroscope data reads correctly
- [ ] Button cycles through states
- [ ] State machine transitions work
- [ ] Calibration collects 100 samples and auto-returns to Monitoring
- [ ] CLI commands work: `imu.*`, `state.*`
- [ ] Streaming telemetry includes all sensor data
- [ ] GDB can inspect I2C registers and force state transitions

## Intentional Bugs (For Future GDB Exercises)

This lesson can be extended with intentional bugs for debugging practice:

1. **I2C Frequency Too High**: Set to 1 MHz → MPU6050 doesn't respond
2. **Axis Swap**: Read registers in wrong order → X/Y/Z swapped
3. **Calibration Overflow**: Use i16 accumulator → overflows at ~32767
4. **Missing I2C Timeout**: No timeout recovery → hangs on bus errors
5. **State Machine Bug**: Missing transition Calibrating → Monitoring

## Next Steps

**Lesson 05** will integrate everything into a complete posture monitor device with tilt detection, alert thresholds, and advanced debugging workflows!

---

**Status**: ✅ Complete and buildable
**Build**: Compiles without errors (5 warnings)
