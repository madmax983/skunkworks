#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::{ChimeraVM, Value};

    #[test]
    fn test_grimoire_passive_magic() {
        // Strand 0: Enable AutoCast, then Loop
        // genes: [ Push("TestSigil"), Push(1), AutoCast, Jump(0) ]
        let strand0 = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::String("TestSigil".to_string())],
                },
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(1)],
                },
                Gene {
                    op: OpCode::AutoCast,
                    args: vec![],
                },
                Gene {
                    op: OpCode::Jump,
                    args: vec![Nucleotide::Number(0)],
                },
            ],
        };

        // Strand 1: The Spell (Push 777)
        let strand1 = Strand {
            genes: vec![Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(777)],
            }],
        };

        let dna = Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![strand0, strand1],
            },
        };

        let mut vm = ChimeraVM::new(dna);

        // Manually insert Sigil
        // Pattern: 5 at (0,1) relative (East)
        // Note: We need to construct Sigil manually.
        // If Sigil struct fields are public, we can do it.
        // I made them public in my edit.
        let pattern = vec![(0, 1, Value::Int(5))];
        let sigil = chimera_lang::vm::nova_sigil::Sigil {
            pattern,
            strand_idx: 1,
            auto_cast: false,
        };
        vm.sigil_registry.insert("TestSigil".to_string(), sigil);

        // Step 1: Execute Strand 0 to enable AutoCast
        // 0: Push "TestSigil"
        vm.step();
        // 1: Push 1
        vm.step();
        // 2: AutoCast
        vm.step();

        assert!(vm.sigil_registry.get("TestSigil").unwrap().auto_cast);

        // Step 2: Setup Grid Condition
        // Context is (8,8). Target is (8,9).
        vm.grid[8][9] = Value::Int(5);

        // Step 3: Step VM. Passive scan should trigger.
        vm.step();

        // Verify
        assert!(vm
            .output
            .iter()
            .any(|s| s.contains("AUTO_CAST: Triggered 'TestSigil'")));
        assert!(!vm.organelles.is_empty());
        assert_eq!(vm.organelles[0].ip.0, 1);

        // Also verify the grid consumed the pattern
        assert_eq!(vm.grid[8][9], Value::Int(0));
    }
}
