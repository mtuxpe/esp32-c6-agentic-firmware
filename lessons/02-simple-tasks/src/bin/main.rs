//! # Lesson 02: Simple Task Scheduler
//!
//! Demonstrates a basic cooperative task scheduler.
//!
//! **Hardware:**
//! - ESP32-C6 development board
//! - Same as Lesson 01 (no additional hardware)
//!
//! **Pins:**
//! - GPIO13: Output (LED)
//! - GPIO9: Input (reads GPIO13 state)
//!
//! **What You'll Learn:**
//! - Build a simple cooperative task scheduler
//! - Run multiple tasks at different rates
//! - Organize code into modules

#![no_std]
#![no_main]

use esp_hal::{
    delay::Delay,
    gpio::{Input, InputConfig, Level, Output, OutputConfig},
    main,
};
use log::info;

use lesson_02_simple_tasks::scheduler::{Context, Task};
use lesson_02_simple_tasks::tasks::{blink_task, monitor_task};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

const LED_PIN: u8 = 13;
const INPUT_PIN: u8 = 9;

#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();
    log::set_max_level(log::LevelFilter::Info);

    info!("🚀 Starting Lesson 02: Simple Task Scheduler\n");

    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    // Configure GPIO
    let led = Output::new(peripherals.GPIO13, Level::Low, OutputConfig::default());
    info!("✓ GPIO{} configured as output", LED_PIN);

    let input = Input::new(peripherals.GPIO9, InputConfig::default());
    info!("✓ GPIO{} configured as input", INPUT_PIN);

    info!("✓ Task scheduler ready\n");

    // Create task list
    let mut tasks = [
        Task {
            run: blink_task,
            period_ms: 500,
            last_run: 0,
        },
        Task {
            run: monitor_task,
            period_ms: 100,
            last_run: 0,
        },
    ];

    let mut ctx = Context { led, input };

    info!("🔄 Starting task scheduler loop...\n");

    // Simple cooperative scheduler
    let mut current_time_ms: u64 = 0;
    const TICK_MS: u64 = 10;

    loop {
        delay.delay_millis(TICK_MS as u32);
        current_time_ms += TICK_MS;

        for task in &mut tasks {
            if task.should_run(current_time_ms) {
                task.execute(current_time_ms, &mut ctx);
            }
        }
    }
}
