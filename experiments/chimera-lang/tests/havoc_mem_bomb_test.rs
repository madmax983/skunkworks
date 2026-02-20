use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use chimera_lang::opcode::OpCode;
use chimera_lang::vm::ChimeraVM;

#[test]
#[ignore]
fn test_mem_bomb() {
    // 1. Construct the bomb DNA
    let mut genes = Vec::new();

    // Pump Energy first
    for _ in 0..50 {
        genes.push(Gene {
            op: OpCode::Photosynthesize,
            args: vec![],
        });
    }

    // Push "A" (1 byte)
    genes.push(Gene {
        op: OpCode::Push,
        args: vec![Nucleotide::String("A".to_string())],
    });

    // Double it 30 times -> 1 GB
    for _ in 0..30 {
        // Keep energy up
        genes.push(Gene {
            op: OpCode::Photosynthesize,
            args: vec![],
        });
        genes.push(Gene {
            op: OpCode::Photosynthesize,
            args: vec![],
        });

        genes.push(Gene {
            op: OpCode::Dup,
            args: vec![],
        });
        genes.push(Gene {
            op: OpCode::Add,
            args: vec![],
        });
    }

    // Recursion Bomb: Call Simulate on Self
    genes.push(Gene {
        op: OpCode::Push,
        args: vec![Nucleotide::Number(0)],
    });
    genes.push(Gene {
        op: OpCode::Push,
        args: vec![Nucleotide::Number(100)],
    });
    genes.push(Gene {
        op: OpCode::Simulate,
        args: vec![],
    });

    let strand = Strand { genes };
    let dna = Dna {
        helix: Helix {
            strands: vec![strand],
        },
    };

    let mut vm = ChimeraVM::new(dna);
    // Give initial boost to be safe
    vm.energy = 10000;

    println!("👺 Havoc: Detonating 11GB Memory Bomb...");
    // This loop should crash with OOM
    while !vm.halted && vm.tick_counter < 10000 {
        vm.step();
        if vm.energy < 10 {
            vm.energy = 10000; // Cheat mode: infinite energy from "God" (Test harness)
        }
    }
}
