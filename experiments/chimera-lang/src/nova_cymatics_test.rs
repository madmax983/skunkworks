#![cfg(all(test, feature = "nova", feature = "resonance"))]

use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use crate::opcode::OpCode;
use crate::vm::{ChimeraVM, Value};
use resonance_audio::audio::AudioSnapshot;

fn make_vm(genes: Vec<Gene>) -> ChimeraVM {
    let dna = Dna { evolution_config: None,
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    };
    ChimeraVM::new(dna)
}

#[test]
fn test_cymatics_sift() {
    // Sift(radius=1)
    // Setup grid: center (8,8) has value 10.
    // Neighbor (8,9) has value 0.
    // Audio snapshot: center amp = 1.0, neighbor amp = 0.1.
    // Sift should move 10 from center to neighbor (towards node).

    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)], // radius
        },
        Gene {
            op: OpCode::Sift,
            args: vec![],
        },
    ];
    let mut vm = make_vm(genes);

    // Setup Grid
    vm.grid[8][8] = Value::Int(10);

    // Setup Audio Snapshot (Mocking Resonance)
    // 16x16 = 256
    // Initialize to high amplitude so particles don't drift to default 0.0 cells
    let mut snapshot = vec![1.0; 256];
    let center_idx = 8 * 16 + 8;
    let neighbor_idx = 8 * 16 + 9;

    snapshot[center_idx] = 1.0; // High amp
    snapshot[neighbor_idx] = 0.1; // Low amp (Node)

    vm.audio_snapshot.pressure = snapshot;

    vm.step(); // Execute Push
    vm.step(); // Execute Sift

    // Check results
    // Original should be empty
    assert_eq!(vm.grid[8][8], Value::Int(0), "Center should be empty");
    // Neighbor should have value
    assert_eq!(
        vm.grid[8][9],
        Value::Int(10),
        "Neighbor should receive value"
    );

    // Check output
    assert!(vm.output.iter().any(|s| s.contains("SIFT: Moved 1")));
}

#[test]
fn test_cymatics_reshape_solidify() {
    // Reshape(threshold=50, mode=0 (Solidify))
    // Snapshot: center > 0.5.
    // Expect membrane at center to become 15 (Wall).

    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(50)], // threshold 0.5
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)], // mode 0
        },
        Gene {
            op: OpCode::Reshape,
            args: vec![],
        },
    ];
    let mut vm = make_vm(genes);

    let mut snapshot = vec![0.0; 256];
    let center_idx = 8 * 16 + 8;
    snapshot[center_idx] = 0.8; // > 0.5

    vm.audio_snapshot.pressure = snapshot;

    vm.step();
    vm.step();
    vm.step(); // Reshape

    assert_eq!(vm.membranes[8][8], 15, "Membrane should be solidified");
    assert!(vm.output.iter().any(|s| s.contains("Solidified 1 cells")));
}

#[test]
fn test_cymatics_reshape_liquefy() {
    // Reshape(threshold=50, mode=1 (Liquefy))
    // Snapshot: center > 0.5.
    // Membrane starts as 15.
    // Expect membrane to become 0.

    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(50)], // threshold 0.5
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)], // mode 1
        },
        Gene {
            op: OpCode::Reshape,
            args: vec![],
        },
    ];
    let mut vm = make_vm(genes);

    let mut snapshot = vec![0.0; 256];
    let center_idx = 8 * 16 + 8;
    snapshot[center_idx] = 0.8;

    vm.audio_snapshot.pressure = snapshot;
    vm.membranes[8][8] = 15; // Start with wall

    vm.step();
    vm.step();
    vm.step(); // Reshape

    assert_eq!(vm.membranes[8][8], 0, "Membrane should be liquefied");
    assert!(vm.output.iter().any(|s| s.contains("Liquefied 1 cells")));
}
