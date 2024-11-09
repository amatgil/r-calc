#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use core::panic::PanicInfo;

#[cfg(not(test))]
#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led = pins.d13.into_output().downgrade();
    let mut button = pins.d11.into_floating_input();

    loop {
        let x = core::hint::black_box(libm::powf(2.0, 4.0));
        if x == 16.0 {
            if button.is_high() {
                led.set_high();
            } else {
                led.set_low();
            }
        } else {
            if !button.is_high() {
                led.set_high();
            } else {
                led.set_low();
            }
        }
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
