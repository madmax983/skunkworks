use chimera_lang::prelude::*;
use rand::Rng;

pub fn generate_random_dna(num_strands: usize, max_len: usize) -> Dna {
    let mut rng = rand::thread_rng();
    let mut strands = Vec::new();

    for _ in 0..num_strands {
        let len = rng.gen_range(1..=max_len);
        let mut genes = Vec::new();

        for _ in 0..len {
            // Pick a random OpCode
            // Using only Core and Nova OpCodes
            let op_idx = rng.gen_range(0..20);
            let op = match op_idx {
                0 => OpCode::Push,
                1 => OpCode::Add,
                2 => OpCode::Sub,
                3 => OpCode::Mul,
                4 => OpCode::Div,
                5 => OpCode::Photosynthesize,
                6 => OpCode::GRead,
                7 => OpCode::GWrite,
                8 => OpCode::Radiate,
                9 => OpCode::Siphon,
                10 => OpCode::Mitosis,
                11 => OpCode::Apoptosis,
                12 => OpCode::Transcribe,
                13 => OpCode::Splice,
                14 => OpCode::Recombine,
                15 => OpCode::Entropy,
                16 => OpCode::Lumine,
                17 => OpCode::Knot, // Requires Nova
                18 => OpCode::Weave, // Requires Nova
                _ => OpCode::Nop,
            };

            // Generate Args
            let mut args = Vec::new();
            if matches!(op, OpCode::Push) {
                args.push(Nucleotide::Number(rng.gen_range(0..100)));
            } else if matches!(op, OpCode::Radiate | OpCode::Siphon) {
                args.push(Nucleotide::Number(rng.gen_range(1..10))); // Radius
            }

            genes.push(Gene { op, args });
        }
        strands.push(Strand { genes });
    }

    Dna {
        helix: Helix { strands },
        evolution_config: None,
    }
}
