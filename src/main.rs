#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use hd44780_driver::{Cursor, CursorBlink, Display, DisplayMode};
use r_calc::{Calculadora, Operacio, Paren, Token};

#[cfg(not(test))]
#[arduino_hal::entry]
fn main() -> ! {
    use arduino_hal::Delay;
    use hd44780_driver::HD44780;

    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    //let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut calculadora = Calculadora::default();
    let mut held = false;

    // membrane pad
    let cols = [
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

    // lcd
    let rs = pins.a0.into_output().downgrade();
    let en = pins.a1.into_output().downgrade();
    let d4 = pins.a2.into_output().downgrade();
    let d5 = pins.a3.into_output().downgrade();
    let d6 = pins.a4.into_output().downgrade();
    let d7 = pins.a5.into_output().downgrade();

    //lcd_init(&mut rs, &mut en, &mut d4, &mut d5, &mut d6, &mut d7);
    //lcd_clear(&mut rs, &mut en, &mut d4, &mut d5, &mut d6, &mut d7);
    let mut delay = Delay::new();

    let mut lcd = HD44780::new_4bit(rs, en, d4, d5, d6, d7, &mut delay).unwrap();
    let _ = lcd.reset(&mut delay);

    let _ = lcd.clear(&mut delay);

    let _ = lcd.set_cursor_visibility(Cursor::Visible, &mut delay);
    let _ = lcd.set_cursor_blink(CursorBlink::On, &mut delay);
    loop {
        if !calculadora.is_cache_valid {
            calculadora.is_cache_valid = true;
            let _ = lcd.reset(&mut delay);

            let _ = lcd.write_str(
                &core::str::from_utf8(&calculadora.display).unwrap_or("oopsie"),
                &mut delay,
            );
            let _ = lcd.set_cursor_pos(calculadora.cursor as u8, &mut delay);
        }

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

        //let _ = ufmt::uwriteln!(&mut serial, "Cursor: {:?}", calculadora.cursor);

        if !held && pressed[0] {
            calculadora.add_token(Token::Paren(Paren::Open));
            calculadora.is_cache_valid = false;
        }
        if !held && pressed[1] {
            calculadora.add_token(Token::Op(Operacio::Add));
            calculadora.is_cache_valid = false;
        }
        if !held && pressed[2] {
            calculadora.del_token();
            calculadora.is_cache_valid = false;
        }
        if !held && pressed[3] {
            calculadora.clear();
            calculadora.is_cache_valid = false;
        }
        if !held && pressed[4] {
            calculadora.cursor_back();
            calculadora.is_cache_valid = false;
        }
        if !held && pressed[5] {
            calculadora.cursor_advance();
            calculadora.is_cache_valid = false;
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
