# Setup Hardware Lesson

**Usage:** `/setup-hardware-lesson <lesson_number> <lesson_name>`

**Examples:**
- `/setup-hardware-lesson 10 uart-streaming` - Create Lesson 10: UART Streaming
- `/setup-hardware-lesson 11 i2c-sensors` - Create Lesson 11: I2C Sensors

## What This Command Does

Creates a new ESP32-C6 hardware lesson with all necessary boilerplate and templates:

1. **Creates lesson directory structure** - Standard layout matching existing lessons
2. **Copies hardware templates** - UART test template, Python reader, device discovery script
3. **Generates Cargo.toml** - Pre-configured with esp-hal 1.0.0 and common dependencies
4. **Sets up build files** - .cargo/config.toml, build.rs, rust-toolchain.toml
5. **Creates minimal README** - Basic structure with wiring, usage, and troubleshooting sections
6. **Adds UART test binary** - Minimal UART test for hardware verification

## Requirements

- Lesson number must not already exist
- Lesson name should be lowercase with hyphens (e.g., `uart-streaming`, `i2c-sensors`)

## Implementation

I'll create the new lesson structure using the following steps:

1. **Verify lesson doesn't exist:**
```bash
if [ -d "lessons/{{arg1}}-{{arg2}}" ]; then
    echo "ERROR: Lesson {{arg1}} already exists!"
    exit 1
fi
```

2. **Create directory structure:**
```bash
mkdir -p "lessons/{{arg1}}-{{arg2}}/src/bin"
mkdir -p "lessons/{{arg1}}-{{arg2}}/.cargo"
```

3. **Copy templates from .claude/templates/:**
- `uart_test_minimal.rs` → `src/bin/uart_test_minimal.rs`
- Note: `read_uart.py` and `find-esp32-ports.sh` are in global scripts/, no need to copy

4. **Generate Cargo.toml:**
```toml
[package]
name = "lesson-{{arg1}}-{{arg2}}"
version = "0.1.0"
edition = "2021"
rust-version = "1.88"

[[bin]]
name = "main"
path = "src/bin/main.rs"

[[bin]]
name = "uart_test_minimal"
path = "src/bin/uart_test_minimal.rs"

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
codegen-units = 1
debug = 2
debug-assertions = false
incremental = false
lto = 'fat'
opt-level = 's'
overflow-checks = false
```

5. **Create .cargo/config.toml:**
```toml
[build]
target = "riscv32imac-unknown-none-elf"

[target.riscv32imac-unknown-none-elf]
runner = "espflash flash --monitor"
```

6. **Create build.rs:**
```rust
fn main() {
    println!("cargo:rustc-link-arg=-Tlinkall.x");
}
```

7. **Create rust-toolchain.toml:**
```toml
[toolchain]
channel = "stable"
components = ["rust-src"]
targets = ["riscv32imac-unknown-none-elf"]
```

8. **Create minimal main.rs template:**
```rust
#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{delay::Delay, main};
use esp_println::println;

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    println!("=== Lesson {{arg1}}: {{arg2}} ===");

    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    // TODO: Implement lesson code here

    loop {
        println!("Hello from Lesson {{arg1}}!");
        delay.delay_millis(1000);
    }
}
```

9. **Create minimal README.md:**
```markdown
# Lesson {{arg1}}: {{arg2}}

Brief description of what this lesson teaches.

## Hardware Setup

**Required:**
- ESP32-C6 development board
- USB cable (for programming and power)

**Optional:**
- FTDI USB-to-UART adapter (for UART testing)
- Breadboard and jumper wires

## Wiring

(TODO: Add wiring diagram or table)

## Building and Flashing

\`\`\`bash
# Build the project
cargo build --release

# Flash to ESP32-C6
cargo run --release

# Or specify port manually
espflash flash --port /dev/cu.usbmodem* target/riscv32imac-unknown-none-elf/release/main
\`\`\`

## Testing UART (if applicable)

\`\`\`bash
# Test UART pins first
./scripts/test-uart-pins.sh 23 15 5

# Read UART output
python3 .claude/templates/read_uart.py /dev/cu.usbserial* 5
\`\`\`

## Expected Output

(TODO: Document expected serial output)

## Troubleshooting

### Build fails
- Check you're using rust stable toolchain
- Try \`cargo clean\` then rebuild

### Flash fails
- Verify ESP32-C6 is connected via USB
- Check USB port: \`ls /dev/cu.usbmodem*\` (macOS) or \`ls /dev/ttyACM*\` (Linux)

### No UART output
- Run \`./scripts/test-uart-pins.sh 23 15 5\` to verify pins
- Check wiring: ESP32 TX → Adapter RX, ESP32 RX → Adapter TX
- Verify GND connection

## References

- [esp-hal documentation](https://docs.esp-rs.org/esp-hal/)
- [ESP32-C6 datasheet](https://www.espressif.com/sites/default/files/documentation/esp32-c6_datasheet_en.pdf)
```

10. **Print success message with next steps:**
```
✓ Created Lesson {{arg1}}: {{arg2}}

Next steps:
1. cd lessons/{{arg1}}-{{arg2}}
2. Edit src/bin/main.rs to implement lesson code
3. Test UART pins: ../../scripts/test-uart-pins.sh 23 15 5
4. Build and flash: cargo run --release
5. Update README.md with wiring and expected output
```

## Output

You'll get a complete lesson directory ready for hardware development:

```
lessons/{{arg1}}-{{arg2}}/
├── src/
│   ├── bin/
│   │   ├── main.rs           # Main lesson code (template)
│   │   └── uart_test_minimal.rs  # UART pin testing
├── .cargo/
│   └── config.toml           # Build configuration
├── Cargo.toml                # Dependencies
├── build.rs                  # Linker script config
├── rust-toolchain.toml       # Rust toolchain
└── README.md                 # Documentation template
```

## Notes

- All templates use esp-hal 1.0.0 API patterns
- UART test binary included by default (can remove if not needed)
- Python reader and device discovery scripts are in global `scripts/` directory
- Follow "Lean Lessons Approach" - keep code simple, test on hardware early

## Related Commands

- `/test-uart-pins <tx> <rx>` - Test UART pins on hardware
- `/test-lesson <number>` - Run full lesson tests
- `scripts/find-esp32-ports.sh` - Detect USB ports
