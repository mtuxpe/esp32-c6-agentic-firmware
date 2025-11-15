#![no_std]
#![no_main]

use core::fmt::Write;
use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Level, Output, OutputConfig},
    main,
    uart::{Config as UartConfig, Uart},
};
use heapless::String;
use log::info;

esp_bootloader_esp_idf::esp_app_desc!();

// GPIO configuration
const LED_PIN: u8 = 12;

// UART configuration (FTDI adapter)
const UART_TX_PIN: u8 = 23;
const UART_RX_PIN: u8 = 15;
const UART_BAUD: u32 = 115200;

// CLI buffer size
const CMD_BUFFER_SIZE: usize = 128;

// Operating modes
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
enum Mode {
    CLI = 0,
    Streaming = 1,
}

// Global mode variable (modifiable from GDB)
#[no_mangle]
static mut MODE: Mode = Mode::CLI;

// Global statistics
#[no_mangle]
static mut GPIO_CHANGES: u32 = 0;

#[no_mangle]
static mut UPTIME_MS: u32 = 0;

#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();
    log::set_max_level(log::LevelFilter::Info);

    info!("\n=== Lesson 02: UART CLI + Streaming Infrastructure ===\n");

    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut delay = Delay::new();

    // Initialize LED
    info!("[INIT] Configuring GPIO{} as output for LED...", LED_PIN);
    let mut led = Output::new(peripherals.GPIO12, Level::Low, OutputConfig::default());
    info!("[INIT] LED ready");

    // Initialize UART (TX=GPIO23, RX=GPIO15)
    info!("[INIT] Configuring UART on GPIO{} (TX), GPIO{} (RX) @ {} baud...",
          UART_TX_PIN, UART_RX_PIN, UART_BAUD);

    let uart_config = UartConfig::default().with_baudrate(UART_BAUD);
    let mut uart = Uart::new(peripherals.UART1, uart_config)
        .unwrap()
        .with_tx(peripherals.GPIO23)
        .with_rx(peripherals.GPIO15);

    info!("[INIT] UART ready\n");

    // Send welcome message over UART
    let _ = uart.write_str("\r\n=== ESP32-C6 UART CLI ===\r\n");
    let _ = uart.write_str("Type 'help' for commands\r\n\r\n> ");

    let mut cmd_buffer: String<CMD_BUFFER_SIZE> = String::new();
    let mut led_state = false;
    let mut counter: u32 = 0;
    let mut last_stream_time_ms: u32 = 0;

    loop {
        unsafe {
            UPTIME_MS = UPTIME_MS.wrapping_add(10);
        }

        // Check current mode (can be changed via GDB)
        let current_mode = unsafe { MODE };

        match current_mode {
            Mode::CLI => {
                // CLI mode: process commands from UART
                let mut rx_byte = [0u8; 1];
                if uart.read(&mut rx_byte).is_ok() {
                    let ch = rx_byte[0] as char;

                    if ch == '\r' || ch == '\n' {
                        if !cmd_buffer.is_empty() {
                            // Echo newline
                            let _ = uart.write_str("\r\n");

                            // Process command
                            process_command(&cmd_buffer, &mut led, &mut led_state, &mut uart);

                            // Clear buffer
                            cmd_buffer.clear();

                            // Show prompt
                            let _ = uart.write_str("> ");
                        }
                    } else if ch == '\x08' || ch == '\x7f' {
                        // Backspace
                        if cmd_buffer.pop().is_some() {
                            let _ = uart.write_str("\x08 \x08");
                        }
                    } else if ch.is_ascii_graphic() || ch == ' ' {
                        // Echo character
                        let _ = uart.write(&[ch as u8]);

                        // Add to buffer (ignore if full)
                        let _ = cmd_buffer.push(ch);
                    }
                }
            }
            Mode::Streaming => {
                // Streaming mode: output telemetry at 10 Hz
                let current_time_ms = unsafe { UPTIME_MS };
                if current_time_ms.wrapping_sub(last_stream_time_ms) >= 100 {
                    last_stream_time_ms = current_time_ms;
                    counter = counter.wrapping_add(1);

                    // Stream telemetry in parseable format
                    let mut msg: String<128> = String::new();
                    write!(
                        msg,
                        "[gpio{}={} counter={} uptime_ms={}]\r\n",
                        LED_PIN,
                        if led_state { 1 } else { 0 },
                        counter,
                        current_time_ms
                    )
                    .ok();

                    let _ = uart.write_str(&msg);
                }
            }
        }

        delay.delay_millis(10);
    }
}

fn process_command<W: Write>(
    cmd: &str,
    led: &mut Output,
    led_state: &mut bool,
    uart: &mut W,
) {
    let cmd_trimmed = cmd.trim();
    let parts: heapless::Vec<&str, 4> = cmd_trimmed.split_whitespace().collect();

    if parts.is_empty() {
        return;
    }

    match parts[0] {
        "help" => {
            let _ = uart.write_str("Commands:\r\n");
            let _ = uart.write_str("  gpio.init <pin>     - Initialize GPIO as output\r\n");
            let _ = uart.write_str("  gpio.on <pin>       - Set GPIO high\r\n");
            let _ = uart.write_str("  gpio.off <pin>      - Set GPIO low\r\n");
            let _ = uart.write_str("  gpio.deinit <pin>   - Deinitialize GPIO\r\n");
            let _ = uart.write_str("  stream.start        - Start streaming mode\r\n");
            let _ = uart.write_str("  stream.stop         - Stop streaming (back to CLI)\r\n");
            let _ = uart.write_str("  help                - Show this help\r\n");
        }
        "gpio.init" => {
            if parts.len() < 2 {
                let _ = uart.write_str("ERROR: Usage: gpio.init <pin>\r\n");
            } else if let Ok(pin) = parts[1].parse::<u8>() {
                if pin == LED_PIN {
                    let _ = uart.write_str("OK [GPIO");
                    let mut buf: String<16> = String::new();
                    write!(buf, "{}", pin).ok();
                    let _ = uart.write_str(&buf);
                    let _ = uart.write_str(" initialized as output]\r\n");
                } else {
                    let _ = uart.write_str("ERROR: Only GPIO12 supported in this lesson\r\n");
                }
            } else {
                let _ = uart.write_str("ERROR: Invalid pin number\r\n");
            }
        }
        "gpio.on" => {
            if parts.len() < 2 {
                let _ = uart.write_str("ERROR: Usage: gpio.on <pin>\r\n");
            } else if let Ok(pin) = parts[1].parse::<u8>() {
                if pin == LED_PIN {
                    led.set_high();
                    *led_state = true;
                    unsafe { GPIO_CHANGES += 1; }
                    let _ = uart.write_str("OK [GPIO");
                    let mut buf: String<16> = String::new();
                    write!(buf, "{}", pin).ok();
                    let _ = uart.write_str(&buf);
                    let _ = uart.write_str(" = HIGH]\r\n");
                } else {
                    let _ = uart.write_str("ERROR: Only GPIO12 supported\r\n");
                }
            } else {
                let _ = uart.write_str("ERROR: Invalid pin number\r\n");
            }
        }
        "gpio.off" => {
            if parts.len() < 2 {
                let _ = uart.write_str("ERROR: Usage: gpio.off <pin>\r\n");
            } else if let Ok(pin) = parts[1].parse::<u8>() {
                if pin == LED_PIN {
                    led.set_low();
                    *led_state = false;
                    unsafe { GPIO_CHANGES += 1; }
                    let _ = uart.write_str("OK [GPIO");
                    let mut buf: String<16> = String::new();
                    write!(buf, "{}", pin).ok();
                    let _ = uart.write_str(&buf);
                    let _ = uart.write_str(" = LOW]\r\n");
                } else {
                    let _ = uart.write_str("ERROR: Only GPIO12 supported\r\n");
                }
            } else {
                let _ = uart.write_str("ERROR: Invalid pin number\r\n");
            }
        }
        "gpio.deinit" => {
            if parts.len() < 2 {
                let _ = uart.write_str("ERROR: Usage: gpio.deinit <pin>\r\n");
            } else if let Ok(pin) = parts[1].parse::<u8>() {
                let _ = uart.write_str("OK [GPIO");
                let mut buf: String<16> = String::new();
                write!(buf, "{}", pin).ok();
                let _ = uart.write_str(&buf);
                let _ = uart.write_str(" deinitialized]\r\n");
            } else {
                let _ = uart.write_str("ERROR: Invalid pin number\r\n");
            }
        }
        "stream.start" => {
            unsafe { MODE = Mode::Streaming; }
            let _ = uart.write_str("[Switching to streaming mode...]\r\n");
        }
        "stream.stop" => {
            unsafe { MODE = Mode::CLI; }
            let _ = uart.write_str("[Switching to CLI mode...]\r\n");
        }
        _ => {
            let _ = uart.write_str("ERROR: Unknown command. Type 'help' for commands.\r\n");
        }
    }
}

// GDB-callable functions for hardware validation

#[no_mangle]
pub extern "C" fn validate_gpio_out_reg(expected_bit12: bool) -> bool {
    const GPIO_OUT_REG: *const u32 = 0x60004004 as *const u32;
    unsafe {
        let reg_value = GPIO_OUT_REG.read_volatile();
        let bit12_set = (reg_value & (1 << 12)) != 0;
        bit12_set == expected_bit12
    }
}

#[no_mangle]
pub extern "C" fn get_mode() -> u8 {
    unsafe { MODE as u8 }
}

#[no_mangle]
pub extern "C" fn set_mode(mode: u8) {
    unsafe {
        MODE = if mode == 0 {
            Mode::CLI
        } else {
            Mode::Streaming
        };
    }
}

// GDB Usage Examples:
//
// 1. Validate GPIO state after command:
//    (gdb) call validate_gpio_out_reg(true)
//    $1 = true  # Bit 12 is set as expected
//
// 2. Switch modes from GDB:
//    (gdb) call set_mode(1)  # Enter streaming mode
//    (gdb) call get_mode()
//    $2 = 1  # Confirmed in streaming mode
//
// 3. Check statistics:
//    (gdb) print GPIO_CHANGES
//    $3 = 5
//    (gdb) print UPTIME_MS
//    $4 = 12340
//
// 4. Set breakpoint on mode change:
//    (gdb) watch MODE
//    Hardware watchpoint 1: MODE
