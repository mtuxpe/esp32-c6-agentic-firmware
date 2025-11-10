# ESP32-C6 Agentic Firmware - Quick Start Guide

## ✅ Status: WORKING!

The blinky firmware successfully builds and flashes to ESP32-C6!

## Hardware Detected
- **Chip**: ESP32-C6 v0.1
- **Flash**: 8MB
- **Features**: WiFi 6, BT 5
- **MAC**: f0:f5:bd:01:88:2c
- **Port**: /dev/cu.usbserial-10

## Quick Commands

### Build Firmware
```bash
cd ~/Desktop/esp32-c6-agentic-firmware/lessons/01-blinky
cargo build --release
```

### Flash to ESP32-C6
```bash
cargo run --release
# OR manually specify port:
espflash flash --monitor target/riscv32imac-unknown-none-elf/release/blinky --port /dev/cu.usbserial-10
```

### Monitor Serial Output Only
```bash
espflash monitor /dev/cu.usbserial-10
```

## What Works Now

✅ **Build System**: Compiles successfully with esp-hal 1.0.0
✅ **Flashing**: Successfully flashes to ESP32-C6
✅ **Dependencies**: All version conflicts resolved
✅ **Linker**: Proper linker script configuration
✅ **Logging**: esp-println with log macros

## Binary Info
- **Size**: 1.0M (34,944 bytes app)
- **Target**: riscv32imac-unknown-none-elf
- **Features**: WiFi 6, Bluetooth 5 capable (not yet used)

## Key Dependencies (Lesson 01)
```toml
esp-hal = { version = "1.0.0", features = ["esp32c6", "rt", "unstable", "log-04"] }
esp-backtrace = { version = "0.18", features = ["esp32c6", "panic-handler", "println"] }
esp-bootloader-esp-idf = { version = "0.4", features = ["esp32c6"] }
esp-println = { version = "0.16", features = ["esp32c6", "log-04"] }
```

## Expected Behavior

When flashed, the firmware should:
1. Initialize peripherals
2. Configure GPIO8 as output
3. Blink LED every 1 second
4. Output log messages via serial:
   - "Starting Blinky LED Example..."
   - "LED ON - iteration N"
   - "LED OFF - iteration N"
   - Milestone logs every 10 iterations

## Next Steps

1. **Verify LED Blinking**: Check if GPIO8 LED is blinking on your board
2. **Monitor Serial Output**: Run `espflash monitor /dev/cu.usbserial-10` to see logs
3. **Try Different GPIOs**: Modify main.rs to use different pins
4. **Add More Lessons**: State machines, sensors, WiFi, etc.

## Troubleshooting

### Build Fails
- Ensure Rust stable toolchain is installed
- Run `rustup target add riscv32imac-unknown-none-elf`
- Check that all dependencies are at correct versions

### Flash Fails
- Verify USB cable is connected
- Check that port `/dev/cu.usbserial-10` exists: `ls /dev/cu.*`
- Try holding BOOT button while plugging in USB

### No Serial Output
- Increase ESP_LOG_LEVEL: `export ESP_LOG_LEVEL=debug`
- Check serial port permissions
- Try different terminal/baud rate

## Resources

- **GitHub Repo**: https://github.com/shanemmattner/esp32-c6-agentic-firmware
- **esp-hal Docs**: https://docs.esp-rs.org/esp-hal/
- **esp-hal Examples**: Check `submodules/esp-hal/examples/`
- **Rust ESP Book**: https://docs.esp-rs.org/book/

---

*Generated with Claude Code on 2025-11-09*
