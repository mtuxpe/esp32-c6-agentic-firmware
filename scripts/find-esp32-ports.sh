#!/bin/bash
# Auto-detect ESP32-C6 USB-JTAG and FTDI UART ports
#
# Usage:
#   source scripts/find-esp32-ports.sh
#
# Exports:
#   USB_CDC_PORT - ESP32 USB-JTAG port (for flashing/debugging)
#   FTDI_PORT    - FTDI UART port (for data streaming)
#
# Example:
#   source scripts/find-esp32-ports.sh
#   espflash flash --port $USB_CDC_PORT target/.../main
#   python3 read_uart.py $FTDI_PORT 5

echo "=== ESP32 Device Detection ==="

# Find ESP32 USB-JTAG (usually has "usbmodem" on macOS, "ttyACM" on Linux)
USB_JTAG=$(ls /dev/cu.usbmodem* 2>/dev/null | head -1)
if [ -z "$USB_JTAG" ]; then
    USB_JTAG=$(ls /dev/ttyACM* 2>/dev/null | head -1)
fi

# Find FTDI UART (usually has "usbserial" on macOS, "ttyUSB" on Linux)
FTDI_UART=$(ls /dev/cu.usbserial* 2>/dev/null | head -1)
if [ -z "$FTDI_UART" ]; then
    FTDI_UART=$(ls /dev/ttyUSB* 2>/dev/null | head -1)
fi

# Export variables
export USB_CDC_PORT="$USB_JTAG"
export FTDI_PORT="$FTDI_UART"

# Display results
echo "USB-JTAG (for flashing/debugging): ${USB_CDC_PORT:-NOT FOUND}"
echo "FTDI UART (for data streaming):    ${FTDI_PORT:-NOT FOUND}"
echo ""

# Verify at least USB-JTAG was found
if [ -z "$USB_CDC_PORT" ]; then
    echo "ERROR: ESP32 USB-JTAG not found. Is device plugged in?"
    echo "Expected to find: /dev/cu.usbmodem* (macOS) or /dev/ttyACM* (Linux)"
    return 1 2>/dev/null || exit 1
fi

if [ -z "$FTDI_PORT" ]; then
    echo "WARNING: FTDI UART not found. UART streaming may not work."
    echo "Expected to find: /dev/cu.usbserial* (macOS) or /dev/ttyUSB* (Linux)"
fi

echo "Device discovery complete. Variables exported:"
echo "  \$USB_CDC_PORT = $USB_CDC_PORT"
echo "  \$FTDI_PORT = $FTDI_PORT"
