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
    fn test_superpose() {
        // [ push(1) push(2) superpose() ]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(2)],
            },
            Gene {
                op: OpCode::Superpose,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        vm.step();
        vm.step();
        vm.step();

        assert_eq!(vm.stack.len(), 1);
        if let Value::Superposition(states) = &vm.stack[0] {
            assert_eq!(states.len(), 2);
            // We can't guarantee order, but we check contents
            let has_1 = states
                .iter()
                .any(|(v, p)| *v == Value::Int(1) && (*p - 0.5).abs() < 1e-6);
            let has_2 = states
                .iter()
                .any(|(v, p)| *v == Value::Int(2) && (*p - 0.5).abs() < 1e-6);
            assert!(has_1);
            assert!(has_2);
        } else {
            panic!("Expected Superposition");
        }
    }

    #[test]
    fn test_quantum_add() {
        // [ push(1) push(2) superpose() push(10) add() ]
        // Ψ(1:0.5, 2:0.5) + 10 = Ψ(11:0.5, 12:0.5)
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(2)],
            },
            Gene {
                op: OpCode::Superpose,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                op: OpCode::Add,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        while !vm.halted {
            vm.step();
        }

        assert_eq!(vm.stack.len(), 1);
        if let Value::Superposition(states) = &vm.stack[0] {
            assert_eq!(states.len(), 2);
            let has_11 = states
                .iter()
                .any(|(v, p)| *v == Value::Int(11) && (*p - 0.5).abs() < 1e-6);
            let has_12 = states
                .iter()
                .any(|(v, p)| *v == Value::Int(12) && (*p - 0.5).abs() < 1e-6);
            assert!(has_11);
            assert!(has_12);
        } else {
            panic!("Expected Superposition");
        }
    }

    #[test]
    fn test_collapse() {
        // [ push(1) push(2) superpose() collapse() ]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(2)],
            },
            Gene {
                op: OpCode::Superpose,
                args: vec![],
            },
            Gene {
                op: OpCode::Collapse,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        while !vm.halted {
            vm.step();
        }

        assert_eq!(vm.stack.len(), 1);
        match &vm.stack[0] {
            Value::Int(n) => assert!(n == &1 || n == &2),
            _ => panic!("Expected Int after collapse, got {:?}", vm.stack[0]),
        }
    }
}
