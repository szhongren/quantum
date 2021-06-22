mod chap_1;

use chap_1::section_2::QubitVector;
use num::{Complex, One, Zero};

use crate::chap_1::section_3::{cnot_gate, h_gate, x_gate, z_gate};

fn main() {
    let complex = QubitVector::from_vec(vec![Complex::zero(), Complex::one()]);
    println!("{}", complex);
    println!("{}", x_gate(&complex));
    println!("{}", z_gate(&complex));
    println!("{}", h_gate(&complex));
    println!("{}", h_gate(&h_gate(&complex)));
    let zero = QubitVector::from_vec(vec![Complex::one(), Complex::zero()]);
    let one = QubitVector::from_vec(vec![Complex::zero(), Complex::one()]);
    println!("kronecker products");
    println!("{}", zero.kronecker(&zero));
    println!("{}", zero.kronecker(&one));
    println!("{}", one.kronecker(&zero));
    println!("{}", one.kronecker(&one));
    println!("cnot gates");
    println!("{}", cnot_gate(zero.kronecker(&zero)));
    println!("{}", cnot_gate(zero.kronecker(&one)));
    println!("{}", cnot_gate(one.kronecker(&zero)));
    println!("{}", cnot_gate(one.kronecker(&one)));
    println!("{}", cnot_gate(cnot_gate(cnot_gate(zero.kronecker(&zero)))));
}
