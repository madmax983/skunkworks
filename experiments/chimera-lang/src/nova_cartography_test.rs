#[cfg(test)]
#[cfg(feature = "nova")]
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
    fn test_locate() {
        let genes = vec![Gene {
            op: OpCode::Locate,
            args: vec![],
        }];
        let mut vm = ChimeraVM::new(make_dna(genes));
        vm.context_loc = (5, 10);
        vm.step();

        assert_eq!(vm.stack.pop(), Some(Value::Int(10))); // x
        assert_eq!(vm.stack.pop(), Some(Value::Int(5))); // y
    }

    #[test]
    fn test_chart_and_atlas() {
        // [ push(42) push(5) push(5) chart() push(5) push(5) atlas() ]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(42)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                op: OpCode::Chart,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                op: OpCode::Atlas,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        while !vm.halted && vm.ip.1 < 7 {
            vm.step();
        }

        assert_eq!(vm.stack.pop(), Some(Value::Int(42)));
        assert_eq!(vm.cartography_grid[5][5], Value::Int(42));
    }

    #[test]
    fn test_scan() {
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(2)],
            },
            Gene {
                op: OpCode::Scan,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        vm.context_loc = (8, 8); // Default

        // Seed grid
        vm.grid[8][8] = Value::Int(99); // Center
        vm.grid[8][9] = Value::Int(1); // Right 1
        vm.grid[8][11] = Value::Int(2); // Right 3 (out of range 2)

        vm.step(); // push
        vm.step(); // scan

        if let Some(Value::Junction(_, vals)) = vm.stack.pop() {
            assert!(vals.contains(&Value::Int(99)));
            assert!(vals.contains(&Value::Int(1)));
            assert!(!vals.contains(&Value::Int(2)));
        } else {
            panic!("Expected Junction from Scan");
        }
    }
}
