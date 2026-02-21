use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use chimera_lang::opcode::OpCode;
use chimera_lang::vm::ChimeraVM;

#[test]
fn test_lisp_recursion_bomb() {
    // Create a deeply nested Lisp expression
    // 20000 nested additions: (add (add ... ))
    let depth = 20000;
    let mut lisp_code = String::new();
    for _ in 0..depth {
        lisp_code.push_str("(add ");
    }
    lisp_code.push_str("1 1");
    for _ in 0..depth {
        lisp_code.push(')');
    }

    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::String(lisp_code)],
        },
        Gene {
            op: OpCode::LispEval,
            args: vec![],
        },
    ];

    let dna = Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    };
    let mut vm = ChimeraVM::new(dna);

    // Step 1: Push string to stack
    vm.step();

    // Step 2: LispEval
    vm.step();

    // Assert that we caught the error
    let found_error = vm
        .output
        .iter()
        .any(|s| s.contains("Recursion limit exceeded"));
    assert!(
        found_error,
        "Expected recursion error, got output: {:?}",
        vm.output
    );
}
