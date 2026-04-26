use embedded_hal::digital::{OutputPin, InputPin};
use nrf52840_hal as hal;
use hal::gpio::{Level, Input, Output, PushPull, Floating, Disconnected};
use crate::log_info;

pub struct HcSr04 {
    trig: hal::gpio::p0::P0_14<Output<PushPull>>,
    echo: hal::gpio::p0::P0_15<Input<Floating>>,
}

impl HcSr04 {
    pub fn new(
        trig: hal::gpio::p0::P0_14<Disconnected>,
        echo: hal::gpio::p0::P0_15<Disconnected>,
    ) -> Self {
        HcSr04 {
            trig: trig.into_push_pull_output(Level::Low),
            echo: echo.into_floating_input(),
        }
    }
}

pub fn distance_measurement_task() -> ! {
    let p = unsafe { hal::pac::Peripherals::steal() };
    let port0 = hal::gpio::p0::Parts::new(p.P0);
    let mut sensor = HcSr04::new(port0.p0_14, port0.p0_15);

    // Timer configuration: 16MHz / 2^4 = 1MHz (1 tick = 1 microsecond)
    let timer = p.TIMER0;
    timer.bitmode.write(|w| w.bitmode()._32bit());
    timer.prescaler.write(|w| unsafe { w.prescaler().bits(4) }); 
    
    log_info!("\n=== Testing Sensor with Hardware Timer ===\n");

    loop {
        // 1. Send 10us Trigger Pulse
        let _ = sensor.trig.set_high();
        for _ in 0..160 { cortex_m::asm::nop(); } 
        let _ = sensor.trig.set_low();

        // 2. Wait for ECHO to go HIGH
        let mut timeout = 0;
        while sensor.echo.is_low().unwrap_or(true) && timeout < 1_000_000 {
            timeout += 1;
        }

        // 3. Start Timer
        timer.tasks_clear.write(|w| unsafe { w.bits(1) });
        timer.tasks_start.write(|w| unsafe { w.bits(1) });

        // 4. Wait for ECHO to go LOW
        timeout = 0;
        while sensor.echo.is_high().unwrap_or(false) && timeout < 1_000_000 {
            timeout += 1;
        }
        
        // 5. Capture Timer Value
        timer.tasks_capture[0].write(|w| unsafe { w.bits(1) });
        let duration = timer.cc[0].read().bits();
        timer.tasks_stop.write(|w| unsafe { w.bits(1) });
        
        // 6. Calculate Distance (Speed of sound = 343m/s -> duration / 58)
        let distance_cm = duration as f32 / 58.0;

        // 7. Output Result with Visual Bar (Max 4m = 23200us)
        if duration > 0 && duration < 23200 { 
            log_info!("Dist: {} cm (Raw: {} us)", distance_cm as u32, duration);
        } else {
            // Case: No object within 4m or noise detected
            log_info!("Check Sensor... (Raw: {} us)", duration);
        }

        // 8. Measurement interval (~60ms for interactive response)
        for _ in 0..40_000 {
            cortex_m::asm::nop();
        }
    }
}



// use embedded_hal::digital::{OutputPin, InputPin};
// use nrf52840_hal as hal;
// use hal::gpio::{Level, Input, Output, PushPull, Floating, Disconnected};
// use crate::log_info;

// /// HC-SR04 Ultrasonic Distance Sensor
// /// 
// /// Wiring:
// /// - VCC: 5V
// /// - GND: GND
// /// - TRIG: P0_14
// /// - ECHO: P0_15
// pub struct HcSr04 {
//     trig: hal::gpio::p0::P0_14<Output<PushPull>>,
//     echo: hal::gpio::p0::P0_15<Input<Floating>>,
// }

// pub struct DistanceMeasurement {
//     pub distance_cm: f32,
//     pub pulse_duration_us: u32,
// }

// impl HcSr04 {
//     /// Initialize HC-SR04 sensor
//     pub fn new(
//         trig: hal::gpio::p0::P0_14<Disconnected>,
//         echo: hal::gpio::p0::P0_15<Disconnected>,
//     ) -> Self {
//         HcSr04 {
//             trig: trig.into_push_pull_output(Level::Low),
//             echo: echo.into_floating_input(),
//         }
//     }

//     /// Measure distance using HC-SR04
//     /// Returns distance in cm
//     pub fn measure(&mut self) -> DistanceMeasurement {
//         // Send 10µs trigger pulse
//         let _ = self.trig.set_high();
        
//         // Simple delay loop for ~10µs (not precise but works for this purpose)
//         for _ in 0..10 {
//             cortex_m::asm::nop();
//         }
        
//         let _ = self.trig.set_low();

//         // Wait for ECHO pin to go HIGH
//         let mut timeout = 0u32;
//         while self.echo.is_low().unwrap_or(true) && timeout < 1000000 {
//             timeout += 1;
//         }

//         // Measure pulse duration (approximate microseconds)
//         let mut pulse_duration = 0u32;
//         timeout = 0;
        
//         while self.echo.is_high().unwrap_or(false) && timeout < 100000 {
//             pulse_duration += 1;
//             timeout += 1;
//         }

//         // Convert pulse duration to distance
//         // Speed of sound: ~343 m/s
//         // Distance = (pulse_duration / 2) * (343 / 1_000_000) m
//         // Simplified: distance_cm = pulse_duration / 58
//         let distance_cm = pulse_duration as f32 / 58.0;

//         DistanceMeasurement {
//             distance_cm,
//             pulse_duration_us: pulse_duration,
//         }
//     }
// }

// /// Continuous distance measurement task with RTT logging via defmt
// pub fn distance_measurement_task() -> ! {
//     let p = unsafe { hal::pac::Peripherals::steal() };
//     let port0 = hal::gpio::p0::Parts::new(p.P0);

//     let mut sensor = HcSr04::new(port0.p0_14, port0.p0_15);

//     log_info!("\n=== HC-SR04 Distance Sensor Started ===\n");

//     loop {
//         let measurement = sensor.measure();
        
//         // Log via defmt RTT
//         log_info!(
//             "Distance: {}.{} cm | Pulse: {} µs",
//             measurement.distance_cm as u32,
//             ((measurement.distance_cm * 10.0) as u32) % 10,
//             measurement.pulse_duration_us
//         );
        
//         // Delay ~100ms between measurements
//         for _ in 0..30000 {
//             cortex_m::asm::nop();
//         }
//     }
// }
