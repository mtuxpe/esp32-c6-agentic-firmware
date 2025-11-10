//! Simple cooperative task scheduler
//!
//! Provides a basic task scheduling system with fixed-period tasks.

use esp_hal::gpio::{Input, Output};

/// Hardware context passed to all tasks
pub struct Context {
    pub led: Output<'static>,
    pub input: Input<'static>,
}

/// A task that runs periodically
pub struct Task {
    pub run: fn(&mut Context),
    pub period_ms: u64,
    pub last_run: u64,
}

impl Task {
    /// Check if this task should run based on current time
    pub fn should_run(&self, now: u64) -> bool {
        (now - self.last_run) >= self.period_ms
    }

    /// Execute the task and update last run time
    pub fn execute(&mut self, now: u64, ctx: &mut Context) {
        (self.run)(ctx);
        self.last_run = now;
    }
}
