use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use chimera_lang::opcode::OpCode;
use chimera_lang::vm::ChimeraVM;

fn make_dna(genes: Vec<Gene>) -> Dna {
    Dna { evolution_config: None,
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    }
}

#[test]
fn test_havoc_rate() {
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(100)],
        }, // Rate 1.0
        Gene {
            op: OpCode::HavocRate,
            args: vec![],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(7)],
        }, // Scope: Memory + Stack + Exec (1+2+4=7)
        Gene {
            op: OpCode::HavocScope,
            args: vec![],
        },
        // Loop to let havoc run
        Gene {
            op: OpCode::Jump,
            args: vec![Nucleotide::Number(0)],
        },
    ];
    let mut vm = ChimeraVM::new(make_dna(genes));

    // Execute setup
    vm.step(); // push 100
    vm.step(); // rate -> 1.0
    vm.step(); // push 7
    vm.step(); // scope -> 7

    assert_eq!(vm.havoc.rate, 1.0);
    assert_eq!(vm.havoc.scope, 7);

    // Run for a while. With rate 1.0, SOMETHING should happen.
    // Normal step consumes 1 energy.
    // Havoc execution fault might drain half energy.
    // Havoc stack fault might drop/dup stack.
    // Havoc memory fault corrupts grid.

    let start_output_len = vm.output.len();

    for _ in 0..10 {
        vm.step();
    }

    let havoc_triggered = vm
        .output
        .iter()
        .skip(start_output_len)
        .any(|s| s.contains("HAVOC"));
    assert!(
        havoc_triggered,
        "Havoc engine should have triggered a fault (Rate 1.0)"
    );
}
