use nalgebra::Matrix2;
use num::complex::Complex;

use super::section_2::QubitVector;

pub fn x_gate(qubit: QubitVector) -> () {
    let mat = Matrix2::new(
        Complex::new(0.0, 0.0),
        Complex::new(1.0, 0.0),
        Complex::new(1.0, 0.0),
        Complex::new(0.0, 0.0),
    );
    println!("{}", mat);
    println!("{}", mat * qubit); // operation on the qubit, gate goes first
}
