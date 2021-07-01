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
