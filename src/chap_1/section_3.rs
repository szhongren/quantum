use nalgebra::DMatrix;
use num::{Complex, One, Zero};

pub fn x_gate(qubit: &super::section_2::QubitVector) -> super::section_2::QubitVector {
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

pub fn z_gate(qubit: &super::section_2::QubitVector) -> super::section_2::QubitVector {
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

pub fn h_gate(qubit: &super::section_2::QubitVector) -> super::section_2::QubitVector {
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

pub fn cnot_gate(kronecker: super::section_2::QubitVector) -> super::section_2::QubitVector {
    // any multiple qubit logic gate may be composed from cnot and single qubit gates, similar to how nand is universal
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
    mat * kronecker
}

// |0> and |1> are not the only basis that you can use for a qubit
// |0> and |1> are top and bottom of the bloch sphere, on the z axis
// |+> and |-> are front and back of the bloch sphere, on the x axis
// |R> and |L> are right and left of the bloch sphere, on the y axis
// this means that you can define a qubit in any of the above bases, and the vector is not necessarily |0> and |1>
// each of these pairs must be orthonormal, to preserve |a|^2 + |b|^2 = 1, where orthonormal here just means opposite on bloch sphere
