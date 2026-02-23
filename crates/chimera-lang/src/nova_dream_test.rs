use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use crate::opcode::OpCode;
use crate::vm::{ChimeraVM, Value};

#[test]
fn test_dream_execution() {
    // [ push(0) push(10) dream() photosynthesize() ]
    // Dream runs strand 0 for 10 ticks.
    // Strand 0: [ ... dream() ... ]
    // Actually, dream() runs the target strand from index 0.
    // If we dream strand 0, we recurse?
    // Dream clones VM. Clone's IP is set to (0,0).
    // So Clone executes: push(0), push(10), dream()...
    // This is recursion!
    // But Dream doesn't carry over recursion depth?
    // Wait, clone copies recursion depth.
    // But we reset IP.
    // If dream calls dream, it might loop until stack overflow or timeout.
    // Let's make a separate strand for dreaming to avoid infinite dream recursion in the test.

    let strand_main = Strand {
        genes: vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            }, // Dream strand 1
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            }, // 10 ticks
            Gene {
                op: OpCode::Dream,
                args: vec![],
            },
        ],
    };

    let strand_dream = Strand {
        genes: vec![
            Gene {
                op: OpCode::Photosynthesize,
                args: vec![],
            },
            Gene {
                op: OpCode::Jump,
                args: vec![Nucleotide::Number(1)],
            }, // Loop
        ],
    };

    let dna = Dna { evolution_config: None,
        helix: Helix {
            strands: vec![strand_main, strand_dream],
        },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.energy = 100; // Give buffer

    vm.step(); // push 1
    vm.step(); // push 10
    vm.step(); // dream

    // Stack should have result (0 or 1)
    assert_eq!(vm.stack.len(), 1);
    let result = vm.stack.pop().unwrap();
    match result {
        Value::Int(0) | Value::Int(1) => (),
        _ => panic!("Expected 0 or 1, got {:?}", result),
    }

    // Energy should be consumed.
    // Base 50 + 10/2 = 55.
    // Initial 100 - 3 (steps) - 55 = ~42.
    assert!(vm.energy < 50);
}
