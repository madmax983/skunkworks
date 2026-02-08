#![cfg(all(test, feature = "nova"))]

use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use crate::opcode::OpCode;
use crate::vm::{ChimeraVM, Value};

fn make_vm(genes: Vec<Gene>) -> ChimeraVM {
    let dna = Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    };
    ChimeraVM::new(dna)
}

#[test]
fn test_encryption_round_trip() {
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(12345)],
        },
        Gene {
            op: OpCode::Encrypt,
            args: vec![],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(12345)],
        },
        Gene {
            op: OpCode::Swap,
            args: vec![],
        },
        Gene {
            op: OpCode::Decrypt,
            args: vec![],
        },
        // Add infinite loop to stop execution from falling through to new strand immediately?
        // Or just let it run and check output.
    ];

    let mut vm = make_vm(genes);

    // Run until we see DECRYPT success in output or timeout
    for _ in 0..100 {
        if vm.output.iter().any(|s| s.contains("DECRYPT: Success")) {
            break;
        }
        if vm.halted { break; }
        vm.step();
    }

    // Check if we have the result
    let found = vm.stack.iter().any(|v| match v {
        Value::Int(idx) => *idx >= 1,
        _ => false,
    });

    assert!(found, "Did not find new strand index in stack");
}

#[test]
fn test_verify_success() {
    let genes = vec![
        // Sign(strand=0, key=42) -> [sig]
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(42)] },
        Gene { op: OpCode::Sign, args: vec![] },

        // Verify(strand=0, sig=sig, key=42)
        // Stack: [sig]
        // Push 0 -> [sig, 0]
        // Swap -> [0, sig]
        // Push 42 -> [0, sig, 42]
        // Verify pops: Key(42), Sig(sig), Strand(0).
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
        Gene { op: OpCode::Swap, args: vec![] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(42)] },
        Gene { op: OpCode::VerifySig, args: vec![] },
    ];

    let mut vm = make_vm(genes);
    for _ in 0..100 {
        if vm.halted { break; }
        vm.step();
    }

    assert_eq!(vm.stack.len(), 1);
    assert_eq!(vm.stack[0], Value::Int(1));
}

#[test]
fn test_verify_fail() {
    let genes = vec![
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(42)] },
        Gene { op: OpCode::Sign, args: vec![] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
        Gene { op: OpCode::Swap, args: vec![] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(999)] }, // Wrong key
        Gene { op: OpCode::VerifySig, args: vec![] },
    ];

    let mut vm = make_vm(genes);
    for _ in 0..100 {
        if vm.halted { break; }
        vm.step();
    }

    assert_eq!(vm.stack.len(), 1);
    assert_eq!(vm.stack[0], Value::Int(0));
}
