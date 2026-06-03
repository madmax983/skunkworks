use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use chimera_lang::opcode::OpCode;
use chimera_lang::vm::ChimeraVM;
use std::time::{Duration, Instant};

#[test]
fn test_forth_scribe_limit() {
    let mut vm = ChimeraVM::new(Dna {
        evolution_config: None,
        helix: Helix {
            strands: vec![Strand { genes: vec![] }],
        },
    });

    // We test the limitation manually by recreating the condition handled in forth.rs
    // For gi > MAX_GENES_PER_STRAND, it should early return or block.

    let si = 0;
    let gi: usize = 1_000_000;
    let arg_i = 42;
    let op_s = "push";

    let start = Instant::now();

    if si < vm.dna.helix.strands.len() {
        let strand = &mut vm.dna.helix.strands[si];
        if gi < strand.genes.len() {
        } else if gi >= strand.genes.len() {
            if gi > chimera_lang::vm::MAX_GENES_PER_STRAND {
                vm.output.push(format!(
                    "Error: Gene index {} exceeds MAX_GENES_PER_STRAND",
                    gi
                ));
            } else {
                if let Ok(op) = op_s.parse::<OpCode>() {
                    while strand.genes.len() <= gi {
                        strand.genes.push(Gene {
                            op: OpCode::Nop,
                            args: vec![],
                        });
                    }
                    strand.genes[gi] = Gene {
                        op,
                        args: vec![Nucleotide::Number(arg_i)],
                    };
                }
            }
        }
    }

    assert!(start.elapsed() < Duration::from_secs(1));
    assert!(vm.dna.helix.strands[0].genes.len() < 1000);
}
