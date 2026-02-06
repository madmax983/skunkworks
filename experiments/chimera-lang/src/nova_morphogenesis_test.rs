#[cfg(feature = "nova")]
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
    fn test_morphogenesis_expansion() {
        // Axiom: "A"
        // Rules: "A=AB,B=A"
        // Iterations: 2
        // Expansion: A -> AB -> ABA

        // Stack: "A", "A=AB,B=A", 2, Morph
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("A".to_string())],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("A=AB,B=A".to_string())],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(2)],
            },
            Gene {
                op: OpCode::Morph,
                args: vec![],
            }, // Should produce "ABA"
        ];

        let mut vm = ChimeraVM::new(make_dna(genes));
        while !vm.halted {
            vm.step();
        }

        assert_eq!(vm.stack.pop(), Some(Value::Str("ABA".to_string())));
    }

    #[test]
    fn test_morphogenesis_grow() {
        // Simple Square: F+F+F+F
        // Start at 8,8
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("F+F+F+F".to_string())],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(8)],
            }, // Y
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(8)],
            }, // X
            Gene {
                op: OpCode::Grow,
                args: vec![],
            },
        ];

        let mut vm = ChimeraVM::new(make_dna(genes));
        while !vm.halted {
            vm.step();
        }

        // Check if grid was written to
        // F moves and writes. Start 8,8.
        // F (8,8) -> (8,9)
        // + (turn right -> South)
        // F (8,9) -> (9,9)
        // + (turn right -> West)
        // F (9,9) -> (9,8)
        // + (turn right -> North)
        // F (9,8) -> (8,8)

        // We expect these cells to be non-zero (or specifically marked)
        // Let's assume Grow writes a "Structure" value, maybe String "Structure" or Int(100).
        // For now, just check non-zero.

        // Note: The implementation detail of what Grow writes needs to be decided.
        // Let's assume it writes Value::Str("#") for now.

        // Verify path
        // Start 8,8. Dir N (-Y).
        // F -> Draw 8,8. Move 8,7.
        // + -> E.
        // F -> Draw 8,7. Move 9,7.
        // + -> S.
        // F -> Draw 9,7. Move 9,8.
        // + -> W.
        // F -> Draw 9,8. Move 8,8.

        assert!(matches!(vm.grid[8][8], Value::Str(ref s) if s == "#"));
        assert!(matches!(vm.grid[7][8], Value::Str(ref s) if s == "#"));
        assert!(matches!(vm.grid[7][9], Value::Str(ref s) if s == "#"));
        assert!(matches!(vm.grid[8][9], Value::Str(ref s) if s == "#"));
    }
}
