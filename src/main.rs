mod chap_1;

use chap_1::section_2::QubitVector;
use chap_1::section_3::{h_gate, x_gate, z_gate};
use num::Complex;

fn main() {
    let complex = QubitVector::new(Complex::new(0.0, 0.0), Complex::new(1.0, 0.0));
    println!("{}", complex);
    println!("{}", x_gate(complex));
    println!("{}", z_gate(complex));
    println!("{}", h_gate(complex));
    println!("{}", h_gate(h_gate(complex)));
}
