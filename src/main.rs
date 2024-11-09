#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use arduino_hal::prelude::_unwrap_infallible_UnwrapInfallible;
use hd44780_driver::{Cursor, CursorBlink, Display, DisplayMode};
use r_calc::{Calculadora, Operacio, Paren, Token};

#[cfg(not(test))]
#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut calculadora = Calculadora::default();
    let mut held = false;

    // membrane pad
    let mut cols = [
        pins.d8.into_pull_up_input().downgrade(),
        pins.d9.into_pull_up_input().downgrade(),
        pins.d10.into_pull_up_input().downgrade(),
        pins.d11.into_pull_up_input().downgrade(),
    ];

    let mut rows = [
        pins.d4.into_output().downgrade(),
        pins.d5.into_output().downgrade(),
        pins.d6.into_output().downgrade(),
        pins.d7.into_output().downgrade(),
    ];

    loop {
        // read 4x4 pad
        let mut pressed = [false; 16];
        for row in 0..4 {
            rows[row].set_low();
            for col in 0..4 {
                if cols[col].is_low() {
                    pressed[row * 4 + col] = true;
                }
            }
            rows[row].set_high();
        }

        //let _ = ufmt::uwriteln!(&mut serial, "Buttons pressed: {:?}", pressed);
        //let _ = ufmt::uwriteln!(
        //    &mut serial,
        //    "State: {}",
        //    core::str::from_utf8(&calculadora.display).unwrap()
        //);
        let _ = ufmt::uwriteln!(
            &mut serial,
            "State: {:?}",
            //core::str::from_utf8(&calculadora.display).unwrap()
            calculadora.toks
        );
        /*let _ = ufmt::uwriteln!(
            &mut serial,
            "buttons: {:?}",
            pressed.map(|b| if b { 1 } else { 0 })
        );*/
        let _ = ufmt::uwriteln!(&mut serial, "Cursor: {:?}", calculadora.cursor);

        if !held && pressed[0] {
            calculadora.add_token(Token::Paren(Paren::Open));
        }
        if !held && pressed[1] {
            calculadora.add_token(Token::Op(Operacio::Add));
        }
        if !held && pressed[2] {
            calculadora.del_token();
        }
        if !held && pressed[3] {
            calculadora.clear();
        }
        if !held && pressed[4] {
            calculadora.cursor_back();
        }
        if !held && pressed[5] {
            calculadora.cursor_advance();
        }

        if pressed.iter().all(|b| !b) {
            held = false;
        } else {
            held = true;
        }
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
