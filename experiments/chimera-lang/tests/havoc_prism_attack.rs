#[cfg(feature = "nova")]
#[test]
fn test_prism_cascade() {
    use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::{ChimeraVM, Value};

    // Strand 0: [ Push(8) Push(5) Salvo ]
    let genes_0 = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(8)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(5)],
        },
        Gene {
            op: OpCode::Salvo,
            args: vec![],
        },
    ];
    // Strand 1: [ Jump(1) ] (Infinite Loop to keep VM alive)
    let genes_1 = vec![Gene {
        op: OpCode::Jump,
        args: vec![Nucleotide::Number(1)],
    }];

    let dna = Dna {
        evolution_config: None,
        helix: Helix {
            strands: vec![Strand { genes: genes_0 }, Strand { genes: genes_1 }],
        },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.energy = 1_000_000;

    for y in 0..16 {
        for x in 0..16 {
            vm.grid[y][x] = Value::Str("PRISM:".to_string());
        }
    }

    // Detonate
    for i in 0..=50 {
        vm.step();
        let count = vm.projectiles.len();
        if i % 10 == 0 || i < 10 {
            println!("Tick {}: {} projectiles", i, count);
        }

        if count > 100_000 {
            panic!(
                "SUCCESS: Unbounded projectile growth detected! Count: {}",
                count
            );
        }
    }
}
