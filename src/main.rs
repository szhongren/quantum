mod chap_1;

use chap_1::section_3::x_gate;
use num::Complex;

use crate::chap_1::section_2::QubitVector;

fn main() {
    let complex = QubitVector::new(Complex::new(0.0, 0.0), Complex::new(1.0, 0.0));
    println!("{}", complex);
    x_gate(complex);
}
