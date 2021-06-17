use nalgebra::{Matrix2, Vector2};
use num::complex::Complex;

use super::section_2::Qubit;

pub fn x_gate(_: Qubit) -> () {
    let qubit = Vector2::new(Complex::new(1.0, 0.0), Complex::new(0.0, 0.0));
    let mat = Matrix2::new(
        Complex::new(0.0, 0.0),
        Complex::new(1.0, 0.0),
        Complex::new(1.0, 0.0),
        Complex::new(0.0, 0.0),
    );
    println!("{}", mat);
    println!("{}", mat * qubit); // operation on the qubit, gate goes first
}
