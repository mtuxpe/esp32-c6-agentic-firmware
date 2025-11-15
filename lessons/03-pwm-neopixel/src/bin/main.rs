#![no_std]
#![no_main]

use core::fmt::Write;
use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Level, Output, OutputConfig},
    main,
    rmt::Rmt,
    time::Rate,
    uart::{Config as UartConfig, Uart},
    Blocking,
};
use esp_hal_smartled::{buffer_size, color_order, SmartLedsAdapter, Ws2812Timing};
use heapless::String;
use log::info;
use smart_leds::{SmartLedsWrite, RGB8};

esp_bootloader_esp_idf::esp_app_desc!();

// GPIO configuration
const LED_PWM_PIN: u8 = 12;
const NEOPIXEL_PIN: u8 = 8;
const UART_TX_PIN: u8 = 23;
const UART_RX_PIN: u8 = 15;
const UART_BAUD: u32 = 115200;

const CMD_BUFFER_SIZE: usize = 128;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
enum Mode {
    CLI = 0,
    Streaming = 1,
}

#[no_mangle]
static mut MODE: Mode = Mode::CLI;

#[no_mangle]
static mut UPTIME_MS: u32 = 0;

#[no_mangle]
static mut PWM_DUTY: u8 = 0;

#[no_mangle]
static mut NEO_R: u8 = 0;
#[no_mangle]
static mut NEO_G: u8 = 0;
#[no_mangle]
static mut NEO_B: u8 = 0;

#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();
    log::set_max_level(log::LevelFilter::Info);

    info!("\n=== Lesson 03: PWM + Neopixel Drivers ===\n");

    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut delay = Delay::new();

    // Initialize UART
    info!("[INIT] Configuring UART...");
    let uart_config = UartConfig::default().with_baudrate(UART_BAUD);
    let mut uart = Uart::new(peripherals.UART1, uart_config)
        .unwrap()
        .with_tx(peripherals.GPIO23)
        .with_rx(peripherals.GPIO15);

    // Initialize GPIO12 as output (simplified - PWM will be added in future revision)
    info!("[INIT] Configuring GPIO12 for LED...");
    let mut led_gpio = Output::new(peripherals.GPIO12, Level::Low, OutputConfig::default());

    // Initialize Neopixel (RMT)
    info!("[INIT] Configuring Neopixel (RMT)...");
    let rmt = Rmt::new(peripherals.RMT, Rate::from_mhz(80)).expect("Failed to init RMT");
    let mut led = SmartLedsAdapter::<{ buffer_size(1) }, Blocking, color_order::Rgb, Ws2812Timing>::new_with_memsize(
        rmt.channel0,
        peripherals.GPIO8,
        2,
    )
    .expect("Failed to create SmartLedsAdapter");

    info!("[INIT] All peripherals ready\n");

    let _ = uart.write_str("\r\n=== ESP32-C6 CLI (Lesson 03) ===\r\n");
    let _ = uart.write_str("Commands: help, gpio.*, pwm.*, neo.*, stream.*\r\n\r\n> ");

    let mut cmd_buffer: String<CMD_BUFFER_SIZE> = String::new();
    let mut counter: u32 = 0;
    let mut last_stream_time_ms: u32 = 0;

    loop {
        unsafe {
            UPTIME_MS = UPTIME_MS.wrapping_add(10);
        }

        let current_mode = unsafe { MODE };

        match current_mode {
            Mode::CLI => {
                let mut rx_byte = [0u8; 1];
                if uart.read(&mut rx_byte).is_ok() {
                    let ch = rx_byte[0] as char;

                    if ch == '\r' || ch == '\n' {
                        if !cmd_buffer.is_empty() {
                            let _ = uart.write_str("\r\n");
                            process_command(&cmd_buffer, &mut led_gpio, &mut led, &mut uart);
                            cmd_buffer.clear();
                            let _ = uart.write_str("> ");
                        }
                    } else if ch == '\x08' || ch == '\x7f' {
                        if cmd_buffer.pop().is_some() {
                            let _ = uart.write_str("\x08 \x08");
                        }
                    } else if ch.is_ascii_graphic() || ch == ' ' {
                        let _ = uart.write(&[ch as u8]);
                        let _ = cmd_buffer.push(ch);
                    }
                }
            }
            Mode::Streaming => {
                let current_time_ms = unsafe { UPTIME_MS };
                if current_time_ms.wrapping_sub(last_stream_time_ms) >= 100 {
                    last_stream_time_ms = current_time_ms;
                    counter = counter.wrapping_add(1);

                    let mut msg: String<128> = String::new();
                    let (pwm, r, g, b) = unsafe { (PWM_DUTY, NEO_R, NEO_G, NEO_B) };
                    write!(
                        msg,
                        "[pwm{}={}% neo_r={} neo_g={} neo_b={} counter={} uptime_ms={}]\r\n",
                        LED_PWM_PIN, pwm, r, g, b, counter, current_time_ms
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
    led_gpio: &mut Output,
    neopixel: &mut SmartLedsAdapter<{ buffer_size(1) }, Blocking, color_order::Rgb, Ws2812Timing>,
    uart: &mut W,
) {
    let cmd_trimmed = cmd.trim();
    let parts: heapless::Vec<&str, 5> = cmd_trimmed.split_whitespace().collect();

    if parts.is_empty() {
        return;
    }

    match parts[0] {
        "help" => {
            let _ = uart.write_str("Commands:\r\n");
            let _ = uart.write_str("  gpio.* - From Lesson 02\r\n");
            let _ = uart.write_str("  pwm.duty <percent>  - Set PWM duty cycle (0-100)\r\n");
            let _ = uart.write_str("  neo.color <r> <g> <b> - Set Neopixel RGB (0-255)\r\n");
            let _ = uart.write_str("  neo.off             - Turn Neopixel off\r\n");
            let _ = uart.write_str("  stream.start/stop   - Toggle streaming mode\r\n");
        }
        "gpio.on" => {
            led_gpio.set_high();
            unsafe { PWM_DUTY = 100; }
            let _ = uart.write_str("OK [GPIO12 = HIGH]\r\n");
        }
        "gpio.off" => {
            led_gpio.set_low();
            unsafe { PWM_DUTY = 0; }
            let _ = uart.write_str("OK [GPIO12 = LOW]\r\n");
        }
        "neo.color" => {
            if parts.len() < 4 {
                let _ = uart.write_str("ERROR: Usage: neo.color <r> <g> <b>\r\n");
            } else if let (Ok(r), Ok(g), Ok(b)) = (
                parts[1].parse::<u8>(),
                parts[2].parse::<u8>(),
                parts[3].parse::<u8>(),
            ) {
                neopixel.write([RGB8::new(r, g, b)].into_iter()).ok();
                unsafe {
                    NEO_R = r;
                    NEO_G = g;
                    NEO_B = b;
                }
                let mut buf: String<64> = String::new();
                write!(buf, "OK [Neopixel RGB=({},{},{})]\r\n", r, g, b).ok();
                let _ = uart.write_str(&buf);
            } else {
                let _ = uart.write_str("ERROR: Invalid RGB values\r\n");
            }
        }
        "neo.off" => {
            neopixel.write([RGB8::new(0, 0, 0)].into_iter()).ok();
            unsafe {
                NEO_R = 0;
                NEO_G = 0;
                NEO_B = 0;
            }
            let _ = uart.write_str("OK [Neopixel OFF]\r\n");
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
            let _ = uart.write_str("ERROR: Unknown command. Type 'help'\r\n");
        }
    }
}
