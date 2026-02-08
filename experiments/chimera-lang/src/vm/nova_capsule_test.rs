#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};
    use std::thread;
    use std::time::Duration;

    #[test]
    #[cfg(feature = "nova")]
    fn test_capsule_immediate() {
        // [ push("key1") push(100) push(0) encapsulate() push("key1") decapsulate() ]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("key1".to_string())],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(100)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)], // 0 seconds
            },
            Gene {
                op: OpCode::Encapsulate,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("key1".to_string())],
            },
            Gene {
                op: OpCode::Decapsulate,
                args: vec![],
            },
        ];

        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.energy = 1000;

        while !vm.halted {
            vm.step();
        }

        // Expected stack: [ 100 ]
        assert_eq!(vm.stack.len(), 1);
        if let Value::Int(v) = vm.stack[0] {
            assert_eq!(v, 100);
        } else {
            panic!("Expected Int(100), got {:?}", vm.stack[0]);
        }
    }

    #[test]
    #[cfg(feature = "nova")]
    fn test_capsule_locked() {
        // [ push("key2") push(200) push(10) encapsulate() push("key2") decapsulate() ]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("key2".to_string())],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(200)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)], // 10 seconds
            },
            Gene {
                op: OpCode::Encapsulate,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("key2".to_string())],
            },
            Gene {
                op: OpCode::Decapsulate,
                args: vec![],
            },
        ];

        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.energy = 1000;

        while !vm.halted {
            vm.step();
        }

        // Expected stack: [ WaitTime(approx 10) ]
        assert_eq!(vm.stack.len(), 1);
        if let Value::Int(v) = vm.stack[0] {
            // It should be around 9 or 10
            assert!(v >= 9 && v <= 10, "Expected wait time around 10, got {}", v);
        } else {
            panic!("Expected Int(wait_time), got {:?}", vm.stack[0]);
        }
    }

    #[test]
    #[cfg(feature = "nova")]
    fn test_capsule_wait() {
        // [ push("key3") push(300) push(1) encapsulate() ]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("key3".to_string())],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(300)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)], // 1 second
            },
            Gene {
                op: OpCode::Encapsulate,
                args: vec![],
            },
        ];

        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.energy = 1000;

        // Run encapsulate
        while !vm.halted {
            vm.step();
        }

        // Wait 2 seconds to be safe
        thread::sleep(Duration::from_secs(2));

        // Create new VM to test persistence (or reuse same one)
        // Let's reuse same one but reset IP/Stack
        vm.ip = (0, 0);
        vm.stack.clear();
        vm.halted = false;

        // New Program: [ push("key3") decapsulate() ]
        vm.dna.helix.strands[0].genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("key3".to_string())],
            },
            Gene {
                op: OpCode::Decapsulate,
                args: vec![],
            },
        ];

        while !vm.halted {
            vm.step();
        }

        // Expected stack: [ 300 ]
        assert_eq!(vm.stack.len(), 1);
        if let Value::Int(v) = vm.stack[0] {
            assert_eq!(v, 300);
        } else {
            panic!("Expected Int(300), got {:?}", vm.stack[0]);
        }
    }
}
