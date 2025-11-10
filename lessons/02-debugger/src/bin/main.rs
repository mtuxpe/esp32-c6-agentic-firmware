//! # Lesson 02: Debugger with probe-rs
//!
//! Debugging GPIO state using ESP32-C6's built-in USB-JTAG.
//! Same code as Lesson 01, but now we'll use the debugger to inspect it.
//!
//! **What We're Debugging:**
//! - Breakpoints in the blink loop
//! - GPIO register values (set vs cleared)
//! - Local variable changes (cycle counter)
//! - Call stack unwinding
//!
//! **Hardware:**
//! - ESP32-C6 with built-in USB-JTAG (GPIO 12/13)
//! - Same blinky LED on GPIO13 as Lesson 01

#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    gpio::{Input, InputConfig, Level, Output, OutputConfig},
    main,
};
use log::info;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
esp_bootloader_esp_idf::esp_app_desc!();

// ============================================================================
// PIN CONFIGURATION
// ============================================================================

const LED_PIN: u8 = 13;        // GPIO13 - LED output
const INPUT_PIN: u8 = 9;       // GPIO9 - Input (detects LED state)
const BLINK_DELAY_MS: u32 = 500;

// ============================================================================
// MAIN
// ============================================================================

#[main]
fn main() -> ! {
    // Initialize logging
    esp_println::logger::init_logger_from_env();
    log::set_max_level(log::LevelFilter::Info);

    info!("🚀 Starting Lesson 02: Debugger with probe-rs");
    info!("📍 Set breakpoints below to inspect GPIO state\n");

    // Initialize hardware
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    let delay = Delay::new();

    // Configure GPIO13 as output (LED)
    let mut led = Output::new(
        peripherals.GPIO13,
        Level::Low,
        OutputConfig::default(),
    );
    info!("✓ GPIO{} configured as output", LED_PIN);

    // Configure GPIO9 as input (detector)
    let input = Input::new(peripherals.GPIO9, InputConfig::default());
    info!("✓ GPIO{} configured as input", INPUT_PIN);

    info!("Starting GPIO demonstration...\n");

    // Demo: Turn LED ON and read input
    info!("--- GPIO Output Test ---");
    led.set_high();
    info!("Set GPIO13 HIGH");
    info!("  GPIO9 reads: {}", if input.is_high() { "HIGH" } else { "LOW" });
    delay.delay_millis(1000);

    // Demo: Turn LED OFF and read input
    led.set_low();
    info!("Set GPIO13 LOW");
    info!("  GPIO9 reads: {}", if input.is_high() { "HIGH" } else { "LOW" });
    delay.delay_millis(1000);

    info!("\n--- Blinking Loop ---");
    info!("(Set breakpoints in the loop below)\n");

    // Main loop: blink and show GPIO state
    // 📍 BREAKPOINT #1: After led.set_high()
    // 📍 BREAKPOINT #2: After led.set_low()
    // 📍 BREAKPOINT #3: Inside the loop to watch `cycle` increment
    let mut cycle = 0;
    loop {
        led.set_high();  // 📍 BREAKPOINT #1: Inspect GPIO registers after this
        info!("🔴 LED ON  → GPIO9: {}", if input.is_high() { "HIGH" } else { "LOW" });
        delay.delay_millis(BLINK_DELAY_MS);

        led.set_low();   // 📍 BREAKPOINT #2: Inspect GPIO registers after this
        info!("⚫ LED OFF → GPIO9: {}", if input.is_high() { "HIGH" } else { "LOW" });
        delay.delay_millis(BLINK_DELAY_MS);

        cycle += 1;  // 📍 BREAKPOINT #3: Watch this variable change
        if cycle % 10 == 0 {
            info!("  └─ {} cycles completed", cycle);
        }
    }
}
