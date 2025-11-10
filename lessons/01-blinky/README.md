# Lesson 01: Blinky

## Overview
The classic "Hello World" of embedded systems - blinking an LED. This lesson demonstrates:
- Basic GPIO output control
- Using the ESP-IDF HAL in Rust
- Logging statements for debugging
- Delay/timing functionality

## Hardware Requirements
- ESP32-C6 development board
- USB-C cable for programming and power
- Onboard LED (GPIO8 on most ESP32-C6 DevKits)

## Learning Objectives
1. Initialize the ESP32-C6 with `esp-idf-svc`
2. Configure a GPIO pin as output
3. Toggle GPIO state in a loop
4. Use logging for runtime feedback
5. Understand the basic firmware structure

## Code Structure

### Initialization
```rust
esp_idf_svc::sys::link_patches();
esp_idf_svc::log::EspLogger::initialize_default();
```
These lines are required for proper ESP-IDF integration.

### GPIO Configuration
```rust
let peripherals = Peripherals::take().unwrap();
let mut led = PinDriver::output(peripherals.pins.gpio8).unwrap();
```
- `Peripherals::take()` gives us access to all hardware peripherals
- `PinDriver::output()` configures GPIO8 as an output

### Main Loop
```rust
loop {
    led.set_high().unwrap();  // Turn LED on
    FreeRtos::delay_ms(1000); // Wait 1 second
    led.set_low().unwrap();   // Turn LED off
    FreeRtos::delay_ms(1000); // Wait 1 second
}
```

## Building and Flashing

### Prerequisites
Install the Rust ESP toolchain:
```bash
cargo install espup
espup install
. $HOME/export-esp.sh
```

### Build
```bash
cd lessons/01-blinky
cargo build --release
```

### Flash to ESP32-C6
```bash
cargo run --release
```

Or specify the port manually:
```bash
espflash flash target/riscv32imc-esp-espidf/release/blinky --port /dev/cu.usbserial-10
```

## Expected Behavior
- Onboard LED blinks on/off with 1-second intervals
- Serial monitor shows log messages:
  ```
  Starting Blinky LED Example for ESP32-C6
  LED pin configured on GPIO8
  LED ON
  LED OFF
  LED ON
  LED OFF
  ...
  ```

## Monitoring Serial Output
```bash
espflash monitor /dev/cu.usbserial-10
```

## Common Issues

### Wrong GPIO Pin
If the LED doesn't blink, check your board's schematic. Common LED pins:
- ESP32-C6-DevKitC-1: GPIO8
- Custom boards: Check schematic

### Build Errors
- Ensure `espup` is installed and sourced
- Verify Rust version >= 1.77
- Check that `esp-idf-svc` dependencies are up to date

## Next Steps
- Try changing the blink interval
- Add more logging statements
- Experiment with different GPIO pins
- Move on to Lesson 02: Button Input

## Resources
- [ESP32-C6 Technical Reference Manual](https://www.espressif.com/sites/default/files/documentation/esp32-c6_technical_reference_manual_en.pdf)
- [esp-idf-svc Documentation](https://docs.esp-rs.org/esp-idf-svc/)
- [Rust Embedded Book](https://rust-embedded.github.io/book/)
