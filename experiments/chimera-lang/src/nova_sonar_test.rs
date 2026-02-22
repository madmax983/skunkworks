#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna { evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_sonar_hit() {
        // [ push(0) push(1) sonar() ] -> Ping East (dy=0, dx=1)
        // Setup: Place an obstacle at (0, 5)
        // Start at (0,0)
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            }, // dy
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            }, // dx
            Gene {
                op: OpCode::Sonar,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        vm.context_loc = (0, 0);
        vm.grid[0][5] = Value::Int(99);

        // Step 1: push(1)
        vm.step();
        // Step 2: push(0)
        vm.step();
        // Step 3: sonar()
        vm.step();

        // Expect stack: [5, 99] (Distance, Value)
        assert_eq!(vm.stack.len(), 2);
        let val = vm.stack.pop().unwrap();
        let dist = vm.stack.pop().unwrap();

        assert_eq!(dist, Value::Int(5));
        assert_eq!(val, Value::Int(99));

        // Target should be set
        assert_eq!(vm.sonar_target, Some((0, 5)));
    }

    #[test]
    fn test_sonar_miss() {
        // [ push(1) push(0) sonar() ] -> Ping East
        // Setup: No obstacles
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                op: OpCode::Sonar,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        vm.context_loc = (0, 0);

        // Run
        vm.step();
        vm.step();
        vm.step();

        // Expect stack: [16, 0] (Max Distance, Empty)
        assert_eq!(vm.stack.len(), 2);
        let val = vm.stack.pop().unwrap();
        let dist = vm.stack.pop().unwrap();

        assert_eq!(dist, Value::Int(16));
        assert_eq!(val, Value::Int(0));

        // Target should be None (or not set by logic? logic says sonar_target is set if found)
        // Code: if !found { ... } -> doesn't set sonar_target.
        // vm.sonar_target is None (cleared at start of step)
        assert_eq!(vm.sonar_target, None);
    }

    #[test]
    fn test_sonar_wrap() {
        // [ push(0) push(-1) sonar() ] -> Ping West (dy=0, dx=-1)
        // Setup: Obstacle at (0, 15) which is 1 step West from (0,0)
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(-1)],
            },
            Gene {
                op: OpCode::Sonar,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        vm.context_loc = (0, 0);
        vm.grid[0][15] = Value::Int(77);

        vm.step();
        vm.step();
        vm.step();

        assert_eq!(vm.stack.len(), 2);
        let val = vm.stack.pop().unwrap();
        let dist = vm.stack.pop().unwrap();

        assert_eq!(dist, Value::Int(1)); // 1 step west wraps to 15
        assert_eq!(val, Value::Int(77));
        assert_eq!(vm.sonar_target, Some((0, 15)));
    }
}
