use chimera_lang::vm::ChimeraVM;
use chimera_lang::ast::{Dna, Helix, Strand, Gene, Nucleotide};
use chimera_lang::opcode::OpCode;

fn make_dna(genes: Vec<Gene>) -> Dna {
    Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    }
}

#[test]
#[cfg(feature = "nova")]
fn test_glob_match_stack_overflow() {
    // 👺 HAVOC: Stack Overflow via Infinite Recursion in glob_match
    // The glob_match function is recursive and splits on '*'.
    // A pattern with many '*'s causes deep recursion.
    // 20,000 '*'s should be enough to overflow the stack.

    // Using a large number to force recursion depth
    let pattern = "*".repeat(20000);
    // The target doesn't even need to match, just needs to trigger the parsing/recursion
    let target = "a";

    let genes = vec![
        // Push pattern
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::String(pattern)],
        },
        // Push target
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::String(target.to_string())],
        },
        // Match
        Gene {
            op: OpCode::Match,
            args: vec![],
        },
    ];

    let mut vm = ChimeraVM::new(make_dna(genes));

    println!("Attempting to crash with glob_match recursion...");

    // Execute
    // 1. Push pattern
    vm.step();
    // 2. Push target
    vm.step();
    // 3. Match - BOOM
    vm.step();

    // If we survive, assert success (but we expect to crash before this)
    assert!(!vm.stack.is_empty());
    println!("Survived!");
}
