#![cfg(all(test, feature = "nova"))]

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
fn test_meme_spreading() {
    let s0 = Strand {
        genes: vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(42)],
            },
            Gene {
                op: OpCode::Meme,
                args: vec![],
            },
        ],
    };
    let s1 = Strand { genes: vec![] }; // Empty target

    let dna = Dna {
        helix: Helix {
            strands: vec![s0, s1],
        },
    };
    let mut vm = ChimeraVM::new(dna);

    // Step 1: Push(42)
    vm.step();
    assert_eq!(vm.stack.last(), Some(&Value::Int(42)));
    assert!(vm.last_gene.is_some());

    // Step 2: Meme()
    // Loop until s1 has genes to ensure infection
    let mut infected = false;
    for _ in 0..100 {
        // Run Push(42) to set last_gene
        vm.ip = (0, 0);
        vm.step();

        // Run Meme
        // ip is (0, 1) now
        vm.step();

        if !vm.dna.helix.strands[1].genes.is_empty() {
            infected = true;
            break;
        }
    }

    assert!(infected, "Meme failed to infect strand 1");
    let infected_gene = &vm.dna.helix.strands[1].genes[0];
    assert_eq!(infected_gene.op, OpCode::Push);
    if let Nucleotide::Number(n) = infected_gene.args[0] {
        assert_eq!(n, 42);
    } else {
        panic!("Wrong gene args in infected strand");
    }
}

#[test]
fn test_poly_int() {
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(5)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(5)],
        },
        Gene {
            op: OpCode::Poly,
            args: vec![
                Nucleotide::String("add".to_string()),
                Nucleotide::String("sub".to_string()),
            ],
        },
    ];
    let mut vm = ChimeraVM::new(make_dna(genes));

    vm.step(); // Push 5
    vm.step(); // Push 5
    vm.step(); // Poly -> Peeks 5 (Int) -> Add. Pops 5, 5. Pushes 10.

    assert_eq!(vm.stack.len(), 1);
    assert_eq!(vm.stack[0], Value::Int(10));
}

#[test]
fn test_poly_str() {
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(5)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::String("foo".to_string())],
        },
        Gene {
            op: OpCode::Poly,
            args: vec![
                Nucleotide::String("add".to_string()),
                Nucleotide::String("drop".to_string()),
            ],
        },
    ];
    let mut vm = ChimeraVM::new(make_dna(genes));

    vm.step(); // Push 5
    vm.step(); // Push "foo"
    vm.step(); // Poly -> Peeks "foo" (Str) -> Drop. Pops "foo".

    assert_eq!(vm.stack.len(), 1);
    assert_eq!(vm.stack[0], Value::Int(5));
}

#[test]
fn test_drift() {
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        }, // Noise
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        }, // Noise
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(100)],
        }, // Prob
        Gene {
            op: OpCode::Drift,
            args: vec![],
        },
    ];

    let mut vm = ChimeraVM::new(make_dna(genes));

    // Run enough steps
    for _ in 0..4 {
        vm.step();
    }

    // Check if genes changed.
    let strand = &vm.dna.helix.strands[0];
    let mut changed = false;
    for (i, gene) in strand.genes.iter().enumerate() {
        if i < 3 && gene.op != OpCode::Push {
            changed = true;
        }
    }

    assert!(changed, "Drift failed to mutate genes");
}
