#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::ChimeraVM;

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna { evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_chaos_injection() {
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(50)], // Amount
            },
            Gene {
                op: OpCode::Chaos,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        vm.step(); // Push
        vm.step(); // Chaos

        // Havoc rate is f64, default 0.0.
        // Chaos(50) -> adds 50/1000 = 0.05 to rate.
        assert!(vm.havoc.rate > 0.0);
        assert!(vm.glitch_level > 0.0);

        // Check entropy grid
        let mut total_entropy = 0;
        for row in &vm.entropy_grid {
            for cell in row {
                total_entropy += cell;
            }
        }
        assert!(total_entropy > 0);
    }

    #[test]
    fn test_genesis_reboot() {
        // Setup:
        // 1. Sow a rule (e.g. Rule 2: B1/S1)
        // 2. Write pattern to grid
        // 3. Trigger Genesis(2)
        // Expect: DNA cleared, new DNA created from grid state.

        let genes = vec![
            // Sow Rule 2: B1/S1 (Anything with 1 neighbor is born/survives)
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("B1/S1".to_string())],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(2)],
            },
            Gene {
                op: OpCode::Sow,
                args: vec![],
            },
            // Write some "Life" to grid
            // Grid[0][0] = 5 (Push(5))
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                op: OpCode::GWrite,
                args: vec![],
            },
            // Grid[0][1] = "add"
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("add".to_string())],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::GWrite,
                args: vec![],
            },
            // Genesis(2)
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(2)],
            },
            Gene {
                op: OpCode::Genesis,
                args: vec![],
            },
        ];

        let mut vm = ChimeraVM::new(make_dna(genes));

        // Execute setup
        // We have 8 genes. Step 8 times?
        // Sow takes 3 ops (Push, Push, Sow).
        // GWrite 1 takes 4 ops.
        // GWrite 2 takes 4 ops.
        // Genesis takes 2 ops.
        // Total 13 steps.

        for _ in 0..20 {
            if vm.dna.helix.strands.len() == 1
                && vm.dna.helix.strands[0].genes.len() > 2
                && vm.tick_counter == 0
            {
                // Genesis resets tick_counter? No, I didn't reset tick_counter in code, but I reset IP.
                // Wait, exec_genesis resets IP to (0,0).
                // And clears old DNA.
                // So if we detect a new strand that is NOT our original setup, we know it worked.
            }
            vm.step();
            if vm
                .output
                .last()
                .map(|s| s.contains("born"))
                .unwrap_or(false)
            {
                break;
            }
        }

        // Verify Genesis
        assert_eq!(vm.dna.helix.strands.len(), 1);
        let new_strand = &vm.dna.helix.strands[0];

        // We expect some genes.
        let ops: Vec<String> = new_strand.genes.iter().map(|g| g.op.to_string()).collect();
        // With B1/S1, (0,0) and (0,1) should survive/be born.
        assert!(ops.contains(&"push".to_string()));
        assert!(ops.contains(&"add".to_string()));
    }
}
