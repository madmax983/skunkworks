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
    fn test_metabolism_hibernate() {
        // [ metabolism(0) push(1) ]
        // Should execute metabolism(0), then stop. push(1) should NOT happen in subsequent steps.
        let genes = vec![
            Gene {
                op: OpCode::Metabolism,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        // Push 0 for metabolism arg
        vm.stack.push(Value::Int(0));

        // Step 1: Execute metabolism(0).
        // Initial rate is 1. Cost 1.
        vm.step();

        assert_eq!(vm.metabolic_rate, 0);
        let energy_after_sleep = vm.energy;

        // Step 2: Should hibernate. Cost 0. IP should not change.
        let ip_before = vm.ip;
        vm.step();

        assert_eq!(vm.energy, energy_after_sleep);
        assert_eq!(vm.ip, ip_before);
        assert!(vm.stack.is_empty()); // push(1) did not happen
    }

    #[test]
    fn test_metabolism_overclock() {
        // [ metabolism(2) push(1) push(1) ]
        // Step 1: Set rate 2.
        // Step 2: Should execute push(1) AND push(1) in one step.
        let genes = vec![
            Gene {
                op: OpCode::Metabolism,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(2)],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        // Push 2 for metabolism arg
        vm.stack.push(Value::Int(2));

        // Step 1: Execute metabolism(2).
        vm.step();
        assert_eq!(vm.metabolic_rate, 2);

        let energy_before = vm.energy;

        // Step 2: Overclocked step.
        // Should execute push(1) and push(2).
        // Cost should be 2*2 = 4.
        vm.step();

        assert_eq!(vm.energy, energy_before - 4);
        assert_eq!(vm.stack.len(), 2);
        assert_eq!(vm.stack[0], Value::Int(1));
        assert_eq!(vm.stack[1], Value::Int(2));
    }

    #[test]
    fn test_reflex_wakeup() {
        // [ metabolism(0) ] ... [ push(99) ]
        // Strand 0 sleeps. Reflex wakes up and jumps to Strand 1.
        let strand0 = Strand {
            genes: vec![
                Gene { op: OpCode::Metabolism, args: vec![] },
            ]
        };
        let strand1 = Strand {
            genes: vec![
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(99)] },
            ]
        };
        let dna = Dna { helix: Helix { strands: vec![strand0, strand1] } };
        let mut vm = ChimeraVM::new(dna);

        vm.stack.push(Value::Int(0));
        vm.step(); // Execute metabolism(0)
        assert_eq!(vm.metabolic_rate, 0);

        // Register reflex
        vm.reflexes.insert(1, 1); // Event 1 -> Strand 1

        // Trigger reflex
        vm.trigger_reflex(1);

        assert_eq!(vm.metabolic_rate, 1); // Should be awake
        assert_eq!(vm.ip, (1, 0)); // Should be at strand 1

        // Step
        vm.step();
        assert_eq!(vm.stack.pop(), Some(Value::Int(99)));
    }
}
