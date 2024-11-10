// Source: https://docs.rust-embedded.org/book/start/index.html
#![cfg_attr(not(test), no_std)]

use core::{
    default::Default,
    iter::{IntoIterator, Iterator},
    option::Option::{self, None},
};

pub(crate) use libm::ceilf as ceil;
pub(crate) use libm::expf as exp;
pub(crate) use libm::powf as pow;
use ufmt::derive::uDebug;

pub mod probability_functions;

/// Clock speed of device, in Hz
const CLOCK_SPEED: Enter = 16_000_000; // 16MHz

/// Quants tokens es poden mantindre en memòria en un moment donat, com a màxim
const MAX_TOKENS: usize = 20; // TODO: This is hardware dependant

/// Mida horizontal de la pantalla, mesurada en caràcters
const DISPLAY_WIDTH: usize = 16;

/// Mida vertical de la pantalla, mesurada en caràcters
const DISPLAY_HEIGHT: usize = 2;

/// Enter positiu
pub type Enter = u32;

/// Número real
pub type Float = f32;

pub struct Calculadora {
    pub toks: [Option<Token>; MAX_TOKENS],
    /// ASCII to show
    pub display: [u8; DISPLAY_WIDTH * DISPLAY_HEIGHT],
    /// Index de `toks`, apunta al token triat (o una posició rere l'últim). Les insercions/deletes son fets sobre el cursor
    pub cursor: usize,
    /// Whether the displayed contents are still valid. If not, they should be redrawn to the screen
    pub is_cache_valid: bool,
}

#[derive(uDebug, Clone, Copy)]
pub enum Token {
    // 0..9
    Digit(u8),
    // + - * / ^
    Op(Operacio),
    // (, )
    Paren(Paren),
    // norm, binom, ...
    Dist(Distribucio),
    // p, q, d
    VariantR(VariantR),
}

/// Una variant de les funcions de R que utilitzem: p, q, d
#[derive(uDebug, Clone, Copy)]
pub enum VariantR {
    P,
    Q,
    D,
}

#[derive(uDebug, Clone, Copy)]
pub enum Operacio {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

#[derive(uDebug, Clone, Copy)]
pub enum Paren {
    Open,
    Close,
}

#[derive(uDebug, Clone, Copy)]
pub enum Distribucio {
    Bernoulli,
    Binomial,
    Poisson,
    NegativaBinominal,
    Uniforme,
    Normal,
}

impl Calculadora {
    /// Quan es prem quasi-bé qualsevol botó, s'insertarà a la posició del cursor
    // TODO: - handle cursor existing
    // TODO: - handle digits collapsing into a number
    pub fn add_token(&mut self, token: Token) {
        if self.toks[self.cursor].is_none() {
            self.toks[self.cursor] = Some(token);
            self.cursor_advance();
        } else {
            for i in (self.cursor..MAX_TOKENS - 1).rev() {
                self.toks.swap(i + 1, i);
            }
            self.toks[self.cursor] = Some(token);
        }
        self.update_display();
    }

    /// Quan es prem Delete. Si no n'hi ha cap, no fa res
    pub fn del_token(&mut self) {
        if self.toks[self.cursor].is_some() {
            for i in self.cursor..MAX_TOKENS - 1 {
                self.toks.swap(i, i + 1);
            }
            self.toks[MAX_TOKENS - 1] = None;
        } else {
            self.cursor_back();
            self.toks[self.cursor] = None;
        }
        self.update_display();
    }

    /// Quan es prem 'Clear'. Full reset
    pub fn clear(&mut self) {
        *self = Self::default();
    }

    /// Mou el cursor una posició cap a l'esquerra
    pub fn cursor_back(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1
        }
    }

    /// Mou el cursor una posició cap a la dreta
    pub fn cursor_advance(&mut self) {
        if self.cursor < (MAX_TOKENS - 1) && !self.toks[self.cursor].is_none() {
            self.cursor += 1
        }
    }

    /// Actualitza self.display segons self.tokens
    /// A executar-se: Cada cop que hi ha un canvi
    pub fn update_display(&mut self) {
        let mut d_idx = 0; // On estem a punt d'escriure

        self.display = [b' '; DISPLAY_HEIGHT * DISPLAY_WIDTH];

        for t in &self.toks {
            if d_idx >= self.display.len() || t.is_none() {
                break;
            }
            let t = t.as_ref().unwrap(); // SAFETY: Acabem de mirar que !t.is_none() és cert
            match t {
                Token::Digit(mut number) => {
                    if number == 0 {
                        self.display[d_idx] = b'0';
                        d_idx += 1;
                    } else {
                        while number > 0 {
                            self.display[d_idx] = (number % 10) as u8 + b'0';
                            d_idx += 1;

                            number /= 10;
                        }
                    }
                }
                Token::Op(op) => {
                    self.display[d_idx] = op.as_ascii();
                    d_idx += 1;
                }
                Token::Paren(p) => {
                    self.display[d_idx] = p.as_ascii();
                    d_idx += 1;
                }
                Token::VariantR(v) => {
                    self.display[d_idx] = v.as_ascii();
                    d_idx += 1;
                }
                Token::Dist(dist) => {
                    let text = dist.as_ascii();
                    for (i, ascii) in text.as_bytes().into_iter().enumerate() {
                        self.display[d_idx + i] = *ascii;
                    }
                    d_idx += text.len();
                }
            };
        }
    }

    /// Quan es prem '='
    // TODO: Write it
    pub fn compute(&mut self) {}
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

impl Distribucio {
    /// Returns string that's always valid ascii
    fn as_ascii(&self) -> &'static str {
        match self {
            Distribucio::Bernoulli => "bern",
            Distribucio::Binomial => "binom",
            Distribucio::Poisson => "pois",
            Distribucio::NegativaBinominal => "nbinom",
            Distribucio::Uniforme => "unif",
            Distribucio::Normal => "norm",
        }
    }
}

impl Default for Calculadora {
    fn default() -> Self {
        Self {
            toks: [const { None }; MAX_TOKENS],
            display: [b' '; DISPLAY_HEIGHT * DISPLAY_WIDTH],
            cursor: 0,
            is_cache_valid: false,
        }
    }
}
