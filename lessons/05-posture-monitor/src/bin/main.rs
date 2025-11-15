#![no_std]
#![no_main]

use core::fmt::Write;
use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull},
    i2c::master::{Config as I2cConfig, I2c},
    main,
    rmt::Rmt,
    time::Rate,
    uart::{Config as UartConfig, Uart},
    Blocking,
};
use esp_hal_smartled::{buffer_size, color_order, SmartLedsAdapter, Ws2812Timing};
use heapless::String;
use lesson_05_posture_monitor as mpu;
use log::info;
use smart_leds::{SmartLedsWrite, RGB8};

esp_bootloader_esp_idf::esp_app_desc!();

// GPIO configuration
const LED_PIN: u8 = 12;
const BUTTON_PIN: u8 = 9;
const NEOPIXEL_PIN: u8 = 8;
const UART_TX_PIN: u8 = 23;
const UART_RX_PIN: u8 = 15;
const I2C_SDA_PIN: u8 = 2;
const I2C_SCL_PIN: u8 = 11;

const UART_BAUD: u32 = 115200;
const I2C_FREQ: u32 = 100_000;
const CMD_BUFFER_SIZE: usize = 128;

// Posture thresholds (degrees)
const TILT_WARNING_THRESHOLD: f32 = 30.0;
const TILT_ALERT_THRESHOLD: f32 = 60.0;

// LED blink frequencies
const LED_BLINK_WARNING_HZ: u32 = 1;  // 1 Hz
const LED_BLINK_ALERT_HZ: u32 = 5;    // 5 Hz

// Device states
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
enum DeviceState {
    Sleep = 0,
    Monitoring = 1,
    Calibrating = 2,
}

// Alert levels (sub-states of Monitoring)
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
enum AlertLevel {
    Normal = 0,   // 0-30°
    Warning = 1,  // 30-60°
    Alert = 2,    // >60°
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
enum Mode {
    CLI = 0,
    Streaming = 1,
}

// Global state (GDB-accessible)
#[no_mangle]
static mut DEVICE_STATE: DeviceState = DeviceState::Sleep;

#[no_mangle]
static mut ALERT_LEVEL: AlertLevel = AlertLevel::Normal;

#[no_mangle]
static mut MODE: Mode = Mode::CLI;

#[no_mangle]
static mut UPTIME_MS: u32 = 0;

#[no_mangle]
static mut TILT_ANGLE: f32 = 0.0;

#[no_mangle]
static mut IMU_ACCEL_X: i16 = 0;
#[no_mangle]
static mut IMU_ACCEL_Y: i16 = 0;
#[no_mangle]
static mut IMU_ACCEL_Z: i16 = 0;

#[no_mangle]
static mut CAL_OFFSET_X: i16 = 0;
#[no_mangle]
static mut CAL_OFFSET_Y: i16 = 0;
#[no_mangle]
static mut CAL_OFFSET_Z: i16 = 0;

#[no_mangle]
static mut CALIBRATION_SAMPLES: u16 = 0;

#[no_mangle]
static mut LED_STATE: bool = false;

#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();
    log::set_max_level(log::LevelFilter::Info);

    info!("\n=== Lesson 05: Posture Monitor Device ===\n");

    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut delay = Delay::new();

    // Initialize UART
    info!("[INIT] UART...");
    let uart_config = UartConfig::default().with_baudrate(UART_BAUD);
    let mut uart = Uart::new(peripherals.UART1, uart_config)
        .unwrap()
        .with_tx(peripherals.GPIO23)
        .with_rx(peripherals.GPIO15);

    // Initialize I2C
    info!("[INIT] I2C...");
    let i2c_config = I2cConfig::default().with_frequency(Rate::from_hz(I2C_FREQ));
    let mut i2c = I2c::new(peripherals.I2C0, i2c_config)
        .unwrap()
        .with_sda(peripherals.GPIO2)
        .with_scl(peripherals.GPIO11);

    // Initialize MPU6050
    info!("[INIT] MPU6050...");
    if mpu::wake_sensor(&mut i2c).is_ok() {
        delay.delay_millis(100);
        if let Ok(who_am_i) = mpu::read_who_am_i(&mut i2c) {
            info!("[INIT] MPU WHO_AM_I = 0x{:02X}", who_am_i);
        }
    }

    // Initialize button
    info!("[INIT] Button...");
    let button = Input::new(peripherals.GPIO9, InputConfig::default().with_pull(Pull::Up));

    // Initialize LED
    info!("[INIT] LED...");
    let mut led = Output::new(peripherals.GPIO12, Level::Low, OutputConfig::default());

    // Initialize Neopixel
    info!("[INIT] Neopixel...");
    let rmt = Rmt::new(peripherals.RMT, Rate::from_mhz(80)).expect("Failed to init RMT");
    let mut neopixel = SmartLedsAdapter::<{ buffer_size(1) }, Blocking, color_order::Rgb, Ws2812Timing>::new_with_memsize(
        rmt.channel0,
        peripherals.GPIO8,
        2,
    )
    .expect("Failed to create SmartLedsAdapter");

    info!("[INIT] All peripherals ready\n");
    info!("[DEVICE] Starting in Sleep mode. Press button to start monitoring.\n");

    let _ = uart.write_str("\r\n=== Posture Monitor Device ===\r\n");
    let _ = uart.write_str("Type 'help' for commands\r\n\r\n> ");

    let mut cmd_buffer: String<CMD_BUFFER_SIZE> = String::new();
    let mut counter: u32 = 0;
    let mut last_stream_time_ms: u32 = 0;
    let mut button_last_state = button.is_high();
    let mut button_press_time: u32 = 0;
    let mut led_last_toggle_ms: u32 = 0;

    // Calibration accumulators
    let mut cal_accel_x_sum: i32 = 0;
    let mut cal_accel_y_sum: i32 = 0;
    let mut cal_accel_z_sum: i32 = 0;

    loop {
        unsafe {
            UPTIME_MS = UPTIME_MS.wrapping_add(10);
        }

        let current_time_ms = unsafe { UPTIME_MS };

        // Button handling (short vs long press)
        let button_current = button.is_high();
        if button_last_state && !button_current {
            // Button just pressed
            button_press_time = current_time_ms;
        } else if !button_last_state && button_current {
            // Button just released
            let press_duration = current_time_ms.wrapping_sub(button_press_time);

            if press_duration >= 3000 {
                // Long press (3s) - toggle Sleep
                unsafe {
                    DEVICE_STATE = if DEVICE_STATE == DeviceState::Sleep {
                        info!("[STATE] Long press: Sleep → Monitoring");
                        neopixel.write([RGB8::new(0, 30, 0)].into_iter()).ok(); // Green
                        DeviceState::Monitoring
                    } else {
                        info!("[STATE] Long press: → Sleep");
                        neopixel.write([RGB8::new(0, 0, 0)].into_iter()).ok(); // Off
                        led.set_low();
                        LED_STATE = false;
                        DeviceState::Sleep
                    };
                }
            } else if press_duration >= 50 {
                // Short press - calibrate zero
                unsafe {
                    if DEVICE_STATE == DeviceState::Monitoring {
                        info!("[STATE] Short press: Calibrating zero orientation");
                        DEVICE_STATE = DeviceState::Calibrating;
                        CALIBRATION_SAMPLES = 0;
                        cal_accel_x_sum = 0;
                        cal_accel_y_sum = 0;
                        cal_accel_z_sum = 0;
                        neopixel.write([RGB8::new(30, 30, 0)].into_iter()).ok(); // Yellow
                    }
                }
            }
        }
        button_last_state = button_current;

        // State machine
        let current_state = unsafe { DEVICE_STATE };
        match current_state {
            DeviceState::Sleep => {
                // Minimal activity
            }
            DeviceState::Monitoring => {
                // Read IMU
                if let Ok(accel) = mpu::read_accel(&mut i2c) {
                    unsafe {
                        IMU_ACCEL_X = accel.x;
                        IMU_ACCEL_Y = accel.y;
                        IMU_ACCEL_Z = accel.z;

                        // Apply calibration offsets
                        let ax = (accel.x - CAL_OFFSET_X) as f32;
                        let ay = (accel.y - CAL_OFFSET_Y) as f32;
                        let az = (accel.z - CAL_OFFSET_Z) as f32;

                        // Calculate tilt angle (pitch from X-Z plane)
                        // Using atan2(sqrt(x² + y²), z) for total tilt from vertical
                        let xy_magnitude = libm::sqrtf(ax * ax + ay * ay);
                        TILT_ANGLE = libm::atan2f(xy_magnitude, az) * 180.0 / 3.14159265;

                        // Determine alert level
                        let prev_alert = ALERT_LEVEL;
                        ALERT_LEVEL = if TILT_ANGLE < TILT_WARNING_THRESHOLD {
                            AlertLevel::Normal
                        } else if TILT_ANGLE < TILT_ALERT_THRESHOLD {
                            AlertLevel::Warning
                        } else {
                            AlertLevel::Alert
                        };

                        // Update Neopixel and LED based on alert level
                        if ALERT_LEVEL != prev_alert {
                            match ALERT_LEVEL {
                                AlertLevel::Normal => {
                                    neopixel.write([RGB8::new(0, 30, 0)].into_iter()).ok(); // Green
                                    led.set_low();
                                    LED_STATE = false;
                                    info!("[ALERT] Normal (tilt={:.1}°)", TILT_ANGLE);
                                }
                                AlertLevel::Warning => {
                                    neopixel.write([RGB8::new(30, 30, 0)].into_iter()).ok(); // Yellow
                                    info!("[ALERT] Warning (tilt={:.1}°)", TILT_ANGLE);
                                }
                                AlertLevel::Alert => {
                                    neopixel.write([RGB8::new(30, 0, 0)].into_iter()).ok(); // Red
                                    info!("[ALERT] Alert! (tilt={:.1}°)", TILT_ANGLE);
                                }
                            }
                        }

                        // Handle LED blinking for Warning/Alert
                        match ALERT_LEVEL {
                            AlertLevel::Normal => {
                                led.set_low();
                                LED_STATE = false;
                            }
                            AlertLevel::Warning => {
                                // Blink at 1 Hz (500ms period)
                                if current_time_ms.wrapping_sub(led_last_toggle_ms) >= 500 {
                                    LED_STATE = !LED_STATE;
                                    if LED_STATE {
                                        led.set_high();
                                    } else {
                                        led.set_low();
                                    }
                                    led_last_toggle_ms = current_time_ms;
                                }
                            }
                            AlertLevel::Alert => {
                                // Blink at 5 Hz (100ms period)
                                if current_time_ms.wrapping_sub(led_last_toggle_ms) >= 100 {
                                    LED_STATE = !LED_STATE;
                                    if LED_STATE {
                                        led.set_high();
                                    } else {
                                        led.set_low();
                                    }
                                    led_last_toggle_ms = current_time_ms;
                                }
                            }
                        }
                    }
                }
            }
            DeviceState::Calibrating => {
                // Collect calibration samples
                if let Ok(accel) = mpu::read_accel(&mut i2c) {
                    unsafe {
                        if CALIBRATION_SAMPLES < 100 {
                            cal_accel_x_sum += accel.x as i32;
                            cal_accel_y_sum += accel.y as i32;
                            cal_accel_z_sum += accel.z as i32;
                            CALIBRATION_SAMPLES += 1;

                            if CALIBRATION_SAMPLES >= 100 {
                                CAL_OFFSET_X = (cal_accel_x_sum / 100) as i16;
                                CAL_OFFSET_Y = (cal_accel_y_sum / 100) as i16;
                                CAL_OFFSET_Z = (cal_accel_z_sum / 100) as i16 - 16384; // Gravity offset
                                info!(
                                    "[CALIB] Complete! Offsets: x={}, y={}, z={}",
                                    CAL_OFFSET_X, CAL_OFFSET_Y, CAL_OFFSET_Z
                                );
                                DEVICE_STATE = DeviceState::Monitoring;
                                neopixel.write([RGB8::new(0, 30, 0)].into_iter()).ok(); // Green
                            }
                        }
                    }
                }
            }
        }

        // CLI vs Streaming mode
        let current_mode = unsafe { MODE };
        match current_mode {
            Mode::CLI => {
                let mut rx_byte = [0u8; 1];
                if uart.read(&mut rx_byte).is_ok() {
                    let ch = rx_byte[0] as char;

                    if ch == '\r' || ch == '\n' {
                        if !cmd_buffer.is_empty() {
                            let _ = uart.write_str("\r\n");
                            process_command(&cmd_buffer, &mut led, &mut neopixel, &mut i2c, &mut uart);
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
                if current_time_ms.wrapping_sub(last_stream_time_ms) >= 100 {
                    last_stream_time_ms = current_time_ms;
                    counter = counter.wrapping_add(1);

                    let mut msg: String<256> = String::new();
                    let (state, alert, tilt, ax, ay, az, led_st) = unsafe {
                        (
                            DEVICE_STATE,
                            ALERT_LEVEL,
                            TILT_ANGLE,
                            IMU_ACCEL_X,
                            IMU_ACCEL_Y,
                            IMU_ACCEL_Z,
                            LED_STATE,
                        )
                    };

                    write!(
                        msg,
                        "[dev=PostureMonitor state={:?} alert={:?} tilt={:.1}° accel=({},{},{}) led={} cnt={} t={}]\r\n",
                        state, alert, tilt, ax, ay, az, if led_st { "on" } else { "off" }, counter, current_time_ms
                    )
                    .ok();

                    let _ = uart.write_str(&msg);
                }
            }
        }

        delay.delay_millis(10);
    }
}

fn process_command<W: Write, Dm: esp_hal::DriverMode>(
    cmd: &str,
    led: &mut Output,
    neopixel: &mut SmartLedsAdapter<{ buffer_size(1) }, Blocking, color_order::Rgb, Ws2812Timing>,
    i2c: &mut I2c<Dm>,
    uart: &mut W,
) {
    let cmd_trimmed = cmd.trim();
    let parts: heapless::Vec<&str, 5> = cmd_trimmed.split_whitespace().collect();

    if parts.is_empty() {
        return;
    }

    match parts[0] {
        "help" => {
            let _ = uart.write_str("Posture Monitor Commands:\r\n");
            let _ = uart.write_str("  device.start        - Start monitoring\r\n");
            let _ = uart.write_str("  device.cal_zero     - Calibrate zero orientation\r\n");
            let _ = uart.write_str("  device.sleep        - Enter sleep mode\r\n");
            let _ = uart.write_str("  device.status       - Show device status\r\n");
            let _ = uart.write_str("  gpio.on/off         - LED control\r\n");
            let _ = uart.write_str("  neo.color <r> <g> <b> - Set Neopixel\r\n");
            let _ = uart.write_str("  imu.read            - Read accel data\r\n");
            let _ = uart.write_str("  stream.start/stop   - Toggle streaming\r\n");
        }
        "device.start" => {
            unsafe {
                DEVICE_STATE = DeviceState::Monitoring;
            }
            let _ = uart.write_str("OK [Posture Monitor started]\r\n");
        }
        "device.cal_zero" => {
            unsafe {
                DEVICE_STATE = DeviceState::Calibrating;
                CALIBRATION_SAMPLES = 0;
            }
            let _ = uart.write_str("OK [Calibrating zero orientation...]\r\n");
        }
        "device.sleep" => {
            unsafe {
                DEVICE_STATE = DeviceState::Sleep;
            }
            neopixel.write([RGB8::new(0, 0, 0)].into_iter()).ok();
            led.set_low();
            let _ = uart.write_str("OK [Sleep mode]\r\n");
        }
        "device.status" => {
            let (state, alert, tilt) = unsafe { (DEVICE_STATE, ALERT_LEVEL, TILT_ANGLE) };
            let mut buf: String<128> = String::new();
            write!(
                buf,
                "Device: Posture Monitor\r\nState: {:?}\r\nAlert: {:?} (tilt={:.1}°)\r\n",
                state, alert, tilt
            )
            .ok();
            let _ = uart.write_str(&buf);
        }
        "gpio.on" => {
            led.set_high();
            let _ = uart.write_str("OK [LED ON]\r\n");
        }
        "gpio.off" => {
            led.set_low();
            let _ = uart.write_str("OK [LED OFF]\r\n");
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
                let mut buf: String<64> = String::new();
                write!(buf, "OK [Neopixel RGB=({},{},{})]\r\n", r, g, b).ok();
                let _ = uart.write_str(&buf);
            }
        }
        "imu.read" => {
            if let Ok(accel) = mpu::read_accel(i2c) {
                let mut buf: String<128> = String::new();
                write!(buf, "accel: x={} y={} z={}\r\n", accel.x, accel.y, accel.z).ok();
                let _ = uart.write_str(&buf);
            } else {
                let _ = uart.write_str("ERROR: Failed to read IMU\r\n");
            }
        }
        "stream.start" => {
            unsafe {
                MODE = Mode::Streaming;
            }
            let _ = uart.write_str("[Switching to streaming mode...]\r\n");
        }
        "stream.stop" => {
            unsafe {
                MODE = Mode::CLI;
            }
            let _ = uart.write_str("[Switching to CLI mode...]\r\n");
        }
        _ => {
            let _ = uart.write_str("ERROR: Unknown command. Type 'help'\r\n");
        }
    }
}
