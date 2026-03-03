use chimera_lang::ast::{Dna, Gene, Helix, JunctionType, Nucleotide, Strand};
use chimera_lang::opcode::OpCode;
use chimera_lang::vm::ChimeraVM;

fn make_dna(genes1: Vec<Gene>, genes2: Vec<Gene>) -> Dna {
    Dna {
        evolution_config: None,
        helix: Helix {
            strands: vec![Strand { genes: genes1 }, Strand { genes: genes2 }],
        },
    }
}

fn main() {
    let chord = vec![
        Nucleotide::String("Fiat".to_string()),
        Nucleotide::String("Lux".to_string()),
    ];

    let main_strand = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Junction(JunctionType::All, chord.clone())],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        },
        Gene {
            op: OpCode::Harmonize,
            args: vec![],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Junction(JunctionType::All, chord)],
        },
        Gene {
            op: OpCode::Choir,
            args: vec![],
        },
        Gene {
            op: OpCode::Jump,
            args: vec![Nucleotide::Number(0)],
        },
    ];

    let effect_strand = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(100)],
        },
        Gene {
            op: OpCode::Ret,
            args: vec![],
        },
    ];

    let mut vm = ChimeraVM::new(make_dna(main_strand, effect_strand));
    vm.metamorphism_enabled = false;
    vm.energy = 1000;

    for i in 0..12 {
        println!(
            "Step {}: IP={:?}, OP={:?}",
            i,
            vm.ip,
            vm.dna.helix.strands[vm.ip.0]
                .genes
                .get(vm.ip.1)
                .map(|g| &g.op)
        );
        vm.step();
        println!("  Output: {:?}", vm.output.last());
        println!("  Chorus Buffer: {:?}", vm.chorus_buffer);
        println!("  Organelles: {}", vm.organelles.len());
        println!("  Stack: {:?}", vm.stack);
    }
}
