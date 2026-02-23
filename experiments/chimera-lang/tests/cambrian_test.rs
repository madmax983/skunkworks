#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::{ChimeraVM, Value};

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_morphogen_emission_and_diffusion() {
        // [ Push(0) Push(100) Morphogen ]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)], // Channel 0
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(100)], // Amount 100
            },
            Gene {
                op: OpCode::Morphogen,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        vm.context_loc = (5, 5);

        // Execute Push(0)
        vm.step();
        // Execute Push(100)
        vm.step();
        // Execute Morphogen
        vm.step();

        // Immediate check: Center should have 100 (written after diffusion of that tick)
        assert_eq!(vm.hormone_grid[5][5][0], 100);

        // Step again to trigger diffusion
        vm.step();

        // With Biome inertia, center retention is substantial.
        assert!(vm.hormone_grid[5][5][0] < 100);
        assert!(vm.hormone_grid[5][5][0] > 10);
        assert!(vm.hormone_grid[5][6][0] > 0);
    }

    #[test]
    fn test_hox_switch() {
        // Strand 0: [ Push(0), Push(50), Push(1), HoxSwitch ]
        // Strand 1: [ Push(999) ] (Target)

        let strand0 = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(0)],
                }, // Channel
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(50)],
                }, // Threshold
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(1)],
                }, // Target Strand
                Gene {
                    op: OpCode::HoxSwitch,
                    args: vec![],
                },
            ],
        };

        let strand1 = Strand {
            genes: vec![Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(999)],
            }],
        };

        let dna = Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![strand0, strand1],
            },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.context_loc = (5, 5);

        // Case 1: Low Hormone -> No Jump
        vm.hormone_grid[5][5][0] = 10;
        vm.step(); // Push 0
        vm.step(); // Push 50
        vm.step(); // Push 1
        vm.step(); // HoxSwitch

        assert_eq!(vm.ip, (0, 4)); // Advanced past genes, didn't jump.
        assert!(vm.stack.is_empty()); // Args popped

        // Case 2: High Hormone -> Jump
        // Reset IP and Stack
        vm.ip = (0, 0);
        vm.stack.clear();

        vm.step(); // Push 0
        vm.step(); // Push 50
        vm.step(); // Push 1

        // Inject hormone right before HoxSwitch to avoid diffusion decay
        vm.hormone_grid[5][5][0] = 1000;

        vm.step(); // HoxSwitch

        // Should have jumped to Strand 1
        assert_eq!(vm.ip, (1, 0));

        // Execute target
        vm.step();
        assert_eq!(vm.stack.pop(), Some(Value::Int(999)));
    }
}
