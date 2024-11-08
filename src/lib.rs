// Source: https://docs.rust-embedded.org/book/start/index.html
#![cfg_attr(not(test), no_std)]

use core::{
    default::Default,
    iter::{IntoIterator, Iterator},
    option::Option::{self, None},
    panic::PanicInfo,
};

pub mod float_math;
pub mod probability_functions;

/// Quants tokens es poden mantindre en memòria en un moment donat, com a màxim
const MAX_TOKENS: usize = 20; // TODO: This is hardware dependant

/// Mida horizontal de la pantalla, mesurada en caràcters
const DISPLAY_WIDTH: usize = 32;

/// Mida vertical de la pantalla, mesurada en caràcters
const DISPLAY_HEIGHT: usize = 16;

/// Enter positiu
pub type Enter = u32;

/// Número real
pub type Float = f32;

struct Calculadora {
    toks: [Option<Token>; MAX_TOKENS],
    /// ASCII to show
    display: [u8; DISPLAY_WIDTH * DISPLAY_HEIGHT],
    /// Index de `toks`, apunta al token triat (o una posició rere l'últim). Les insercions/deletes son fets sobre el cursor
    cursor_pos: usize,
}

enum Token {
    // 0..9
    // TODO: reasses number type (hardware dependant)
    Number(usize),
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
enum VariantR {
    P,
    Q,
    D,
}

enum Operacio {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

enum Paren {
    Open,
    Close,
}

enum Distribucio {
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
        // TODO: Tindre en consideració del cursor"
        //let on = self.toks.iter().position(|s| s.is_none());
        //if let Some(p) = on {
        //    self.toks[p] = Some(token);
        //}
        self.update_display();
    }

    /// Quan es prem Delete. Si no n'hi ha cap, no fa res
    pub fn del_token(&mut self) {
        // TODO: Tindre en consideració del cursor"
        let q = self.toks.iter().take_while(|s| s.is_some()).count();
        if q > 0 {
            self.toks[q - 1] = None;
        }
        self.update_display();
    }

    /// Quan es prem 'Clear'. Full reset
    pub fn clear(&mut self) {
        *self = Self::default();
    }

    /// Actualitza self.display segons self.tokens
    /// A executar-se: Cada cop que hi ha una entrada
    pub fn update_display(&mut self) {
        let mut d_idx = 0; // On estem a punt d'escriure

        self.display = [b' '; DISPLAY_HEIGHT * DISPLAY_WIDTH];

        for t in &self.toks {
            if d_idx < self.display.len() || t.is_none() {
                break;
            }
            let t = t.as_ref().unwrap(); // SAFETY: Acabem de mirar que !t.is_none() és cert
            match t {
                Token::Number(mut number) => {
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
            cursor_pos: 0,
        }
    }
}
