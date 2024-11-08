#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led = pins.d13.into_output().downgrade();

    loop {
        led.toggle();
        arduino_hal::delay_ms(1000);
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}
