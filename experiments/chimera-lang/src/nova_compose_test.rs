#![cfg(feature = "nova")]
use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use crate::opcode::OpCode;
use crate::vm::{ChimeraVM, Value};

fn make_dna(genes: Vec<Gene>) -> Dna {
    Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    }
}

#[test]
fn test_bard_compose_genetic_song() {
    // We will compose a song that, when executed as DNA, calculates 2 + 2 = 4.
    // Mapping:
    // C (0) -> Push(duration)
    // D (2) -> Add

    // Song:
    // 1. C (60), Duration 2 -> Push(2)
    // 2. C (60), Duration 2 -> Push(2)
    // 3. D (62), Duration 4 -> Add (duration ignored for Add)

    let genes = vec![
        // Note 1: C (60), Dur 2
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(100)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(2)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(60)],
        },
        Gene {
            op: OpCode::Note,
            args: vec![],
        },
        // Note 2: C (60), Dur 2
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(100)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(2)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(60)],
        },
        Gene {
            op: OpCode::Note,
            args: vec![],
        },
        // Note 3: D (62), Dur 4
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
        // Compose
        Gene {
            op: OpCode::Compose,
            args: vec![],
        },
    ];

    let mut vm = ChimeraVM::new(make_dna(genes));

    // Execute until halted.
    // Compose will create Strand 1.
    // VM will automatically proceed to execute Strand 1 after Strand 0 finishes.
    while !vm.halted {
        vm.step();
    }

    // Expected Stack:
    // Bottom: Strand Index (from Compose)
    // Top: Result of Strand 1 (4)

    assert_eq!(vm.stack.len(), 2, "Stack should contain [index, result]");

    let result = vm.stack.pop().unwrap();
    let index = vm.stack.pop().unwrap();

    // Check Result (2 + 2 = 4)
    if let Value::Int(res) = result {
        assert_eq!(res, 4, "Result of genetic song should be 4");
    } else {
        panic!("Expected integer result");
    }

    // Check Index
    if let Value::Int(idx) = index {
        assert_eq!(idx, 1, "New strand index should be 1");
    } else {
        panic!("Expected strand index");
    }

    // Verify DNA structure
    assert_eq!(vm.dna.helix.strands.len(), 2);
    let new_strand = &vm.dna.helix.strands[1];
    assert_eq!(new_strand.genes.len(), 3);
    assert_eq!(new_strand.genes[0].op, OpCode::Push);
    assert_eq!(new_strand.genes[2].op, OpCode::Add);
}
