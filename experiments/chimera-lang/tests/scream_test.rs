#![cfg(feature = "resonance")]

use chimera_lang::prelude::*;
use chimera_lang::vm::ChimeraVM;
use crossbeam_channel::unbounded;

#[test]
fn test_scream_shockwave() {
    // [ push(50) push(100) scream ]
    // duration=50, strength=100
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(50)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(100)],
        },
        Gene {
            op: OpCode::Scream,
            args: vec![],
        },
    ];
    let dna = Dna { evolution_config: None,
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    };
    let mut vm = ChimeraVM::new(dna);

    // Setup audio channel
    let (tx, _rx) = unbounded();
    vm.set_audio_tx(tx);

    // Set context location to center
    vm.context_loc = (8, 8);

    vm.step(); // Push 50
    vm.step(); // Push 100
    vm.step(); // Scream

    // Check output log
    if let Some(last_log) = vm.output.last() {
        assert!(last_log.contains("SCREAM: 8,8 str=10.00 dur=50"));
    } else {
        panic!("No output log generated");
    }

    // Check visual snapshot (Ring radius 5)
    // Center (8,8) is dist 0. Radius is 5. |0-5|=5 > 1.5. Center is unaffected.
    // Point (13,8) is dist 5. |5-5|=0 < 1.5. Affected.
    // idx = 8*16 + 13 = 141.
    if vm.audio_snapshot.pressure.len() > 141 {
        assert_eq!(vm.audio_snapshot.pressure[141], 5.0);
    } else {
        panic!("Snapshot too small");
    }
}
