#[cfg(feature = "nova")]
use chimera_lang::ast::{Dna, Gene, Helix, JunctionType, Nucleotide, Strand};
#[cfg(feature = "nova")]
use chimera_lang::opcode::OpCode;
#[cfg(feature = "nova")]
use chimera_lang::vm::nova::OrganelleType;
#[cfg(feature = "nova")]
use chimera_lang::vm::{ChimeraVM, Value};

#[test]
#[cfg(feature = "nova")]
fn test_viral_rewrite_grid() {
    // 1. Setup Grid Content
    let mut grid = vec![vec![Value::Int(0); 16]; 16];
    grid[5][5] = Value::Str("target".to_string());

    // 2. Define Grammar: Match "target"
    let grammar = Value::Junction(
        JunctionType::Any,
        vec![
            Value::Str("Match".to_string()),
            Value::Str("target".to_string()),
        ],
    );

    // 3. Create Virus via Strand
    // Stack: [ ..., (grammar), (quorum_action, quorum_threshold), payload_idx, mutation_rate, pattern_str, name_str, mode ]
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Junction(
                JunctionType::Any,
                vec![
                    Nucleotide::String("Match".to_string()),
                    Nucleotide::String("target".to_string()),
                ],
            )],
        }, // Grammar
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(-1)],
        }, // Payload
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(100)],
        }, // Rate
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::String("target".to_string())],
        }, // Pattern
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::String("Rewriter".to_string())],
        }, // Name
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        }, // Mode 1 = RewriteGrid
        Gene {
            op: OpCode::Infect,
            args: vec![],
        },
    ];

    let dna = Dna {
        helix: Helix {
            strands: vec![
                Strand { genes }, // Strand 0: Infect
                Strand {
                    genes: vec![Gene {
                        op: OpCode::Outbreak,
                        args: vec![],
                    }],
                }, // Strand 1: Outbreak
            ],
        },
    };

    let mut vm = ChimeraVM::new(dna);
    vm.grid = grid;
    vm.context_loc = (5, 5); // Set context for Infect

    // Run Strand 0 (Infect)
    while vm.ip.0 == 0 && !vm.halted {
        vm.step();
    }

    // 4. Verify Infection
    assert!(vm.viral_grid[5][5].is_some());
    let v_state = vm.viral_grid[5][5].unwrap();
    assert_eq!(v_state.infection_level, 100);

    // Run Strand 1 (Outbreak)
    vm.ip = (1, 0);
    // Outbreak usually runs as a single op, but we step until done
    while vm.ip.0 == 1 && !vm.halted {
        vm.step();
    }

    // 6. Check Result
    if let Value::Str(s) = &vm.grid[5][5] {
        println!("Original: target, New: {}", s);
        assert_ne!(s, "target", "Grid content should have been rewritten");
    } else {
        panic!("Grid content type changed!");
    }
}

#[test]
#[cfg(feature = "nova")]
fn test_viral_rewrite_dna() {
    // 1. Setup DNA
    // Strand 0: Setup Infect
    // Strand 1: Outbreak
    // Strand 2: Victim [ push(1) ]

    let infect_genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Junction(
                JunctionType::Any,
                vec![
                    Nucleotide::String("Regex".to_string()),
                    Nucleotide::String(".*".to_string()),
                ],
            )],
        }, // Grammar
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(-1)],
        }, // Payload
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(100)],
        }, // Rate
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::String(".*".to_string())],
        }, // Pattern
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::String("GeneHacker".to_string())],
        }, // Name
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(2)],
        }, // Mode 2 = RewriteDNA
        Gene {
            op: OpCode::Infect,
            args: vec![],
        },
    ];

    let dna = Dna {
        helix: Helix {
            strands: vec![
                Strand {
                    genes: infect_genes,
                },
                Strand {
                    genes: vec![Gene {
                        op: OpCode::Outbreak,
                        args: vec![],
                    }],
                },
                // Victim Strand: Infinite Loop so it stays alive
                Strand {
                    genes: vec![
                        Gene {
                            op: OpCode::Push,
                            args: vec![Nucleotide::Number(1)],
                        },
                        Gene {
                            op: OpCode::Jump,
                            args: vec![Nucleotide::Number(2)],
                        }, // Jump to self (Strand 2)
                    ],
                },
            ],
        },
    };
    let mut vm = ChimeraVM::new(dna);

    // 2. Spawn Organelle executing Strand 2
    use chimera_lang::vm::nova::Organelle;
    vm.organelle_id_counter += 1;
    let organelle = Organelle {
        stack: Vec::new(),
        ip: (2, 0), // Executing strand 2
        context_loc: (5, 5),
        call_stack: Vec::new(),
        recursion_depth: 0,
        halted: false,
        kind: OrganelleType::Worker,
        direction: (0, 0),
        ttl: None,
        name: "Victim".to_string(),
        traits: vec![],
        id: vm.organelle_id_counter,
        tissue_id: None,
        genome_id: 0,
        energy: 50,
        experience: 0,
        stage: 0,
    };
    vm.organelles.push(organelle);

    vm.context_loc = (5, 5); // Context for infect

    // 3. Run Infect
    while vm.ip.0 == 0 && !vm.halted {
        vm.step();
    }

    // 4. Verify Infection
    assert!(vm.viral_grid[5][5].is_some());

    // 5. Run Outbreak
    vm.ip = (1, 0);
    while vm.ip.0 == 1 && !vm.halted {
        vm.step();
    }

    // 6. Verify Organelle IP changed
    let org = &vm.organelles[0];
    println!("Organelle IP: {:?}", org.ip);

    // Should NOT be strand 2 anymore (unless rewrite failed silently)
    // If successful, it points to a new strand (index 3).
    // If compilation fails, it stays at 2? No, logic says "if Ok(cst)..."
    // With ".*" regex, it should parse.

    assert_ne!(
        org.ip.0, 2,
        "Organelle should have been moved to a new strand"
    );
    assert!(vm.dna.helix.strands.len() > 3);
}
