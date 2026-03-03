use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use chimera_lang::opcode::OpCode;
use rand::Rng;

pub fn generate_rhythm_dna(instrument_id: usize, sustain: i64, rest: i64) -> Dna {
    let play_cmd = format!("PLAY:{}:{}", instrument_id, sustain);
    let sleep_cmd = format!("SLEEP:{}", rest);

    let genes = vec![
        // Gene 0: Push play command
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::String(play_cmd)],
        },
        // Gene 1: Print (Signal Host)
        Gene {
            op: OpCode::Print,
            args: vec![],
        },
        // Gene 2: Push sleep command
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::String(sleep_cmd)],
        },
        // Gene 3: Print (Signal Host)
        Gene {
            op: OpCode::Print,
            args: vec![],
        },
        // Gene 4: Jump to start (0)
        Gene {
            op: OpCode::Jump,
            args: vec![Nucleotide::Number(0)],
        },
    ];

    Dna {
        evolution_config: None,
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rhythm_dna_generation() {
        let dna = generate_rhythm_dna(0, 50, 450);
        assert_eq!(dna.helix.strands.len(), 1);
        assert_eq!(dna.helix.strands[0].genes.len(), 5);
    }

    #[test]
    fn test_jazz_dna_generation() {
        let dna = generate_jazz_dna(3);
        assert_eq!(dna.helix.strands.len(), 1);
        // 20 notes * 4 genes + 1 jump = 81 genes
        assert_eq!(dna.helix.strands[0].genes.len(), 81);
    }
}

pub fn generate_jazz_dna(instrument_count: usize) -> Dna {
    let mut rng = rand::thread_rng();
    let mut genes = Vec::new();

    // Generate a sequence of 20 random notes
    for _ in 0..20 {
        let inst = rng.gen_range(0..instrument_count);
        let dur = rng.gen_range(50..150);
        let rest = rng.gen_range(100..400);

        let play_cmd = format!("PLAY:{}:{}", inst, dur);
        let sleep_cmd = format!("SLEEP:{}", rest);

        genes.push(Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::String(play_cmd)],
        });
        genes.push(Gene {
            op: OpCode::Print,
            args: vec![],
        });
        genes.push(Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::String(sleep_cmd)],
        });
        genes.push(Gene {
            op: OpCode::Print,
            args: vec![],
        });
    }

    // Loop back to start
    genes.push(Gene {
        op: OpCode::Jump,
        args: vec![Nucleotide::Number(0)],
    });

    Dna {
        evolution_config: None,
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    }
}
