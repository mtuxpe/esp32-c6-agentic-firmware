//! Blinky LED Example for ESP32-C6
//!
//! Blinks an LED connected to GPIO8 every second with comprehensive logging.
//! This demonstrates basic GPIO output and delay functionality using esp-hal 1.0.0.

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Level, Output, OutputConfig},
    main,
};
use log::info;

// Required for proper linker symbols
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    // Initialize logging first
    esp_println::logger::init_logger_from_env();

    info!("Starting Blinky LED Example for ESP32-C6 with esp-hal 1.0.0");

    // Initialize ESP-HAL with default config
    let peripherals = esp_hal::init(esp_hal::Config::default());

    info!("Peripherals initialized successfully");

    // Configure GPIO8 as output (onboard LED on most ESP32-C6 devkits)
    // Start with LED off (Level::Low)
    let mut led = Output::new(peripherals.GPIO8, Level::Low, OutputConfig::default());

    info!("LED pin configured on GPIO8");

    // Create a delay provider
    let delay = Delay::new();

    info!("Entering main blink loop...");

    // Blink loop with iteration counter
    let mut counter = 0u32;
    loop {
        info!("LED ON - iteration {}", counter);
        led.set_high();
        delay.delay_millis(1000);

        info!("LED OFF - iteration {}", counter);
        led.set_low();
        delay.delay_millis(1000);

        counter += 1;

        // Log milestone every 10 iterations
        if counter % 10 == 0 {
            info!("Completed {} blink cycles", counter);
        }
    }
}
