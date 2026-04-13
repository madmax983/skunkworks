use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use chimera_lang::opcode::OpCode;
use chimera_lang::vm::{ChimeraVM, Value};

fn make_dna(genes: Vec<Gene>) -> Dna {
    Dna {
        evolution_config: None,
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    }
}

#[test]
#[cfg(feature = "nova")]
fn test_evolve_default() {
    // Test default GoL rule (B3/S23)
    // Setup a blinker at (5,5), (5,6), (5,7)
    // | 0 0 0 |      | 0 1 0 |
    // | 1 1 1 |  ->  | 0 1 0 |
    // | 0 0 0 |      | 0 1 0 |

    let genes = vec![
        // Write 1 to (5,5), (5,6), (5,7)
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(5)],
        }, // y
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(5)],
        }, // x
        Gene {
            op: OpCode::GWrite,
            args: vec![],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(5)],
        }, // y
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(6)],
        }, // x
        Gene {
            op: OpCode::GWrite,
            args: vec![],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(5)],
        }, // y
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(7)],
        }, // x
        Gene {
            op: OpCode::GWrite,
            args: vec![],
        },
        // Evolve
        Gene {
            op: OpCode::Evolve,
            args: vec![],
        },
    ];

    let mut vm = ChimeraVM::new(make_dna(genes));
    while !vm.halted {
        vm.step();
    }

    // Check center (5,6) should be alive
    if let Value::Int(v) = vm.grid[5][6] {
        assert_eq!(v, 1);
    } else {
        panic!("Grid value mismatch");
    }

    // Check neighbors (4,6) and (6,6) should be alive (vertical)
    if let Value::Int(v) = vm.grid[4][6] {
        assert_eq!(v, 1);
    } else {
        panic!("Grid value mismatch at 4,6");
    }
    if let Value::Int(v) = vm.grid[6][6] {
        assert_eq!(v, 1);
    } else {
        panic!("Grid value mismatch at 6,6");
    }

    // Check old horizontal neighbors (5,5) and (5,7) should be dead
    if let Value::Int(v) = vm.grid[5][5] {
        assert_eq!(v, 0);
    }
    if let Value::Int(v) = vm.grid[5][7] {
        assert_eq!(v, 0);
    }
}
