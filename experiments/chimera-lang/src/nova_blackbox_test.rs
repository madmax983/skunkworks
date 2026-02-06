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
fn test_blackbox_recording_and_dump() {
    // [ push(42) blackbox() ]
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(42)],
        },
        Gene {
            op: OpCode::Blackbox,
            args: vec![],
        },
    ];
    let mut vm = ChimeraVM::new(make_dna(genes));

    // Step 1: Push 42
    vm.step();
    // Step 2: Blackbox
    vm.step();

    // Verify stack has dump. Stack should be: [42, dump]
    assert_eq!(vm.stack.len(), 2);

    let dump = vm.stack.pop().unwrap();
    let val_42 = vm.stack.pop().unwrap();

    assert_eq!(val_42, Value::Int(42));

    if let Value::Str(s) = dump {
        println!("{}", s);
        assert!(s.contains("BLACKBOX DUMP"));
        // We expect to see the history.
        // Frame 0: Push 42. Top should be empty (before push) or 42 (after)?
        // The record() is called at the *beginning* of step().
        // Step 1: IP=(0,0), Op=push. Stack empty.
        // Step 2: IP=(0,1), Op=blackbox. Stack has 42.

        assert!(s.contains("Op=push"));
        assert!(s.contains("Op=blackbox"));
        assert!(s.contains("Top=42"));
    } else {
        panic!("Expected string on stack");
    }
}
