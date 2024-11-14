use crate::*;

pub fn compute_tokens(toks: &[Option<Token>]) -> Result<TextArea, ComputationError> {
    //Err(ComputationError::NotYetImplemented)
    Err(ComputationError::MismatchedParens)
}

#[derive(Debug)]
pub enum ComputationError {
    NotYetImplemented,
    MismatchedParens,
    RVariantNotFollowedByDistFn,
    DistNotPrecededByRVariant,
    // TODO: Afegir-ne
}

impl ComputationError {
    pub fn as_text(&self) -> TextArea {
        let mut r = [b' '; DISPLAY_HEIGHT * DISPLAY_WIDTH];
        let s = match self {
            ComputationError::NotYetImplemented => "No implementat",
            ComputationError::MismatchedParens => "Error de parentesi",
            ComputationError::RVariantNotFollowedByDistFn => "p/q/d sense funció",
            ComputationError::DistNotPrecededByRVariant => "funció sense p/q/d",
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
    FuncioR(VariantR, Distribucio),
    Paren(Paren),
    Op(Operacio),
}

/// Shunting yard implementation
fn to_postfix(
    input: &[Option<Token>; MAX_TOKENS],
) -> Result<[Option<PseudoToken>; MAX_TOKENS], ComputationError> {
    use PseudoOp as POp;
    use PseudoToken as PT;

    let mut op_stack: [Option<POp>; MAX_TOKENS] = [None; MAX_TOKENS];
    let mut op_idx = 0; // Where we are in the operator stack

    let mut output_stack: [Option<PT>; MAX_TOKENS] = [None; MAX_TOKENS];
    let mut output_idx = 0; // Where we are in the output stack

    let mut current_number: Option<Enter> = None; // Tokens go digit by digit, this is where the partial number is stored
    let mut token_idx = 0;
    while token_idx < MAX_TOKENS {
        let Some(token) = input[token_idx] else { break };
        match token {
            Token::Digit(d) => match current_number {
                None => current_number = Some(d as Enter),
                Some(n) => current_number = Some(n * 10 + d as Enter),
            },
            Token::Op(curr_op) => {
                // SAFETY: shortcircuiting means we've always seen one operator when we index into it
                if op_idx == 0 || (PseudoOp::Op(curr_op) > op_stack[op_idx].unwrap()) {
                    push_to_op_stack(&mut op_stack, &mut op_idx, POp::Op(curr_op));
                } else {
                    // TODO
                }
            }
            Token::Paren(Paren::Open) => {
                push_to_op_stack(&mut op_stack, &mut op_idx, PseudoOp::Paren(Paren::Open))
            }
            Token::Paren(Paren::Close) => {
                let end_range = op_idx; // Non-inclusive
                while op_idx > 0 {
                    op_idx -= 1;
                    if matches!(op_stack[op_idx], Some(POp::Paren(Paren::Close))) {
                        // TODO
                        for i in op_idx + 1..end_range {}
                        break;
                    }
                }
            }
            Token::Dist(_) => return Err(ComputationError::DistNotPrecededByRVariant),
            Token::VariantR(v) => {
                if token_idx + 1 < MAX_TOKENS {
                    match input[token_idx + 1] {
                        Some(Token::Dist(d)) => {
                            push_to_op_stack(&mut op_stack, &mut op_idx, PseudoOp::FuncioR(v, d));
                            token_idx += 1; // We took 2 instead of 1
                            op_idx += 1;
                        }
                        _ => return Err(ComputationError::RVariantNotFollowedByDistFn),
                    }
                }
            }
        }

        token_idx += 1;
        if current_number.is_some() && !matches!(token, Token::Digit(_)) {
            push_to_output_stack(
                &mut output_stack,
                &mut output_idx,
                PseudoToken::Number(current_number.unwrap()),
            );
            current_number = None;
        }
    }

    while op_idx > 0 {
        op_idx -= 1;
        let op = op_stack[op_idx].unwrap();
        push_to_output_stack(&mut output_stack, &mut output_idx, PseudoToken::Op(op));
    }

    Ok(output_stack)
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
    let mut output = [
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

struct Stack {}
