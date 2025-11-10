//! Task implementations
//!
//! Simple tasks that run periodically via the scheduler.

use crate::scheduler::Context;
use log::info;

/// Blink the LED every 500ms
pub fn blink_task(ctx: &mut Context) {
    ctx.led.toggle();
    let state = if ctx.led.is_set_high() { "ON" } else { "OFF" };
    info!("[Blink] LED {}", state);
}

/// Monitor GPIO9 state every 100ms
pub fn monitor_task(ctx: &mut Context) {
    let state = if ctx.input.is_high() { "HIGH" } else { "LOW" };
    info!("[Monitor] GPIO9: {}", state);
}
