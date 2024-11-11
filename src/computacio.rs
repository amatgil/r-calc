use crate::*;

pub fn compute_tokens(toks: &[Option<Token>]) -> Result<TextArea, ComputationError> {
    //Err(ComputationError::NotYetImplemented)
    Err(ComputationError::MismatchedParens)
}

pub enum ComputationError {
    NotYetImplemented,
    MismatchedParens,
    // TODO: Afegir-ne
}

impl ComputationError {
    pub fn as_text(&self) -> TextArea {
        let mut r = [b' '; DISPLAY_HEIGHT * DISPLAY_WIDTH];
        let s = match self {
            ComputationError::NotYetImplemented => "No implementat",
            ComputationError::MismatchedParens => "Error de parentesi",
        };

        r[..s.len().min(DISPLAY_HEIGHT * DISPLAY_WIDTH)].copy_from_slice(s.as_bytes());
        r
    }
}
