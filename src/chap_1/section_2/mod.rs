use num::Complex;
use std::fmt::Display;

pub trait QubitState {
    fn get_probability_of_0(&self) -> Complex<f64>;
    fn get_probability_of_1(&self) -> Complex<f64>;
}

pub struct Qubit {
    alpha: Complex<f64>,
    beta: Complex<f64>,
}

impl Qubit {
    pub fn new(alpha_re: f64, alpha_im: f64, beta_re: f64, beta_im: f64) -> Self {
        Self {
            alpha: Complex::new(alpha_re, alpha_im),
            beta: Complex::new(beta_re, beta_im),
        }
    }
}

impl QubitState for Qubit {
    fn get_probability_of_0(&self) -> Complex<f64> {
        self.alpha.powi(2)
    }

    fn get_probability_of_1(&self) -> Complex<f64> {
        self.beta.powi(2)
    }
}

impl Display for Qubit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "({})|0> + ({})|1>", self.alpha, self.beta)
    }
}
