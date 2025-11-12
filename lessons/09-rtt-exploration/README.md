# Lesson 09: RTT Multi-Channel Exploration

Real-Time Transfer (RTT) for non-blocking debugging via JTAG ring buffers.

## What is RTT?

RTT is a non-blocking debugging protocol using in-memory ring buffers to communicate between host and device via JTAG. Unlike UART (14 KB/s), RTT achieves 1-10 MB/s throughput without blocking firmware execution.

**Key properties:**
- Non-blocking - Device never waits for host
- Multi-channel - Separate channels for different data types
- Bidirectional - Host ↔ device communication
- High throughput - 1-10 MB/s via JTAG
- No GPIO overhead - Uses internal USB-JTAG

**Typical channels:**
- Channel 0: Structured logging (defmt)
- Channel 1: Binary sensor data
- Channel 2: Memory probes
- Channel 3+: Host commands

## Hardware Setup

### Components
- ESP32-C6-WROOM DevKit
- MPU9250 9-DOF IMU (I2C)
- WS2812 NeoPixel LED
- Push button (active LOW)
- USB cable (for JTAG access)

### Wiring

| Component | GPIO | Notes |
|-----------|------|-------|
| Button | GPIO9 | Active LOW |
| NeoPixel | GPIO8 | Data line |
| MPU9250 SDA | GPIO2 | I2C data |
| MPU9250 SCL | GPIO11 | I2C clock |
| UART TX | GPIO15 | USB CDC |
| UART RX | GPIO23 | USB CDC |

## Quick Start

### 1. Prerequisites

```bash
# probe-rs (Rust-native debugger)
cargo install probe-rs --locked

# defmt tooling
cargo install defmt-print
```

### 2. Build & Flash

```bash
cd lessons/09-rtt-exploration
cargo build --release
cargo run --release  # Flash to ESP32-C6
```

### 3. Monitor RTT Output

```bash
# With probe-rs running firmware
probe-rs run --chip esp32c6 target/riscv32imac-unknown-none-elf/release/main
```

Firmware will output structured logs via RTT.

## What You'll Learn

1. **Multi-channel RTT setup** - Using defmt with multiple channels
2. **Autonomous debugging patterns** - How Claude Code uses RTT for self-correction
3. **Memory probing** - Inspecting variables without adding code
4. **Event counters** - Non-blocking high-frequency event tracking
5. **RTT bandwidth planning** - When RTT saturates and how to optimize

## Checkpoints

- **C4:** Multi-channel RTT setup with defmt + structured types
- **C5:** ADS1015 ADC driver (test-driven, RTT-observable)
- **C6:** Python RTT explorer tool for autonomous debugging
- **C7:** Sweep testing (sample rates, variable counts, performance limits)

## Development Workflow

RTT enables iterative autonomous development:

```
Claude Code generates firmware → Flash via probe-rs →
Observe RTT output + memory probes → Analyze behavior →
Identify issue + generate fix → Repeat
```

Unlike UART (blocking), RTT never blocks firmware, giving accurate feedback for autonomous debugging.

## Key Difference: RTT vs UART

| Aspect | UART | RTT |
|--------|------|-----|
| Throughput | 14-250 KB/s | 1-10 MB/s |
| Blocking | Yes | No |
| GPIO used | Yes | No (JTAG) |
| Best for | Production | Development/Autonomous |

For lessons 8-9, RTT is ideal because firmware never blocks, enabling clean autonomous debugging.

## References

- [esp-hal RTT Documentation](https://docs.esp-rs.org/esp-hal/latest/esp_hal_rtt/index.html)
- [defmt Docs](https://defmt.ferrous-systems.com/)
- [probe-rs Guide](https://probe.rs/)
- [ESP32-C6 Technical Reference](https://www.espressif.com/sites/default/files/documentation/esp32-c6_technical_reference_manual_en.pdf)

See `CLAUDE.md` for embedded debugging strategies and RTT bandwidth planning.

---

**Ready to explore autonomous debugging with RTT!** 🎯
