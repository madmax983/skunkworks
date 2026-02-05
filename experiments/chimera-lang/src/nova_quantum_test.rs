#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::ChimeraVM;

    fn make_dna(strands: Vec<Vec<Gene>>) -> Dna {
        Dna {
            helix: Helix {
                strands: strands
                    .into_iter()
                    .map(|g| std::rc::Rc::new(Strand { genes: g }))
                    .collect(),
            },
        }
    }

    #[test]
    fn test_entangle_transcribe() {
        // Strand 0: [ push(0) push(1) entangle() push(0) push(0) push(0) push(42) transcribe() ]
        // Strand 1: [ push(0) ]

        // We entangle 0 and 1.
        // Then we transcribe Strand 0: Gene 0 (Push), Arg 0 -> 42.
        // Entanglement should propagate this to Strand 1: Gene 0 (Push), Arg 0 -> 42.

        let s0 = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            }, // s1
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            }, // s2
            Gene {
                op: OpCode::Entangle,
                args: vec![],
            },
            // Transcribe: strand 0, gene 0, arg 0 -> 42
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            }, // s
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            }, // g
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            }, // a
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(42)],
            }, // v
            Gene {
                op: OpCode::Transcribe,
                args: vec![],
            },
        ];

        let s1 = vec![Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        }];

        let mut vm = ChimeraVM::new(make_dna(vec![s0, s1]));

        // Step 1: Push 0
        vm.step();
        // Step 2: Push 1
        vm.step();
        // Step 3: Entangle
        vm.step();

        assert!(vm.entangled_pairs.contains_key(&0));
        assert_eq!(vm.entangled_pairs.get(&0), Some(&1));
        assert_eq!(vm.entangled_pairs.get(&1), Some(&0));

        // Step 4-8: Setup transcribe
        for _ in 0..5 {
            vm.step();
        }

        // Check Strand 0 modified
        if let Nucleotide::Number(n) = vm.dna.helix.strands[0].genes[0].args[0] {
            assert_eq!(n, 42);
        } else {
            panic!("Strand 0 not modified");
        }

        // Check Strand 1 modified (Action at a distance!)
        if let Nucleotide::Number(n) = vm.dna.helix.strands[1].genes[0].args[0] {
            assert_eq!(n, 42);
        } else {
            panic!("Strand 1 not modified by entanglement");
        }
    }

    #[test]
    fn test_decohere() {
        let s0 = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Entangle,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                op: OpCode::Decohere,
                args: vec![],
            },
        ];
        let s1 = vec![];

        let mut vm = ChimeraVM::new(make_dna(vec![s0, s1]));

        vm.step();
        vm.step();
        vm.step(); // Entangle
        assert!(vm.entangled_pairs.contains_key(&0));

        vm.step();
        vm.step(); // Decohere
        assert!(!vm.entangled_pairs.contains_key(&0));
        assert!(!vm.entangled_pairs.contains_key(&1));
    }
}
