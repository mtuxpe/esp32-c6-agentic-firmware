# Lesson 04: MPU6050 + State Machine (Extend CLI)

**Status**: Placeholder - To be implemented
**Duration**: 240-300 minutes
**Complexity**: ⭐⭐⭐⭐☆
**Hardware**: ESP32-C6, MPU6050 IMU sensor, Button, LED, Neopixel, UART

## Overview

This lesson will extend the CLI from Lessons 02-03 with I2C sensor support and state machine patterns.

## Planned Features

### Hardware
- MPU6050 IMU sensor (I2C on GPIO2=SDA, GPIO11=SCL)
- Onboard button (GPIO9) for state transitions
- LED, Neopixel, UART from previous lessons

### Extended CLI Commands

**New commands**:
```
imu.init              # Initialize MPU6050
imu.whoami            # Read WHO_AM_I register
imu.read              # Read accel/gyro data
imu.cal               # Calibrate (collect 100 samples)
state.get             # Get current state
state.set <state>     # Force state transition
stream.start          # Stream all data
```

### State Machine
```
Sleep → Monitoring → Calibrating → Monitoring
```

### Intentional Bugs
1. I2C frequency too high (1 MHz, MPU6050 supports max 400 kHz)
2. Axis swap bug (tilting forward changes Z instead of Y)
3. Calibration overflow (i16 accumulator overflows)
4. Missing I2C timeout recovery

### GDB Techniques
- Conditional breakpoints: `break i2c_read if ret != I2C_OK`
- Tracepoints for performance profiling (measure I2C transaction rate)
- Watchpoints on state transitions
- Register inspection for I2C peripheral debugging

## Implementation Status

This lesson is planned but not yet implemented. The curriculum framework is complete through Lesson 03.

**To implement**:
1. Create Cargo project structure
2. Write MPU6050 I2C driver
3. Implement button-driven state machine
4. Add CLI commands for sensor access
5. Create comprehensive README with debugging workflows
6. Test on hardware

## References

- MPU6050 Datasheet: https://invensense.tdk.com/products/motion-tracking/6-axis/mpu-6050/
- ESP32-C6 I2C Chapter (Technical Reference Manual)
- esp-hal I2C documentation

---

**This lesson will be fully implemented in a future curriculum revision.**
