//! Button input handling with edge detection and debouncing.
//!
//! This module reads button state and detects press events (LOW → HIGH transition).
//! When a press is detected, it toggles the shared LED_ENABLED atomic.

use crate::{toggle_led_enabled, DEBOUNCE_MS};
use esp_hal::{delay::Delay, gpio::Input};
use log::info;

/// Button state for edge detection
static mut BUTTON_WAS_PRESSED: bool = false;

/// Button task: Read button state and update shared LED state
///
/// This function should be called periodically by the scheduler.
/// It detects button press events (transition from released to pressed)
/// and toggles the LED state atomically.
///
/// # Arguments
/// * `button` - Reference to the GPIO input pin
/// * `delay` - Reference to delay peripheral for debouncing
pub fn button_task(button: &Input, delay: &Delay) {
    let button_pressed = button.is_low();

    // Detect button press (transition to LOW, since button is active LOW)
    unsafe {
        if button_pressed && !BUTTON_WAS_PRESSED {
            info!("📍 [button_task] Button press detected!");

            // Toggle LED state using atomic operation
            toggle_led_enabled();

            info!("📍 [button_task] LED toggled");

            // Debounce: wait before allowing another press detection
            delay.delay_millis(DEBOUNCE_MS);
        }

        // Update previous state for next edge detection
        BUTTON_WAS_PRESSED = button_pressed;
    }
}
