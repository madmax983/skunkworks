use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use crate::opcode::OpCode;
use crate::vm::{ChimeraVM, Value};

fn make_vm(genes: Vec<Gene>) -> ChimeraVM {
    let dna = Dna {
        evolution_config: None,
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    };
    ChimeraVM::new(dna)
}

#[test]
fn test_apoptosis_to_graveyard() {
    // [ push(10) push(0) apoptosis() ]
    // Should result in empty strand 0, but graveyard has 1 strand with push(10) etc.
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(10)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        }, // strand idx
        Gene {
            op: OpCode::Apoptosis,
            args: vec![],
        },
    ];
    let mut vm = make_vm(genes);
    vm.step(); // push 10
    vm.step(); // push 0
    vm.step(); // apoptosis

    assert!(vm.dna.helix.strands[0].genes.is_empty());
    assert_eq!(vm.graveyard.len(), 1);
    assert_eq!(vm.graveyard[0].genes[0].op, OpCode::Push);
}

#[test]
fn test_exhume() {
    // 1. Bury a strand
    // 2. Exhume it
    // Initial: [ push(0) bury() exhume() ]
    // Note: burying current strand stops it. So we need 2 strands.
    let strand0 = Strand {
        genes: vec![Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(99)],
        }],
    };
    let strand1 = Strand {
        genes: vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                op: OpCode::Bury,
                args: vec![],
            },
            Gene {
                op: OpCode::Exhume,
                args: vec![],
            },
        ],
    };

    let dna = Dna {
        evolution_config: None,
        helix: Helix {
            strands: vec![strand0, strand1],
        },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.ip = (1, 0); // Start at strand 1

    // Step 1: Push 0
    vm.step();
    // Step 2: Bury 0
    vm.step();
    assert_eq!(vm.graveyard.len(), 1);
    assert!(vm.dna.helix.strands[0].genes.is_empty());

    // Step 3: Exhume
    vm.step();
    assert_eq!(vm.graveyard.len(), 0);
    assert_eq!(vm.dna.helix.strands.len(), 3); // 0 (empty), 1 (controller), 2 (resurrected)

    // Stack should have new index (2)
    assert_eq!(vm.stack.last(), Some(&Value::Int(2)));

    // Check content of resurrected strand
    if let Nucleotide::Number(n) = vm.dna.helix.strands[2].genes[0].args[0] {
        assert_eq!(n, 99);
    } else {
        panic!("Resurrected strand corrupted");
    }
}

#[test]
fn test_seance() {
    // Execute a dead strand without restoring it
    let strand1_genes = vec![Gene {
        op: OpCode::Seance,
        args: vec![],
    }];

    let dna = Dna {
        evolution_config: None,
        helix: Helix {
            strands: vec![
                Strand {
                    genes: vec![Gene {
                        op: OpCode::Push,
                        args: vec![Nucleotide::Number(77)],
                    }],
                },
                Strand {
                    genes: strand1_genes,
                },
            ],
        },
    };
    let mut vm = ChimeraVM::new(dna);

    // Manually bury strand 0
    vm.graveyard.push(vm.dna.helix.strands[0].clone());
    vm.dna.helix.strands[0].genes.clear();

    // Start at strand 1
    vm.ip = (1, 0);

    // Seance should execute the ghost (push 77)
    vm.step();

    assert_eq!(vm.stack.last(), Some(&Value::Int(77)));
    // Graveyard should still have the ghost
    assert_eq!(vm.graveyard.len(), 1);
}

#[test]
fn test_mourn() {
    let mut vm = make_vm(vec![]);
    vm.graveyard.push(Strand { genes: vec![] });
    vm.graveyard.push(Strand { genes: vec![] });
    vm.graveyard.push(Strand { genes: vec![] });

    // Add Mourn manually
    vm.dna.helix.strands[0].genes.push(Gene {
        op: OpCode::Mourn,
        args: vec![],
    });

    let initial_energy = vm.energy;
    vm.step();

    // Cost 1 per step. Gain 2 * 3 = 6. Net +5.
    assert_eq!(vm.energy, initial_energy + 5);
    assert_eq!(vm.stack.last(), Some(&Value::Int(6)));
}

#[test]
fn test_resurrect_from_graveyard() {
    // Setup VM with a strand in graveyard
    let mut vm = make_vm(vec![]);
    vm.graveyard.push(Strand {
        genes: vec![Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(42)],
        }],
    });

    // Resurrect index 0
    let res = vm.resurrect_from_graveyard(0);
    assert!(res.is_ok());

    // Check it moved to helix
    assert_eq!(vm.graveyard.len(), 0);
    assert_eq!(vm.dna.helix.strands.len(), 2); // 1 initial empty + 1 resurrected
    assert_eq!(
        vm.dna.helix.strands[1].genes[0].args[0],
        Nucleotide::Number(42)
    );
}
