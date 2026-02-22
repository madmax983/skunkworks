#[cfg(feature = "nova")]
#[test]
fn test_chromatin_opcode() {
    use chimera_lang::prelude::*;
    use chimera_lang::vm::prologue::chromatin::Constraint;

    // Create DNA with Chromatin instruction
    // "adj(A,B)" -> Chromatin
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::String("adj(A,B)".to_string())],
        },
        Gene {
            op: OpCode::Chromatin,
            args: vec![],
        },
    ];
    let dna = Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    };

    let mut vm = ChimeraVM::new(dna);

    // Step 1: Push
    vm.step();
    // Step 2: Chromatin
    vm.step();

    assert!(vm.prologue_state.chromatin_state.active);
    assert_eq!(vm.prologue_state.chromatin_state.constraints.len(), 1);
    match &vm.prologue_state.chromatin_state.constraints[0] {
        Constraint::Adjacent(a, b) => {
            assert_eq!(a, "A");
            assert_eq!(b, "B");
        }
        _ => panic!("Wrong constraint parsed"),
    }
}
