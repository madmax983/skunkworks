use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use chimera_lang::opcode::OpCode;
use chimera_lang::vm::ChimeraVM;
use chimera_lang::vm::Value;

fn make_vm(genes: Vec<Gene>) -> ChimeraVM {
    let dna = Dna { evolution_config: None,
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    };
    ChimeraVM::new(dna)
}

#[test]
#[cfg(feature = "nova")]
fn test_conceive() {
    let dna_genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(100)],
        }, // Fid
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(100)],
        }, // Vir
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(2)],
        }, // Len
        Gene {
            op: OpCode::Conceive,
            args: vec![],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(99)],
        }, // Target for capture
    ];

    let mut vm = make_vm(dna_genes);

    // Step 1-3: Push args
    vm.step();
    vm.step();
    vm.step();

    // Step 4: Conceive
    // Should capture Conceive (itself) and Push(99).
    vm.step();

    assert_eq!(vm.meme_pool.memes.len(), 1);
    let meme = &vm.meme_pool.memes[0];
    assert_eq!(meme.genes.len(), 2);
    assert_eq!(meme.genes[1].op, OpCode::Push);

    // Check stack has meme_id (0)
    assert_eq!(vm.stack.last(), Some(&Value::Int(0)));
}

#[test]
#[cfg(feature = "nova")]
fn test_propagate() {
    let strand0 = Strand {
        genes: vec![
            // Setup stack for Conceive: Fid=100, Vir=100, Len=1
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(100)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(100)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Conceive,
                args: vec![],
            }, // Captures itself
            // Setup stack for Propagate: Target=1, Meme=0 (from stack)
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            }, // Target
            Gene {
                op: OpCode::Propagate,
                args: vec![],
            },
        ],
    };

    let strand1 = Strand { genes: vec![] };

    let dna = Dna { evolution_config: None,
        helix: Helix {
            strands: vec![strand0, strand1],
        },
    };
    let mut vm = ChimeraVM::new(dna);

    // Push 3 args
    vm.step();
    vm.step();
    vm.step();

    // Conceive -> Pushes MemeID 0
    vm.step();

    // Push Target 1
    vm.step();

    // Propagate
    vm.step();

    // Check Strand 1
    let s1 = &vm.dna.helix.strands[1];
    assert!(!s1.genes.is_empty());
    assert_eq!(s1.genes[0].op, OpCode::Conceive);
}

#[test]
#[cfg(feature = "nova")]
fn test_shibboleth() {
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(5)],
        }, // Initial value
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::String("push".to_string())],
        }, // From
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::String("drop".to_string())],
        }, // To
        Gene {
            op: OpCode::Shibboleth,
            args: vec![],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(10)],
        }, // Should act as Drop
    ];

    let mut vm = make_vm(genes);

    vm.step(); // Push 5
    assert_eq!(vm.stack.len(), 1);

    vm.step(); // Push "drop"
    vm.step(); // Push "push"
    vm.step(); // Shibboleth

    // Stack should have 5 (args popped)
    assert_eq!(vm.stack.len(), 1);
    assert_eq!(vm.stack[0], Value::Int(5));

    // Now exec Push(10). It should be mapped to Drop.
    // Drop pops the top value (5).
    vm.step();

    assert_eq!(vm.stack.len(), 0);
}
