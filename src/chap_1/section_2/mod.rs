use num::complex::Complex;
use std::fmt::Display;

struct BasicQBit {
    alpha: f64,
    beta: f64,
}

impl BasicQBit {
    fn verify_total_probability(&self) -> bool {
        self.alpha.powi(2) + self.beta.powi(2) == 1.0
    }
}

impl Display for BasicQBit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}|0> + {}|1>", self.alpha, self.beta)
    }
}

struct QBit {
    alpha: Complex<f64>,
    beta: Complex<f64>,
}

impl QBit {
    fn verify_total_probability(&self) -> bool {
        self.alpha.powi(2) + self.beta.powi(2) == Complex::new(1.0, 0.0)
    }
}

impl Display for QBit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}|0> + {}|1>", self.alpha, self.beta)
    }
}
