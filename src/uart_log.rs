
use nrf52840_hal as hal;
use hal::uarte::{Baudrate, Parity, Uarte};
use hal::gpio::p0::Parts as P0Parts;
use hal::pac;
use core::fmt::{Write, Arguments};

static mut UART_LOGGER: Option<Uarte<pac::UARTE0>> = None;

    // 4. The macro so you can use log_info!("...")
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        $crate::uart_log::_print(format_args!($($arg)*));
        $crate::uart_log::_print(format_args!("\r\n"));
    };
}

pub fn init_uart_logger() {
    let p = pac::Peripherals::take().unwrap();
    let port0 = P0Parts::new(p.P0);

    let tx = port0.p0_06.into_push_pull_output(hal::gpio::Level::Low).degrade();
    let rx = port0.p0_08.into_floating_input().degrade();

    let uart = Uarte::new(
        p.UARTE0,
        hal::uarte::Pins { txd: tx, rxd: rx, cts: None, rts: None },
        Parity::EXCLUDED,
        Baudrate::BAUD115200,
    );

    unsafe {
        // 1. Get a raw mutable pointer to the static without creating a reference yet
        let logger_ptr = core::ptr::addr_of_mut!(UART_LOGGER);
        
        // 2. Assign the value
        *logger_ptr = Some(uart);
        
        // 3. Access it safely for the initial message
        if let Some(logger) = (*logger_ptr).as_mut() {
            let _ = logger.write_str("\r\n--- UART Logger Initialized ---\r\n");
        }
    }
}

pub fn _print(args: Arguments) {
    unsafe {
        let logger_ptr = core::ptr::addr_of_mut!(UART_LOGGER);
        if let Some(logger) = (*logger_ptr).as_mut() {
            let _ = logger.write_fmt(args);
        }
    }
}

