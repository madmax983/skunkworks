use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use chimera_lang::opcode::OpCode;
use chimera_lang::vm::{ChimeraVM, MAX_STRANDS};

#[test]
#[cfg(feature = "nova")]
fn test_biohack_oom_prevention() {
    // 1. Setup a VM with a Meme defined
    let setup_genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        }, // Len
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(100)],
        }, // Vir
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(100)],
        }, // Fid
        Gene {
            op: OpCode::Conceive,
            args: vec![],
        }, // Meme 0
    ];

    let dna = Dna {
        helix: Helix {
            strands: vec![Strand { genes: setup_genes }],
        },
    };
    let mut vm = ChimeraVM::new(dna);

    // Execute setup
    for _ in 0..10 {
        vm.step();
    }
    assert_eq!(vm.meme_pool.memes.len(), 1, "Meme creation failed");

    // 2. Loop BioHack until we hit limit + 1
    // We manually push args and call BioHack logic via step

    // We can replace the current strand with a BioHack loop.
    let loop_genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::String("Virus".to_string())],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Junction(
                chimera_lang::ast::JunctionType::Any,
                vec![],
            )],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::BioHack,
            args: vec![],
        },
        Gene {
            op: OpCode::Jump,
            args: vec![Nucleotide::Number(0)],
        },
    ];

    vm.dna.helix.strands[0] = Strand { genes: loop_genes };
    vm.ip = (0, 0); // Reset IP
    vm.halted = false; // Reset halted state
    vm.telomeres[0] = 100000; // Prevent senescence

    // Run enough times to exceed MAX_STRANDS
    // Each loop is 5 steps.
    // We want to add MAX_STRANDS strands.
    let target = MAX_STRANDS + 10;
    let max_steps = target * 6;

    for i in 0..max_steps {
        vm.energy = 1000; // Prevent starvation
        if vm.halted {
            break;
        }
        vm.step();
        if i % 1000 == 0 {
            println!("Step {}: Strands {}", i, vm.dna.helix.strands.len());
        }
    }

    println!("VM Output: {:?}", vm.output);
    println!("Final Strand Count: {}", vm.dna.helix.strands.len());

    // If vulnerable, this will be > MAX_STRANDS
    // We assert it should be <= MAX_STRANDS (which will fail initially)
    assert!(
        vm.dna.helix.strands.len() <= MAX_STRANDS,
        "Exceeded MAX_STRANDS limit: {}",
        vm.dna.helix.strands.len()
    );
}

#[test]
#[cfg(feature = "nova")]
fn test_scavenge_oom_prevention() {
    let loop_genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::String("Cargo.toml".to_string())],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(10)],
        }, // Len
        Gene {
            op: OpCode::Scavenge,
            args: vec![],
        },
        Gene {
            op: OpCode::Jump,
            args: vec![Nucleotide::Number(0)],
        },
    ];

    let dna = Dna {
        helix: Helix {
            strands: vec![Strand { genes: loop_genes }],
        },
    };
    let mut vm = ChimeraVM::new(dna);

    // Set sandbox root to current dir so it can find Cargo.toml
    vm.sandbox_root = std::env::current_dir().unwrap();
    vm.halted = false; // Reset halted state just in case, though new() implies false
    vm.telomeres[0] = 100000; // Prevent senescence

    let target = MAX_STRANDS + 10;
    let max_steps = target * 5;

    for _ in 0..max_steps {
        vm.energy = 1000; // Prevent starvation
        if vm.halted {
            break;
        }
        vm.step();
    }

    println!(
        "Final Strand Count (Scavenge): {}",
        vm.dna.helix.strands.len()
    );
    assert!(
        vm.dna.helix.strands.len() <= MAX_STRANDS,
        "Exceeded MAX_STRANDS limit via Scavenge: {}",
        vm.dna.helix.strands.len()
    );
}
