#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    #[test]
    #[cfg(feature = "nova")]
    fn test_time_warp_acceleration() {
        // Main strand:
        // 1. Spawn worker at (5,5) running strand 1
        // 2. Set TimeWarp(2) at (5,5) radius 1
        // 3. Wait

        // Strand 1 (Organelle):
        // [ push(1) add() ] (Assuming stack has 0 initially? No, stack is empty. Push(1). )
        // Let's make it simply push(1).
        // If it runs twice, stack len will be 2.

        let s0 = Strand {
            genes: vec![
                // Move to 5,5 (Start is 8,8)
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(-3)],
                }, // dx
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(-3)],
                }, // dy
                Gene {
                    op: OpCode::Migrate,
                    args: vec![],
                }, // context_loc is now 5,5
                // Spawn Organelle (Type 0=Worker) running Strand 1
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(1)],
                }, // Strand 1
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(0)],
                }, // Type 0
                Gene {
                    op: OpCode::Spawn,
                    args: vec![],
                },
                // Set TimeWarp(2) radius 1 at 5,5
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(1)],
                }, // Radius
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(2)],
                }, // Factor
                Gene {
                    op: OpCode::TimeWarp,
                    args: vec![],
                },
            ],
        };

        let s1 = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(1)],
                },
                Gene {
                    op: OpCode::Jump,
                    args: vec![Nucleotide::Number(1)],
                }, // Loop
            ],
        };

        let mut vm = ChimeraVM::new(Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![s0, s1],
            },
        });
        vm.energy = 1000;

        // Run setup (Strand 0)
        // 1. Push 5
        // 2. Push 5
        // 3. Migrate
        // 4. Push 1
        // 5. Push 0
        // 6. Spawn
        // 7. Push 1
        // 8. Push 2
        // 9. TimeWarp
        for _ in 0..9 {
            vm.step();
        }

        assert_eq!(vm.organelles.len(), 1);
        assert_eq!(vm.time_grid[5][5], 2);

        // Now run 1 step.
        // Organelle is at 5,5. Dilation is 2.
        // Should execute 2 ticks.
        // Tick 1: Push(1)
        // Tick 2: Jump(1)

        // Wait, "Loop to keep running".
        // If I just have Push(1), it finishes and halts?
        // Organelle halts if it runs out of genes.
        // So Loop is good.

        vm.step(); // Main thread steps. Organelles processed.

        let org = &vm.organelles[0];
        // Stack should have 1 item: 1.
        // Wait, Jump consumes 1 tick?
        // Let's see:
        // Tick 1: Gene 0 (Push 1). IP -> (1, 1).
        // Tick 2: Gene 1 (Jump 1). IP -> (1, 0).

        // Due to setup steps, stack already has items.
        // Setup runs steps 0..9.
        // Step 6: Spawn. Org runs Push(1). Stack [1].
        // Step 7: Org runs Jump(1).
        // Step 8: Org runs Push(1). Stack [1, 1].
        // Step 9: Org runs Jump(1).
        // Before verification step: Stack [1, 1].

        // Verification Step (Factor 2):
        // Tick 1: Push(1). Stack [1, 1, 1].
        // Tick 2: Jump(1).

        assert_eq!(org.stack.len(), 2);
        assert_eq!(org.stack[0], Value::Int(1));

        // Step again
        vm.step();
        // Tick 3: Push 1. Stack -> [1, 1, 1, 1].
        // Tick 4: Jump 1.

        let org = &vm.organelles[0];
        assert_eq!(org.stack.len(), 3);
    }

    #[test]
    #[cfg(feature = "nova")]
    fn test_time_warp_stasis() {
        // Spawn worker, Set Warp(0). Worker should do nothing.

        let s0 = Strand {
            genes: vec![
                // Move to 5,5
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(-3)],
                },
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(-3)],
                },
                Gene {
                    op: OpCode::Migrate,
                    args: vec![],
                },
                // Spawn Organelle
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(1)],
                },
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(0)],
                },
                Gene {
                    op: OpCode::Spawn,
                    args: vec![],
                },
                // Set TimeWarp(0)
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(1)],
                },
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(0)],
                },
                Gene {
                    op: OpCode::TimeWarp,
                    args: vec![],
                },
            ],
        };

        let s1 = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(99)],
                },
                Gene {
                    op: OpCode::Jump,
                    args: vec![Nucleotide::Number(0)],
                },
            ],
        };

        let mut vm = ChimeraVM::new(Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![s0, s1],
            },
        });
        vm.energy = 1000;

        for _ in 0..9 {
            vm.step();
        }

        assert_eq!(vm.time_grid[5][5], 0);
        assert_eq!(vm.organelles.len(), 1);

        let initial_len = vm.organelles[0].stack.len();
        // Should be > 0 because it ran during setup steps
        assert!(initial_len > 0);

        // Step
        vm.step();

        // Organelle should be frozen. Stack length unchanged.
        let org = &vm.organelles[0];
        assert_eq!(org.stack.len(), initial_len);
    }
}
