mod chap_1;

use chap_1::section_2::QubitVector;
use chap_1::section_3::{h_gate, x_gate, z_gate};
use num::{Complex, One, Zero};

use crate::chap_1::section_3::cnot_gate;

fn main() {
    let complex = QubitVector::new(Complex::new(0.0, 0.0), Complex::new(1.0, 0.0));
    println!("{}", complex);
    println!("{}", x_gate(complex));
    println!("{}", z_gate(complex));
    println!("{}", h_gate(complex));
    println!("{}", h_gate(h_gate(complex)));
    // let zero = QubitVector::new(Complex::one(), Complex::zero());
    // let one = QubitVector::new(Complex::zero(), Complex::one());
    // let (a, i) = cnot_gate(zero, zero);
    // let (b, j) = cnot_gate(zero, one);
    // let (c, k) = cnot_gate(one, zero);
    // let (d, l) = cnot_gate(one, one);
    // println!("{}", a);
    // println!("{}", i);
    // println!("{}", b);
    // println!("{}", j);
    // println!("{}", c);
    // println!("{}", k);
    // println!("{}", d);
    // println!("{}", l);
}
