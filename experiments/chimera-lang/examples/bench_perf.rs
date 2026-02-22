use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use chimera_lang::opcode::OpCode;
use chimera_lang::vm::ChimeraVM;
use std::time::Instant;

fn main() {
    // [ push(1) push(1) add() jump(0) ]
    // Infinite loop adding numbers.
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
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

    let dna = Dna { evolution_config: None,
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    };

    let mut vm = ChimeraVM::new(dna);
    // Give lots of energy so it doesn't die immediately
    vm.energy = 1_000_000_000;

    let start = Instant::now();
    let steps = 1_000_000;

    for _ in 0..steps {
        vm.step();
        if vm.halted {
            break;
        }
    }

    let duration = start.elapsed();
    println!("Executed {} steps in {:?}", steps, duration);
    println!("Stack top: {:?}", vm.stack.last());
}
