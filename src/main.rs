#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use arduino_hal::prelude::_unwrap_infallible_UnwrapInfallible;
use hd44780_driver::{Cursor, CursorBlink, Display, DisplayMode};
use r_calc::{
    BufferType, Calculadora, Operacio, Paren, ShiftStatus, Token, DISPLAY_HEIGHT, DISPLAY_WIDTH,
    LCD_INTERNAL_WIDTH, SCAN_MATRIX_HEIGHT, SCAN_MATRIX_WIDTH,
};
use ufmt::uwriteln;

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
        if !calculadora.is_cache_valid {
            //uwriteln!(&mut serial, "REDRAWING").unwrap_infallible();
            calculadora.is_cache_valid = true;
            let _ = lcd.reset(&mut delay);

            let inner = calculadora.display();
            let (top, bottom) = inner.split_at(DISPLAY_WIDTH);

            let _ = lcd.set_cursor_pos(0, &mut delay);
            let _ = lcd.write_bytes(top, &mut delay);
            let _ = lcd.set_cursor_pos(LCD_INTERNAL_WIDTH as u8, &mut delay);
            let _ = lcd.write_bytes(bottom, &mut delay);
            let _ = lcd.set_cursor_pos(calculadora.cursor as u8, &mut delay);
            let _ = match calculadora.shift_status {
                ShiftStatus::On => lcd.set_cursor_blink(CursorBlink::On, &mut delay),
                ShiftStatus::Off => lcd.set_cursor_blink(CursorBlink::Off, &mut delay),
            };
        }

        pressed = [false; SCAN_MATRIX_HEIGHT * SCAN_MATRIX_WIDTH];
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

        // No pressing multiple keys allowed
        if pressed.map(|b| if b { 1 } else { 0 }).iter().sum::<Enter>() > 1 {
            continue;
        }

        let is_curr_pressed = pressed.iter().any(|&b| b);

        // If nothing is pressed, no need to change
        if let Some(pressed_idx) = pressed.iter().position(|b| *b) {
            match calculadora.currently_shown_buffer {
                BufferType::Tokens => {
                    use ShiftStatus as S;
                    if !held {
                        calculadora.is_cache_valid = false;
                        match (calculadora.shift_status, pressed_idx) {
                            (S::Off, 0) => calculadora.add_token(Token::Paren(Paren::Open)),
                            (S::Off, 1) => calculadora.add_token(Token::Op(Operacio::Add)),
                            (S::Off, 2) => calculadora.del_token(),
                            (S::Off, 3) => calculadora.clear(),
                            (S::Off, 4) => calculadora.cursor_back(),
                            (S::Off, 5) => calculadora.cursor_advance(),
                            (S::Off, 8) => calculadora.add_token(Token::Digit(0)),
                            (S::Off, 9) => calculadora.add_token(Token::Digit(1)),
                            (S::Off, 10) => calculadora.add_token(Token::Digit(2)),
                            (S::Off, 11) => calculadora.add_token(Token::Digit(3)),
                            (S::Off, 12) => calculadora.add_token(Token::Digit(4)),
                            (S::Off, 13) => calculadora
                                .add_token(Token::Dist(r_calc::Distribucio::NegativaBinominal)),
                            (S::Off, 14) => calculadora.toggle_shift(),
                            (S::Off, 15) => {
                                calculadora.compute();
                                calculadora.currently_shown_buffer = BufferType::Resultat;

                                let _ = lcd.set_cursor_visibility(Cursor::Invisible, &mut delay);
                                let _ = lcd.set_cursor_blink(CursorBlink::Off, &mut delay);
                            }
                            (S::On, 13) => {
                                calculadora.add_token(Token::Dist(r_calc::Distribucio::Normal))
                            }

                            (S::On, _) => {
                                calculadora.shift_status = S::Off;
                            }
                            _ => {} // unreachable if scan matrix is set up right
                        }
                    }
                }
                BufferType::Resultat => {
                    if !held && is_curr_pressed {
                        calculadora.is_cache_valid = false;
                        calculadora.currently_shown_buffer = BufferType::Tokens;

                        let _ = lcd.set_cursor_visibility(Cursor::Visible, &mut delay);
                        let _ = lcd.set_cursor_blink(CursorBlink::On, &mut delay);
                    } else {
                        held = false;
                    }
                }
            }
        }
        match (held, is_curr_pressed) {
            (true, false) => held = false,
            (false, true) => held = true,
            _ => {}
        }
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
