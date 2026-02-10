#[cfg(test)]
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
    #[cfg(feature = "nova")]
    fn test_babel_parser_match() {
        // [ push("A") parser_match() push("A") parse() ]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("A".to_string())],
            },
            Gene {
                op: OpCode::ParserMatch,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("A".to_string())],
            },
            Gene {
                op: OpCode::Parse,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        while !vm.halted && vm.ip.0 < 1 {
            vm.step();
        }

        // Output should say Success
        assert!(vm.output.iter().any(|s| s.contains("PARSE: Success")));
        // Stack should have "A"
        assert_eq!(vm.stack.last(), Some(&Value::Str("A".to_string())));
    }

    #[test]
    #[cfg(feature = "nova")]
    fn test_babel_parser_alt() {
        // [ push("A") parser_match() push("B") parser_match() parser_alt() dup() push("B") parse() ]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("A".to_string())],
            },
            Gene {
                op: OpCode::ParserMatch,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("B".to_string())],
            },
            Gene {
                op: OpCode::ParserMatch,
                args: vec![],
            },
            Gene {
                op: OpCode::ParserAlt,
                args: vec![],
            },
            Gene {
                op: OpCode::Dup,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("B".to_string())],
            },
            Gene {
                op: OpCode::Parse,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        while !vm.halted && vm.ip.0 < 1 {
            vm.step();
        }

        assert!(vm.output.iter().any(|s| s.contains("PARSE: Success")));
        assert_eq!(vm.stack.last(), Some(&Value::Str("B".to_string())));
    }

    #[test]
    #[cfg(feature = "nova")]
    fn test_babel_parser_seq() {
        // [ push("A") parser_match() push("B") parser_match() parser_seq() push("AB") parse() ]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("A".to_string())],
            },
            Gene {
                op: OpCode::ParserMatch,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("B".to_string())],
            },
            Gene {
                op: OpCode::ParserMatch,
                args: vec![],
            },
            Gene {
                op: OpCode::ParserSeq,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("AB".to_string())],
            },
            Gene {
                op: OpCode::Parse,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        while !vm.halted && vm.ip.0 < 1 {
            vm.step();
        }

        assert!(vm.output.iter().any(|s| s.contains("PARSE: Success")));
        // Result is Junction(All, ["A", "B"])
        if let Some(Value::Junction(crate::ast::JunctionType::All, vals)) = vm.stack.last() {
            assert_eq!(vals.len(), 2);
            assert_eq!(vals[0], Value::Str("A".to_string()));
            assert_eq!(vals[1], Value::Str("B".to_string()));
        } else {
            panic!("Expected Sequence Junction, got {:?}", vm.stack.last());
        }
    }

    #[test]
    #[cfg(feature = "nova")]
    fn test_babel_parser_regex() {
        // [ push("[a-z]+") parser_regex() push("hello") parse() ]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("[a-z]+".to_string())],
            },
            Gene {
                op: OpCode::ParserRegex,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("hello".to_string())],
            },
            Gene {
                op: OpCode::Parse,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        while !vm.halted && vm.ip.0 < 1 {
            vm.step();
        }

        assert!(vm.output.iter().any(|s| s.contains("PARSE: Success")));
        assert_eq!(vm.stack.last(), Some(&Value::Str("hello".to_string())));
    }

    #[test]
    #[cfg(feature = "nova")]
    fn test_babel_grammar_mutate() {
        // [ push("A") parser_match() grammar_mutate() ]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("A".to_string())],
            },
            Gene {
                op: OpCode::ParserMatch,
                args: vec![],
            },
            Gene {
                op: OpCode::GrammarMutate,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        while !vm.halted && vm.ip.0 < 1 {
            vm.step();
        }

        assert!(vm.output.iter().any(|s| s.contains("GRAMMAR MUTATED")));
        if let Some(Value::Junction(_, args)) = vm.stack.last() {
            // It should still be a grammar (Junction)
            assert!(!args.is_empty());
        } else {
            panic!("Expected Grammar Junction, got {:?}", vm.stack.last());
        }
    }

    #[test]
    #[cfg(feature = "nova")]
    fn test_babel_grammar_breed() {
        // [ push("A") parser_match() push("B") parser_match() grammar_breed() ]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("A".to_string())],
            },
            Gene {
                op: OpCode::ParserMatch,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("B".to_string())],
            },
            Gene {
                op: OpCode::ParserMatch,
                args: vec![],
            },
            Gene {
                op: OpCode::GrammarBreed,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        while !vm.halted && vm.ip.0 < 1 {
            vm.step();
        }

        assert!(vm.output.iter().any(|s| s.contains("GRAMMAR BRED")));
        if let Some(Value::Junction(_, args)) = vm.stack.last() {
            assert!(!args.is_empty());
        } else {
            panic!("Expected Grammar Junction, got {:?}", vm.stack.last());
        }
    }
}
