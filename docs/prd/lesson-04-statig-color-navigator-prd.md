# Lesson 04: Statig State Machine - Color Navigator - PRD

## Overview
- **Lesson Number**: 04
- **Feature**: Interactive Color Navigator with State Machine
- **Duration**: 2-3 hours (including hardware validation)
- **Difficulty**: Intermediate
- **Prerequisites**: Lesson 02 (Task Scheduler, Atomics), Lesson 03 (MPU9250 IMU)

## Learning Objectives
What engineers will learn:
1. Using statig state machine library in no_std embedded Rust
2. Designing hierarchical state machines for interactive systems
3. Event-driven architecture with external event sources (button, IMU)
4. Combining multiple peripherals (button, IMU, NeoPixel) through state machine coordination
5. HSV to RGB color space conversion for smooth color transitions
6. Responsive IMU-based control with accelerometer data

## Hardware Requirements
- ESP32-C6 development board
- MPU9250 9-DOF IMU module (I2C)
- WS2812 NeoPixel LED
- Push button (active LOW)

**Pin Configuration**:
- GPIO9: Button input (pull-up, active LOW) - from Lesson 02
- GPIO8: NeoPixel data (RMT channel) - from Lesson 02
- GPIO2: I2C SDA (MPU9250) - from Lesson 03
- GPIO11: I2C SCL (MPU9250) - from Lesson 03

## Software Requirements
- esp-hal 1.0.0 features: gpio, rmt, i2c, delay
- Additional crates:
  - `statig = "0.3"` - Hierarchical state machine library
  - `smart-leds = "0.4"` - NeoPixel driver (from Lesson 02)
- Critical sections for shared state
- probe-rs for debugging (optional but recommended)

## Expected Behavior

### Serial Output Patterns (Reference Behavior)
This is the "oracle" - what success looks like:

```
🚀 Starting Lesson 04: Statig Color Navigator

✓ I2C initialized (GPIO2=SDA, GPIO11=SCL)
✓ MPU9250 awake
✓ WHO_AM_I: 0x71
✓ Button configured (GPIO9, active LOW)
✓ NeoPixel initialized (GPIO8)
✓ State machine initialized

🎨 State: RedBase
🔄 Starting interactive loop...

[User presses button]
🔘 Event: ButtonPressed
🎨 Transition: RedBase → GreenBase
💡 LED: HSV(120°, 100%, 100%) → RGB(0, 255, 0)

[User tilts board forward]
📊 IMU: accel_x=2000, accel_y=14000
🎨 Event: ImuUpdate
💡 LED: HSV(125°, 100%, 80%) → RGB(0, 204, 102)

[User presses button again]
🔘 Event: ButtonPressed
🎨 Transition: GreenBase → BlueBase
💡 LED: HSV(240°, 100%, 100%) → RGB(0, 0, 255)

[User tilts board right]
📊 IMU: accel_x=12000, accel_y=8000
🎨 Event: ImuUpdate
💡 LED: HSV(250°, 100%, 90%) → RGB(42, 0, 229)

[Continuous updates as board moves...]
```

**Critical patterns to verify**:
- ✅ Initialization messages (I2C, MPU9250, Button, NeoPixel, State Machine)
- ✅ State transitions on button press (RedBase ↔ GreenBase ↔ BlueBase)
- ✅ IMU updates showing accelerometer values
- ✅ LED color changes with HSV and RGB values logged
- ✅ Responsive tilt control (small tilts produce visible color changes)
- ❌ No ERROR or panic messages
- ❌ No blocking delays (scheduler keeps running)

### Register States to Verify (probe-rs inspection points)
When using probe-rs debugger, these registers should show:

1. **After peripheral initialization**:
   - `I2C0_CONF_REG` (0x60004000): I2C enabled
   - `GPIO_OUT_REG` (0x600A4004): GPIO8 controlled by RMT
   - `RMT_CH0CONF0_REG`: RMT channel configured for WS2812

2. **During operation**:
   - State machine enum value should cycle: 0 (RedBase) → 1 (GreenBase) → 2 (BlueBase)
   - Accelerometer values updating in memory (~100ms intervals)
   - NeoPixel color buffer updating based on tilt

## Functional Requirements

1. **REQ-1: State Machine with Three Color Base States**
   - State machine shall have three states: `RedBase`, `GreenBase`, `BlueBase`
   - Each state represents a base hue: Red (0°), Green (120°), Blue (240°)
   - States form a cycle: RedBase → GreenBase → BlueBase → RedBase

2. **REQ-2: Button Event Handling**
   - Button press shall generate `ButtonPressed` event
   - `ButtonPressed` event shall transition to next color base state
   - Debouncing from Lesson 02 shall prevent multiple transitions

3. **REQ-3: IMU Event Handling**
   - IMU task shall read accelerometer data every 100ms
   - Accelerometer X/Y values shall generate `ImuUpdate(x, y)` events
   - Each state shall handle `ImuUpdate` to adjust hue within ±30° of base hue

4. **REQ-4: Color Calculation**
   - X-axis tilt shall map to hue offset: -15° to +15° from base hue
   - Y-axis tilt shall map to brightness: 50% to 100%
   - HSV values shall be converted to RGB for NeoPixel
   - Very responsive: ±15° tilt produces noticeable changes
   - No dead zone: respond to all IMU changes immediately

5. **REQ-5: LED Output**
   - NeoPixel shall display current color from state machine
   - LED task shall update at 50ms intervals
   - Smooth color transitions as board tilts

6. **REQ-6: Task Scheduler Integration**
   - Reuse cooperative scheduler from Lesson 02
   - Button task: 10ms period
   - IMU task: 100ms period
   - LED task: 50ms period
   - State machine update: on-demand when events occur

## Technical Specifications

### Timing
- Button polling: 10ms (from Lesson 02)
- IMU reading: 100ms (from Lesson 03)
- LED update: 50ms (from Lesson 02)
- Debounce period: 200ms (from Lesson 02)
- State transition: <1ms (synchronous event handling)

### Color Mapping
**Tilt to Color Mapping**:
- **X-axis tilt** (left/right): Hue adjustment
  - Range: ±15° from base hue
  - Formula: `hue_offset = (accel_x / 16000.0) * 15.0`
  - Base hues: Red=0°, Green=120°, Blue=240°

- **Y-axis tilt** (forward/back): Brightness adjustment
  - Range: 50% to 100% brightness
  - Formula: `brightness = 50 + ((accel_y / 16000.0) * 50.0)`

**HSV to RGB Conversion**:
- Manual implementation (educational)
- Algorithm: Standard HSV→RGB conversion with sector-based calculation
- Input: H (0-360°), S (0-100%), V (0-100%)
- Output: R, G, B (0-255)

### Memory
- Flash usage estimate: ~60KB (with statig state machine)
- RAM usage estimate: ~8KB (state machine + peripheral drivers)
- No heap allocations (no_std, static state machine)

### Error Handling
- I2C communication failure: Log error, continue with last known orientation
- Button read failure: Log error, skip event
- State machine event handling: Always succeeds (events are handled or ignored)

## Implementation Plan

### Code Structure
```rust
// ============================================================================
// MODULES
// ============================================================================
mod button;      // Button task (from Lesson 02)
mod imu;         // IMU task (from Lesson 03)
mod led;         // LED task with color output
mod scheduler;   // Cooperative scheduler (from Lesson 02)
mod color;       // HSV→RGB conversion
mod state_machine; // Statig state machine

// ============================================================================
// STATE MACHINE
// ============================================================================

#[derive(Debug)]
pub enum Event {
    ButtonPressed,
    ImuUpdate { accel_x: i16, accel_y: i16 },
}

#[derive(Default)]
pub struct ColorNavigator;

#[state_machine(initial = "State::red_base()")]
impl ColorNavigator {
    #[state]
    fn red_base(event: &Event) -> Response<State> {
        match event {
            Event::ButtonPressed => Transition(State::green_base()),
            Event::ImuUpdate { accel_x, accel_y } => {
                // Calculate color: base_hue=0°, adjust by tilt
                update_led_color(0, *accel_x, *accel_y);
                Handled
            }
        }
    }

    #[state]
    fn green_base(event: &Event) -> Response<State> {
        match event {
            Event::ButtonPressed => Transition(State::blue_base()),
            Event::ImuUpdate { accel_x, accel_y } => {
                update_led_color(120, *accel_x, *accel_y);
                Handled
            }
        }
    }

    #[state]
    fn blue_base(event: &Event) -> Response<State> {
        match event {
            Event::ButtonPressed => Transition(State::red_base()),
            Event::ImuUpdate { accel_x, accel_y } => {
                update_led_color(240, *accel_x, *accel_y);
                Handled
            }
        }
    }
}

// ============================================================================
// COLOR CONVERSION
// ============================================================================

struct HsvColor {
    hue: u16,        // 0-360 degrees
    saturation: u8,  // 0-100 percent
    value: u8,       // 0-100 percent (brightness)
}

fn hsv_to_rgb(hsv: HsvColor) -> (u8, u8, u8) {
    // Manual HSV→RGB conversion
    // Educational implementation
}

// ============================================================================
// SHARED STATE (Atomic)
// ============================================================================

static CURRENT_COLOR: AtomicU32 = AtomicU32::new(0); // Packed RGB
```

### Key Implementation Points
1. Initialize all peripherals (I2C, Button, NeoPixel) as in previous lessons
2. Create statig state machine with `ColorNavigator::default().state_machine()`
3. Tasks generate events and submit to state machine via `sm.handle(&event)`
4. State handlers calculate colors and update atomic shared state
5. LED task reads atomic color state and updates NeoPixel
6. Add strategic logging:
   - Button presses and state transitions
   - IMU readings (every 10th sample to reduce spam)
   - Color calculations (HSV and RGB)
   - State machine events
7. Mark potential breakpoint locations in comments

### Logging Strategy
- **info!()**: State transitions, button events, major milestones
- **debug!()**: IMU readings (throttled), color calculations
- **warn!()**: I2C communication failures (recoverable)
- **error!()**: Peripheral initialization failures

## Testing Requirements (Mandatory)

### Unit Tests (`src/color.rs` or `tests/`)
- Test HSV→RGB conversion with known values:
  - Red: HSV(0°, 100%, 100%) → RGB(255, 0, 0)
  - Green: HSV(120°, 100%, 100%) → RGB(0, 255, 0)
  - Blue: HSV(240°, 100%, 100%) → RGB(0, 0, 255)
  - Half brightness: HSV(0°, 100%, 50%) → RGB(127, 0, 0)
- Test tilt-to-hue mapping calculations
- Test boundary conditions (min/max tilt values)

### Manual Hardware Tests
1. **Button cycling**: Press button, verify state transitions (Red→Green→Blue→Red)
2. **Tilt left/right**: Verify hue shifts within color family
3. **Tilt forward/back**: Verify brightness changes
4. **Combined tilt**: Verify both hue and brightness adjust smoothly
5. **Rapid button presses**: Verify debouncing works (no double-transitions)

**Note**: On-device tests with defmt-test are optional for this lesson (state machine behavior is easily verified manually).

## Success Criteria (All Mandatory)
- [ ] Code builds without warnings
- [ ] All unit tests pass (HSV→RGB conversion)
- [ ] Serial output matches expected patterns
- [ ] Button press cycles through three distinct base colors
- [ ] IMU tilt changes LED color smoothly (very responsive, no dead zone)
- [ ] No blocking delays (scheduler remains responsive)
- [ ] State machine transitions logged correctly
- [ ] User validates on real hardware

## Edge Cases

1. **Rapid button presses during IMU updates**
   - State machine handles events sequentially
   - Debouncing prevents multiple transitions
   - IMU updates continue in new state

2. **Extreme tilt values (board upside down)**
   - Clamp hue offset to ±30° from base
   - Clamp brightness to 0-100%
   - Prevent color wrapping outside valid range

3. **I2C communication failure (IMU disconnected)**
   - Log warning, continue with last known orientation
   - LED continues displaying last calculated color
   - System remains responsive to button presses

4. **Stationary board (no tilt)**
   - Display pure base color (hue_offset=0°, brightness=100%)
   - State machine remains in current state
   - System responsive to button and future tilts

## References
- [statig crate documentation](https://docs.rs/statig/latest/statig/)
- [statig GitHub repository](https://github.com/mdeloof/statig)
- [HSV color model (Wikipedia)](https://en.wikipedia.org/wiki/HSL_and_HSV)
- [MPU9250 Datasheet](https://invensense.tdk.com/wp-content/uploads/2015/02/MPU-9250-Datasheet.pdf)
- [esp-hal GPIO Module](https://docs.esp-rs.org/esp-hal/esp-hal/gpio/index.html)
- [esp-hal I2C Module](https://docs.esp-rs.org/esp-hal/esp-hal/i2c/index.html)
- [WS2812 Timing Specification](https://cdn-shop.adafruit.com/datasheets/WS2812.pdf)

---

**Status**: Draft
**Next Steps**: User review and approval before implementation
**Created**: 2025-11-10
