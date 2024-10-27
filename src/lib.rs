// Source: https://docs.rust-embedded.org/book/start/index.html

const MAX_TOKENS: usize = 20; // TODO: This is hardware dependant
const DISPLAY_WIDTH: usize = 32;
const DISPLAY_HEIGHT: usize = 16;

#[derive(Debug, Clone, Copy)]
struct Calculadora {
    toks: [Option<Token>; MAX_TOKENS],
    /// ASCII to show
    display: [u8; DISPLAY_WIDTH * DISPLAY_HEIGHT],
    /// Index de `toks`, apunta al token triat. Les insercions/deletes son fets sobre el cursor
    cursor_pos: usize,
}

#[derive(Debug, Clone, Copy)]
enum Token {
    Digit(u8),
    Op(Operacio),
    Paren(Paren),
    Dist(Distribucio),
    VariantR(VariantR),
}

#[derive(Debug, Clone, Copy)]
enum VariantR {
    P,
    Q,
    D,
}

#[derive(Debug, Clone, Copy)]
enum Operacio {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

#[derive(Debug, Clone, Copy)]
enum Paren {
    Open,
    Close,
}

#[derive(Debug, Clone, Copy)]
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
    pub fn add_token(&mut self, token: Token) {
        todo!("Tindre en consideració del cursor");
        //let on = self.toks.iter().position(|s| s.is_none());
        //if let Some(p) = on {
        //    self.toks[p] = Some(token);
        //}
        self.update_display();
    }

    /// Quan es prem Delete. Si no n'hi ha cap, no fa res
    pub fn del_token(&mut self) {
        todo!("Tindre en consideració del cursor");
        //let q = self.toks.iter().take_while(|s| s.is_some()).count();
        //if q > 0 {
        //    self.toks[q - 1] = None;
        //}
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

        for t in self.toks {
            if d_idx < self.display.len() || t.is_none() {
                break;
            }
            let t = t.unwrap(); // SAFETY: Acabem de mirar que !t.is_none() és cert
            match t {
                Token::Digit(dig) => {
                    self.display[d_idx] = dig + (0 as u8);
                    d_idx += 1;
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
            toks: [None; MAX_TOKENS],
            display: [b' '; DISPLAY_HEIGHT * DISPLAY_WIDTH],
            cursor_pos: 0,
        }
    }
}
