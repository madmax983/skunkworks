#[cfg(feature = "nova")]
#[cfg(feature = "oracle")]
mod tests {
    use chimera_lang::ast::{Dna, Gene, Helix, JunctionType, Nucleotide, Strand};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::{ChimeraVM, Value};

    #[test]
    fn test_divergence_mechanism() {
        // Construct a scenario where Divergence *can* find a solution.
        // Scenario:
        // [ push(0) push(0) push(0) g_write() ... ]
        // We want cell(0,0,1).
        // Mutation can change `push(0)` to `push(1)`.

        let genes = vec![
            // Gene 0: Push(0) - Value. Target for mutation.
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            // Gene 1: Push(0) - Y
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            // Gene 2: Push(0) - X
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            // Gene 3: GWrite
            Gene {
                op: OpCode::GWrite,
                args: vec![],
            },
            // Query: cell(0,0,1)
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Junction(
                    JunctionType::Any,
                    vec![
                        Nucleotide::String("cell".to_string()),
                        Nucleotide::Number(0),
                        Nucleotide::Number(0),
                        Nucleotide::Number(1),
                    ],
                )],
            },
            // Count: 200 tries
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(200)],
            },
            // Divergence
            Gene {
                op: OpCode::Divergence,
                args: vec![],
            },
        ];

        let dna = Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        // Enable Chaos to ensure mutation is allowed
        vm.chaos_mode = true;

        // Run steps
        for _ in 0..10 {
            vm.step();
            if let Some(Value::Int(1)) = vm.stack.last() {
                // Success!
                // Verify grid state
                // Since simulation runs for 10 steps, GWrite should have happened.
                assert_eq!(
                    vm.grid[0][0],
                    Value::Int(1),
                    "Grid should contain 1 in the successful timeline"
                );
                return;
            }
        }

        // If we reach here, it failed.
    }
}
