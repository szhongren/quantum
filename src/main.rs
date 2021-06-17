mod chap_1;

use chap_1::section_2::Qubit;
use chap_1::section_2::QubitState;
use chap_1::section_3::x_gate;

fn main() {
    let complex: Qubit = Qubit::new(0.0, 0.0, 1.0, 0.0);
    println!("{}", complex);
    println!("{}", complex.get_probability_of_0());
    println!("{}", complex.get_probability_of_1());
    x_gate(complex);
}
