#[cfg(all(test, feature = "nova"))]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};
    use std::fs;
    use std::path::Path;

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_ipc_signal_receive() {
        // Clean up before test
        let ether_dir = Path::new(".chimera_ether");
        if ether_dir.exists() {
            let _ = fs::remove_dir_all(ether_dir);
        }

        // [ push("Hello IPC") push(42) signal() ]
        // stack order: channel (top), value (bottom) -> No, Wait.
        // OpCode::Signal implementation:
        // let value = vm.stack.pop().unwrap();
        // let channel_val = vm.stack.pop().unwrap();
        // So stack should be [ channel, value ] -> pop value, pop channel.
        // Wait, standard stack is LIFO.
        // If I push A then B. Stack is [A, B].
        // pop() gives B. pop() gives A.
        // `signal` code:
        // let value = vm.stack.pop().unwrap(); // Gets B
        // let channel_val = vm.stack.pop().unwrap(); // Gets A
        // So B is value, A is channel.
        // Stack: [ channel, value ]
        // Push Channel, Push Value.

        // Code:
        // push(42) (Channel)
        // push("Hello IPC") (Value)
        // signal()

        let signal_genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(42)], // Channel 42
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("Hello IPC".to_string())],
            },
            Gene {
                op: OpCode::Signal,
                args: vec![],
            },
        ];

        let mut vm_sender = ChimeraVM::new(make_dna(signal_genes));

        // Execute 3 steps
        for _ in 0..3 {
            vm_sender.step();
        }

        // Verify file created
        let channel_dir = ether_dir.join("42");
        assert!(channel_dir.exists(), "Channel dir 42 should exist");
        let count = fs::read_dir(&channel_dir).unwrap().count();
        assert_eq!(count, 1, "Should have 1 message file");

        // [ push(42) receive() ]
        let receive_genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(42)],
            },
            Gene {
                op: OpCode::Receive,
                args: vec![],
            },
        ];

        let mut vm_receiver = ChimeraVM::new(make_dna(receive_genes));

        // Execute 2 steps
        for _ in 0..2 {
            vm_receiver.step();
        }

        assert_eq!(vm_receiver.stack.len(), 1);
        if let Some(val) = vm_receiver.stack.pop() {
            assert_eq!(val, Value::Str("Hello IPC".to_string()));
        } else {
            panic!("Stack empty");
        }

        // Verify file consumed
        let count_after = fs::read_dir(&channel_dir).unwrap().count();
        assert_eq!(count_after, 0, "Message file should be consumed");

        // Cleanup
        let _ = fs::remove_dir_all(ether_dir);
    }
}
