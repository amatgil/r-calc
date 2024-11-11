#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use hd44780_driver::{Cursor, CursorBlink, Display, DisplayMode};
use r_calc::{
    BufferType, Calculadora, Operacio, Paren, Token, DISPLAY_HEIGHT, DISPLAY_WIDTH,
    LCD_INTERNAL_WIDTH, SCAN_MATRIX_HEIGHT, SCAN_MATRIX_WIDTH,
};

//#[cfg(not(test))]
#[arduino_hal::entry]
fn main() -> ! {
    use arduino_hal::Delay;
    use hd44780_driver::HD44780;
    use r_calc::Enter;

    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

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

    let mut delay = Delay::new();

    let mut lcd = HD44780::new_4bit(rs, en, d4, d5, d6, d7, &mut delay).unwrap();
    let _ = lcd.reset(&mut delay);

    let _ = lcd.clear(&mut delay);

    let _ = lcd.set_cursor_visibility(Cursor::Visible, &mut delay);
    let _ = lcd.set_cursor_blink(CursorBlink::On, &mut delay);

    let mut pressed: [bool; SCAN_MATRIX_HEIGHT * SCAN_MATRIX_WIDTH];
    loop {
        pressed = [false; SCAN_MATRIX_HEIGHT * SCAN_MATRIX_WIDTH];
        if !calculadora.is_cache_valid {
            calculadora.is_cache_valid = true;
            let _ = lcd.reset(&mut delay);

            let inner = calculadora.display();
            let (top, bottom) = inner.split_at(DISPLAY_WIDTH);

            let _ = lcd.set_cursor_pos(0, &mut delay);
            let _ = lcd.write_bytes(top, &mut delay);

            let _ = lcd.set_cursor_pos(LCD_INTERNAL_WIDTH as u8, &mut delay);
            let _ = lcd.write_bytes(bottom, &mut delay);

            let _ = lcd.set_cursor_pos(calculadora.cursor as u8, &mut delay);
        }

        // read scan matrix
        for row in 0..SCAN_MATRIX_HEIGHT {
            rows[row].set_low();
            for col in 0..SCAN_MATRIX_WIDTH {
                if cols[col].is_low() {
                    pressed[row * SCAN_MATRIX_WIDTH + col] = true;
                }
            }
            rows[row].set_high();
        }
        if pressed
            .map(|b| if b { 1 as Enter } else { 0 })
            .iter()
            .sum::<Enter>()
            > 1
        {
            continue; // No pressing multiple keys allowed
        }

        if !held && pressed.iter().any(|&b| b) {
            calculadora.is_cache_valid = false;
        }
        match calculadora.currently_shown_buffer {
            BufferType::Tokens => {
                if !held && pressed[0] {
                    calculadora.add_token(Token::Paren(Paren::Open));
                } else if !held && pressed[1] {
                    calculadora.add_token(Token::Op(Operacio::Add));
                } else if !held && pressed[2] {
                    calculadora.del_token();
                } else if !held && pressed[3] {
                    calculadora.clear();
                } else if !held && pressed[4] {
                    calculadora.cursor_back();
                } else if !held && pressed[5] {
                    calculadora.cursor_advance();
                } else if !held && pressed[8] {
                    calculadora.add_token(Token::Digit(0));
                } else if !held && pressed[9] {
                    calculadora.add_token(Token::Digit(1));
                } else if !held && pressed[10] {
                    calculadora.add_token(Token::Digit(2));
                } else if !held && pressed[11] {
                    calculadora.add_token(Token::Digit(3));
                } else if !held && pressed[12] {
                    calculadora.add_token(Token::Digit(4));
                } else if !held && pressed[13] {
                    calculadora.add_token(Token::Dist(r_calc::Distribucio::NegativaBinominal));
                } else if !held && pressed[15] {
                    calculadora.compute();
                    calculadora.currently_shown_buffer = BufferType::Resultat;

                    let _ = lcd.set_cursor_visibility(Cursor::Invisible, &mut delay);
                    let _ = lcd.set_cursor_blink(CursorBlink::Off, &mut delay);
                }
            }
            BufferType::Resultat => {
                if !held && pressed.iter().any(|&b| b) {
                    calculadora.currently_shown_buffer = BufferType::Tokens;

                    let _ = lcd.set_cursor_visibility(Cursor::Visible, &mut delay);
                    let _ = lcd.set_cursor_blink(CursorBlink::On, &mut delay);
                } else {
                    held = false;
                }
            }
        }
        match (held, pressed.iter().any(|&b| b)) {
            (false, true) => held = true,
            (true, false) => held = false,
            _ => {}
        }
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
