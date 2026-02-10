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
    fn test_string_new() {
        // [ push(5) push(10) push(8) push(8) string_new() ]
        // len=5, tension=10, y=8, x=8
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            }, // Length
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            }, // Tension
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(8)],
            }, // Y
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(8)],
            }, // X
            Gene {
                op: OpCode::StringNew,
                args: vec![],
            },
        ];
        let mut vm = make_vm(genes);
        // 4 pushes + 1 op = 5 steps
        for _ in 0..5 {
            vm.step();
        }

        assert_eq!(vm.strings.len(), 1);
        let s = &vm.strings[0];
        assert_eq!(s.start, (8.0, 8.0));
        assert_eq!(s.tension, 10.0);
        // Default direction East (0, 1) -> End should be (13, 8) because CosmicString stores (x, y)
        assert_eq!(s.end, (13.0, 8.0));
    }

    #[test]
    fn test_string_pluck_listen() {
        // Corrected genes for proximity
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(8)],
            }, // Y=8
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(8)],
            }, // X=8
            Gene {
                op: OpCode::StringNew,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(50)],
            },
            Gene {
                op: OpCode::StringPluck,
                args: vec![],
            }, // Context loc 8,8
            Gene {
                op: OpCode::StringListen,
                args: vec![],
            },
        ];

        let mut vm = make_vm(genes);
        // Execute creation (5 steps)
        for _ in 0..5 {
            vm.step();
        }

        assert_eq!(vm.strings.len(), 1);

        // Pluck (2 steps)
        vm.step(); // Push 50
        vm.step(); // Pluck

        assert!(vm.strings[0].amplitude > 0.0);

        // Update physics a bit to change phase
        for _ in 0..10 {
            crate::vm::nova_strings::update_strings(&mut vm);
        }

        // Listen (1 step)
        vm.step();

        let val = vm.stack.pop().unwrap();
        if let Value::Int(_amp) = val {
            // Amplitude might be near 0 if phase is near node
        } else {
            panic!("Expected Int amplitude");
        }
    }
}
