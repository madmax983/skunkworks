use chimera_lang::ast::{Dna, Gene, Helix, Strand};
use chimera_lang::opcode::OpCode;
use chimera_lang::vm::ChimeraVM;

#[test]
fn test_stack_pop_underflow_safety() {
    let dna_bad = Dna {
        evolution_config: None,
        helix: Helix {
            strands: vec![Strand {
                genes: vec![
                    Gene::new(OpCode::Add, vec![]),    // Requires 2
                    Gene::new(OpCode::Sub, vec![]),    // Requires 2
                    Gene::new(OpCode::Mul, vec![]),    // Requires 2
                    Gene::new(OpCode::Div, vec![]),    // Requires 2
                    Gene::new(OpCode::Splice, vec![]), // Nova block specific that we fixed
                ],
            }],
        },
    };
    let mut vm2 = ChimeraVM::new(dna_bad);
    // Execute all genes
    for _ in 0..10 {
        vm2.step();
    }
    // Should not panic.
    assert_eq!(vm2.stack.len(), 0);
}
