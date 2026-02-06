#[cfg(all(test, feature = "nova"))]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::ChimeraVM;

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_fractal_state() {
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("A".to_string())],
            },
            Gene {
                op: OpCode::LSystem,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("A".to_string())],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("AB".to_string())],
            },
            Gene {
                op: OpCode::Fractal,
                args: vec![],
            },
        ];

        let mut vm = ChimeraVM::new(make_dna(genes));
        while !vm.halted {
            vm.step();
        }

        assert_eq!(vm.fractal_axiom, "A");
        assert_eq!(vm.fractal_rules.get(&'A'), Some(&"AB".to_string()));
    }

    #[test]
    fn test_fractal_drawing() {
        // [ push("F") l_system() push("F+F") push("F") fractal() push(1) grow() ]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("F".to_string())],
            },
            Gene {
                op: OpCode::LSystem,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("F".to_string())],
            }, // Key
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("F+F".to_string())],
            }, // Value
            Gene {
                op: OpCode::Fractal,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Grow,
                args: vec![],
            },
        ];

        let mut vm = ChimeraVM::new(make_dna(genes));
        while !vm.halted {
            vm.step();
        }

        // F+F -> 2 lines
        assert_eq!(vm.fractal_lines.len(), 2);

        // Check line coordinates roughly
        // Start 0,0, Angle -90 (Up)
        // Line 1: 0,0 -> 0,-Step
        // Rotate +90 -> Angle 0 (Right)
        // Line 2: 0,-Step -> Step,-Step

        let line1 = vm.fractal_lines[0];
        assert!((line1.x1).abs() < 1e-5);
        assert!((line1.y1).abs() < 1e-5);
        // y2 should be negative
        assert!(line1.y2 < 0.0);
    }
}
