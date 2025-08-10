#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use riego_rs::{map, panic as _};

// For some reason the sensor values are inverted
const MIN_MOISTURE: u16 = 540;
const MAX_MOISTURE: u16 = 230;

const MOISTURE_THRESHOLD: u16 = 30; // Threshold for activating the relay
const MOISTURE_TARGET: u16 = 50; // Target moisture level to maintain
const WATERING_DURATION: u32 = 200; // Duration to activate the relay in milliseconds

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

    let mut watering = false;

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
        ufmt::uwrite!(&mut serial, "Target: {}%\r\n", MOISTURE_TARGET).unwrap_infallible();

        // Water if below threshold until the moisture level reaches the target
        if !watering && moisture_percentage < MOISTURE_THRESHOLD || watering && moisture_percentage < MOISTURE_TARGET {
            relay.set_low();
            watering = true;
            ufmt::uwrite!(&mut serial, "Watering until {}%\r\n", MOISTURE_TARGET).unwrap_infallible();
            ufmt::uwrite!(
                &mut serial,
                "Relay activated for watering during {} ms.\r\n",
                WATERING_DURATION
            )
            .unwrap_infallible();
            arduino_hal::delay_ms(WATERING_DURATION);
            relay.set_high();
            ufmt::uwrite!(&mut serial, "Relay deactivated.\r\n").unwrap_infallible();
        } else {
            watering = false;
            ufmt::uwrite!(&mut serial, "No watering needed.\r\n").unwrap_infallible();
        }

        arduino_hal::delay_ms(1000);
    }
}