//! # Blinky - Lesson 01
//!
//! The simplest possible ESP32-C6 firmware using esp-hal 1.0.0.
//! Blinks an LED every second to demonstrate basic GPIO control.
//!
//! ## Hardware
//! - ESP32-C6 development board
//! - Onboard LED on GPIO8
//!
//! ## What You'll Learn
//! - Basic esp-hal 1.0.0 initialization
//! - GPIO output configuration
//! - Delay timing
//! - Logging for debugging

#![no_std]
#![no_main]

// ============================================================================
// IMPORTS
// ============================================================================

use esp_backtrace as _;  // Panic handler - prints backtrace on crash
use esp_hal::{
    delay::Delay,                           // Blocking delay provider
    gpio::{Level, Output, OutputConfig},    // GPIO types
    main,                                    // Entry point macro
};
use log::info;  // Logging macros

// Required for proper linker symbols - don't worry about this for now
esp_bootloader_esp_idf::esp_app_desc!();

// ============================================================================
// MAIN ENTRY POINT
// ============================================================================

/// Main firmware entry point
///
/// The #[main] attribute marks this as the entry point.
/// The `-> !` means this function never returns (runs forever).
#[main]
fn main() -> ! {
    // Step 1: Initialize logging
    // This must be done first so we can see debug output
    esp_println::logger::init_logger_from_env();
    info!("🚀 Starting Blinky (Lesson 01)");

    // Step 2: Initialize the hardware abstraction layer
    // This gives us access to all ESP32-C6 peripherals
    let peripherals = esp_hal::init(esp_hal::Config::default());
    info!("✓ HAL initialized");

    // Step 3: Configure GPIO8 as an output
    // - peripherals.GPIO8: The specific pin we want to use
    // - Level::Low: Start with LED off
    // - OutputConfig::default(): Standard push-pull output
    let mut led = Output::new(
        peripherals.GPIO8,
        Level::Low,
        OutputConfig::default()
    );
    info!("✓ GPIO8 configured as output");

    // Step 4: Create a delay provider
    // Used for blocking delays (simple but blocks other code)
    let delay = Delay::new();

    // Step 5: Main loop - blink forever!
    info!("💡 Entering blink loop...");
    loop {
        led.set_high();              // Turn LED ON
        delay.delay_millis(1000);    // Wait 1 second

        led.set_low();               // Turn LED OFF
        delay.delay_millis(1000);    // Wait 1 second
    }

    // Note: We never reach here because of the infinite loop
}

// ============================================================================
// LEARNING NOTES
// ============================================================================

// 🎓 Key Concepts:
//
// 1. #![no_std] - We don't use Rust's standard library (too big for embedded)
// 2. #![no_main] - We provide our own entry point, not the standard one
// 3. -> ! - The "never" type - function runs forever
// 4. loop {} - Infinite loop (required in embedded - can't return from main)
//
// 🔍 What's happening:
//
// 1. Panic handler (esp_backtrace) catches crashes and prints debug info
// 2. HAL init gives us safe access to hardware
// 3. GPIO Output controls a single pin
// 4. Delay blocks execution for specified time
// 5. Loop toggles LED state forever
//
// 💡 Best Practices Used:
//
// - Comprehensive comments for learning
// - Logging at key points
// - Descriptive variable names
// - Clear step-by-step progression
// - Error handling through types (no unwrap() needed!)
//
// 🎯 Next Steps:
//
// - Try changing the delay time
// - Try a different GPIO pin
// - Try using led.toggle() instead of set_high/set_low
// - Move on to Lesson 02 (async with Embassy)
