#![cfg(all(test, feature = "biophysics"))]

use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use crate::opcode::OpCode;
use crate::vm::ChimeraVM;
use crate::value::Value;

fn make_dna(genes: Vec<Gene>) -> Dna {
    Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    }
}

#[test]
fn test_neurogenesis() {
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(5)], // x
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(5)], // y
        },
        Gene {
            op: OpCode::NeuroGenesis,
            args: vec![],
        },
    ];
    let mut vm = ChimeraVM::new(make_dna(genes));
    // Step enough to execute instructions
    for _ in 0..5 {
        vm.step();
    }

    assert!(vm.neurons.contains_key(&(5, 5)));
    assert!(vm.output.iter().any(|s| s.contains("NEUROGENESIS")));
}

#[test]
fn test_stimulation_and_response() {
    let genes = vec![
        // Create neuron at 5,5
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(5)], // x
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(5)], // y
        },
        Gene {
            op: OpCode::NeuroGenesis,
            args: vec![],
        },
        // Stimulate it: 100 current
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(100)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(5)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(5)],
        },
        Gene {
            op: OpCode::Stimulate,
            args: vec![],
        },
        // Wait a few ticks for voltage to rise (simulation runs in vm.step())
        // Dummy pushes to waste time/cycles
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::Drop,
            args: vec![],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::Drop,
            args: vec![],
        },
        // Read Dendrite
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(5)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(5)],
        },
        Gene {
            op: OpCode::Dendrite,
            args: vec![],
        },
    ];

    let mut vm = ChimeraVM::new(make_dna(genes));

    // Run enough steps
    for _ in 0..20 {
        vm.step();
    }

    let val = vm.stack.last().expect("Stack should have voltage");
    if let Value::Int(v) = val {
        println!("Neuron Voltage: {}", v);
        // Resting is -65. With stimulation, it should rise significantly.
        // Due to cast to i64, -65.0 is -65.
        assert!(*v > -65, "Voltage {} should be higher than resting -65", v);
    } else {
        panic!("Expected Int voltage, got {:?}", val);
    }
}
