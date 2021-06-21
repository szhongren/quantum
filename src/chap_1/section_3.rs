use nalgebra::DMatrix;
use num::{Complex, One, Zero};

use super::section_2::QubitVector;

pub fn x_gate(qubit: &QubitVector) -> QubitVector {
    // a|0> + b|1> -> b|0> + a|1>
    let mat = DMatrix::from_vec(
        2,
        2,
        vec![
            Complex::zero(),
            Complex::one(),
            Complex::one(),
            Complex::zero(),
        ],
    );
    mat * qubit
}

pub fn z_gate(qubit: &QubitVector) -> QubitVector {
    // a|0> + b|1> -> a|0> - b|1>
    let mat = DMatrix::from_vec(
        2,
        2,
        vec![
            Complex::one(),
            Complex::zero(),
            Complex::zero(),
            -Complex::one(),
        ],
    );
    mat * qubit
}

pub fn h_gate(qubit: &QubitVector) -> QubitVector {
    // a|0> + b|1> -> a(|0> + |1>)/sqrt(2) + b(|0> - |1>)/sqrt(2)
    let scalar = 1.0 / 2.0_f64.sqrt();
    let mat = DMatrix::from_vec(
        2,
        2,
        vec![
            Complex::new(scalar, 0.0),
            Complex::new(scalar, 0.0),
            Complex::new(scalar, 0.0),
            Complex::new(-scalar, 0.0),
        ],
    );
    mat * qubit
}

// generally, any single qubit gate can be decomposed into:
// | cos(y/2), -sin(y/2) |
// | sin(y/2), cos(y/2)  |, which is a rotation
// and
// | e^(-ib/2), 0 |
// | 0, e^(ib/2)  |, which is a rotation about the z axis

pub fn cnot_gate(qubit1: &QubitVector, qubit2: &QubitVector) -> QubitVector {
    let mat = DMatrix::from_vec(
        4,
        4,
        vec![
            Complex::one(),
            Complex::zero(),
            Complex::zero(),
            Complex::zero(),
            Complex::zero(),
            Complex::one(),
            Complex::zero(),
            Complex::zero(),
            Complex::zero(),
            Complex::zero(),
            Complex::zero(),
            Complex::one(),
            Complex::zero(),
            Complex::zero(),
            Complex::one(),
            Complex::zero(),
        ],
    );
    mat * qubit1.kronecker(&qubit2)
}
