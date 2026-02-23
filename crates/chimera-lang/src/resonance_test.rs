#![cfg(feature = "resonance")]
use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use crate::opcode::OpCode;
use crate::vm::ChimeraVM;
use crossbeam_channel::unbounded;
use resonance_audio::audio::AudioCommand;

fn make_dna(genes: Vec<Gene>) -> Dna {
    Dna { evolution_config: None,
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    }
}

#[test]
fn test_pluck() {
    // [ push(100) pluck() ]
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(100)],
        },
        Gene {
            op: OpCode::Pluck,
            args: vec![],
        },
    ];

    let mut vm = ChimeraVM::new(make_dna(genes));
    let (tx, rx) = unbounded();
    vm.set_audio_tx(tx);

    // Initial check
    assert!(rx.is_empty());

    // Run
    // Step 1: Push 100
    vm.step();
    // Step 2: Pluck
    vm.step();

    // Verify
    if let Ok(cmd) = rx.try_recv() {
        match cmd {
            AudioCommand::Pluck { x, y, strength } => {
                assert_eq!(x, 8); // Default context_loc is (8,8)
                assert_eq!(y, 8);
                assert!((strength - 1.0).abs() < f32::EPSILON);
            }
            _ => panic!("Expected Pluck command"),
        }
    } else {
        panic!("No audio command received");
    }
}
