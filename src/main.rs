#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use riego_rs::map;

use riego_rs::{panic as _};

// For some reason the sensor values are inverted
const MIN_MOISTURE: u16 = 535;
const MAX_MOISTURE: u16 = 265;


#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    // Moisture sensor connected to A0
    let sensor_pin = pins.a0.into_analog_input(&mut adc);

    loop {
        let sensor_value: u16 = sensor_pin.analog_read(&mut adc);
        ufmt::uwrite!(&mut serial, "Raw Sensor Value: {}\r\n", sensor_value).unwrap_infallible();
        // Adjust the sensor value to a range to avoid overflow
        let sensor_value = sensor_value.clamp(MAX_MOISTURE, MIN_MOISTURE);

        // Convert the raw ADC value to a percentage
        let moisture_percentage = 100 - map(sensor_value, MAX_MOISTURE, MIN_MOISTURE, 0, 100);

        // Print the moisture percentage to the serial monitor
        ufmt::uwrite!(&mut serial, "Moisture Level: {}%\r\n", moisture_percentage).unwrap_infallible();

        arduino_hal::delay_ms(1000);
    }
}
