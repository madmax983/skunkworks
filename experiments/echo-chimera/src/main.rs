use chimera_lang::prelude::*;

fn main() {
    // DNA: [ photosynthesize(), jump(0) ]
    // This organism sits in the sun and loops forever.
    let genes = vec![
        Gene {
            op: OpCode::Photosynthesize,
            args: vec![],
        },
        Gene {
            op: OpCode::Jump,
            args: vec![Nucleotide::Number(0)],
        },
    ];

    let dna = Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    };
    let mut vm = ChimeraVM::new(dna);

    // Run for 100 ticks
    for _ in 0..100 {
        vm.step();
        assert!(vm.energy > 0); // Still alive!
    }
}
