use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use crate::opcode::OpCode;
use crate::vm::{ChimeraVM, Value};

fn make_dna(genes: Vec<Gene>) -> Dna {
    Dna {
        evolution_config: None,
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    }
}

#[test]
fn test_bard_composition() {
    // Composition: Middle C (60) for 4 ticks, D (62) for 4 ticks.
    // Stack for Note: [velocity, duration, pitch] (pitch is top)
    // Wait, let's check bard.rs stack order.
    // stack: velocity, duration, pitch (top)
    // So push velocity, push duration, push pitch.

    let genes = vec![
        // Note 1: C (60)
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(100)],
        }, // Velocity
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(4)],
        }, // Duration (1/4 note if L:1/16)
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(60)],
        }, // Pitch
        Gene {
            op: OpCode::Note,
            args: vec![],
        },
        // Note 2: D (62)
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(100)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(4)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(62)],
        },
        Gene {
            op: OpCode::Note,
            args: vec![],
        },
        // Export
        Gene {
            op: OpCode::Perform,
            args: vec![],
        },
    ];

    let mut vm = ChimeraVM::new(make_dna(genes));
    while !vm.halted {
        vm.step();
    }

    // Check stack for result
    if let Some(Value::Str(abc)) = vm.stack.pop() {
        println!("Generated ABC:\n{}", abc);
        // My implementation: 60 -> c, 62 -> d.
        // Duration 4 -> "4".
        // Header: "X:1\nT:Chimera Composition\nM:4/4\nL:1/16\nK:C\n"
        // Body: "c4 d4 "

        assert!(abc.contains("K:C"));
        assert!(abc.contains("c4 d4"));
    } else {
        panic!("Expected ABC string on stack");
    }
}

#[test]
fn test_bard_rest_and_tempo() {
    let genes = vec![
        // Tempo 120
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(120)],
        },
        Gene {
            op: OpCode::Tempo,
            args: vec![],
        },
        // Rest 8
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(8)],
        },
        Gene {
            op: OpCode::Rest,
            args: vec![],
        },
        // Perform
        Gene {
            op: OpCode::Perform,
            args: vec![],
        },
    ];

    let mut vm = ChimeraVM::new(make_dna(genes));
    while !vm.halted {
        vm.step();
    }

    if let Some(Value::Str(abc)) = vm.stack.pop() {
        println!("Generated ABC:\n{}", abc);
        assert!(abc.contains("z8"));
    } else {
        panic!("Expected ABC string");
    }

    // Verify Tempo log
    assert!(vm
        .output
        .iter()
        .any(|s: &String| s.contains("TEMPO: Set to 120 BPM")));
}
