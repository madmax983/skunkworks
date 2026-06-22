//! Binary crate documentation.
use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use chimera_lang::opcode::OpCode;
use chimera_lang::vm::ChimeraVM;

fn main() {
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(255)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(100)],
        },
        Gene {
            op: OpCode::Luciferin,
            args: vec![],
        },
    ];
    let dna = Dna {
        evolution_config: None,
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    };
    let mut vm = ChimeraVM::new(dna);

    vm.step(); // push
    vm.step(); // push
    vm.step(); // push
    vm.step(); // push
    vm.step(); // luciferin

    #[cfg(feature = "nova")]
    println!("Light grid at (8,8): {}", vm.light_grid[8][8]);
    println!("Output: {:?}", vm.output);
}
