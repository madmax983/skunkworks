#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            helix: Helix {
                strands: vec![std::rc::Rc::new(Strand { genes })],
            },
        }
    }

    #[test]
    fn test_migrate_wrap() {
        // [ push(1) push(15) migrate() ]
        // Starts at (8, 8).
        // Move dx=15 (should wrap to 8+15 = 23 -> 7)
        // Move dy=1 (should go to 9)
        // Result: (9, 7)
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            }, // dy
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(15)],
            }, // dx
            Gene {
                op: OpCode::Migrate,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        // Execute
        vm.step();
        vm.step();
        vm.step();

        assert_eq!(vm.context_loc, (9, 7));

        // Test negative wrapping
        // [ push(-1) push(-1) migrate() ]
        // From (9, 7) -> (8, 6)
        // But let's test wrap across 0.
        // Current x=7. Move dx = -8 -> x = -1 -> 15.
        // Current y=9.

        // We need new genes. Reset VM or append?
        // Let's create a new VM for negative test.
        let genes2 = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            }, // dy
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(-9)],
            }, // dx
            Gene {
                op: OpCode::Migrate,
                args: vec![],
            },
        ];
        let mut vm2 = ChimeraVM::new(make_dna(genes2));
        // Start at (8, 8).
        // dx = -9. 8 - 9 = -1. Wrap to 15.
        // dy = 0.

        vm2.step();
        vm2.step();
        vm2.step();
        assert_eq!(vm2.context_loc, (8, 15));
    }

    #[test]
    fn test_cas9_cut_boundary() {
        // [ push(0) push(3) cas9_cut() ]
        // Strand 0 has 3 genes. Cut at 3 (end).
        // Stack: strand_idx (0), cut_idx (3). Top is 3.
        // Should create empty strand.
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(3)],
            },
            Gene {
                op: OpCode::Cas9Cut,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        // Execute
        vm.step();
        vm.step();
        vm.step();

        // Strand 0 should still have 3 genes.
        // Cut at 3 means split_off(3).
        // Genes indices 0, 1, 2. Length 3.
        // split_off(3) returns empty. Original keeps 0..3.
        assert_eq!(vm.dna.helix.strands[0].genes.len(), 3);

        // New strand (index 1) should be empty
        assert_eq!(vm.dna.helix.strands.len(), 2);
        assert!(vm.dna.helix.strands[1].genes.is_empty());

        // What if we cut at 0?
        // [ push(0) push(0) cas9_cut() ]
        // Strand 0 becomes empty. Strand 1 gets all genes.
        let genes3 = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                op: OpCode::Cas9Cut,
                args: vec![],
            },
        ];
        let mut vm3 = ChimeraVM::new(make_dna(genes3));

        vm3.step();
        vm3.step();
        vm3.step();

        // Original strand (0) empty
        assert!(vm3.dna.helix.strands[0].genes.is_empty());
        // New strand (1) has 3 genes
        assert_eq!(vm3.dna.helix.strands[1].genes.len(), 3);
    }

    #[test]
    fn test_ligase_boundary() {
        // Strand 0: [ push(0) push(1) ligase() ]
        // Stack: recipient=0, donor=1. Top=1.
        // Strand 1: [ ] (Empty)
        // Join 1 to 0.
        // Should result in Strand 0 unchanged (length 3), Strand 1 empty.

        let s0 = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(0)],
                },
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(1)],
                },
                Gene {
                    op: OpCode::Ligase,
                    args: vec![],
                },
            ],
        };
        let s1 = Strand { genes: vec![] };

        let dna = Dna {
            helix: Helix {
                strands: vec![s0.into(), s1.into()],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        vm.step();
        vm.step();
        vm.step();

        assert_eq!(vm.dna.helix.strands[0].genes.len(), 3);
        assert_eq!(vm.dna.helix.strands[1].genes.len(), 0);

        // Test joining TO an empty strand.
        // Strand 0: [ push(1) push(0) ligase() ] -> Recipient 1, Donor 0.
        // Stack: 1, 0. Top=0 (Donor).
        // Strand 1: []
        // Result: Strand 1 has 0's genes. Strand 0 empty.

        let s0_2 = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(1)],
                },
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(0)],
                },
                Gene {
                    op: OpCode::Ligase,
                    args: vec![],
                },
            ],
        };
        let s1_2 = Strand { genes: vec![] };
        let dna2 = Dna {
            helix: Helix {
                strands: vec![s0_2.into(), s1_2.into()],
            },
        };
        let mut vm2 = ChimeraVM::new(dna2);

        // Warning: if we move genes from current executing strand, what happens to execution?
        // Ligase moves genes. `strand_d` genes are cleared.
        // VM is executing strand 0.
        // If we move it to strand 1, strand 0 becomes empty.
        // Does VM crash?
        // VM step checks `ip`.
        // After execute_gene, `step` continues.
        // `step` checks `strand_len` AFTER execute?
        // No, check `step` code:
        // `jump_target` logic.
        // Then `ip` update.
        // Next loop iteration: `strand_len = ...`.
        // If strand 0 is empty, `strand_len` is 0.
        // `if self.ip.1 >= strand_len`.
        // `ip.1` was incremented to 3 (after ligase).
        // 3 >= 0. True.
        // `ip.0 += 1`. Moves to strand 1.
        // So it naturally flows to next strand.
        // SAFE.

        vm2.step();
        vm2.step();
        vm2.step();

        assert!(vm2.dna.helix.strands[0].genes.is_empty());
        assert_eq!(vm2.dna.helix.strands[1].genes.len(), 3);
    }

    #[test]
    fn test_spore_branching() {
        // 1. Create Spore 0.
        // 2. Change state (push 100).
        // 3. Create Spore 1.
        // 4. Germinate 0.
        // 5. Verify Spore 1 still exists in `vm.spores`.
        // 6. Germinate 1 (from state 0).
        // 7. Verify stack has 100.

        let genes = vec![
            Gene {
                op: OpCode::Sporulate,
                args: vec![],
            }, // 0
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(100)],
            }, // 1
            Gene {
                op: OpCode::Sporulate,
                args: vec![],
            }, // 2
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            }, // 3: ID 0
            Gene {
                op: OpCode::Germinate,
                args: vec![],
            }, // 4
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        vm.energy = 1000;

        vm.step(); // Sporulate 0. Stack: [0].
        vm.step(); // Push 100. Stack: [0, 100].
        vm.step(); // Sporulate 1. Stack: [0, 100, 1].

        assert_eq!(vm.spores.len(), 2);

        vm.step(); // Push 0. Stack: [0, 100, 1, 0].

        // Germinate 0.
        // Restore state to start of Sporulate 0.
        // Snapshot taken BEFORE pushing ID.
        // So Stack is [] (Empty).
        vm.execute_gene_inner(OpCode::Germinate, &[]);

        // After germinate(0):
        // Stack: [].
        // Spores: Still 2? Yes.
        assert_eq!(vm.spores.len(), 2);
        assert_eq!(vm.stack.len(), 0);

        // Now we are in timeline A (restored).
        // But we have knowledge of Spore 1 (timeline B).
        // Can we jump to Spore 1?
        // We need to push ID 1.
        vm.stack.push(Value::Int(1));
        vm.execute_gene_inner(OpCode::Germinate, &[]);

        // After germinate(1):
        // Restore state to start of Sporulate 1.
        // Snapshot was taken when stack was [0, 100].
        // So Stack should be [0, 100].
        assert_eq!(vm.stack.len(), 2);

        // Check values. The first value (0) is ID of Spore 0?
        // Wait, T0 -> Sporulate(0) pushes 0.
        // So Stack was [0].
        // Then push 100. Stack [0, 100].
        // Then Sporulate(1). Snapshot stack [0, 100].
        // Then pushes 1.
        // So restored stack is [0, 100].

        if let Value::Int(n) = vm.stack[0] {
            assert_eq!(n, 0);
        } else {
            panic!("Expected 0 at stack[0]");
        }
        if let Value::Int(n) = vm.stack[1] {
            assert_eq!(n, 100);
        } else {
            panic!("Expected 100 at stack[1]");
        }
    }

    #[test]
    fn test_integrase_append() {
        // [ push(0) push(3) push("push") push(99) integrase() ]
        // Strand len 5. Append at 5.
        // 0: push
        // 1: push
        // 2: push
        // 3: push
        // 4: integrase
        // 5: [New]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            }, // Index 5 (append)
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("push".to_string())],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(99)],
            },
            Gene {
                op: OpCode::Integrase,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        for _ in 0..5 {
            vm.step();
        } // Execute all

        assert_eq!(vm.dna.helix.strands[0].genes.len(), 6);
        assert_eq!(vm.dna.helix.strands[0].genes[5].op, OpCode::Push);
    }

    #[test]
    fn test_excision_last() {
        // [ push(0) push(2) excision() ]
        // Remove gene 2 (excision itself).
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(2)],
            },
            Gene {
                op: OpCode::Excision,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        vm.step();
        vm.step();
        // Execute excision
        vm.step();

        assert_eq!(vm.dna.helix.strands[0].genes.len(), 2);
        // Excision removed itself.
    }

    #[test]
    fn test_simulate_recursion_limit() {
        // Strand 0: [ push(0) push(10) simulate() ]
        // Calls itself recursively.
        // Each call executes 10 ticks.
        // Within 10 ticks, it calls Simulate again.
        // Should eventually hit depth limit.

        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            }, // strand 0
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            }, // ticks
            Gene {
                op: OpCode::Simulate,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        vm.energy = 10000; // Lots of energy

        // We expect it NOT to panic.
        // We expect output to contain "Recursion limit exceeded" eventually.
        // But since we are creating nested VMs, `vm.output` is cloned.
        // The error will be in the deepest VM's output.
        // The top-level VM will just see the result of the simulation.
        // If simulation fails (e.g. error), it might return 0 status.

        // Run for enough steps to trigger recursion
        for _ in 0..100 {
            vm.step();
            if vm.halted {
                break;
            }
        }

        // Check output for error?
        // Or check that recursion depth is managed.
        // If we modify `nova.rs` to check recursion depth in Simulate, it should push an error.

        // Assert that the VM did not crash and is still valid
        assert!(
            !vm.output
                .iter()
                .any(|s| s.contains("Recursion limit exceeded")),
            "Outer VM should not report recursion error from inner VM"
        );
        // VM halts because it finishes the strand. Check it didn't starve.
        if vm.halted {
            assert!(!vm.output.contains(&"DEATH: STARVATION".to_string()));
        }
    }

    #[test]
    fn test_recombine_edge_cases() {
        // Strand 0: [ push(0) push(1) push(3) recombine() ]
        // Split at 3 (end of strand 0).
        // Strand 1: [ push(99) ] (len 1)
        // Split at 3 is > len 1. Should error.

        let s0 = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(0)],
                },
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(1)],
                },
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(3)],
                },
                Gene {
                    op: OpCode::Recombine,
                    args: vec![],
                },
            ],
        };
        let s1 = Strand {
            genes: vec![Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(99)],
            }],
        };
        let dna = Dna {
            helix: Helix {
                strands: vec![s0.into(), s1.into()],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        for _ in 0..4 {
            vm.step();
        }

        assert!(vm
            .output
            .iter()
            .any(|s| s.contains("Split point out of bounds")));
    }

    #[test]
    fn test_crispr_scan_edge_cases() {
        // Target: [ push(1) push(2) ]
        // Guide: [ push(1) ] (Matches index 0)
        // Guide 2: [ push(3) ] (No match)
        // Guide 3: [ ] (Empty match?)

        let target_strand = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(1)],
                },
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(2)],
                },
            ],
        };
        let guide_match = Strand {
            genes: vec![Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            }],
        };
        let guide_fail = Strand {
            genes: vec![Gene {
                op: OpCode::Add,
                args: vec![],
            }],
        };
        let guide_empty = Strand { genes: vec![] };

        // We need a runner strand to execute CrisprScan
        // [ push(0) push(1) crispr_scan() ] -> scan target(0) using guide(1)
        let runner = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(0)],
                }, // target
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(1)],
                }, // guide
                Gene {
                    op: OpCode::CrisprScan,
                    args: vec![],
                },
            ],
        };

        let dna = Dna {
            helix: Helix {
                strands: vec![
                    target_strand.into(),
                    guide_match.into(),
                    guide_fail.into(),
                    guide_empty.into(),
                    runner.into(),
                ],
            },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.ip = (4, 0); // Start at runner

        // Test Match
        vm.step(); // push 0
        vm.step(); // push 1
        vm.step(); // scan
        assert_eq!(vm.stack.pop(), Some(Value::Int(0))); // Found at 0

        // Test Fail
        // Reset stack? Or just push new args.
        // [ push(0) push(2) crispr_scan() ]
        vm.execute_gene_inner(OpCode::Push, &[Nucleotide::Number(0)]);
        vm.execute_gene_inner(OpCode::Push, &[Nucleotide::Number(2)]);
        vm.execute_gene_inner(OpCode::CrisprScan, &[]);
        assert_eq!(vm.stack.pop(), Some(Value::Int(-1))); // Not found

        // Test Empty
        // [ push(0) push(3) crispr_scan() ]
        vm.execute_gene_inner(OpCode::Push, &[Nucleotide::Number(0)]);
        vm.execute_gene_inner(OpCode::Push, &[Nucleotide::Number(3)]);
        vm.execute_gene_inner(OpCode::CrisprScan, &[]);
        assert_eq!(vm.stack.pop(), Some(Value::Int(-1)));
    }

    #[test]
    fn test_gravitate_movement() {
        // Setup grid:
        // Center (8,8).
        // Item A at (8, 6) (dist 2)
        // Item B at (8, 5) (dist 3)
        // Gravitate(5).
        // A should move to (8, 7) (dist 1).
        // B should move to (8, 6) (dist 2).
        // Ensure B doesn't jump to (8, 7) or further.

        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                op: OpCode::Gravitate,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        vm.grid[8][6] = Value::Int(1); // A
        vm.grid[8][5] = Value::Int(2); // B

        vm.step(); // push
        vm.step(); // gravitate

        // Verify A moved to (8, 7)
        if let Value::Int(v) = vm.grid[8][7] {
            assert_eq!(v, 1, "Item A should be at (8, 7)");
        } else {
            panic!("Item A missing at (8, 7)");
        }

        // Verify B moved to (8, 6)
        if let Value::Int(v) = vm.grid[8][6] {
            assert_eq!(v, 2, "Item B should be at (8, 6)");
        } else {
            panic!("Item B missing at (8, 6)");
        }

        // Verify (8, 5) is empty
        assert_eq!(vm.grid[8][5], Value::Int(0));
    }

    #[test]
    fn test_entangle_mutation() {
        // Strand 0: [ push(1) ]
        // Strand 1: [ push(1) ]
        // Entangle them. Mutate 0. Check 1.
        let s0 = Strand {
            genes: vec![Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            }],
        };
        let s1 = Strand {
            genes: vec![Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            }],
        };
        let dna = Dna {
            helix: Helix {
                strands: vec![s0.into(), s1.into()],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        // Manually entangle
        vm.entangled_pairs.insert(0, 1);
        vm.entangled_pairs.insert(1, 0);

        // Force mutation on strand 0
        // We need to loop until mutation happens on strand 0?
        // Or call mutate() and hope RNG picks 0?
        // Let's force it by creating a VM with only 2 strands and calling mutate repeatedly.
        // But mutate picks random strand.
        // Better: test internal logic of mutate if possible?
        // No, mutate is public.

        // Loop until we see change.
        let max_tries = 1000;
        let mut changed = false;
        for _ in 0..max_tries {
            vm.mutate();
            // Check if strand 0 changed
            let g0 = &vm.dna.helix.strands[0].genes[0];
            let g1 = &vm.dna.helix.strands[1].genes[0];

            if g0.op != OpCode::Push || g0.args[0] != Nucleotide::Number(1) {
                // Strand 0 changed.
                // Verify Strand 1 matches Strand 0.
                assert_eq!(g0.op, g1.op, "Entangled ops mismatch");
                assert_eq!(g0.args.len(), g1.args.len(), "Entangled args len mismatch");
                if !g0.args.is_empty() {
                    // Start checking args
                    if let (Nucleotide::Number(n0), Nucleotide::Number(n1)) =
                        (&g0.args[0], &g1.args[0])
                    {
                        assert_eq!(n0, n1, "Entangled arg mismatch");
                    }
                }
                changed = true;
                break;
            }
        }
        assert!(changed, "Mutation never occurred");
    }
}
