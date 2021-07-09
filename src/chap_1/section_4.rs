pub fn section_4() {
    classical_computations();
}

fn classical_computations() {
    let permutations = vec![
        (false, false, false),
        (false, false, true),
        (false, true, false),
        (false, true, true),
        (true, false, false),
        (true, false, true),
        (true, true, false),
        (true, true, true),
    ];

    for (a, b, c) in permutations {
        println!(
            "{}, {}, {} -> {}",
            a as u8,
            b as u8,
            c as u8,
            classical_toffoli_gate(a, b, c) as u8
        );
    }
}

pub fn classical_toffoli_gate(a: bool, b: bool, c: bool) -> bool {
    if a && b {
        !c
    } else {
        c
    }
}

// hadamard transform is when you have n parallel Hadamard gates operating on n qubits
// transforms each qubit into a superposition of |0> and |1>
// results in 1/sqrt(2^n)*sum(|x>)
// to calculate the output of all values of x, we can do 1/sqrt(2^n)*sum(|x>|f(x)>)

// a QNOT b == (a + b) % 2
// Deutsch's algorithm
// |v0> = |01>
// |v1> = [(|0> + |1>)/sqrt(2)][(|0> - |1>)/sqrt(2)]
// |v2> = +-[(|0> + |1>)/sqrt(2)][(|0> - |1>)/sqrt(2)] if f(0) == f(1)
//        +-[(|0> - |1>)/sqrt(2)][(|0> - |1>)/sqrt(2)] if f(0) != f(1)
// |v3> = +-|0>[(|0> - |1>)/sqrt(2)] if f(0) == f(1)
//        +-|1>[(|0> - |1>)/sqrt(2)] if f(0) != f(1)
//      = +-|f(0) QNOT f(1)>[(|0> - |1>)/sqrt(2)]
// thus, we can measure the first qubit and get information about the outputs of the function for both values of x
