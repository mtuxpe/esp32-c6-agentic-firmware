# Test UART Pins on ESP32-C6

**Usage:** `/test-uart-pins <tx_pin> <rx_pin> [duration_seconds]`

**Examples:**
- `/test-uart-pins 23 15` - Test GPIO23 (TX), GPIO15 (RX) for 5 seconds (default)
- `/test-uart-pins 16 17 10` - Test GPIO16 (TX), GPIO17 (RX) for 10 seconds
- `/test-uart-pins 4 5 3` - Test GPIO4 (TX), GPIO5 (RX) for 3 seconds

## What This Command Does

This slash command automates the complete UART pin verification workflow on ESP32-C6 hardware:

1. **Auto-detects ESP32 devices** - Finds USB-JTAG and FTDI UART ports automatically
2. **Creates test firmware** - Generates minimal firmware with your specified pins
3. **Builds and flashes** - Compiles and uploads to hardware
4. **Monitors output** - Captures UART data for specified duration
5. **Reports results** - Shows success/failure with troubleshooting hints

## Requirements

Before running this command, ensure:

- ✅ ESP32-C6 connected via USB (for flashing via USB-JTAG)
- ✅ FTDI UART adapter connected to test GPIO pins
- ✅ Wiring is correct:
  - ESP32 TX pin → UART Adapter RX pin
  - ESP32 RX pin → UART Adapter TX pin
  - ESP32 GND → UART Adapter GND
- ✅ Python3 with pyserial installed (`pip3 install pyserial`)
- ✅ espflash and cargo installed

## Implementation

Execute the hardware pin testing script:

```bash
./scripts/test-uart-pins.sh {{arg1}} {{arg2}} {{arg3}}
```

**Important:** This command flashes firmware to hardware. Make sure you have:
- Physical ESP32-C6 device connected
- UART adapter properly wired to the pins you're testing
- No other processes using the serial ports

## Expected Output

### Success Case (Pins Working):
```
✓ SUCCESS: GPIO23 (TX) and GPIO15 (RX) are working!

You can now use these pins in your firmware:
  .with_tx(peripherals.GPIO23)
  .with_rx(peripherals.GPIO15)
```

### Failure Case (Pins Not Working):
```
✗ FAILURE: No UART data received

Troubleshooting:
  1. Verify wiring:
     ESP32 GPIO16 (TX) → UART Adapter RX
     ESP32 GPIO17 (RX) → UART Adapter TX
     ESP32 GND → UART Adapter GND

  2. Check UART adapter is on: /dev/cu.usbserial-FT58PFX4

  3. Try different GPIO pins (common pairs: 16/17, 23/15, 4/5)
```

## Common ESP32-C6 UART Pin Pairs

| TX Pin | RX Pin | Notes |
|--------|--------|-------|
| GPIO23 | GPIO15 | Alternate UART1 (commonly used) |
| GPIO16 | GPIO17 | Default UART1 |
| GPIO4 | GPIO5 | Another option |

## Troubleshooting

### "ERROR: ESP32 USB-JTAG not found"
- Check ESP32 is connected via USB
- Try unplugging and replugging
- Run `ls /dev/cu.usbmodem*` (macOS) or `ls /dev/ttyACM*` (Linux)

### "ERROR: FTDI UART not found"
- Check UART adapter is connected
- Run `ls /dev/cu.usbserial*` (macOS) or `ls /dev/ttyUSB*` (Linux)

### "Build failed"
- Ensure cargo and rust toolchain are installed
- Check you're in the project directory
- Try `cargo clean` then re-run

### "No UART data received" but pins look correct
- Double-check TX/RX orientation (ESP32 TX → Adapter RX)
- Verify GND connection
- Try swapping TX and RX pins
- Test with known-working pins first (23/15)

## Related Commands

- `/test-lesson 08` - Full Lesson 08 testing (includes UART tests)
- `scripts/find-esp32-ports.sh` - Just detect ports (no testing)

## Notes

- This command creates a temporary project in /tmp, builds firmware, flashes it, monitors output, then cleans up
- The test duration defaults to 5 seconds if not specified
- Exit code 0 = success, 1 = failure, 2 = partial success (data received but wrong format)
- All temporary files are automatically cleaned up after the test
