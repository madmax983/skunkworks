#![cfg(all(test, feature = "nova"))]

use crate::ast::{Dna, Gene, Helix, JunctionType, Nucleotide, Strand};
use crate::opcode::OpCode;
use crate::vm::{ChimeraVM, Value};

fn make_vm(genes: Vec<Gene>) -> ChimeraVM {
    let dna = Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    };
    ChimeraVM::new(dna)
}

#[test]
fn test_nucleate() {
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)], // Type 1
        },
        Gene {
            op: OpCode::Nucleate,
            args: vec![],
        },
    ];
    let mut vm = make_vm(genes);
    // Center is 8,8
    vm.step(); // Push
    vm.step(); // Nucleate

    match &vm.grid[8][8] {
        Value::Junction(JunctionType::All, vals) => {
            assert_eq!(vals[0], Value::Str("Crystal".to_string()));
            assert_eq!(vals[1], Value::Int(1));
        }
        _ => panic!("Expected Crystal at 8,8"),
    }
}

#[test]
fn test_accrete() {
    // 1. Nucleate
    // 2. Accrete (Grow)
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        },
        Gene {
            op: OpCode::Nucleate,
            args: vec![],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(100)],
        }, // 100% rate
        Gene {
            op: OpCode::Accrete,
            args: vec![],
        },
    ];
    let mut vm = make_vm(genes);
    vm.step();
    vm.step(); // Nucleate at 8,8

    // Set a neighbor to be eligible (empty/int) - default is 0 (Int)

    vm.step();
    vm.step(); // Accrete

    // Neighbors of 8,8 should be crystals
    // (8,9)
    match &vm.grid[8][9] {
        Value::Junction(JunctionType::All, vals) => {
            assert_eq!(vals[0], Value::Str("Crystal".to_string()));
        }
        _ => panic!("Expected Crystal growth at 8,9"),
    }
}

#[test]
fn test_shatter() {
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        },
        Gene {
            op: OpCode::Nucleate,
            args: vec![],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(100)],
        }, // 100% force
        Gene {
            op: OpCode::Shatter,
            args: vec![],
        },
    ];
    let mut vm = make_vm(genes);
    vm.step();
    vm.step(); // Nucleate

    // Verify crystal
    if !matches!(vm.grid[8][8], Value::Junction(_, _)) {
        panic!("Setup failed");
    }

    vm.step();
    vm.step(); // Shatter

    // Should be Int now
    match vm.grid[8][8] {
        Value::Int(_) => {} // Good
        _ => panic!("Expected Int after shatter"),
    }
}

#[test]
fn test_anneal() {
    // Setup unsorted grid around center
    // 8,8 is center
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        }, // Iterations (pushed first, so bottom)
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        }, // Radius (pushed last, so top)
        Gene {
            op: OpCode::Anneal,
            args: vec![],
        },
    ];
    let mut vm = make_vm(genes);

    // Set 5 cells (Radius 1 von Neumann neighborhood + center)
    // Order of traversal (Y then X):
    // 0: grid[7][8]
    // 1: grid[8][7]
    // 2: grid[8][8]
    // 3: grid[8][9]
    // 4: grid[9][8]

    vm.grid[7][8] = Value::Int(20);
    vm.grid[8][7] = Value::Int(5);
    vm.grid[8][8] = Value::Int(10);
    vm.grid[8][9] = Value::Int(1);
    vm.grid[9][8] = Value::Int(2);

    vm.step();
    vm.step();
    vm.step(); // Anneal

    // Sorted: 1, 2, 5, 10, 20.

    assert_eq!(vm.grid[7][8], Value::Int(1));
    assert_eq!(vm.grid[8][7], Value::Int(2));
    assert_eq!(vm.grid[8][8], Value::Int(5));
    assert_eq!(vm.grid[8][9], Value::Int(10));
    assert_eq!(vm.grid[9][8], Value::Int(20));
}
