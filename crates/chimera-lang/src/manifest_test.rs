#![cfg(feature = "oracle")]
#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, JunctionType, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    fn make_dna(strands: Vec<Vec<Gene>>) -> Dna {
        let strands = strands.into_iter().map(|genes| Strand { genes }).collect();
        Dna { evolution_config: None,
            helix: Helix { strands },
        }
    }

    fn gene(name: &str, arg: Option<i64>) -> Gene {
        Gene {
            op: name.parse().unwrap_or(OpCode::Nop),
            args: if let Some(n) = arg {
                vec![Nucleotide::Number(n)]
            } else {
                vec![]
            },
        }
    }

    #[test]
    fn test_manifest_grid() {
        // Goal: Transform all cells with value 1 to value 99.
        // Setup: Grid has (5,5)=1, (6,6)=1.
        // Query: cell(?X, ?Y, 1)
        // Transform: cell(?X, ?Y, 99)

        let program = vec![]; // We manipulate VM directly
        let dna = make_dna(vec![program]);
        let mut vm = ChimeraVM::new(dna);

        // Setup Grid
        vm.grid[5][5] = Value::Int(1);
        vm.grid[6][6] = Value::Int(1);
        vm.grid[7][7] = Value::Int(2); // Should not change

        // Query: cell(?X, ?Y, 1)
        let query = Value::Junction(
            JunctionType::Any,
            vec![
                Value::Str("cell".to_string()),
                Value::Str("?X".to_string()),
                Value::Str("?Y".to_string()),
                Value::Int(1),
            ],
        );

        // Transform: cell(?X, ?Y, 99)
        let transform = Value::Junction(
            JunctionType::Any,
            vec![
                Value::Str("cell".to_string()),
                Value::Str("?X".to_string()),
                Value::Str("?Y".to_string()),
                Value::Int(99),
            ],
        );

        vm.stack.push(query);
        vm.stack.push(transform);

        crate::vm::oracle::exec_oracle_op(&mut vm, OpCode::Manifest, &[]);

        // Check results
        assert_eq!(vm.grid[5][5], Value::Int(99));
        assert_eq!(vm.grid[6][6], Value::Int(99));
        assert_eq!(vm.grid[7][7], Value::Int(2));
    }

    #[test]
    fn test_manifest_dna() {
        // Goal: Replace "push" with "drop"
        // Query: gene(0, ?Idx, "push")
        // Transform: gene(0, ?Idx, "drop")

        // Strand 0: [ push(1), add(), push(2) ]
        let s0 = vec![
            gene("push", Some(1)),
            gene("add", None),
            gene("push", Some(2)),
        ];

        let dna = make_dna(vec![s0]);
        let mut vm = ChimeraVM::new(dna);

        // Query: gene(0, ?Idx, "push")
        let query = Value::Junction(
            JunctionType::Any,
            vec![
                Value::Str("gene".to_string()),
                Value::Int(0),
                Value::Str("?Idx".to_string()),
                Value::Str("push".to_string()),
            ],
        );

        // Transform: gene(0, ?Idx, "drop")
        let transform = Value::Junction(
            JunctionType::Any,
            vec![
                Value::Str("gene".to_string()),
                Value::Int(0),
                Value::Str("?Idx".to_string()),
                Value::Str("drop".to_string()),
            ],
        );

        vm.stack.push(query);
        vm.stack.push(transform);

        crate::vm::oracle::exec_oracle_op(&mut vm, OpCode::Manifest, &[]);

        // Check results
        // Gene 0 and 2 should be Drop. Gene 1 should remain Add.
        assert_eq!(vm.dna.helix.strands[0].genes[0].op, OpCode::Drop);
        assert_eq!(vm.dna.helix.strands[0].genes[1].op, OpCode::Add);
        assert_eq!(vm.dna.helix.strands[0].genes[2].op, OpCode::Drop);
    }
}
