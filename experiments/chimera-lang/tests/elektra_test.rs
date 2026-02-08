#![cfg(feature = "elektra")]
use chimera_lang::prelude::*;
use chimera_lang::vm::Value;

fn make_dna(genes: Vec<Gene>) -> Dna {
    Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    }
}

#[test]
fn test_simple_circuit() {
    let genes = vec![
        // Battery at 5,5 with 100V
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(100)] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(5)] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(5)] },
        Gene { op: OpCode::Battery, args: vec![] },

        // Wire at 5,6
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(5)] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(6)] },
        Gene { op: OpCode::GWrite, args: vec![] },

        // Wire at 5,7
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(5)] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(7)] },
        Gene { op: OpCode::GWrite, args: vec![] },

        // Ground at 5,8
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(5)] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(8)] },
        Gene { op: OpCode::Ground, args: vec![] },
    ];

    let mut vm = ChimeraVM::new(make_dna(genes));

    // Execute setup
    // 4 + 4 + 4 + 3 = 15 ops
    for _ in 0..20 {
        vm.step();
    }

    // Step again to update circuitry (circuitry updates at start of step)
    vm.step();

    // Check voltage at Wire (5,6)
    assert_eq!(vm.voltage_grid[5][6], 100.0);
    // Check current (Closed loop to ground)
    assert_eq!(vm.current_grid[5][6], 100.0);
}

#[test]
fn test_open_circuit() {
    // Battery but no Ground
    let genes = vec![
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(50)] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(2)] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(2)] },
        Gene { op: OpCode::Battery, args: vec![] },

        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(2)] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(3)] },
        Gene { op: OpCode::GWrite, args: vec![] },
    ];

    let mut vm = ChimeraVM::new(make_dna(genes));

    for _ in 0..10 {
        vm.step();
    }

    vm.step();

    assert_eq!(vm.voltage_grid[2][3], 50.0);
    assert_eq!(vm.current_grid[2][3], 0.0); // No flow
}
