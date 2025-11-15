# Lesson 05: Posture Monitor Device (Full Integration)

**Status**: Placeholder - To be implemented
**Duration**: 300-420 minutes
**Complexity**: ⭐⭐⭐⭐⭐
**Hardware**: All peripherals (LED, Neopixel, Button, MPU6050, UART)

## Overview

This lesson integrates all previous lessons into a complete, functional posture/orientation monitoring device.

## Device Specification

**Name**: Posture/Orientation Monitor
**Purpose**: Alert user when device tilts beyond safe angle

**Behavior**:
- **Normal** (0-30° tilt): Neopixel green, LED off
- **Warning** (30-60° tilt): Neopixel yellow, LED slow blink (1 Hz)
- **Alert** (>60° tilt): Neopixel red, LED fast blink (5 Hz)
- **Button short press**: Calibrate "zero" orientation
- **Button long press** (3s): Enter sleep mode
- **Sleep + button press**: Wake → Calibrating → Monitoring

## Complete CLI

**All commands from Lessons 02-04** plus:
```
device.start              # Start posture monitor
device.cal_zero           # Calibrate zero orientation
device.sleep              # Enter sleep mode
device.wake               # Wake from sleep
device.status             # Show device state
```

## Advanced GDB Debugging

### Planned Techniques
1. **Watchpoints for race conditions** (Button ISR vs main loop)
2. **Tracepoints for power profiling** (verify sleep mode reduces I2C calls)
3. **Statistical anomaly detection** (analyze 10,000 telemetry samples for patterns)
4. **Python GDB scripting** (automate state transition testing)

### Intentional Bugs
1. Tilt calculation uses wrong axis
2. State machine missing backward transitions
3. Calibration offsets not applied to raw values
4. Sleep mode doesn't reduce power (I2C still polling)
5. Race condition between button ISR and main loop
6. I2C errors spike during button presses (ISR disables interrupts too long)

## Implementation Status

This lesson is planned but not yet implemented. It represents the culmination of the curriculum where all learned skills are applied to build a complete, useful device.

**To implement**:
1. Extend Lesson 04 with device-level logic
2. Implement nested state machine (device states + alert states)
3. Add tilt calculation and threshold detection
4. Implement button short/long press detection
5. Add LED blink patterns for different states
6. Create comprehensive testing suite
7. Add Python GDB automation scripts
8. Document all debugging workflows

## Success Criteria

By the end of this lesson:
- Complete working posture monitor device
- CLI with 20+ commands across all peripherals
- Hardware-based unit testing for all subsystems
- Mastery of advanced GDB techniques
- Understanding of real-world embedded firmware patterns

## References

- All previous lessons (01-04)
- WS2812 timing specifications
- ESP32-C6 power management
- Embedded state machine patterns

---

**This lesson will be fully implemented in a future curriculum revision.**

**Note**: The CLI framework built in Lessons 02-03 provides the foundation. All peripheral drivers can be tested interactively without reflashing!
