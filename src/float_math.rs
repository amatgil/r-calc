use crate::{Enter, Float};

/// How many terms of the taylor series of [exp] to compute
const EXP_TAYLOR_TERMS: Enter = 13;

/// How many terms of the taylor series of [ln] to compute
const LN_TAYLOR_TERMS: Enter = 15; // TODO: Make sure this runs fast enough

/// Pre-computed table of factorial (up to 2^32)
const FACTORIAL_LOOKUP: [Enter; 13] = [
    1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800, 39916800, 479001600,
];

const _ASSERT_TAYLOR_TERMS_ARE_UNDER_FACT_TABLE: () = {
    if EXP_TAYLOR_TERMS > FACTORIAL_LOOKUP.len() as u32 {
        panic!("Taylor cannot use more terms than we know the factorial function of")
    }
};

/// n!
#[inline(always)]
pub fn factorial(n: Enter) -> Enter {
    FACTORIAL_LOOKUP[n as usize]
}

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
    for i in 0..EXP_TAYLOR_TERMS {
        r += numerator / factorial(i) as Float;
        numerator *= x;
    }
    r
}

// TODO: Write
/// Logaritme base e
// https://stackoverflow.com/questions/64894566/how-to-write-a-simple-logarithm-function-without-math-h
pub fn ln(mut x: Float) -> Float {
    const E: Float = core::f64::consts::E as Float;
    if x <= 0.0 {
        Float::NAN
    } else {
        let mut power_adjust = 0;
        while x > 1.0 {
            x /= E;
            power_adjust += 1;
        }
        while x < 0.25 {
            x *= E;
            power_adjust -= 1;
        }

        x -= 1.0;
        let mut t = 0.0;
        let mut s = 1.0;
        let mut z = x;
        for k in 1..LN_TAYLOR_TERMS {
            t += z * s / k as Float;
            z *= x;
            s = -s;
        }
        t + power_adjust as Float
    }
}

#[cfg(test)]
mod tests {
    const FLOAT_DELTA: Float = 0.0005;
    const E: Float = core::f64::consts::E as Float;

    use super::*;

    #[test]
    fn exp_test() {
        let xs = [1.0, 2.0, 3.0, 4.0, 0.5, 0.1, 10.0];

        for x in xs {
            let y = exp(x);
            let expected = E.powf(x);
            dbg!(x, y, expected);
            assert!((y - expected).abs() < FLOAT_DELTA);
        }
    }

    #[test]
    fn ln_test() {
        let xs = [
            1.0, 2.0, 3.0, 4.0, 0.5, 0.1, 10.0, 100.0, 40.0, 55.0, 7.0, 123.5,
        ];

        for x in xs {
            let y = ln(x);
            let expected = f32::ln(x);
            dbg!(x, y, expected);
            assert!((y - expected).abs() < FLOAT_DELTA);
        }
    }
}
