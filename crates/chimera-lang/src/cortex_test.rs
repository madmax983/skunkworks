#[cfg(test)]
#[cfg(feature = "cortex")]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    fn make_dna(strands: Vec<Vec<Gene>>) -> Dna {
        let strands = strands.into_iter().map(|genes| Strand { genes }).collect();
        Dna { evolution_config: None,
            helix: Helix { strands },
        }
    }

    #[test]
    fn test_neural_circuit() {
        // Strand 0: [ push(1) link() push(10) spark() ]
        // Strand 1: [ sense() ]

        let s0 = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Link,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                op: OpCode::Spark,
                args: vec![],
            },
        ];

        let s1 = vec![Gene {
            op: OpCode::Sense,
            args: vec![],
        }];

        let mut vm = ChimeraVM::new(make_dna(vec![s0, s1]));

        // 1. push(1)
        vm.step();
        // 2. link() - Link S0 to S1
        vm.step();
        assert!(vm.synapse_map[0].contains(&1));

        // 3. push(10)
        vm.step();
        // 4. spark() - Adds 10 to S1 activation
        vm.step();
        assert_eq!(vm.activation_levels[1], 10);

        // 5. End of S0. Step moves IP to S1. Decay: 10 -> 9.
        vm.step();
        assert_eq!(vm.ip, (1, 0));
        assert_eq!(vm.activation_levels[1], 9);

        // 6. sense() - Pushes activation (which decayed again 9 -> 8)
        vm.step();

        // Check stack
        let val = vm.stack.pop().expect("Stack underflow");
        assert_eq!(val, Value::Int(8));
    }

    #[test]
    fn test_gating() {
        // Strand 0: [ gate(5) push(100) ]
        // We will run this twice. Once with activation 0 (gate should skip push),
        // once with activation 10 (gate should allow push).

        let s0 = vec![
            Gene {
                op: OpCode::Gate,
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(100)],
            },
        ];

        // Scenario A: Low activation
        let mut vm = ChimeraVM::new(make_dna(vec![s0.clone()]));
        vm.activation_levels[0] = 0;

        vm.step(); // gate(5). 0 < 5, so skip next.
                   // IP should be at (0, 2) (end of strand)
        assert_eq!(vm.ip, (0, 2));
        // Run again to trigger end of strand logic
        vm.step(); // Moves to next strand (1, 0)
        vm.step(); // Detects end of helix, halts
        assert!(vm.halted);
        assert!(vm.stack.is_empty());

        // Scenario B: High activation
        let mut vm = ChimeraVM::new(make_dna(vec![s0]));
        vm.activation_levels[0] = 10;

        vm.step(); // gate(5). 10 >= 5, proceed.
                   // IP should be at (0, 1) -> push(100)
        assert_eq!(vm.ip, (0, 1));

        vm.step(); // push(100)
        assert_eq!(vm.stack.pop(), Some(Value::Int(100)));
    }
}
