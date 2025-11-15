#!/bin/bash
# Wrapper for espflash that works without TTY
# Usage: flash_no_tty.sh <binary>

BINARY="$1"

# Try to auto-detect ESP32 port
ESP_PORT=$(ls /dev/cu.usbmodem* 2>/dev/null | head -1)
if [ -z "$ESP_PORT" ]; then
    ESP_PORT=$(ls /dev/ttyACM* 2>/dev/null | head -1)
fi

if [ -z "$ESP_PORT" ]; then
    echo "Error: No ESP32 USB-JTAG port found"
    echo "Looked for: /dev/cu.usbmodem* or /dev/ttyACM*"
    exit 1
fi

echo "Flashing $BINARY to $ESP_PORT..."
espflash flash --chip esp32c6 --port "$ESP_PORT" "$BINARY"
