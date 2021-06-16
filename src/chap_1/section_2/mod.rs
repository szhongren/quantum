use num::complex::Complex;
use std::fmt::Display;

pub trait Qubit {
    fn verify_total_probability(&self) -> bool;
}

struct RealQubit {
    alpha: f64,
    beta: f64,
}

impl Qubit for RealQubit {
    fn verify_total_probability(&self) -> bool {
        self.alpha.powi(2) + self.beta.powi(2) == 1.0
    }
}

impl Display for RealQubit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}|0> + {}|1>", self.alpha, self.beta)
    }
}

struct FullQubit {
    alpha: Complex<f64>,
    beta: Complex<f64>,
}

impl Qubit for FullQubit {
    fn verify_total_probability(&self) -> bool {
        self.alpha.powi(2) + self.beta.powi(2) == Complex::new(1.0, 0.0)
    }
}

impl Display for FullQubit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}|0> + {}|1>", self.alpha, self.beta)
    }
}
