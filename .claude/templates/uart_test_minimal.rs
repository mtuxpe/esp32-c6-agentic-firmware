//! Minimal UART Test Template
//!
//! This is a template for testing UART communication on ESP32-C6.
//! Copy to your lesson's src/bin/ directory and customize the GPIO pins.
//!
//! **Before using:**
//! 1. Replace GPIO23 and GPIO15 with your actual pins in the code below
//! 2. Verify your UART adapter is connected to these pins
//! 3. Build: `cargo build --bin uart_test_minimal`
//! 4. Flash: `espflash flash --port /dev/cu.usbmodem* target/.../uart_test_minimal`
//! 5. Monitor: `python3 read_uart.py /dev/cu.usbserial* 5`
//!
//! **Common ESP32-C6 UART pin combinations:**
//! - GPIO16 (TX) / GPIO17 (RX) - Default UART1
//! - GPIO23 (TX) / GPIO15 (RX) - Alternate UART1
//! - GPIO4 (TX) / GPIO5 (RX) - Another option
//!
//! **Pin selection guide:**
//! - ESP32 TX pin → connects to UART adapter RX pin
//! - ESP32 RX pin → connects to UART adapter TX pin

#![no_std]
#![no_main]

use core::fmt::Write;
use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    main,
    uart::{Config as UartConfig, Uart},
};
use esp_println::println;

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    println!("=== Minimal UART Test ===");

    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    // ========================================================================
    // CONFIGURE YOUR PINS HERE:
    // Replace GPIO23 and GPIO15 with your actual TX and RX pins
    // ========================================================================
    println!("Initializing UART on GPIO23 (TX), GPIO15 (RX)");

    let mut uart = Uart::new(peripherals.UART1, UartConfig::default())
        .expect("Failed to init UART")
        .with_tx(peripherals.GPIO23)  // ← Change this to your TX pin
        .with_rx(peripherals.GPIO15); // ← Change this to your RX pin
    // ========================================================================

    println!("UART initialized successfully!");
    println!("Starting transmission...");
    println!();

    let mut counter: u32 = 0;

    loop {
        // Send via UART using heapless String
        let mut msg = heapless::String::<64>::new();
        write!(&mut msg, "Hello from UART! Counter={}\n", counter).ok();
        uart.write(msg.as_bytes()).ok();

        // Also print to USB console for debugging
        println!("Sent: {}", counter);

        counter += 1;
        delay.delay_millis(1000);
    }
}
