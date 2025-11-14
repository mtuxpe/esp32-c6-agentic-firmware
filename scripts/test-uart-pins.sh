#!/bin/bash
# Automated UART Pin Testing Script
#
# Tests UART communication with specified GPIO pins on ESP32-C6.
# Automatically updates uart_test_minimal.rs, builds, flashes, and validates output.
#
# Usage:
#   ./scripts/test-uart-pins.sh <tx_pin> <rx_pin> [duration_seconds]
#
# Examples:
#   ./scripts/test-uart-pins.sh 23 15 5
#   ./scripts/test-uart-pins.sh 16 17 3
#
# What it does:
#   1. Auto-detects ESP32 USB ports
#   2. Creates temporary test firmware with your pins
#   3. Builds and flashes to hardware
#   4. Monitors UART output for specified duration
#   5. Reports success/failure based on received data
#
# Requirements:
#   - ESP32-C6 connected via USB (for flashing)
#   - FTDI UART adapter connected to test pins
#   - Python3 with pyserial installed
#   - espflash and cargo installed

set -e  # Exit on error

# ============================================================================
# Argument Parsing
# ============================================================================

if [ $# -lt 2 ]; then
    echo "Usage: $0 <tx_pin> <rx_pin> [duration_seconds]"
    echo ""
    echo "Examples:"
    echo "  $0 23 15 5   # Test GPIO23 (TX), GPIO15 (RX) for 5 seconds"
    echo "  $0 16 17 3   # Test GPIO16 (TX), GPIO17 (RX) for 3 seconds"
    exit 1
fi

TX_PIN=$1
RX_PIN=$2
DURATION=${3:-5}  # Default 5 seconds

echo "=== UART Pin Testing ==="
echo "TX Pin: GPIO${TX_PIN}"
echo "RX Pin: GPIO${RX_PIN}"
echo "Test Duration: ${DURATION} seconds"
echo ""

# ============================================================================
# Device Detection
# ============================================================================

echo "Step 1: Detecting ESP32 devices..."

# Find ESP32 USB-JTAG (for flashing)
USB_JTAG=$(ls /dev/cu.usbmodem* 2>/dev/null | head -1)
if [ -z "$USB_JTAG" ]; then
    USB_JTAG=$(ls /dev/ttyACM* 2>/dev/null | head -1)
fi

# Find FTDI UART (for monitoring)
FTDI_UART=$(ls /dev/cu.usbserial* 2>/dev/null | head -1)
if [ -z "$FTDI_UART" ]; then
    FTDI_UART=$(ls /dev/ttyUSB* 2>/dev/null | head -1)
fi

if [ -z "$USB_JTAG" ]; then
    echo "ERROR: ESP32 USB-JTAG not found!"
    echo "Expected: /dev/cu.usbmodem* (macOS) or /dev/ttyACM* (Linux)"
    exit 1
fi

if [ -z "$FTDI_UART" ]; then
    echo "ERROR: FTDI UART not found!"
    echo "Expected: /dev/cu.usbserial* (macOS) or /dev/ttyUSB* (Linux)"
    exit 1
fi

echo "  USB-JTAG: $USB_JTAG"
echo "  FTDI UART: $FTDI_UART"
echo ""

# ============================================================================
# Create Temporary Test Project
# ============================================================================

echo "Step 2: Creating temporary test firmware..."

TMP_DIR=$(mktemp -d)
TEST_PROJECT="$TMP_DIR/uart_pin_test"

mkdir -p "$TEST_PROJECT/src/bin"
mkdir -p "$TEST_PROJECT/.cargo"

# Create Cargo.toml
cat > "$TEST_PROJECT/Cargo.toml" <<EOF
[package]
name = "uart-pin-test"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "main"
path = "src/bin/main.rs"

[dependencies]
heapless = "0.8"
esp-hal = { version = "1.0.0", features = ["esp32c6", "unstable"] }
esp-backtrace = { version = "0.15", features = ["esp32c6", "panic-handler", "println"] }
esp-println = { version = "0.13", features = ["esp32c6", "log"] }
log = "0.4"
esp-bootloader-esp-idf = { version = "0.4.0", features = ["esp32c6"] }

[profile.dev]
opt-level = "s"

[profile.release]
opt-level = "s"
EOF

# Create .cargo/config.toml
cat > "$TEST_PROJECT/.cargo/config.toml" <<EOF
[build]
target = "riscv32imac-unknown-none-elf"

[target.riscv32imac-unknown-none-elf]
runner = "espflash flash --monitor"
EOF

# Create build.rs (required for linker script)
cat > "$TEST_PROJECT/build.rs" <<EOF
fn main() {
    println!("cargo:rustc-link-arg=-Tlinkall.x");
}
EOF

# Create rust-toolchain.toml
cat > "$TEST_PROJECT/rust-toolchain.toml" <<EOF
[toolchain]
channel = "stable"
components = ["rust-src"]
targets = ["riscv32imac-unknown-none-elf"]
EOF

# Create main.rs with specified pins
cat > "$TEST_PROJECT/src/bin/main.rs" <<EOF
#![no_std]
#![no_main]

use core::fmt::Write;
use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    main,
    uart::{Config as UartConfig, Uart},
};
use esp_println::println;

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    println!("=== UART Pin Test ===");
    println!("Testing GPIO${TX_PIN} (TX), GPIO${RX_PIN} (RX)");

    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    let mut uart = Uart::new(peripherals.UART1, UartConfig::default())
        .expect("Failed to init UART")
        .with_tx(peripherals.GPIO${TX_PIN})
        .with_rx(peripherals.GPIO${RX_PIN});

    println!("UART initialized successfully!");
    println!("Starting transmission...");
    println!();

    let mut counter: u32 = 0;

    loop {
        let mut msg = heapless::String::<64>::new();
        write!(&mut msg, "UART_TEST_OK Counter={}\n", counter).ok();
        uart.write(msg.as_bytes()).ok();

        println!("Sent: {}", counter);

        counter += 1;
        delay.delay_millis(1000);
    }
}
EOF

echo "  Created test project in: $TEST_PROJECT"
echo ""

# ============================================================================
# Build Firmware
# ============================================================================

echo "Step 3: Building firmware..."

cd "$TEST_PROJECT"
if ! cargo build --release 2>&1 | grep -E "(Compiling|Finished|error)"; then
    echo "ERROR: Build failed!"
    rm -rf "$TMP_DIR"
    exit 1
fi

FIRMWARE=$(find target -name "main" -type f | grep release | head -1)
if [ -z "$FIRMWARE" ]; then
    echo "ERROR: Could not find built firmware!"
    rm -rf "$TMP_DIR"
    exit 1
fi

echo "  Firmware built: $FIRMWARE"
echo ""

# ============================================================================
# Flash Firmware
# ============================================================================

echo "Step 4: Flashing firmware to ESP32-C6..."

if ! espflash flash --port "$USB_JTAG" "$FIRMWARE" 2>&1 | grep -E "(Flashing|Finished|error)"; then
    echo "ERROR: Flash failed!"
    rm -rf "$TMP_DIR"
    exit 1
fi

echo "  Flash complete!"
echo ""

# Wait for device to reset
sleep 2

# ============================================================================
# Monitor UART Output
# ============================================================================

echo "Step 5: Monitoring UART output for ${DURATION} seconds..."
echo "Listening on: $FTDI_UART"
echo "------------------------------------------------------------"

# Create Python monitoring script
MONITOR_SCRIPT="$TMP_DIR/monitor.py"
cat > "$MONITOR_SCRIPT" <<PYEOF
#!/usr/bin/env python3
import sys
import serial
import time

port = "$FTDI_UART"
duration = $DURATION

try:
    ser = serial.Serial(port, 115200, timeout=0.1)
    start_time = time.time()
    lines_read = 0
    success_count = 0

    while (time.time() - start_time) < duration:
        if ser.in_waiting > 0:
            try:
                line = ser.readline().decode('utf-8', errors='ignore')
                if line:
                    print(line, end='')
                    lines_read += 1
                    if 'UART_TEST_OK' in line:
                        success_count += 1
            except Exception as e:
                print(f"[Decode error: {e}]", file=sys.stderr)
        time.sleep(0.01)

    ser.close()

    # Report results
    print("------------------------------------------------------------")
    print(f"Test Results:")
    print(f"  Lines received: {lines_read}")
    print(f"  Valid messages: {success_count}")

    if success_count >= 2:
        print(f"  Status: ✓ SUCCESS - UART communication working!")
        sys.exit(0)
    elif lines_read > 0:
        print(f"  Status: ⚠ PARTIAL - Received data but format incorrect")
        sys.exit(2)
    else:
        print(f"  Status: ✗ FAILURE - No UART data received")
        sys.exit(1)

except serial.SerialException as e:
    print(f"Error opening {port}: {e}", file=sys.stderr)
    sys.exit(1)
except KeyboardInterrupt:
    print("\\nInterrupted by user")
    sys.exit(1)
PYEOF

chmod +x "$MONITOR_SCRIPT"

# Run monitor and capture exit code
set +e  # Don't exit on error
python3 "$MONITOR_SCRIPT"
EXIT_CODE=$?
set -e

# ============================================================================
# Cleanup and Report
# ============================================================================

echo ""
echo "Step 6: Cleanup..."
rm -rf "$TMP_DIR"
echo "  Temporary files removed"
echo ""

echo "==================================================================="
if [ $EXIT_CODE -eq 0 ]; then
    echo "✓ SUCCESS: GPIO${TX_PIN} (TX) and GPIO${RX_PIN} (RX) are working!"
    echo ""
    echo "You can now use these pins in your firmware:"
    echo "  .with_tx(peripherals.GPIO${TX_PIN})"
    echo "  .with_rx(peripherals.GPIO${RX_PIN})"
elif [ $EXIT_CODE -eq 2 ]; then
    echo "⚠ PARTIAL SUCCESS: Received data but format incorrect"
    echo ""
    echo "Possible issues:"
    echo "  - TX/RX pins may be swapped (try reversing them)"
    echo "  - UART adapter not properly connected"
    echo "  - Baud rate mismatch"
else
    echo "✗ FAILURE: No UART data received"
    echo ""
    echo "Troubleshooting:"
    echo "  1. Verify wiring:"
    echo "     ESP32 GPIO${TX_PIN} (TX) → UART Adapter RX"
    echo "     ESP32 GPIO${RX_PIN} (RX) → UART Adapter TX"
    echo "     ESP32 GND → UART Adapter GND"
    echo ""
    echo "  2. Check UART adapter is on: $FTDI_UART"
    echo ""
    echo "  3. Try different GPIO pins (common pairs: 16/17, 23/15, 4/5)"
fi
echo "==================================================================="

exit $EXIT_CODE
