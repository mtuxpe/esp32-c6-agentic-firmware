#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Level, Output, OutputConfig},
    main,
};
use log::info;

#[main]
fn main() -> ! {
    // Initialize ESP-HAL with default config
    let peripherals = esp_hal::init(esp_hal::Config::default());

    // Initialize logging
    esp_println::logger::init_logger_from_env();

    info!("Starting Blinky LED Example for ESP32-C6 with esp-hal 1.0.0");

    // Create a delay provider
    let delay = Delay::new();

    // Configure GPIO8 as output (onboard LED on most ESP32-C6 devkits)
    let mut led = Output::new(peripherals.GPIO8, Level::Low, OutputConfig::default());

    info!("LED pin configured on GPIO8");

    // Blink loop
    let mut counter = 0u32;
    loop {
        info!("LED ON - iteration {}", counter);
        led.set_high();
        delay.delay_millis(1000);

        info!("LED OFF - iteration {}", counter);
        led.set_low();
        delay.delay_millis(1000);

        counter += 1;
    }
}
