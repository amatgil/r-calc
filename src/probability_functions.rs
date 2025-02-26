use crate::*;
use core::todo;
//  _   _      _
// | | | | ___| |_ __   ___ _ __ ___
// | |_| |/ _ \ | '_ \ / _ \ '__/ __|
// |  _  |  __/ | |_) |  __/ |  \__ \
// |_| |_|\___|_| .__/ \___|_|  |___/
//              |_|
fn choose(n: Enter, r: Enter) -> Enter {
    if r > n {
        0
    } else {
        (1..=r).fold(1, |acc, val| acc * (n - val + 1) / val)
    }
}

fn newton(x0: Float, f: impl Fn(Float) -> Float, fp: impl Fn(Float) -> Float) -> Option<Float> {
    const DELTA: Float = 0.001;
    const MAXITERS: Enter = 30;

    let mut x = x0;
    let mut iters = 0;

    while iters < MAXITERS {
        x = x - f(x) / fp(x);
        if f(x).abs() < DELTA {
            return Some(x);
        }

        iters += 1;
    }
    return None;
}

//  ____  _                   _
// |  _ \(_)___  ___ _ __ ___| |_ ___  ___
// | | | | / __|/ __| '__/ _ \ __/ _ \/ __|
// | |_| | \__ \ (__| | |  __/ ||  __/\__ \
// |____/|_|___/\___|_|  \___|\__\___||___/

pub mod binomial {
    use crate::probability_functions::*;
    use crate::Float;

    /// Quina és la probabilitat de `k` èxits en `n` intents si la probabilitat d'un èxit és `prob`?
    pub fn dbinom(k: Enter, n: Enter, prob: Float) -> Float {
        if k > n {
            0.0
        } else {
            (choose(n, k) as Float) * pow(prob, k as Float) * pow(1.0 - prob, (n - k) as Float)
        }
    }

    /// Quina és la probabilitat de `k` èxits en `n` _o més_ intents si la probabilitat d'un èxit és `prob`?
    pub fn pbinom(q: Enter, n: Enter, prob: Float) -> Float {
        (0..q).map(|i| dbinom(i, n, prob)).sum()
    }

    /// Quin valor d'intents deixa una probabilitat `p` de sortir `n` cops?
    pub fn qbinom(p: Float, n: Enter, prob: Float) -> Float {
        todo!()
    }
}

pub mod poisson {
    use crate::probability_functions::*;

    pub fn dpois(k: Enter, lambda: Float) -> Float {
        let numerador = pow(lambda, k as Float) * exp(-lambda); // lambda^k * e^(-lambda)
        let k_factorial = (1..k).fold(1.0, |acc, f| acc * f as Float); // k!
        numerador / k_factorial
    }
    pub fn ppois(q: Enter, lambda: Float) -> Float {
        (0..q).map(|i| dpois(i, lambda)).sum()
    }
    pub fn qpois(p: Enter, lambda: Float) -> Enter {
        todo!()
    }
}
pub mod nbinom {
    use crate::probability_functions::*;

    pub fn dnbinom(k: Enter, r: Enter, prob: Float) -> Float {
        todo!()
    }
    pub fn pnbinom(q: Enter, r: Enter, prob: Float) -> Float {
        (0..q).map(|i| dnbinom(i, r, prob)).sum()
    }
    pub fn qnbinom(p: Float, r: Enter, prob: Float) -> Enter {
        todo!()
    }
}
//   ____            _   _
//  / ___|___  _ __ | |_(_)_ __  _   _  ___  ___
// | |   / _ \| '_ \| __| | '_ \| | | |/ _ \/ __|
// | |__| (_) | | | | |_| | | | | |_| |  __/\__ \
//  \____\___/|_| |_|\__|_|_| |_|\__,_|\___||___/

pub mod exponencial {
    use crate::probability_functions::*;

    pub fn dexp(x: Enter, lambda: Float) -> Float {
        todo!()
    }
    pub fn pexp(q: Enter, lambda: Float) -> Float {
        todo!()
    }
    pub fn qexp() -> Enter {
        todo!()
    }
}

pub mod normal {
    use crate::probability_functions::*;

    pub fn dnorm(x: Enter, mu: Float, sigma: Float) -> Float {
        todo!()
    }
    pub fn pnorm(q: Enter, mu: Float, sigma: Float) -> Float {
        todo!()
    }
    pub fn qnorm(p: Float, mu: Float, sigma: Float) -> Enter {
        todo!()
    }
}

// _____         _
// |_   _|__  ___| |_ ___
//   | |/ _ \/ __| __/ __|
//   | |  __/\__ \ |_\__ \
//   |_|\___||___/\__|___/
