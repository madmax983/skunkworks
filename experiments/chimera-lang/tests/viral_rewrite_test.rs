#[cfg(feature = "nova")]
use chimera_lang::ast::{Dna, Gene, Helix, JunctionType, Nucleotide, Strand};
#[cfg(feature = "nova")]
use chimera_lang::opcode::OpCode;
#[cfg(feature = "nova")]
#[cfg(feature = "nova")]
use chimera_lang::vm::{ChimeraVM, Value};
#[test]
#[cfg(feature = "nova")]
fn test_viral_rewrite_grid() {
    // 1. Setup Grid Content
    let mut grid = vec![vec![Value::Int(0); 16]; 16];
    grid[5][5] = Value::Str("target".to_string());
    // 2. Define Grammar: Match "target"
    let _grammar = Value::Junction(
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
        evolution_config: None,
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
