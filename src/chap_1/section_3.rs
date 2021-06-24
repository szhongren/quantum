use nalgebra::DMatrix;
use num::{Complex, One, Zero};

use crate::chap_1::section_2::QubitVector;

pub fn section_3() {
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
    // this swaps the 2 qubits
    println!("{:?}", zero.kronecker(&one));
    println!("{}", interchange(zero.kronecker(&one)));
    // creating bell states
    println!("{}", cnot_gate(h_gate(&zero).kronecker(&zero)));
    // |00> -> (|00> + |11>)/sqrt(2)
    println!("{}", cnot_gate(h_gate(&zero).kronecker(&one)));
    // |01> -> (|01> + |10>)/sqrt(2)
    println!("{}", cnot_gate(h_gate(&one).kronecker(&zero)));
    // |10> -> (|00> - |11>)/sqrt(2)
    println!("{}", cnot_gate(h_gate(&one).kronecker(&one)));
    // |11> -> (|01> - |10>)/sqrt(2)
}

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
    // also called cx_gate
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

pub fn interchange(kronecker: super::section_2::QubitVector) -> super::section_2::QubitVector {
    let mat = DMatrix::from_vec(
        4,
        4,
        vec![
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
            Complex::zero(),
            Complex::zero(),
            Complex::zero(),
            Complex::zero(),
            Complex::one(),
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

// there are features allowed in classical circuits that usually don't exist in quantum circuits
// 1. no loops, so quantum circuits are always acyclic
// 2. no FANIN, which is when 2+ wires are joined into 1, which is basically an OR, so quantum circuits cannot have this irreversible operation
// 3. no FANOUT, which is when a single wire becomes 2+ wires, which copies the bit, because quantum mechanics forbids the copying of a qubit

/*
a = 1.0 / sqrt(2.0)

h_gate
[
    a, a,
    a, -a
]

identity
[
    1.0, 0.0,
    0.0, 1.0
]

hadamard tensor identity
[
      a, 0.0,   a, 0.0,
    0.0,   a, 0.0,   a,
      a, 0.0,  -a, 0.0,
    0.0,   a, 0.0,  -a,
]
 */
