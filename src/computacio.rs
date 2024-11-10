use crate::*;

pub fn compute_tokens(toks: &[Option<Token>]) -> Result<TextArea, ComputationError> {
    Err(ComputationError::NotYetImplemented)
}

pub enum ComputationError {
    NotYetImplemented,
    MismatchedParens,
    // TODO: Afegir-ne
}

impl ComputationError {
    pub fn as_text(&self) -> TextArea {
        match self {}
    }
}
