#![no_std]
#![no_main]

mod approtect;
mod hc_sr04;
mod uart_log;

use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    approtect::disable_approtect();

    let _logger = uart_log::init_uart_logger();
    log_info!("Starting nRF52840 HC-SR04 Distance Sensor");
    
    // Start HC-SR04 distance measurement task
    hc_sr04::distance_measurement_task();
}