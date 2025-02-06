#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use arduino_hal::{port::PinOps, prelude::*, *};
use hd44780_driver::{Cursor, CursorBlink, Display, DisplayMode};
use r_calc::{
    BufferType, Calculadora, Dist, Operacio, Paren, ShiftStatus, Token, VariantR, DISPLAY_HEIGHT,
    DISPLAY_WIDTH, LCD_INTERNAL_WIDTH, SCAN_MATRIX_HEIGHT, SCAN_MATRIX_WIDTH,
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

    //let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut calculadora = Calculadora::default();
    let mut held = false;

    // membrane pad
    let cols = [
        pins.d4.into_pull_up_input().downgrade(), // PD4
        pins.d5.into_pull_up_input().downgrade(), // PD5
        pins.d6.into_pull_up_input().downgrade(), // PD6
        pins.d7.into_pull_up_input().downgrade(), // PD7
    ];

    let mut rows = [
        pins.d3.into_output().downgrade(), // PD3
        pins.d2.into_output().downgrade(), // PD2
        pins.d1.into_output().downgrade(), // PD1
        pins.d0.into_output().downgrade(), // PD0
        pins.a5.into_output().downgrade(), // PC5
    ];

    // lcd
    let rs = pins.d9.into_output().downgrade();
    let en = pins.d8.into_output().downgrade();
    let d4 = pins.d10.into_output().downgrade();
    let d5 = pins.d11.into_output().downgrade();
    let d6 = pins.d12.into_output().downgrade();
    let d7 = pins.d13.into_output().downgrade();

    let mut delay = Delay::new();

    let mut lcd = HD44780::new_4bit(rs, en, d4, d5, d6, d7, &mut delay).unwrap();
    let _ = lcd.reset(&mut delay);
    let _ = lcd.clear(&mut delay);
    let _ = lcd.set_cursor_visibility(Cursor::Visible, &mut delay);
    let _ = lcd.set_cursor_blink(CursorBlink::On, &mut delay);

    let mut pressed: [bool; SCAN_MATRIX_HEIGHT * SCAN_MATRIX_WIDTH];

    loop {
        if !calculadora.is_cache_valid {
            calculadora.is_cache_valid = true;
            let _ = lcd.reset(&mut delay);

            let inner = calculadora.display();
            let (top, bottom) = inner.split_at(DISPLAY_WIDTH);

            let _ = lcd.set_cursor_pos(0, &mut delay);
            let _ = lcd.write_bytes(top, &mut delay);
            let _ = lcd.set_cursor_pos(LCD_INTERNAL_WIDTH as u8, &mut delay);
            let _ = lcd.write_bytes(bottom, &mut delay);
            let lcd_cursor_pos = if calculadora.graphical_cursor >= DISPLAY_WIDTH {
                LCD_INTERNAL_WIDTH + (calculadora.graphical_cursor - DISPLAY_WIDTH)
            } else {
                calculadora.graphical_cursor
            };
            //uwriteln!(&mut serial, "REDRAWING {}", lcd_cursor_pos).unwrap_infallible();
            let _ = lcd.set_cursor_pos(lcd_cursor_pos as u8, &mut delay);
            let _ = match calculadora.shift_status {
                ShiftStatus::Si => lcd.set_cursor_blink(CursorBlink::On, &mut delay),
                ShiftStatus::No => lcd.set_cursor_blink(CursorBlink::Off, &mut delay),
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
                            (S::No, 0) => calculadora.add_token(Token::Digit(0)),
                            (S::Si, 0) => {}

                            (S::No, 1) => calculadora.add_token(Token::Paren(Paren::Open)),
                            (S::Si, 1) => calculadora.add_token(Token::DecimalPoint),

                            (S::No, 2) => calculadora.add_token(Token::Paren(Paren::Close)),
                            (S::Si, 2) => calculadora.add_token(Token::Comma),

                            (_, 3) => {
                                calculadora.compute();
                                calculadora.currently_shown_buffer = BufferType::Resultat;
                                let _ = lcd.set_cursor_visibility(Cursor::Invisible, &mut delay);
                                let _ = lcd.set_cursor_blink(CursorBlink::Off, &mut delay);
                            }
                            (S::No, 4) => calculadora.add_token(Token::Digit(1)),
                            (S::Si, 4) => calculadora.add_dist(Dist::Binom),

                            (S::No, 5) => calculadora.add_token(Token::Digit(2)),
                            (S::Si, 5) => calculadora.add_dist_tokens(Dist::NBinom),

                            (S::No, 6) => calculadora.add_token(Token::Digit(3)),
                            (S::Si, 6) => calculadora.add_dist_tokens(Dist::Uniforme),

                            (_, 7) => calculadora.add_token(Token::Op(Operacio::Add)),

                            (S::No, 8) => calculadora.add_token(Token::Digit(4)),
                            (S::Si, 8) => calculadora.add_dist_tokens(Dist::Bernoulli),

                            (S::No, 9) => calculadora.add_token(Token::Digit(5)),
                            (S::Si, 9) => calculadora.add_dist_tokens(Dist::Poisson),

                            (S::No, 10) => calculadora.add_token(Token::Digit(6)),
                            (S::Si, 10) => calculadora.add_dist(Dist::Norm),

                            (S::No, 11) => calculadora.add_token(Token::Op(Operacio::Sub)),
                            (S::Si, 11) => calculadora.add_token(Token::Op(Operacio::Div)),

                            (S::No, 12) => calculadora.add_token(Token::Digit(7)),
                            (S::Si, 12) => calculadora.add_token(Token::VariantR(VariantR::P)),

                            (S::No, 13) => calculadora.add_token(Token::Digit(8)),
                            (S::Si, 13) => calculadora.add_token(Token::VariantR(VariantR::Q)),

                            (S::No, 14) => calculadora.add_token(Token::Digit(9)),
                            (S::Si, 14) => calculadora.add_token(Token::VariantR(VariantR::D)),

                            (S::No, 15) => calculadora.add_token(Token::Op(Operacio::Mul)),
                            (S::Si, 15) => calculadora.add_token(Token::Op(Operacio::Pow)),

                            (_, 16) => calculadora.toggle_shift(),

                            (_, 17) => calculadora.cursor_back(),

                            (_, 18) => calculadora.cursor_advance(),

                            (S::No, 19) => calculadora.clear(),
                            (S::Si, 19) => calculadora.del_token(),

                            // Pressing a button with no defined Shift should reset shift
                            (S::Si, _unassigned_button) => calculadora.toggle_shift(),

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
