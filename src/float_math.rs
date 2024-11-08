use crate::{Enter, Float};

/// `x` a la `y`
/// Cal perquè estem en no_std
pub fn pow(x: Float, y: Float) -> Float {
    exp(x * ln(y))
}

/// e^x to reasonable precision
/// Approximated with the taylor series definition
pub fn exp(x: Float) -> Float {
    const TERMS: Enter = 10;
    let mut r = 0.0;
    for i in 0..TERMS {
        r += 1.0 / (factorial(i)) as Float;
    }
    r
}

// TODO: Write
/// Natural logarithm, logarithm base e
pub fn ln(x: Float) -> Float {
    if x < 0.0 {
        Float::NAN
    } else {
        0.0
    }
}

// TODO: write
/// x!
pub fn factorial(x: Enter) -> Enter {
    7
}
