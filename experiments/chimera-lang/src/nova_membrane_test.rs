#![cfg(feature = "nova")]

use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use crate::opcode::OpCode;
use crate::vm::ChimeraVM;

fn make_dna(genes: Vec<Gene>) -> Dna {
    Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    }
}

#[test]
fn test_membrane_blocking() {
    // 1. Membrane(2) -> South Wall at (8,8)
    // 2. Migrate(1, 0) -> South. Should fail (stay at 8,8).
    // 3. Osmosis(1, 0) -> South. Should succeed (move to 9,8).
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(2)], // South (0=N, 1=E, 2=S, 3=W)
        },
        Gene {
            op: OpCode::Membrane,
            args: vec![],
        },
        // Try Migrate South (dy=1, dx=0)
        // Stack order: dy, dx (top)
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)], // dy
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)], // dx
        },
        Gene {
            op: OpCode::Migrate,
            args: vec![],
        },
        // Try Osmosis South
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)], // dy
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)], // dx
        },
        Gene {
            op: OpCode::Osmosis,
            args: vec![],
        },
    ];

    let mut vm = ChimeraVM::new(make_dna(genes));
    vm.context_loc = (8, 8);
    // Give enough energy
    vm.energy = 100;

    // 1. Membrane
    vm.step(); // Push
    vm.step(); // Membrane

    // Verify Wall
    assert_eq!(vm.membranes[8][8] & 4, 4, "South wall not set at 8,8");
    assert_eq!(vm.membranes[9][8] & 1, 1, "North wall not set at 9,8"); // Reciprocal

    // 2. Migrate
    vm.step(); // Push
    vm.step(); // Push
    vm.step(); // Migrate

    assert_eq!(vm.context_loc, (8, 8), "Migrate should have been blocked");
    assert!(vm.output.last().unwrap().contains("Blocked"), "Expected blockage message");

    // 3. Osmosis
    vm.step(); // Push
    vm.step(); // Push
    vm.step(); // Osmosis

    assert_eq!(vm.context_loc, (9, 8), "Osmosis should have succeeded");
    assert!(vm.energy < 100, "Energy should have been consumed");
}
