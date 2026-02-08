#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, JunctionType, Nucleotide, Strand};
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
    fn test_morphogenesis_expansion_junction() {
        // Axiom: "A"
        // Rules: Junction ["A->AB", "B->A"]
        // Iterations: 2
        // Expansion: A -> AB -> ABA

        let mut vm = ChimeraVM::new(Dna {
            helix: Helix { strands: vec![] },
        });

        let rules = Value::Junction(
            JunctionType::Any,
            vec![
                Value::Str("A->AB".to_string()),
                Value::Str("B->A".to_string()),
            ],
        );

        vm.stack.push(Value::Str("A".to_string()));
        vm.stack.push(rules);
        vm.stack.push(Value::Int(2));

        let gene = Gene {
            op: OpCode::Morph,
            args: vec![],
        };
        vm.dna.helix.strands.push(Strand { genes: vec![gene] });

        vm.step();

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
        // Direction 0 = East (default)
        // F (8,8) -> Draw, Move to (8,9)
        // + (turn right -> South)
        // F (8,9) -> Draw, Move to (9,9)
        // + (turn right -> West)
        // F (9,9) -> Draw, Move to (9,8)
        // + (turn right -> North)
        // F (9,8) -> Draw, Move to (8,8)

        // Expected cells to be "#":
        // (8,8), (8,9), (9,9), (9,8)

        assert!(
            matches!(vm.grid[8][8], Value::Str(ref s) if s == "#"),
            "Grid(8,8)"
        );
        assert!(
            matches!(vm.grid[8][9], Value::Str(ref s) if s == "#"),
            "Grid(8,9)"
        );
        assert!(
            matches!(vm.grid[9][9], Value::Str(ref s) if s == "#"),
            "Grid(9,9)"
        );
        assert!(
            matches!(vm.grid[9][8], Value::Str(ref s) if s == "#"),
            "Grid(9,8)"
        );
    }
}
