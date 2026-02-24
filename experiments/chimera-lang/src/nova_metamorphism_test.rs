#![cfg(feature = "nova")]

use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use crate::opcode::OpCode;
use crate::vm::{ChimeraVM, Value};

fn make_dna(genes: Vec<Gene>) -> Dna {
    Dna {
        evolution_config: None,
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    }
}

#[test]
fn test_metamorphism_compression() {
    // Sequence: [ Push(10), Push(20), Add, Jump(0) ]
    // This should compress to [ Push(30), Jump(0) ] under pressure.
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(10)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(20)],
        },
        Gene {
            op: OpCode::Add,
            args: vec![],
        },
        Gene {
            op: OpCode::Jump,
            args: vec![Nucleotide::Number(0)],
        },
    ];

    let mut vm = ChimeraVM::new(make_dna(genes));

    // Increase pressure (Stack depth > 20)
    for i in 0..25 {
        vm.stack.push(Value::Int(i));
    }

    // Add a second strand that does nothing, so we can metamorphose IT while executing strand 0
    // Actually, `process_metamorphism` picks a random strand.
    // To reliably test, we should have the target strand NOT be the current IP.
    // Let's make strand 1 the target.

    let target_genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(10)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(20)],
        },
        Gene {
            op: OpCode::Add,
            args: vec![],
        },
        Gene {
            op: OpCode::Jump,
            args: vec![Nucleotide::Number(1)],
        },
    ];

    vm.dna.helix.strands.push(Strand {
        genes: target_genes,
    });

    // vm.ip is (0,0). Target is strand 1.

    // Force metamorphism
    // We can call `process_metamorphism` directly if it's pub, but it relies on RNG.
    // Instead, we loop until change happens or timeout.

    let mut changed = false;
    for _ in 0..100 {
        crate::vm::nova_metamorphism::process_metamorphism(&mut vm);

        let s1 = &vm.dna.helix.strands[1];
        if s1.genes.len() < 4 {
            // Compressed!
            // Should be Push(30), Jump(1) => len 2
            if s1.genes.len() == 2 {
                if let Some(Nucleotide::Number(n)) = s1.genes[0].args.first() {
                    if *n == 30 {
                        changed = true;
                        break;
                    }
                }
            }
        }
    }

    assert!(changed, "Metamorphism compression failed to trigger");
}

#[test]
fn test_metamorphism_expansion() {
    // Sequence: [ Push(30), Jump(1) ]
    // Should expand to [ Push(15), Push(15), Add, Jump(1) ] under heat.

    let genes = vec![Gene {
        op: OpCode::Jump,
        args: vec![Nucleotide::Number(0)],
    }];
    let mut vm = ChimeraVM::new(make_dna(genes));

    // Add target strand
    let target_genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(30)],
        },
        Gene {
            op: OpCode::Jump,
            args: vec![Nucleotide::Number(1)],
        },
    ];
    vm.dna.helix.strands.push(Strand {
        genes: target_genes,
    });

    // High Energy (Heat > 100)
    vm.energy = 200;

    // Low Pressure (Stack empty)
    vm.stack.clear();

    let mut changed = false;
    for _ in 0..100 {
        crate::vm::nova_metamorphism::process_metamorphism(&mut vm);

        let s1 = &vm.dna.helix.strands[1];
        if s1.genes.len() > 2 {
            // Expanded!
            // Should be Push(15), Push(15), Add, Jump(1) => len 4
            if s1.genes.len() == 4 {
                if s1.genes[0].op == OpCode::Push
                    && s1.genes[1].op == OpCode::Push
                    && s1.genes[2].op == OpCode::Add
                {
                    changed = true;
                    break;
                }
            }
        }
    }

    assert!(changed, "Metamorphism expansion failed to trigger");
}
