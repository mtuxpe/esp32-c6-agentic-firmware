#!/bin/bash
# Remote build, flash, and monitor via SSH to RPi
# Usage: ./scripts/remote-build-flash.sh [rpi-host] [lesson] [port] [baud]
# Example: ./scripts/remote-build-flash.sh pi@raspberrypi.local 01-blinky /dev/ttyUSB0 115200

set -e

# Configuration with defaults
RPI_HOST="${1:-pi@raspberrypi.local}"
LESSON="${2:-01-blinky}"
RPI_PORT="${3:-/dev/ttyUSB0}"
RPI_BAUD="${4:-115200}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Helper functions
log_step() {
    echo -e "${GREEN}$1${NC}"
}

log_error() {
    echo -e "${RED}$1${NC}"
}

log_info() {
    echo -e "${YELLOW}$1${NC}"
}

# Print header
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
log_info "Remote Build → Flash → Monitor"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "RPi Host:      $RPI_HOST"
echo "Lesson:        $LESSON"
echo "Serial Port:   $RPI_PORT"
echo "Baud Rate:     $RPI_BAUD"
echo ""

# Step 1: Build locally
log_step "🔨 Building lesson locally..."
if [ ! -d "lessons/$LESSON" ]; then
    log_error "✗ Lesson directory not found: lessons/$LESSON"
    exit 1
fi

cd "lessons/$LESSON"
cargo build --release 2>&1 | grep -E "(Compiling|Finished|error)" || true

if [ ! -f "target/riscv32imac-unknown-none-elf/release/$LESSON" ]; then
    log_error "✗ Build failed or binary not found"
    exit 1
fi

BINARY="target/riscv32imac-unknown-none-elf/release/$LESSON"
BINARY_SIZE=$(stat -f%z "$BINARY" 2>/dev/null || stat -c%s "$BINARY" 2>/dev/null)
log_info "  Binary size: $(echo $BINARY_SIZE | awk '{printf "%.1f", $1/1024}') KB"
cd ../..

# Step 2: Copy binary to RPi
log_step "📦 Copying binary to RPi..."
scp -q "lessons/$LESSON/$BINARY" "$RPI_HOST:/tmp/$LESSON" 2>/dev/null
log_info "  Copied to /tmp/$LESSON"

# Step 3: Flash on RPi
log_step "⚡ Flashing on RPi..."
FLASH_OUTPUT=$(ssh "$RPI_HOST" "espflash flash /tmp/$LESSON --port $RPI_PORT" 2>&1)
if echo "$FLASH_OUTPUT" | grep -q "Flashing has completed"; then
    log_info "  ✓ Flash successful"
else
    log_error "  ✗ Flash failed"
    echo "$FLASH_OUTPUT"
    exit 1
fi

# Step 4: Monitor output
log_step "👀 Monitoring serial output..."
log_info "  (Press Ctrl+C to stop)"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

ssh "$RPI_HOST" "python3 /home/pi/monitor.py --port $RPI_PORT --baud $RPI_BAUD"

# Cleanup
CLEANUP_EXIT_CODE=$?
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
log_step "✓ Done!"
echo ""

exit $CLEANUP_EXIT_CODE
