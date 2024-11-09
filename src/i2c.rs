use crate::ceil;
use core::hint::black_box;

use arduino_hal::port::{mode::Output, Pin};

use crate::{Enter, Float, CLOCK_SPEED};

/// Assumes the bus is always open
/// Uses 'standard mode' (100kHz)
// https://www.ti.com/lit/an/sbaa565/sbaa565.pdf?ts=1731129221780&ref_url=https%253A%252F%252Fwww.google.com%252F
pub fn send_data(scl: &mut Pin<Output>, sda: &mut Pin<Output>, address: u8, data: &[u8]) {
    //  === Start seq ===
    // I2C is pull-up: pulling down in this order means we're initiating
    // We claim the bus: we assume it's not claimed because we're the only master
    sda.set_low();
    wait_nanos(6);
    scl.set_low();
    wait_nanos(6);

    // == Send address ==
    // TODO: send address
    // TODO: check acknowledgement

    // === Send data ===
    for byte in data {
        for i in 0..8 {
            scl.set_high();
            wait_nanos(6);

            let i = 7 - i; // MSB order
            if (byte & (1 << i)) > 0 {
                sda.set_high();
            } else {
                sda.set_low();
            }
            wait_nanos(6);
            scl.set_low();

            wait_nanos(6);
        }
        // TODO: Check acknowledgement
    }

    // === Stop seq ===
    // I2C is pull-up: pulling up in this order means we're done
    scl.set_high();
    wait_nanos(6);
    sda.set_high();
    wait_nanos(6);
}

/// Sleep, in nanoseconds
pub fn wait_nanos(ns: u32) {
    arduino_hal::delay_us(ns);
}
/// Sleep, in milliseconds
pub fn wait_millis(ms: u16) {
    arduino_hal::delay_ms(ms);
}
