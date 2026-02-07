#![cfg(all(test, feature = "nova"))]

use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use crate::opcode::OpCode;
use crate::vm::{ChimeraVM, Value, GRID_SIZE};

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
    // [ nucleate() ]
    let genes = vec![Gene {
        op: OpCode::Nucleate,
        args: vec![],
    }];
    let mut vm = make_vm(genes);

    // Set location to 8,8
    vm.context_loc = (8, 8);

    vm.step();

    assert_eq!(vm.grid[8][8], Value::Int(100));
    // Check energy cost (initial 50 - 10 = 40, minus 1 tick = 39)
    assert!(vm.energy < 50);
}

#[test]
fn test_accrete() {
    // [ push(10) push(8) push(9) g_write() push(1) accrete() ]
    // Write 10 to (8,9) (neighbor of 8,8). Accrete radius 1.
    // Note: GWrite args are [val, y, x] -> writes grid[y][x]
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(10)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(9)], // y
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(8)], // x
        },
        Gene {
            op: OpCode::GWrite,
            args: vec![],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)], // radius
        },
        Gene {
            op: OpCode::Accrete,
            args: vec![],
        },
    ];
    let mut vm = make_vm(genes);
    vm.context_loc = (8, 8);

    while !vm.halted {
        vm.step();
    }

    // (8,9) should be drained to 0. (x=8, y=9)
    assert_eq!(vm.grid[9][8], Value::Int(0));
    // (8,8) should have absorbed 10
    assert_eq!(vm.grid[8][8], Value::Int(10));
}

#[test]
fn test_shatter() {
    // [ nucleate() push(2) shatter() ]
    // Create 100 at center, then shatter with force 2.
    // 100 / 2 = 50. 2 fragments of 50 scattered.
    let genes = vec![
        Gene {
            op: OpCode::Nucleate,
            args: vec![],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(2)], // force
        },
        Gene {
            op: OpCode::Shatter,
            args: vec![],
        },
    ];
    let mut vm = make_vm(genes);
    vm.context_loc = (8, 8);

    while !vm.halted {
        vm.step();
    }

    // Center should be cleared
    assert_eq!(vm.grid[8][8], Value::Int(0));

    // Sum of grid should be 100 (conservation of mass, mostly)
    let mut sum = 0;
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            if let Value::Int(n) = vm.grid[y][x] {
                sum += n;
            }
        }
    }
    assert_eq!(sum, 100);
}

#[test]
fn test_anneal() {
    // Write unsorted values: 30, 10, 20
    // [ push(30) push(8) push(8) g_write()
    //   push(10) push(8) push(9) g_write()
    //   push(20) push(9) push(8) g_write()
    //   push(2) anneal() ]
    // Radius 2 includes these points.

    let genes = vec![
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(30)] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(8)] }, // y
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(8)] }, // x
        Gene { op: OpCode::GWrite, args: vec![] },

        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(10)] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(8)] }, // y
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(9)] }, // x
        Gene { op: OpCode::GWrite, args: vec![] },

        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(20)] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(9)] }, // y
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(8)] }, // x
        Gene { op: OpCode::GWrite, args: vec![] },

        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(2)] },
        Gene { op: OpCode::Anneal, args: vec![] },
    ];

    let mut vm = make_vm(genes);
    vm.context_loc = (8, 8);

    while !vm.halted {
        vm.step();
    }

    // Collect non-zero values in radius 2
    let coords = vm.get_circular_coords(8, 8, 2);
    let mut values = Vec::new();
    for (x, y) in coords {
        if let Value::Int(n) = vm.grid[y][x] {
            if n > 0 {
                values.push(n);
            }
        }
    }

    // Check if we have 10, 20, 30
    values.sort();
    assert_eq!(values, vec![10, 20, 30]);
}
