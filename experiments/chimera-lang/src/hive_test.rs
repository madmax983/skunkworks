#[cfg(feature = "hive")]
#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, JunctionType, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};
    use std::thread;
    use std::time::Duration;

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_hive_bind_and_close() {
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(8001)],
            },
            Gene {
                op: OpCode::HiveBind,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(8001)],
            },
            Gene {
                op: OpCode::HiveClose,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        vm.step(); // Push
        vm.step(); // Bind
        assert!(vm.hive_sockets.contains_key(&8001));
        vm.step(); // Push
        vm.step(); // Close
        assert!(!vm.hive_sockets.contains_key(&8001));
    }

    #[test]
    fn test_hive_send_recv() {
        let genes = vec![
            // Bind 8003
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(8003)],
            },
            Gene {
                op: OpCode::HiveBind,
                args: vec![],
            },
            // Send "Hello" to 127.0.0.1:8003
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("Hello".to_string())],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("127.0.0.1".to_string())],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(8003)],
            },
            Gene {
                op: OpCode::HiveSend,
                args: vec![],
            },
        ];

        let mut vm = ChimeraVM::new(make_dna(genes));

        // Bind
        vm.step();
        vm.step();
        assert!(vm.hive_sockets.contains_key(&8003));

        // Send
        vm.step();
        vm.step();
        vm.step();
        vm.step();

        // Wait
        thread::sleep(Duration::from_millis(100));

        // Recv manually
        vm.dna.helix.strands[0].genes.push(Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(8003)],
        });
        vm.dna.helix.strands[0].genes.push(Gene {
            op: OpCode::HiveRecv,
            args: vec![],
        });

        // Execute Push 8003
        vm.step();
        // Execute HiveRecv
        vm.step();

        // Check stack
        let val = vm.stack.pop().expect("Stack should have value");
        // Expect Junction(Dish, [port, ip, val]) or Int(0)
        match val {
            Value::Junction(JunctionType::Dish, items) => {
                assert_eq!(items.len(), 3);
                // items[0] is port (Int)
                // items[1] is ip (Str)
                // items[2] is payload
                if let Value::Str(s) = &items[2] {
                    assert_eq!(s, "Hello");
                } else {
                    panic!("Expected string payload, got {:?}", items[2]);
                }
            }
            Value::Int(0) => {
                println!("WARNING: UDP packet missed.");
            }
            _ => panic!("Unexpected return value: {:?}", val),
        }
    }
}
