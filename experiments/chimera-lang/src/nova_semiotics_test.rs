#[cfg(test)]
#[cfg(feature = "nova")]
mod tests {
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
    fn test_symbolize_interpret() {
        let genes = vec![
            // Push "Meaning"
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("Meaning".to_string())],
            },
            // Signify it -> Symbol
            Gene {
                op: OpCode::Symbolize,
                args: vec![],
            },
            // Interpret the Symbol back to "Meaning"
            Gene {
                op: OpCode::Interpret,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        vm.metamorphism_enabled = false;
        while !vm.halted {
            vm.step();
        }

        assert_eq!(vm.stack.len(), 1);
        if let Value::Str(s) = &vm.stack[0] {
            assert_eq!(s, "Meaning");
        } else {
            panic!("Expected String 'Meaning', got {:?}", vm.stack[0]);
        }
    }

    #[test]
    #[ignore]
    fn test_context_shift() {
        let genes = vec![
            // 1. Create Symbol for "A" in Context 0
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("A".to_string())],
            },
            Gene {
                op: OpCode::Symbolize,
                args: vec![],
            }, // Stack: [Symbol(A)]
            // 2. Duplicate Symbol
            Gene {
                op: OpCode::Dup,
                args: vec![],
            }, // Stack: [Symbol(A), Symbol(A)]
            // 3. Shift Context using "Shift"
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("Shift".to_string())],
            },
            Gene {
                op: OpCode::ContextShift,
                args: vec![],
            }, // Context changed
            // 4. Interpret Symbol in NEW context
            // Should fail (return 0) or return Symbol if logic changes.
            // My implementation pushes 0 on failure.
            Gene {
                op: OpCode::Interpret,
                args: vec![],
            }, // Stack: [Symbol(A), 0]
        ];

        let mut vm = ChimeraVM::new(make_dna(genes));
        vm.metamorphism_enabled = false;
        while !vm.halted {
            vm.step();
        }

        assert_eq!(vm.stack.len(), 2);
        // Top should be 0 (meaningless in new context)
        assert_eq!(vm.stack.pop().unwrap(), Value::Int(0));
        // Bottom is original Symbol
        assert!(matches!(vm.stack.pop().unwrap(), Value::Symbol(_)));
    }

    #[test]
    #[ignore]
    fn test_deconstruct() {
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("ABC".to_string())],
            },
            Gene {
                op: OpCode::Deconstruct,
                args: vec![],
            },
        ];

        let mut vm = ChimeraVM::new(make_dna(genes));
        vm.metamorphism_enabled = false;
        while !vm.halted {
            vm.step();
        }

        assert_eq!(vm.stack.len(), 1);
        if let Value::Junction(_t, vals) = &vm.stack[0] {
            assert_eq!(vals.len(), 3);
            for v in vals {
                assert!(matches!(v, Value::Symbol(_)));
            }
        } else {
            panic!("Expected Junction of Symbols");
        }
    }
}
