#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
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
    fn test_bf_basic_io() {
        // [ push(code) push(input) brainfuck() ]
        // Code: ",.,."
        // Input: "AB"
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String(",.,.".to_string())],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("AB".to_string())],
            },
            Gene {
                op: OpCode::Brainfuck,
                args: vec![],
            },
        ];
        let mut vm = make_vm(genes);

        vm.step(); // push code
        vm.step(); // push input
        vm.step(); // brainfuck

        if let Some(val) = vm.stack.pop() {
            if let Value::Str(s) = val {
                assert_eq!(s, "AB");
            } else {
                panic!("Expected string output, got {:?}", val);
            }
        } else {
            panic!("Stack empty after brainfuck");
        }
    }

    #[test]
    fn test_bf_hello_world() {
        // Hello World
        let code = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String(code.to_string())],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("".to_string())],
            }, // Empty input
            Gene {
                op: OpCode::Brainfuck,
                args: vec![],
            },
        ];
        let mut vm = make_vm(genes);
        vm.energy = 10000; // Needs energy for many cycles

        vm.step();
        vm.step();
        vm.step();

        if let Some(val) = vm.stack.pop() {
            if let Value::Str(s) = val {
                assert_eq!(s, "Hello World!\n");
            } else {
                panic!("Expected string output, got {:?}", val);
            }
        }
    }

    #[test]
    fn test_bf_unmatched_brackets_safe() {
        // Code: "[[[" -> Should not panic, just terminate or run partially
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("[[[".to_string())],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("".to_string())],
            },
            Gene {
                op: OpCode::Brainfuck,
                args: vec![],
            },
        ];
        let mut vm = make_vm(genes);

        vm.step();
        vm.step();
        vm.step(); // Should return gracefully

        assert!(
            vm.output.last().unwrap().contains("BRAINFUCK: Ran"),
            "Should complete execution"
        );
    }

    #[test]
    fn test_bf_input_exhaustion() {
        // Code: ",," Input: "A"
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String(",.,.".to_string())],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("A".to_string())],
            },
            Gene {
                op: OpCode::Brainfuck,
                args: vec![],
            },
        ];
        let mut vm = make_vm(genes);

        vm.step();
        vm.step();
        vm.step();

        if let Some(val) = vm.stack.pop() {
            if let Value::Str(s) = val {
                // String from bytes. 0 byte usually terminates C string, but Rust String can contain \0.
                // However, String::from_utf8_lossy might keep it.
                // 'A' is 65. 0 is 0.
                let bytes = s.as_bytes();
                assert_eq!(bytes[0], 65);
                assert_eq!(bytes[1], 0);
            } else {
                panic!("Expected string output");
            }
        }
    }

    #[test]
    fn test_bf_infinite_loop_safety() {
        // Code: "+[]" infinite loop
        // The VM imposes a `max_cycles` limit (10000).
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("+[]".to_string())],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("".to_string())],
            },
            Gene {
                op: OpCode::Brainfuck,
                args: vec![],
            },
        ];
        let mut vm = make_vm(genes);
        vm.energy = 1000;

        vm.step();
        vm.step();
        vm.step();

        // Should finish because of cycle limit
        let last_msg = vm.output.last().unwrap();
        assert!(last_msg.contains("BRAINFUCK: Ran 10000 cycles"));
    }
}
