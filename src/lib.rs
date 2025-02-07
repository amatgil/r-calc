// Source: https://docs.rust-embedded.org/book/start/index.html
#![cfg_attr(not(test), no_std)]

use core::{
    cmp::Ordering,
    default::Default,
    iter::{IntoIterator, Iterator},
    option::Option::{self, None},
};

use computacio::compute_tokens;
pub(crate) use libm::expf as exp;
pub(crate) use libm::powf as pow;
use ufmt::derive::uDebug;

mod computacio;
pub mod probability_functions;

/// Clock speed of device, in Hz
pub const CLOCK_SPEED: Enter = 16_000_000; // 16MHz

/// Quants tokens es poden mantindre en memòria en un moment donat, com a màxim
pub const MAX_TOKENS: usize = DISPLAY_HEIGHT * DISPLAY_WIDTH; // TODO: This is hardware dependant

/// Mida horizontal de la pantalla, mesurada en caràcters
pub const DISPLAY_WIDTH: usize = 16;

/// Mida vertical de la pantalla, mesurada en caràcters
pub const DISPLAY_HEIGHT: usize = 2;

/// La mida horitzontal del buffer de la primera linia de la LCD
/// Concretament, la HD44780 (1602A)
pub const LCD_INTERNAL_WIDTH: usize = 64; //40;

/// Mida horizontal de la scan matrix
pub const SCAN_MATRIX_WIDTH: usize = 4;

/// Mida vertical de la scan matrix
pub const SCAN_MATRIX_HEIGHT: usize = 5;

/// Enter positiu
pub type Enter = u32;

/// Número real
pub type Float = f32;

/// Array de bytes a mostrar
/// Sempre han de ser ASCII vàlid
pub type TextArea = [u8; DISPLAY_WIDTH * DISPLAY_HEIGHT];

pub struct Calculadora {
    pub toks: [Option<Token>; MAX_TOKENS],
    /// ASCII to show when the user is inputting tokens
    token_display: TextArea,
    /// ASCII to show when the user wants to see the result
    computation_display: TextArea,
    /// Index de `toks`, apunta al token triat (o una posició rere l'últim). Les insercions/deletes son fets sobre el cursor
    token_cursor: usize,
    /// Graphical cursor, where the user thinks they're inserting
    pub graphical_cursor: usize,
    /// Whether the displayed contents are still valid. If not, they should be redrawn to the screen
    pub is_cache_valid: bool,
    /// Which buffer should be displayed at the current moment
    pub currently_shown_buffer: BufferType,
    /// Shift status
    pub shift_status: ShiftStatus,
}

#[derive(uDebug, Clone, Copy, PartialEq)]
pub enum ShiftStatus {
    /// Enabled
    Si,
    // Not enabled
    No,
}
#[derive(uDebug, Clone, Copy)]
pub enum BufferType {
    /// What the user typed
    Tokens,
    /// After computing
    Resultat,
}

#[derive(uDebug, Debug, Clone, Copy, PartialEq)]
pub enum Token {
    // 0..9
    Digit(u8),
    // `.` (as in `3.2`)
    DecimalPoint,
    // `,` (as in `pnorm(3, 2)`)
    Comma,
    // + - * / ^
    Op(Operacio),
    // (, )
    Paren(Paren),
    // norm, binom, ...
    Dist(Dist),
    // p, q, d
    VariantR(VariantR),
}

/// Una variant de les funcions de R que utilitzem: p, q, d
#[derive(uDebug, Debug, Clone, Copy, PartialEq)]
pub enum VariantR {
    P,
    Q,
    D,
}

#[derive(uDebug, Debug, Clone, Copy, PartialEq)]
pub enum Operacio {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

#[derive(uDebug, Debug, Clone, Copy, PartialEq)]
pub enum Paren {
    Open,
    Close,
}

#[derive(uDebug, Debug, Clone, Copy, PartialEq)]
pub enum Dist {
    Bernoulli,
    Binom,
    Poisson,
    NBinom,
    Uniforme,
    Normal,
}

impl Calculadora {
    /// Quan es prem quasi-bé qualsevol botó, s'insertarà a la posició del cursor
    // TODO: - handle cursor existing
    // TODO: - handle digits collapsing into a number
    pub fn add_token(&mut self, token: Token) {
        if self.toks[self.token_cursor].is_none() {
            self.toks[self.token_cursor] = Some(token);
            self.cursor_advance();
        } else {
            for i in (self.token_cursor..MAX_TOKENS - 1).rev() {
                self.toks.swap(i + 1, i);
            }
            self.toks[self.token_cursor] = Some(token);
        }
        self.update_token_display();
    }
    pub fn add_dist_tokens(&mut self, dist: Dist) {
        self.add_token(Token::Dist(dist));
        self.add_token(Token::Paren(Paren::Open));
    }

    /// Quan es prem Delete. Si no n'hi ha cap, no fa res
    pub fn del_token(&mut self) {
        if self.toks[self.token_cursor].is_some() {
            for i in self.token_cursor..MAX_TOKENS - 1 {
                self.toks.swap(i, i + 1);
            }
            self.toks[MAX_TOKENS - 1] = None;
        } else {
            self.cursor_back();
            self.toks[self.token_cursor] = None;
        }
        self.update_token_display();
    }

    /// Quan es prem 'Clear'. Full reset
    pub fn clear(&mut self) {
        *self = Self::default();
    }

    /// Mou el cursor una posició cap a l'esquerra
    pub fn cursor_back(&mut self) {
        if self.token_cursor > 0 {
            self.token_cursor -= 1
        }
    }

    /// Mou el cursor una posició cap a la dreta
    pub fn cursor_advance(&mut self) {
        if self.token_cursor < (MAX_TOKENS - 1) && !self.toks[self.token_cursor].is_none() {
            self.token_cursor += 1
        }
    }

    pub fn toggle_shift(&mut self) {
        self.shift_status = match self.shift_status {
            ShiftStatus::Si => ShiftStatus::No,
            ShiftStatus::No => ShiftStatus::Si,
        }
    }
    pub fn display(&self) -> [u8; DISPLAY_WIDTH * DISPLAY_HEIGHT] {
        match self.currently_shown_buffer {
            BufferType::Tokens => self.token_display,
            BufferType::Resultat => self.computation_display,
        }
    }

    /// Actualitza self.display segons self.tokens
    /// A executar-se: Cada cop que hi ha un canvi
    pub fn update_token_display(&mut self) {
        fn update(display: &mut [u8], d_idx: &mut usize, c: u8) {
            display[*d_idx] = c;
            *d_idx += 1;
        }

        let mut d_idx = 0; // On estem a punt d'escriure
        self.token_display = [b' '; DISPLAY_HEIGHT * DISPLAY_WIDTH];

        for t in &self.toks {
            if d_idx >= self.token_display.len() || t.is_none() {
                break;
            }
            let t = t.as_ref().unwrap(); // SAFETY: Acabem de mirar que !t.is_none() és cert
            match t {
                Token::Digit(n) => update(&mut self.token_display, &mut d_idx, n + b'0'),
                Token::Op(op) => update(&mut self.token_display, &mut d_idx, op.as_ascii()),
                Token::Paren(p) => update(&mut self.token_display, &mut d_idx, p.as_ascii()),
                Token::VariantR(v) => update(&mut self.token_display, &mut d_idx, v.as_ascii()),
                Token::DecimalPoint => update(&mut self.token_display, &mut d_idx, b'.'),
                Token::Comma => update(&mut self.token_display, &mut d_idx, b','),
                Token::Dist(dist) => {
                    let text = dist.as_ascii();
                    if d_idx < DISPLAY_WIDTH && text.len() > DISPLAY_WIDTH - d_idx {
                        d_idx = DISPLAY_WIDTH;
                    } else if d_idx + text.len() > DISPLAY_WIDTH * DISPLAY_HEIGHT {
                        break;
                    }
                    for (i, ascii) in text.as_bytes().into_iter().enumerate() {
                        self.token_display[d_idx + i] = *ascii;
                    }
                    d_idx += text.len();
                }
            };
        }
        self.graphical_cursor = d_idx;
    }

    /// Quan es prem '='
    /// Recorre self.toks i computa el que demana, accessible amb Calculadora::display()
    ///
    /// Si hi ha un error de sintaxi, quedarà escrit també
    // TODO: Write it
    pub fn compute(&mut self) {
        match compute_tokens(&self.toks) {
            Ok(text) => self.computation_display = text,
            Err(e) => self.computation_display = e.as_text(),
        }
    }

    pub fn set_backbuffer_text(&mut self, text: TextArea) {
        self.computation_display = text;
    }
}

impl VariantR {
    fn as_ascii(&self) -> u8 {
        match self {
            VariantR::P => b'p',
            VariantR::Q => b'q',
            VariantR::D => b'd',
        }
    }
}

impl Operacio {
    fn as_ascii(&self) -> u8 {
        match self {
            Operacio::Add => b'+',
            Operacio::Sub => b'-',
            Operacio::Mul => b'*',
            Operacio::Div => b'/',
            Operacio::Pow => b'^',
        }
    }
}
impl Paren {
    fn as_ascii(&self) -> u8 {
        match self {
            Paren::Open => b'(',
            Paren::Close => b')',
        }
    }
}

impl Dist {
    /// Returns string that's always valid ascii
    fn as_ascii(&self) -> &'static str {
        match self {
            Dist::Bernoulli => "bern",
            Dist::Binom => "binom",
            Dist::Poisson => "pois",
            Dist::NBinom => "nbinom",
            Dist::Uniforme => "unif",
            Dist::Normal => "norm",
        }
    }
}

impl Default for Calculadora {
    fn default() -> Self {
        Self {
            toks: [const { None }; MAX_TOKENS],
            token_display: [b' '; DISPLAY_HEIGHT * DISPLAY_WIDTH],
            computation_display: [b' '; DISPLAY_HEIGHT * DISPLAY_WIDTH],
            token_cursor: 0,
            graphical_cursor: 0,
            is_cache_valid: false,
            currently_shown_buffer: BufferType::Tokens,
            shift_status: ShiftStatus::No,
        }
    }
}

// Precendence
impl PartialOrd for Operacio {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match (self, other) {
            (Operacio::Add, Operacio::Add)
            | (Operacio::Sub, Operacio::Sub)
            | (Operacio::Mul, Operacio::Mul)
            | (Operacio::Div, Operacio::Div)
            | (Operacio::Pow, Operacio::Pow)
            | (Operacio::Add, Operacio::Sub)
            | (Operacio::Sub, Operacio::Add)
            | (Operacio::Mul, Operacio::Div)
            | (Operacio::Div, Operacio::Mul) => Ordering::Equal,

            (Operacio::Add, _) => Ordering::Less,
            (Operacio::Sub, _) => Ordering::Less,
            (_, Operacio::Add) => Ordering::Greater,
            (_, Operacio::Sub) => Ordering::Greater,

            (Operacio::Mul, Operacio::Pow) => Ordering::Less,
            (Operacio::Div, Operacio::Pow) => Ordering::Less,

            (Operacio::Pow, _) => Ordering::Greater,
        })
    }
}
