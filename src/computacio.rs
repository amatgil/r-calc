use core::mem;

use crate::*;

// Canvis irrellevants :D

pub fn compute_tokens(toks: &[Option<Token>; MAX_TOKENS]) -> Result<TextArea, ComputError> {
    // Temporal: si no retorna Err, mostra 'yay'. Si no, mostra l'error
    to_postfix(toks).map(|_| {
        [
            b'y', b'a', b'y', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
            b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
            b' ', b' ', b' ', b' ',
        ]
    })
}

#[derive(Debug)]
pub enum ComputError {
    /// Pensive emoji
    NotYetImplemented,
    /// Mismatched parens
    MismatchedParens,
    /// `p`, `q` sense e.g. norm
    RVariantNotFollowedByDistFn,
    /// Functions like `norm` must be preceded by `p`, `q`,..
    DistNotPrecededByRVariant,
    /// Stack overflow in one of the inner stacks
    InnerSO,
    // TODO: Afegir-ne
}

impl ComputError {
    pub fn as_text(&self) -> TextArea {
        let mut r = [b' '; DISPLAY_HEIGHT * DISPLAY_WIDTH];
        let s = match self {
            ComputError::NotYetImplemented => "No implementat",
            ComputError::MismatchedParens => "Error de parentesi",
            ComputError::RVariantNotFollowedByDistFn => "p/q/d sense fn",
            ComputError::DistNotPrecededByRVariant => "fn sense p/q/d",
            ComputError::InnerSO => "Overflow stack intern",
        };

        r[..s.len().min(DISPLAY_HEIGHT * DISPLAY_WIDTH)].copy_from_slice(s.as_bytes());
        r
    }
}

// TODO: use these in to_postfix
#[derive(Clone, Copy, PartialEq, Debug)]
enum PseudoToken {
    Token(Token),
    Number(Enter),
    Op(PseudoOp),
    // TODO: the rest
}
#[derive(Debug, Clone, Copy, PartialEq)]
enum PseudoOp {
    FuncioR(VariantR, Dist),
    Paren(Paren),
    Op(Operacio),
}

/// Shunting yard implementation
fn to_postfix(
    input: &[Option<Token>; MAX_TOKENS],
) -> Result<[Option<PseudoToken>; MAX_TOKENS], ComputError> {
    use PseudoOp as POp;
    use PseudoToken as PT;

    let mut op_stack: Stack<POp> = Stack::default();
    let mut output_stack: Stack<PT> = Stack::default();

    let mut current_number: Option<Enter> = None; // Tokens go digit by digit, this is where the partial number is stored
    let mut token_idx = 0;
    while token_idx < MAX_TOKENS {
        let Some(token) = input[token_idx] else { break };

        // If we were parsing a number but it's over, push to output stack
        if current_number.is_some() && !matches!(token, Token::Digit(_)) {
            output_stack
                .push(PseudoToken::Number(current_number.unwrap()))
                .map_err(|_| ComputError::InnerSO)?;
            current_number = None;
        }

        match token {
            Token::Digit(d) => match current_number {
                None => current_number = Some(d as Enter),
                Some(n) => current_number = Some(n * 10 + d as Enter),
            },
            Token::DecimalPoint | Token::Comma => return Err(ComputError::NotYetImplemented),
            Token::Op(curr_op) => {
                // SAFETY: shortcircuiting means !is_empty() when we unwrap the top() call
                if op_stack.is_empty() || (PseudoOp::Op(curr_op) > *op_stack.top().unwrap()) {
                    op_stack
                        .push(POp::Op(curr_op))
                        .map_err(|_| ComputError::InnerSO)?;
                } else {
                    // op_stack isn't empty from the previous condition (de Morgan my beloved)
                    // TODO: Verify this is what the algorithm says
                    let lower_op = op_stack.pop().unwrap();
                    output_stack
                        .push(PT::Op(lower_op))
                        .map_err(|_| ComputError::InnerSO)?;
                    op_stack
                        .push(POp::Op(curr_op))
                        .map_err(|_| ComputError::InnerSO)?;
                }
            }
            Token::Paren(Paren::Open) => {
                op_stack
                    .push(PseudoOp::Paren(Paren::Open))
                    .map_err(|_| ComputError::InnerSO)?;
            }
            Token::Paren(Paren::Close) => {
                //let end_range = op_idx; // Non-inclusive
                //while op_idx > 0 {
                //    op_idx -= 1;
                //    if matches!(op_stack[op_idx], Some(POp::Paren(Paren::Close))) {
                //        // TODO
                //        for i in op_idx + 1..end_range {}
                //        break;
                //    }
                //}
            }
            Token::Dist(_) => return Err(ComputError::DistNotPrecededByRVariant),
            Token::VariantR(v) => {
                if token_idx + 1 < MAX_TOKENS {
                    match input[token_idx + 1] {
                        Some(Token::Dist(d)) => {
                            op_stack
                                .push(PseudoOp::FuncioR(v, d))
                                .map_err(|_| ComputError::InnerSO)?;
                            token_idx += 1; // We took 2 instead of 1
                        }
                        _ => return Err(ComputError::RVariantNotFollowedByDistFn),
                    }
                }
            }
        }

        token_idx += 1;
    }

    while let Some(op) = op_stack.pop() {
        output_stack
            .push(PT::Op(op))
            .map_err(|_| ComputError::InnerSO)?;
    }

    Ok(output_stack.as_elements())
}

fn push_to_op_stack(
    op_stack: &mut [Option<PseudoOp>; MAX_TOKENS],
    op_idx: &mut usize,
    op: PseudoOp,
) {
    op_stack[*op_idx] = Some(op);
    *op_idx += 1;
}

fn push_to_output_stack(
    output_stack: &mut [Option<PseudoToken>; MAX_TOKENS],
    output_idx: &mut usize,
    ptk: PseudoToken,
) {
    output_stack[*output_idx] = Some(ptk);
    *output_idx += 1;
}

#[test]
fn basic_to_postfix() {
    let input = [
        Some(Token::Digit(1)),
        Some(Token::Digit(2)),
        Some(Token::Digit(3)),
        Some(Token::Op(Operacio::Add)),
        Some(Token::Digit(7)),
        Some(Token::Digit(7)),
        Some(Token::Digit(7)),
        Some(Token::Op(Operacio::Add)),
        Some(Token::Digit(6)),
        Some(Token::Digit(5)),
        Some(Token::Digit(4)),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ];
    let output = [
        Some(PseudoToken::Number(123)),
        Some(PseudoToken::Number(777)),
        Some(PseudoToken::Op(PseudoOp::Op(Operacio::Add))),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ];

    let res = to_postfix(&input).unwrap_or_else(|e| panic!("Failed {e:?}"));
    for i in 0..MAX_TOKENS {
        if res[i] != output[i] {
            dbg!(res, output);
            panic!("Mismatch at {i}: {:?}, {:?}", res[i], output[i]);
        }
    }
}

impl PartialOrd for PseudoOp {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (PseudoOp::Op(o), PseudoOp::Op(u)) => o.partial_cmp(u),
            _ => None,
        }
    }
}

struct Stack<T> {
    /// Held elements
    elements: [Option<T>; MAX_TOKENS],
    /// Possible pointer to the last inserted element
    idx: Option<usize>,
}

impl<T> Stack<T> {
    /// Returns Err when amount of elements exceeds MAX_TOKENS
    fn push(&mut self, elem: T) -> Result<(), ()> {
        match self.idx {
            None => {
                self.elements[0] = Some(elem);
                self.idx = Some(0);
                Ok(())
            }
            Some(i) => {
                if (i + 1) >= MAX_TOKENS {
                    return Err(());
                }
                self.elements[i + 1] = Some(elem);
                self.idx = Some(i + 1);
                Ok(())
            }
        }
    }
    fn pop(&mut self) -> Option<T> {
        match self.idx {
            None => None,
            Some(i) => {
                let mut x = None;
                mem::swap(&mut x, &mut self.elements[i]);
                if i == 0 {
                    self.idx = None
                } else {
                    self.idx = Some(i - 1)
                }
                // SAFETY: Invariant holds, this was pointed to by `i` so it must be valid
                Some(x.unwrap())
            }
        }
    }
    fn is_empty(&self) -> bool {
        self.idx.is_none()
    }
    fn top(&self) -> Option<&T> {
        match self.idx {
            None => None,
            // SAFETY: pointed to by i, must be Some
            Some(i) => Some(self.elements[i].as_ref().unwrap()),
        }
    }
    fn as_elements(self) -> [Option<T>; MAX_TOKENS] {
        self.elements
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self {
            elements: [const { None }; MAX_TOKENS],
            idx: None,
        }
    }
}
