#![cfg(feature = "nova")]
use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use crate::opcode::OpCode;
use crate::vm::{ChimeraVM, Value};

fn make_dna(genes: Vec<Gene>) -> Dna {
    Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    }
}

#[test]
fn test_entangle_g_write() {
    // [ push(2) push(2) push(0) push(0) entangle() push(100) push(0) push(0) g_write() ]
    // Entangle (0,0) -> (2,2). Write 100 to (0,0). Should appear at (2,2).
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(2)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(2)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::Entangle,
            args: vec![],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(100)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::GWrite,
            args: vec![],
        },
    ];
    let mut vm = ChimeraVM::new(make_dna(genes));
    while !vm.halted {
        vm.step();
    }

    assert_eq!(vm.grid[0][0], Value::Int(100));
    assert_eq!(vm.grid[2][2], Value::Int(100));
}

#[test]
fn test_chained_entanglement() {
    // A -> B -> C
    // (0,0) -> (1,1) -> (2,2)
    // Write to (0,0) should update all.
    let genes = vec![
        // (0,0) -> (1,1)
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::Entangle,
            args: vec![],
        },
        // (1,1) -> (2,2)
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(2)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(2)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        },
        Gene {
            op: OpCode::Entangle,
            args: vec![],
        },
        // Write 99 to (0,0)
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(99)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::GWrite,
            args: vec![],
        },
    ];
    let mut vm = ChimeraVM::new(make_dna(genes));
    while !vm.halted {
        vm.step();
    }

    assert_eq!(vm.grid[0][0], Value::Int(99));
    assert_eq!(vm.grid[1][1], Value::Int(99));
    assert_eq!(vm.grid[2][2], Value::Int(99));
}

#[test]
fn test_cycle_prevention() {
    // A -> B -> A
    // Write should not hang.
    let genes = vec![
        // (0,0) -> (1,1)
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::Entangle,
            args: vec![],
        },
        // (1,1) -> (0,0)
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        },
        Gene {
            op: OpCode::Entangle,
            args: vec![],
        },
        // Write 50 to (0,0)
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(50)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::GWrite,
            args: vec![],
        },
    ];
    let mut vm = ChimeraVM::new(make_dna(genes));
    // Should terminate
    let mut steps = 0;
    while !vm.halted && steps < 100 {
        vm.step();
        steps += 1;
    }

    assert!(steps < 100, "VM did not halt (infinite loop?)");
    assert_eq!(vm.grid[0][0], Value::Int(50));
    assert_eq!(vm.grid[1][1], Value::Int(50));
}

#[test]
fn test_decohere() {
    // A -> B
    // Decohere(A)
    // Write A -> B should NOT update.
    let genes = vec![
        // (0,0) -> (1,1)
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::Entangle,
            args: vec![],
        },
        // Decohere (0,0)
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::Decohere,
            args: vec![],
        },
        // Write 77 to (0,0)
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(77)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::GWrite,
            args: vec![],
        },
    ];
    let mut vm = ChimeraVM::new(make_dna(genes));
    while !vm.halted {
        vm.step();
    }

    assert_eq!(vm.grid[0][0], Value::Int(77));
    assert_eq!(vm.grid[1][1], Value::Int(0)); // Should NOT be updated
}
