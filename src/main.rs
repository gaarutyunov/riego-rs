#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use riego_rs::{map, panic as _};

const MIN_MOISTURE: u16 = 535;
const MAX_MOISTURE: u16 = 265;

const MOISTURE_THRESHOLD: u16 = 30; // Threshold for activating the relay
const WATERING_DURATION: u32 = 1000; // Duration to activate the relay in milliseconds

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let mut relay = pins.d2.into_output();
    // Initialize the relay to off
    relay.set_high();

    // Moisture sensor connected to A0
    let sensor_pin = pins.a0.into_analog_input(&mut adc);

    loop {
        let sensor_value: u16 = sensor_pin.analog_read(&mut adc);
        // Adjust the sensor value to a range to avoid overflow
        let sensor_value = sensor_value.clamp(MAX_MOISTURE, MIN_MOISTURE);

        // Convert the raw ADC value to a percentage
        let moisture_percentage = 100 - map(sensor_value, MAX_MOISTURE, MIN_MOISTURE, 0, 100);

        // Print the moisture percentage to the serial monitor
        ufmt::uwrite!(&mut serial, "Moisture Level: {}%\r\n", moisture_percentage)
            .unwrap_infallible();
        ufmt::uwrite!(&mut serial, "Threshold: {}%\r\n", MOISTURE_THRESHOLD).unwrap_infallible();

        // Activate the relay if the moisture level is below the threshold
        if moisture_percentage < MOISTURE_THRESHOLD {
            relay.set_low();
            ufmt::uwrite!(
                &mut serial,
                "Relay activated for watering during {} ms.\r\n",
                WATERING_DURATION
            )
            .unwrap_infallible();
            arduino_hal::delay_ms(WATERING_DURATION);
            relay.set_high();
            ufmt::uwrite!(&mut serial, "Relay deactivated.\r\n").unwrap_infallible();
        }

        arduino_hal::delay_ms(5000);
    }
}
