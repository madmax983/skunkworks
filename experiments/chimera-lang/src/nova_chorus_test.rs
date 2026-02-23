#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_sing_listen() {
        // [ push("Do") sing() listen() ]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("Do".to_string())],
            },
            Gene {
                op: OpCode::Sing,
                args: vec![],
            },
            Gene {
                op: OpCode::Listen,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        vm.step(); // push
        vm.step(); // sing
        vm.step(); // listen

        assert_eq!(vm.chorus_buffer.len(), 1);
        assert_eq!(vm.chorus_buffer[0], "Do");

        let last = vm.stack.pop().unwrap();
        if let Value::Junction(_, vals) = last {
            assert_eq!(vals.len(), 1);
            assert_eq!(vals[0], Value::Str("Do".to_string()));
        } else {
            panic!("Expected Junction from Listen");
        }
    }

    #[test]
    fn test_vitality_chord() {
        // "Vitality": Mi Re Do -> Energy + 50
        // [ push("Mi") sing() push("Re") sing() push("Do") sing() ]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("Mi".to_string())],
            },
            Gene {
                op: OpCode::Sing,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("Re".to_string())],
            },
            Gene {
                op: OpCode::Sing,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("Do".to_string())],
            },
            Gene {
                op: OpCode::Sing,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        // Initial energy 50.
        // Costs: push(0), sing(2) * 3 = 6.
        // Expected before heal: 44.
        // Heal: +50 -> 94.

        while !vm.halted {
            vm.step();
        }

        assert!(
            vm.energy > 80,
            "Energy should be restored significantly (Current: {})",
            vm.energy
        );
        assert!(
            vm.chorus_buffer.is_empty(),
            "Buffer should be cleared after spell"
        );
        assert!(vm.output.iter().any(|s| s.contains("Vitality Chord")));
    }
}
