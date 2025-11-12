# Data-Driven ADS1015 Driver Development

Using RTT telemetry to catch mistakes instantly and get the driver working fast.

## Philosophy

Instead of: "Write driver → test → debug incrementally"
Use: "Write driver with maximum observability → mistakes reveal themselves immediately"

## Key Variables to Track

### I2C Communication Layer

**Why:** Most ADS1015 problems are I2C-related (bus errors, timeouts, address conflicts)

```rust
// Track every I2C transaction
struct I2CStats {
    write_attempts: u32,
    write_success: u32,
    write_failures: u32,
    read_attempts: u32,
    read_success: u32,
    read_failures: u32,
    timeout_count: u32,
    ack_errors: u32,
    last_error_code: u32,
}
```

**Log every 100ms:**
```rust
defmt::info!("i2c: writes={}/{} reads={}/{} timeouts={} acks={}",
    i2c_stats.write_success, i2c_stats.write_attempts,
    i2c_stats.read_success, i2c_stats.read_attempts,
    i2c_stats.timeout_count, i2c_stats.ack_errors
);
```

**What you'll see:**
- `writes=0/5` → I2C write is completely failing (address wrong? pull-ups missing?)
- `reads=0/5` → Device isn't responding (powered? correct address?)
- `acks=5` → Device acknowledged but data corrupted (bit timing? clock stretching?)

### Hardware Configuration

**Why:** Most bugs are in register writes (wrong address, wrong bit fields)

```rust
// Track what we're actually writing to ADS1015
struct ADS1015Config {
    // Config register (0x01) bits that matter
    pga_gain: u8,        // Programmable gain (0-7)
    mux_channel: u8,     // Input multiplexer (0-7)
    os_status: u8,       // Operational status bit
    dr_rate: u8,         // Data rate (0-7)

    // What we think it is vs what hardware says
    config_written: u16,
    config_readback: u16,
}
```

**Log every time we configure:**
```rust
defmt::info!("ads1015_config: written=0x{:04x} readback=0x{:04x} match={}",
    config.config_written, config.config_readback,
    config.config_written == config.config_readback
);
```

**What you'll see:**
- `match=0` → Register write failed (I2C error? wrong address?)
- `written=0xXXXX readback=0xYYYY` → Device modified the bits we wrote (did we set reserved bits?)

### Conversion Results

**Why:** Know if readings are stuck, oscillating, or correct**

```rust
struct ADS1015Reading {
    raw_value: i16,
    volts: f32,

    // Track stuckness
    last_value: i16,
    same_count: u32,           // How many times in a row same value?

    // Track valid range
    min_seen: i16,
    max_seen: i16,
    range: i16,

    // Track conversion timing
    conversion_time_us: u32,
    ready_count: u32,          // How many times READY bit went high?
}
```

**Log every conversion:**
```rust
defmt::info!("adc: raw={} volts={} same_for={} range={}..{} ready_flag={}",
    reading.raw_value, reading.volts, reading.same_count,
    reading.min_seen, reading.max_seen, reading.ready_count
);
```

**What you'll see:**
- `raw=0 volts=0.0 same_for=1000` → Device stuck at 0 (not configured? wrong input selected?)
- `raw=32000 volts=4.096 same_for=1000` → Saturated (input shorted? wrong gain?)
- `ready_flag=0` → Conversion never completes (continuous mode? clock issue?)
- `range=32670..32767` → Reading entire MSB range = good data

### State Machine Tracking

**Why:** Catch logic errors early**

```rust
#[derive(Debug, Clone, Copy)]
enum ADS1015State {
    Uninitialized,
    Initializing,
    ConfigWritten,     // Sent config, waiting for readback
    ConfigVerified,    // Config matches
    Idle,
    ConversionInProgress,
    ResultReady,
    ResultRead,
}

struct ADS1015Debug {
    state: ADS1015State,
    state_changes: u32,
    time_in_state_ms: u32,
    state_stuck_count: u32,    // How many times we logged same state?
}
```

**Log every state change + every 100ms:**
```rust
defmt::info!("ads1015_state: current={:?} changed={} stuck_for={} time_ms={}",
    debug.state, debug.state_changes, debug.state_stuck_count, debug.time_in_state_ms
);
```

**What you'll see:**
- `stuck_for=1000` → State machine frozen (waiting for something that never happens?)
- `time_ms=5000` → Taking too long in one state (slow I2C? polling timeout too long?)
- Rapid state changes → State machine oscillating (logic error in transitions)

## Development Phases with Data Feedback

### Phase 1: I2C Communication
```rust
// Initialize I2C, write config register
// WATCH: Can we even talk to the chip?

defmt::info!("phase1: i2c_ok={} write_ok={} readback_ok={}",
    i2c_initialized, config_write_success, readback_matches
);
```

**Red flags:**
- `write_ok=0` → I2C peripheral not working
- `readback_ok=0` → Device not at address 0x48

### Phase 2: Configuration
```rust
// Set gain, channel, data rate
// WATCH: Do our settings stick?

defmt::info!("phase2: gain=0x{:02x} channel={} rate={} cfg_match={}",
    pga_gain, mux_channel, data_rate, config_matches
);
```

**Red flags:**
- `cfg_match=0` → Settings not persisting (I2C corruption? timing issue?)
- `gain=0xFF` → Read garbage (bad I2C read?)

### Phase 3: Conversions
```rust
// Enable single-shot, wait for ready
// WATCH: Does conversion complete?

defmt::info!("phase3: os_bit={} ready_bit={} retry_count={} result={}",
    os_bit_set, ready_bit_high, retries, raw_result
);
```

**Red flags:**
- `ready_bit=0 retry_count=1000` → Conversion never completes (wrong bit? polling wrong register?)
- `result=0` → Data is all zeros (input not connected? wrong channel selected?)

### Phase 4: Data Quality
```rust
// Read multiple samples
// WATCH: Do we get reasonable values?

defmt::info!("phase4: samples={} range={}..{} variance={} outliers={}",
    sample_count, min, max, variance, outlier_count
);
```

**Red flags:**
- `range=0..0` → All same value (noise floor is zero, input floating?)
- `variance=30000` → Extremely noisy (bad ground? noise on input?)
- `outliers=100` → Random spikes (I2C glitches?)

## Complete Logging Strategy for ADS1015

Log ALL of this every 100ms:

```rust
defmt::info!("ads: state={:?} i2c_err={} cfg_ok={} adc_val={} volts={:.2} ready={} same={}",
    state,              // Current state (Uninitialized/Configuring/Reading/etc)
    i2c_error_count,    // How many I2C failures so far?
    config_matches,     // Does readback match what we wrote?
    raw_adc_value,      // Raw ADC reading
    voltage_reading,    // Converted to volts
    ready_bit_high,     // Is conversion done?
    stuck_value_count   // How many times same value in a row?
);
```

## Example: Debugging a Stuck-at-Zero Problem

**Scenario:** ADC always reads 0, driver doesn't work

**With massive logging:**
```
ads: state=Reading i2c_err=0 cfg_ok=1 adc_val=0 volts=0.00 ready=1 same=100
→ State machine fine, I2C OK, config matches, but ADC stuck at 0

ads: state=Reading i2c_err=0 cfg_ok=1 adc_val=0 mux=0 gain=0 ready=1
→ Channel 0 is selected, gain is 0 (wait... gain=0 might be invalid!)

Check ADS1015 datasheet: Gain 0 = invalid! Must be 1-7
→ BUG FOUND: Default gain is out of range
→ FIX: Set gain to 1 (6.144V full scale)
```

**Time to diagnose:** ~30 seconds (you can SEE the gain is 0)

**Without logging:**
- Spend 30 minutes: "Why is ADC reading zero?"
- Debug I2C (it's fine)
- Check wiring (it's fine)
- Read datasheet again...
- Ah! Gain field might be wrong
- Spend 10 minutes confirming

**Time to diagnose:** ~40 minutes

## Variable Bandwidth Budget for ADS1015

```
Example message with 12 variables:
defmt::info!("ads: state={:?} i2c={} cfg={} val={} volts={:.2} ready={} same={}",
    7 variables...

defmt overhead: ~20 bytes
Per variable: ~5-10 bytes average
Total per message: ~70 bytes

Logging at 100 Hz: 70 bytes × 100 Hz = 7 KB/s
RTT capacity: 1-10 MB/s
Utilization: 0.07% of available bandwidth
```

We have TONS of headroom. Add more variables if needed!

## Summary: Data-Driven ADS1015 Development

**Instead of:** "Write driver, hope it works, debug when broken"
**Do:** "Write driver with comprehensive logging, watch it work in real-time, mistakes appear instantly"

**The variables reveal:**
1. **I2C health** → Can we even talk to the device?
2. **Configuration correctness** → Did our register writes work?
3. **Conversion status** → Is the ADC actually measuring?
4. **Data quality** → Are readings reasonable or garbage?
5. **State machine health** → Is logic flowing correctly?

With all this data streaming @ 100 Hz via RTT, bugs jump out at you.
You don't need to hypothesis-test; the data tells you what's wrong.

---

**Ready to build a robust ADS1015 driver with visibility!** 🔍
