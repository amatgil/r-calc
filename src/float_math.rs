use crate::{Enter, Float};

const TERMS: Enter = 10;
const FACTORIAL_LOOKUP: [Enter; TERMS as usize] = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];

/// `x` a la `y`
/// Cal perquè estem en no_std
pub fn pow(x: Float, y: Float) -> Float {
    exp(x * ln(y))
}

/// e^x to reasonable precision
/// Approximated with the taylor series definition
pub fn exp(x: Float) -> Float {
    let mut r = 0.0;
    let mut numerator = 1.0;
    for i in 0..TERMS {
        r += numerator / FACTORIAL_LOOKUP[i as usize] as Float;
        numerator *= x;
    }
    r
}

// TODO: Write
/// Logaritme base e
pub fn ln(x: Float) -> Float {
    if x <= 0.0 {
        Float::NAN
    } else {
        0.0
    }
}

/// x!
#[inline(always)]
pub fn factorial(x: Enter) -> Enter {
    let mut r = 1;
    for i in 1..x {
        r *= i
    }
    r
}
