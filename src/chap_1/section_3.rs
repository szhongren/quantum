use nalgebra::Matrix2;
use num::complex::Complex;

use super::section_2::QubitVector;

pub fn x_gate(qubit: QubitVector) -> QubitVector {
    // a|0> + b|1> -> b|0> + a|1>
    let mat = Matrix2::new(
        Complex::new(0.0, 0.0),
        Complex::new(1.0, 0.0),
        Complex::new(1.0, 0.0),
        Complex::new(0.0, 0.0),
    );
    mat * qubit
}

pub fn z_gate(qubit: QubitVector) -> QubitVector {
    // a|0> + b|1> -> a|0> - b|1>
    let mat = Matrix2::new(
        Complex::new(1.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(-1.0, 0.0),
    );
    mat * qubit
}

pub fn h_gate(qubit: QubitVector) -> QubitVector {
    // a|0> + b|1> -> a(|0> + |1>)/sqrt(2) + b(|0> - |1>)/sqrt(2)
    let scalar = 1.0 / 2.0_f64.sqrt();
    let mat = Matrix2::new(
        Complex::new(scalar, 0.0),
        Complex::new(scalar, 0.0),
        Complex::new(scalar, 0.0),
        Complex::new(-scalar, 0.0),
    );
    mat * qubit
}
