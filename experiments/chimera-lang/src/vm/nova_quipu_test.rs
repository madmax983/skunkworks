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
fn test_quipu_knot_and_read() {
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(123)],
        },
        Gene {
            op: OpCode::Knot,
            args: vec![],
        },
        Gene {
            op: OpCode::ReadCord,
            args: vec![],
        },
    ];
    let mut vm = make_vm(genes);
    vm.step(); // Push 123
    vm.step(); // Knot 123
    vm.step(); // ReadCord -> 123

    assert_eq!(vm.stack.pop(), Some(Value::Int(123)));
}

#[test]
fn test_quipu_multiple_knots() {
    // 1. Knot(10)
    // 2. Knot(5)
    // Cord should be 10 then 5? Or sum?
    // My implementation of `tie` replaces the cord.
    // Wait, let's re-read `tie` implementation.
    // `self.clusters = new_clusters`. It REPLACES.

    // To accumulate, we should Read, Add, then Knot.
    // Or change `tie` to append.
    // The prompt/thought process said "Tie sets the value".
    // So Knot(10) sets cord to 10. Knot(5) sets cord to 5.

    let genes = vec![
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(10)] },
        Gene { op: OpCode::Knot, args: vec![] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(5)] },
        Gene { op: OpCode::Knot, args: vec![] },
        Gene { op: OpCode::ReadCord, args: vec![] },
    ];
    let mut vm = make_vm(genes);
    for _ in 0..5 { vm.step(); }

    assert_eq!(vm.stack.pop(), Some(Value::Int(5)));
}

#[test]
fn test_quipu_unknot() {
    let genes = vec![
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(42)] },
        Gene { op: OpCode::Knot, args: vec![] },
        Gene { op: OpCode::Unknot, args: vec![] },
    ];
    let mut vm = make_vm(genes);
    vm.step(); // Push
    vm.step(); // Knot
    vm.step(); // Unknot -> Should push 42 back to stack and clear cord

    assert_eq!(vm.stack.pop(), Some(Value::Int(42)));

    // Verify cord is empty (0)
    assert_eq!(vm.quipu.read(), 0);
}

#[test]
fn test_quipu_tangle() {
    // Cord 0: 100
    // Cord 1: 50
    // Select 0, Tangle(1) -> Cord 0 becomes 150

    let genes = vec![
        // Set Cord 0 to 100
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
        Gene { op: OpCode::Cord, args: vec![] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(100)] },
        Gene { op: OpCode::Knot, args: vec![] },

        // Set Cord 1 to 50
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
        Gene { op: OpCode::Cord, args: vec![] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(50)] },
        Gene { op: OpCode::Knot, args: vec![] },

        // Select Cord 0
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
        Gene { op: OpCode::Cord, args: vec![] },

        // Tangle with Cord 1
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
        Gene { op: OpCode::Tangle, args: vec![] },

        // Read Cord 0
        Gene { op: OpCode::ReadCord, args: vec![] },
    ];

    let mut vm = make_vm(genes);
    for _ in 0..13 { vm.step(); }

    assert_eq!(vm.stack.pop(), Some(Value::Int(150)));
}
