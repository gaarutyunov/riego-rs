#![no_std]

use core::ops::{Add, Div, Mul, Sub};

pub fn map<T>(value: T, in_min: T, in_max: T, out_min: T, out_max: T) -> T
where
    T: Copy + PartialOrd + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Add<Output = T>,
{
    (value - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

#[cfg(not(doc))]
#[panic_handler]
pub fn panic(info: &core::panic::PanicInfo) -> ! {
    // disable interrupts - firmware has panicked so no ISRs should continue running
    avr_device::interrupt::disable();

    // get the peripherals so we can access serial and the LED.
    //
    // SAFETY: Because main() already has references to the peripherals this is an
    // unsafe operation - but because no other code can run after the panic
    // handler was called, we know it is okay.
    let dp = unsafe { arduino_hal::Peripherals::steal() };
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    // Print panic location:
    if let Some(loc) = info.location() {
        ufmt::uwriteln!(&mut serial, "  At {}:{}:{}\r", loc.file(), loc.line(), loc.column(),).ok();
    }

    // Blink LED rapidly
    let mut led = pins.d13.into_output();
    loop {
        led.toggle();
        arduino_hal::delay_ms(100);
    }
}
