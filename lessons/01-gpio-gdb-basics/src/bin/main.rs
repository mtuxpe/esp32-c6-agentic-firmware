#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull},
    main,
};
use log::info;

esp_bootloader_esp_idf::esp_app_desc!();

// GPIO pin assignments
const LED_PIN: u8 = 12;
const BUTTON_PIN: u8 = 9;

// Debounce timing (milliseconds)
const DEBOUNCE_MS: u32 = 50;

#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();
    log::set_max_level(log::LevelFilter::Info);

    info!("\n=== Lesson 01: GPIO Basics + GDB Fundamentals ===\n");

    // Initialize peripherals
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    // Initialize LED (GPIO12) as output, starting LOW
    info!("[INIT] Configuring GPIO{} as output for LED...", LED_PIN);
    let mut led = Output::new(peripherals.GPIO12, Level::Low, OutputConfig::default());
    info!("[INIT] GPIO{} configured successfully", LED_PIN);

    // Initialize button (GPIO9) as input with pull-up
    // Button is active LOW (pressed = LOW, released = HIGH)
    info!("[INIT] Configuring GPIO{} as input for button (pull-up)...", BUTTON_PIN);
    let button = Input::new(
        peripherals.GPIO9,
        InputConfig::default().with_pull(Pull::Up),
    );
    info!("[INIT] GPIO{} configured successfully\n", BUTTON_PIN);

    // GDB Register Validation Checkpoint
    // At this point, GDB can inspect GPIO registers to confirm:
    // - GPIO_ENABLE_REG should have bit 12 set (LED output enabled)
    // - GPIO_FUNC_OUT_SEL_CFG_REG[12] should route to GPIO function
    // (gdb) x/16x 0x60004000  # Inspect GPIO registers

    info!("Ready! Press button to toggle LED.");
    info!("(Use GDB to inspect registers and call functions)\n");

    let mut led_state = false;
    let mut button_last_state = button.is_high();
    let mut button_press_count: u32 = 0;

    loop {
        let button_current = button.is_high();

        // Detect button press (transition from HIGH to LOW)
        // HIGH = released (pull-up), LOW = pressed
        if button_last_state && !button_current {
            // Debounce: simple delay-based approach
            delay.delay_millis(DEBOUNCE_MS);

            // Re-check button state after debounce
            if button.is_low() {
                button_press_count += 1;
                info!("[BUTTON] Press #{} detected!", button_press_count);

                // Toggle LED
                led_state = !led_state;
                if led_state {
                    led.set_high();
                    info!("[LED] Turned ON (GPIO{} = HIGH)", LED_PIN);
                } else {
                    led.set_low();
                    info!("[LED] Turned OFF (GPIO{} = LOW)\n", LED_PIN);
                }

                // GDB Validation Point
                // After toggle, use GDB to confirm GPIO_OUT_REG matches expected state:
                // (gdb) x/1xw 0x60004004  # Read GPIO_OUT_REG
                // Bit 12 should match led_state
            }
        }

        button_last_state = button_current;

        // Small delay to prevent busy-waiting
        delay.delay_millis(10);
    }
}

// LED control functions (callable from GDB)
// These functions demonstrate GDB's ability to call firmware functions interactively

#[no_mangle]
pub extern "C" fn led_on(gpio_out_reg: *mut u32) {
    // Manually set bit 12 in GPIO_OUT_REG
    // This simulates LED control at register level
    unsafe {
        let current = gpio_out_reg.read_volatile();
        gpio_out_reg.write_volatile(current | (1 << LED_PIN));
    }
    info!("[GDB] led_on() called - GPIO{} = HIGH", LED_PIN);
}

#[no_mangle]
pub extern "C" fn led_off(gpio_out_reg: *mut u32) {
    unsafe {
        let current = gpio_out_reg.read_volatile();
        gpio_out_reg.write_volatile(current & !(1 << LED_PIN));
    }
    info!("[GDB] led_off() called - GPIO{} = LOW", LED_PIN);
}

#[no_mangle]
pub extern "C" fn led_toggle(gpio_out_reg: *mut u32) {
    unsafe {
        let current = gpio_out_reg.read_volatile();
        gpio_out_reg.write_volatile(current ^ (1 << LED_PIN));
    }
    info!("[GDB] led_toggle() called - GPIO{} toggled", LED_PIN);
}

// GDB Usage Examples (documented in README):
//
// 1. Inspect GPIO registers:
//    (gdb) x/16x 0x60004000
//
// 2. Read GPIO_OUT_REG:
//    (gdb) x/1xw 0x60004004
//
// 3. Call LED functions from GDB:
//    (gdb) call led_on(0x60004004 as *mut u32)
//    (gdb) call led_off(0x60004004 as *mut u32)
//    (gdb) call led_toggle(0x60004004 as *mut u32)
//
// 4. Set breakpoint on button press:
//    (gdb) break main.rs:73
//
// 5. Modify LED state variable:
//    (gdb) set led_state = true
//
// 6. Continue execution:
//    (gdb) continue
