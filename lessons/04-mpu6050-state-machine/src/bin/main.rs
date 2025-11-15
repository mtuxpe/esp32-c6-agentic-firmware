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
use lesson_04_mpu6050_state_machine as mpu;
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
const I2C_FREQ: u32 = 100_000; // 100 kHz for MPU6050
const CMD_BUFFER_SIZE: usize = 128;

// State machine
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
enum DeviceState {
    Sleep = 0,
    Monitoring = 1,
    Calibrating = 2,
}

// Operating modes
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
enum Mode {
    CLI = 0,
    Streaming = 1,
}

// Global state variables (modifiable from GDB)
#[no_mangle]
static mut DEVICE_STATE: DeviceState = DeviceState::Sleep;

#[no_mangle]
static mut MODE: Mode = Mode::CLI;

#[no_mangle]
static mut UPTIME_MS: u32 = 0;

#[no_mangle]
static mut IMU_ACCEL_X: i16 = 0;
#[no_mangle]
static mut IMU_ACCEL_Y: i16 = 0;
#[no_mangle]
static mut IMU_ACCEL_Z: i16 = 0;

#[no_mangle]
static mut IMU_GYRO_X: i16 = 0;
#[no_mangle]
static mut IMU_GYRO_Y: i16 = 0;
#[no_mangle]
static mut IMU_GYRO_Z: i16 = 0;

#[no_mangle]
static mut NEO_R: u8 = 0;
#[no_mangle]
static mut NEO_G: u8 = 0;
#[no_mangle]
static mut NEO_B: u8 = 0;

#[no_mangle]
static mut CALIBRATION_SAMPLES: u16 = 0;

#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();
    log::set_max_level(log::LevelFilter::Info);

    info!("\n=== Lesson 04: MPU6050 + State Machine ===\n");

    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut delay = Delay::new();

    // Initialize UART
    info!("[INIT] Configuring UART...");
    let uart_config = UartConfig::default().with_baudrate(UART_BAUD);
    let mut uart = Uart::new(peripherals.UART1, uart_config)
        .unwrap()
        .with_tx(peripherals.GPIO23)
        .with_rx(peripherals.GPIO15);

    // Initialize I2C for MPU6050
    info!("[INIT] Configuring I2C @ {} Hz...", I2C_FREQ);
    let i2c_config = I2cConfig::default().with_frequency(Rate::from_hz(I2C_FREQ));
    let mut i2c = I2c::new(peripherals.I2C0, i2c_config)
        .unwrap()
        .with_sda(peripherals.GPIO2)
        .with_scl(peripherals.GPIO11);

    // Initialize MPU6050
    info!("[INIT] Waking MPU6050...");
    if mpu::wake_sensor(&mut i2c).is_ok() {
        delay.delay_millis(100);
        if let Ok(who_am_i) = mpu::read_who_am_i(&mut i2c) {
            info!("[INIT] MPU WHO_AM_I = 0x{:02X}", who_am_i);
        }
    }

    // Initialize button
    info!("[INIT] Configuring button (GPIO{})...", BUTTON_PIN);
    let button = Input::new(peripherals.GPIO9, InputConfig::default().with_pull(Pull::Up));

    // Initialize LED
    info!("[INIT] Configuring LED (GPIO{})...", LED_PIN);
    let mut led = Output::new(peripherals.GPIO12, Level::Low, OutputConfig::default());

    // Initialize Neopixel
    info!("[INIT] Configuring Neopixel (GPIO{})...", NEOPIXEL_PIN);
    let rmt = Rmt::new(peripherals.RMT, Rate::from_mhz(80)).expect("Failed to init RMT");
    let mut neopixel = SmartLedsAdapter::<{ buffer_size(1) }, Blocking, color_order::Rgb, Ws2812Timing>::new_with_memsize(
        rmt.channel0,
        peripherals.GPIO8,
        2,
    )
    .expect("Failed to create SmartLedsAdapter");

    info!("[INIT] All peripherals ready\n");

    let _ = uart.write_str("\r\n=== ESP32-C6 CLI (Lesson 04) ===\r\n");
    let _ = uart.write_str("Commands: help, gpio.*, neo.*, imu.*, state.*, stream.*\r\n\r\n> ");

    let mut cmd_buffer: String<CMD_BUFFER_SIZE> = String::new();
    let mut counter: u32 = 0;
    let mut last_stream_time_ms: u32 = 0;
    let mut button_last_state = button.is_high();

    // Calibration accumulators
    let mut cal_accel_x_sum: i32 = 0;
    let mut cal_accel_y_sum: i32 = 0;
    let mut cal_accel_z_sum: i32 = 0;

    loop {
        unsafe {
            UPTIME_MS = UPTIME_MS.wrapping_add(10);
        }

        // Handle button press for state transitions
        let button_current = button.is_high();
        if button_last_state && !button_current {
            delay.delay_millis(50); // Debounce
            if button.is_low() {
                // Cycle states: Sleep → Monitoring → Calibrating → Monitoring
                unsafe {
                    DEVICE_STATE = match DEVICE_STATE {
                        DeviceState::Sleep => {
                            neopixel.write([RGB8::new(0, 0, 30)].into_iter()).ok(); // Blue for monitoring
                            NEO_R = 0; NEO_G = 0; NEO_B = 30;
                            info!("[STATE] Sleep → Monitoring");
                            DeviceState::Monitoring
                        }
                        DeviceState::Monitoring => {
                            neopixel.write([RGB8::new(30, 30, 0)].into_iter()).ok(); // Yellow for calibrating
                            NEO_R = 30; NEO_G = 30; NEO_B = 0;
                            CALIBRATION_SAMPLES = 0;
                            cal_accel_x_sum = 0;
                            cal_accel_y_sum = 0;
                            cal_accel_z_sum = 0;
                            info!("[STATE] Monitoring → Calibrating");
                            DeviceState::Calibrating
                        }
                        DeviceState::Calibrating => {
                            neopixel.write([RGB8::new(0, 0, 0)].into_iter()).ok(); // Off for sleep
                            NEO_R = 0; NEO_G = 0; NEO_B = 0;
                            info!("[STATE] Calibrating → Sleep");
                            DeviceState::Sleep
                        }
                    };
                }
            }
        }
        button_last_state = button_current;

        // State machine behavior
        let current_state = unsafe { DEVICE_STATE };
        match current_state {
            DeviceState::Sleep => {
                // Minimal activity in sleep
            }
            DeviceState::Monitoring => {
                // Read IMU periodically
                if let Ok(accel) = mpu::read_accel(&mut i2c) {
                    unsafe {
                        IMU_ACCEL_X = accel.x;
                        IMU_ACCEL_Y = accel.y;
                        IMU_ACCEL_Z = accel.z;
                    }
                }
                if let Ok(gyro) = mpu::read_gyro(&mut i2c) {
                    unsafe {
                        IMU_GYRO_X = gyro.x;
                        IMU_GYRO_Y = gyro.y;
                        IMU_GYRO_Z = gyro.z;
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
                                info!("[CALIB] Complete! Offsets: x={}, y={}, z={}",
                                      cal_accel_x_sum / 100, cal_accel_y_sum / 100, cal_accel_z_sum / 100);
                                // Auto-transition back to Monitoring
                                DEVICE_STATE = DeviceState::Monitoring;
                                neopixel.write([RGB8::new(0, 0, 30)].into_iter()).ok();
                                NEO_R = 0; NEO_G = 0; NEO_B = 30;
                            }
                        }
                    }
                }
            }
        }

        // Mode handling (CLI vs Streaming)
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
                let current_time_ms = unsafe { UPTIME_MS };
                if current_time_ms.wrapping_sub(last_stream_time_ms) >= 100 {
                    last_stream_time_ms = current_time_ms;
                    counter = counter.wrapping_add(1);

                    let mut msg: String<256> = String::new();
                    let (state, ax, ay, az, gx, gy, gz, r, g, b, cal) = unsafe {
                        (
                            DEVICE_STATE,
                            IMU_ACCEL_X,
                            IMU_ACCEL_Y,
                            IMU_ACCEL_Z,
                            IMU_GYRO_X,
                            IMU_GYRO_Y,
                            IMU_GYRO_Z,
                            NEO_R,
                            NEO_G,
                            NEO_B,
                            CALIBRATION_SAMPLES,
                        )
                    };

                    write!(
                        msg,
                        "[state={:?} accel=({},{},{}) gyro=({},{},{}) neo=({},{},{}) cal={} cnt={} t={}]\r\n",
                        state, ax, ay, az, gx, gy, gz, r, g, b, cal, counter, current_time_ms
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
            let _ = uart.write_str("Commands:\r\n");
            let _ = uart.write_str("  gpio.on/off         - LED control\r\n");
            let _ = uart.write_str("  neo.color <r> <g> <b> - Set Neopixel\r\n");
            let _ = uart.write_str("  neo.off             - Neopixel off\r\n");
            let _ = uart.write_str("  imu.init            - Wake MPU6050\r\n");
            let _ = uart.write_str("  imu.whoami          - Read WHO_AM_I\r\n");
            let _ = uart.write_str("  imu.read            - Read accel/gyro\r\n");
            let _ = uart.write_str("  state.get           - Get current state\r\n");
            let _ = uart.write_str("  state.set <state>   - Set state (sleep/monitor/calib)\r\n");
            let _ = uart.write_str("  stream.start/stop   - Toggle streaming\r\n");
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
                unsafe {
                    NEO_R = r;
                    NEO_G = g;
                    NEO_B = b;
                }
                let mut buf: String<64> = String::new();
                write!(buf, "OK [Neopixel RGB=({},{},{})]\r\n", r, g, b).ok();
                let _ = uart.write_str(&buf);
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
        "imu.init" => {
            if mpu::wake_sensor(i2c).is_ok() {
                let _ = uart.write_str("OK [MPU6050 woken]\r\n");
            } else {
                let _ = uart.write_str("ERROR: Failed to wake MPU6050\r\n");
            }
        }
        "imu.whoami" => {
            if let Ok(who_am_i) = mpu::read_who_am_i(i2c) {
                let mut buf: String<64> = String::new();
                write!(buf, "WHO_AM_I = 0x{:02X}\r\n", who_am_i).ok();
                let _ = uart.write_str(&buf);
            } else {
                let _ = uart.write_str("ERROR: I2C read failed\r\n");
            }
        }
        "imu.read" => {
            if let (Ok(accel), Ok(gyro)) = (mpu::read_accel(i2c), mpu::read_gyro(i2c)) {
                let mut buf: String<128> = String::new();
                write!(
                    buf,
                    "accel: x={} y={} z={}  gyro: x={} y={} z={}\r\n",
                    accel.x, accel.y, accel.z, gyro.x, gyro.y, gyro.z
                )
                .ok();
                let _ = uart.write_str(&buf);
            } else {
                let _ = uart.write_str("ERROR: Failed to read IMU\r\n");
            }
        }
        "state.get" => {
            let state = unsafe { DEVICE_STATE };
            let mut buf: String<64> = String::new();
            write!(buf, "State = {:?}\r\n", state).ok();
            let _ = uart.write_str(&buf);
        }
        "state.set" => {
            if parts.len() < 2 {
                let _ = uart.write_str("ERROR: Usage: state.set <sleep|monitor|calib>\r\n");
            } else {
                match parts[1] {
                    "sleep" => {
                        unsafe {
                            DEVICE_STATE = DeviceState::Sleep;
                        }
                        let _ = uart.write_str("OK [State = Sleep]\r\n");
                    }
                    "monitor" => {
                        unsafe {
                            DEVICE_STATE = DeviceState::Monitoring;
                        }
                        let _ = uart.write_str("OK [State = Monitoring]\r\n");
                    }
                    "calib" => {
                        unsafe {
                            DEVICE_STATE = DeviceState::Calibrating;
                            CALIBRATION_SAMPLES = 0;
                        }
                        let _ = uart.write_str("OK [State = Calibrating]\r\n");
                    }
                    _ => {
                        let _ = uart.write_str("ERROR: Unknown state\r\n");
                    }
                }
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

// GDB-callable functions
#[no_mangle]
pub extern "C" fn get_device_state() -> u8 {
    unsafe { DEVICE_STATE as u8 }
}

#[no_mangle]
pub extern "C" fn set_device_state(state: u8) {
    unsafe {
        DEVICE_STATE = match state {
            0 => DeviceState::Sleep,
            1 => DeviceState::Monitoring,
            2 => DeviceState::Calibrating,
            _ => DeviceState::Sleep,
        };
    }
}
