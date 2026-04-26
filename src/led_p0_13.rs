use embedded_hal::digital::OutputPin;
use nrf52840_hal as hal;
use hal::gpio::{Level};
use hal::Timer;
use embedded_hal::delay::DelayNs;

pub fn led_blink() {
    let p = hal::pac::Peripherals::take().unwrap();
    let port0 = hal::gpio::p0::Parts::new(p.P0);
    let mut led = port0.p0_13.into_push_pull_output(Level::Low);
    let mut timer = Timer::new(p.TIMER0);

    loop {
        led.set_high().unwrap();
        timer.delay_ms(500_u32);
        
        led.set_low().unwrap();
        timer.delay_ms(500_u32);
    }
}