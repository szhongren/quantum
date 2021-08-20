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
// apply hadamard to every qubit
// |v1> = [(|0> + |1>)/sqrt(2)][(|0> - |1>)/sqrt(2)]
// apply U_f
// |v2> = +-[(|0> + |1>)/sqrt(2)][(|0> - |1>)/sqrt(2)] if f(0) == f(1)
//        +-[(|0> - |1>)/sqrt(2)][(|0> - |1>)/sqrt(2)] if f(0) != f(1)
// apply hadamard to only the first qubit
// |v3> = +-|0>[(|0> - |1>)/sqrt(2)] if f(0) == f(1)
//        +-|1>[(|0> - |1>)/sqrt(2)] if f(0) != f(1)
//      = +-|f(0) QNOT f(1)>[(|0> - |1>)/sqrt(2)]
// thus, we can measure the first qubit and get information about the outputs of the function for both values of x

// Deutsch-Jozsa algorithm
// Deutsch's problem statement
// 1. Alice, in Amsterdam, selects a number x from 0 to 2^n - 1, and mails it in a letter to Bob, in Boston
// 2. Bob calculates some function f(x) and reqplies with the result, which is either 0 or 1
// 3. Now, Bob has promised to use a function f which is of one of two kinds:
//    1. either f(x) is constant for all values of x
//    2. or else f(x) is balanced, that is, equal to 1 for exactly half of all the possible x, and 0 for the other half
// 4. Alice's goal is to determine with certainty whether Bob has chosen a constant or a balanced function, corresponding with Bob as little as possible

// in the classical case, assume that Alice can only send Bob one value of x in each letter
// in each letter, Alice sends x bits of information
// in the worst case, Alice needs 2^n/2 + 1 queries to know if the function is balanced or not
// however, in the quantum case, if Alice and Bob exchange qubits, and Bob agrees to calculate f(x) with a unitary transform U_f, then only 1 query is needed
