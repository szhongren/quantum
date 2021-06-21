mod chap_1;

use chap_1::section_2::QubitVector;
use chap_1::section_3::{h_gate, x_gate, z_gate};
use num::{Complex, One, Zero};

fn main() {
    let complex = QubitVector::from_vec(vec![Complex::zero(), Complex::one()]);
    println!("{}", complex);
    println!("{}", x_gate(&complex));
    println!("{}", z_gate(&complex));
    println!("{}", h_gate(&complex));
    println!("{}", h_gate(&h_gate(&complex)));
    let zero = QubitVector::from_vec(vec![Complex::one(), Complex::zero()]);
    let one = QubitVector::from_vec(vec![Complex::zero(), Complex::one()]);
    // // let (a, i) = cnot_gate(zero, zero);
    // // let (b, j) = cnot_gate(zero, one);
    // // let (c, k) = cnot_gate(one, zero);
    // // let (d, l) = cnot_gate(one, one);
    // zero.tensor_product(zero);
    println!("{}", zero.kronecker(&zero));
    println!("{}", zero.kronecker(&one));
    println!("{}", one.kronecker(&zero));
    println!("{}", one.kronecker(&one));
}
