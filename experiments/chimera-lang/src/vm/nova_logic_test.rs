#[cfg(all(test, feature = "nova", feature = "oracle"))]
mod tests {
    use crate::ast::{Dna, Gene, Helix, JunctionType, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    fn make_vm() -> ChimeraVM {
        let dna = Dna {
            helix: Helix { strands: vec![] },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_prolog_call_metabolism() {
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Junction(
                    JunctionType::Any,
                    vec![
                        Nucleotide::String("metabolism".to_string()),
                        Nucleotide::String("?E".to_string()),
                    ],
                )],
            },
            Gene {
                op: OpCode::PrologCall,
                args: vec![],
            },
        ];

        let mut vm = make_vm();
        vm.dna.helix.strands.push(Strand { genes });

        vm.step(); // Push query
        vm.step(); // PrologCall

        let solutions = vm.stack.pop().unwrap();
        if let Value::Junction(JunctionType::All, sols) = solutions {
            assert_eq!(sols.len(), 1);
            let bindings = &sols[0];
            if let Value::Junction(JunctionType::All, pairs) = bindings {
                assert_eq!(pairs.len(), 1);
                if let Value::Junction(JunctionType::All, pair) = &pairs[0] {
                    assert_eq!(pair[0], Value::Str("?E".to_string()));
                    if let Value::Int(e) = pair[1] {
                        assert!(e >= 40 && e <= 50, "Energy {} out of expected range", e);
                    } else {
                        panic!("Expected Int energy");
                    }
                } else {
                    panic!("Invalid binding pair");
                }
            } else {
                panic!("Invalid bindings list");
            }
        } else {
            panic!("Expected solutions list");
        }
    }

    #[test]
    fn test_manifest_spawn_strand() {
        let genes = vec![
            // Push query: energy(?E)
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Junction(
                    JunctionType::Any,
                    vec![
                        Nucleotide::String("energy".to_string()),
                        Nucleotide::String("?E".to_string()),
                    ],
                )],
            },
            // Push transform: spawn_strand([ ["push", 99] ])
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Junction(
                    JunctionType::Any,
                    vec![
                        Nucleotide::String("spawn_strand".to_string()),
                        Nucleotide::Junction(
                            JunctionType::Any, // Genes List
                            vec![
                                Nucleotide::Junction(
                                    JunctionType::Any, // Gene 1
                                    vec![
                                        Nucleotide::String("push".to_string()), // Using explicit string
                                        Nucleotide::Number(99),
                                    ]
                                )
                            ]
                        )
                    ],
                )],
            },
            Gene {
                op: OpCode::Manifest,
                args: vec![],
            },
        ];

        let mut vm = make_vm();
        vm.dna.helix.strands.push(Strand { genes });

        vm.step(); // Push query
        vm.step(); // Push transform
        vm.step(); // Manifest

        if vm.dna.helix.strands.len() != 2 {
            println!("VM Output: {:?}", vm.output);
            panic!("Strand not spawned. Strand count: {}", vm.dna.helix.strands.len());
        }

        let new_strand = &vm.dna.helix.strands[1];
        assert_eq!(new_strand.genes.len(), 1);
        assert_eq!(new_strand.genes[0].op, OpCode::Push);
        match &new_strand.genes[0].args[0] {
            Nucleotide::Number(n) => assert_eq!(*n, 99),
            _ => panic!("Expected number 99"),
        }
    }
}
