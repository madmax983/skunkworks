#[cfg(all(test, feature = "nova"))]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand, JunctionType};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    #[test]
    fn test_song_of_creation() {
        // Strand 0 (Main):
        //   Harmonize(["Fiat", "Lux"], Strand 1)
        //   Choir(["Fiat", "Lux"])
        // Strand 1 (Effect):
        //   Push(100)

        let chord = vec![
            Nucleotide::String("Fiat".to_string()),
            Nucleotide::String("Lux".to_string())
        ];

        let main_strand = Strand {
            genes: vec![
                // Harmonize
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Junction(JunctionType::All, chord.clone())],
                },
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(1)], // Target strand 1
                },
                Gene {
                    op: OpCode::Harmonize,
                    args: vec![],
                },
                // Choir
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Junction(JunctionType::All, chord)],
                },
                Gene {
                    op: OpCode::Choir,
                    args: vec![],
                }
            ]
        };

        let effect_strand = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(100)],
                }
            ]
        };

        let dna = Dna {
            helix: Helix {
                strands: vec![main_strand, effect_strand],
            },
        };

        let mut vm = ChimeraVM::new(dna);
        vm.energy = 1000;

        // Step 1: Harmonize (Register Chord)
        vm.step(); // Push chord
        vm.step(); // Push 1
        vm.step(); // Harmonize

        assert_eq!(vm.chord_registry.len(), 1);

        // Step 2: Spawn Choir
        vm.step(); // Push song
        vm.step(); // Choir

        assert_eq!(vm.organelles.len(), 1);

        // Step 3: Run Choir
        // Choir needs to sing "Fiat" then "Lux".

        vm.step(); // Tick 1 (Choir: "Fiat")
        assert_eq!(vm.chorus_buffer.back(), Some(&"Fiat".to_string()));

        vm.step(); // Tick 2 (Choir: "Lux")
        // Buffer should be cleared if triggered
        assert_eq!(vm.chorus_buffer.len(), 0);

        // Check if triggered (IP moved to 1)
        assert_eq!(vm.ip.0, 1);
    }

    #[test]
    fn test_song_integration() {
       let chord = vec![
            Nucleotide::String("Fiat".to_string()),
            Nucleotide::String("Lux".to_string())
        ];

        let main_strand = Strand {
            genes: vec![
                // 0: Push Chord
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Junction(JunctionType::All, chord.clone())],
                },
                // 1: Push 1
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(1)], // Target strand 1
                },
                // 2: Harmonize
                Gene {
                    op: OpCode::Harmonize,
                    args: vec![],
                },
                // 3: Push Song
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Junction(JunctionType::All, chord)],
                },
                // 4: Choir
                Gene {
                    op: OpCode::Choir,
                    args: vec![],
                },
                // 5: Jump(0) - Loop
                Gene {
                    op: OpCode::Jump,
                    args: vec![Nucleotide::Number(0)],
                }
            ]
        };

        let effect_strand = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(100)],
                },
                Gene {
                    op: OpCode::Ret,
                    args: vec![],
                }
            ]
        };

        let dna = Dna {
            helix: Helix {
                strands: vec![main_strand, effect_strand],
            },
        };

        let mut vm = ChimeraVM::new(dna);
        vm.energy = 1000;

        // Execute steps
        // 0: Push Chord. (IP -> 0,1) Organelle runs "Fiat" (Wait, organelle not spawned yet)
        vm.step();

        // 1: Push 1. (IP -> 0,2)
        vm.step();

        // 2: Harmonize. (IP -> 0,3)
        vm.step();

        // 3: Push Song. (IP -> 0,4)
        vm.step();

        // 4: Choir. (IP -> 0,5). Organelle Spawned.
        // Process Organelles: Choir ticks. "Fiat".
        vm.step();
        assert_eq!(vm.organelles.len(), 1);
        assert_eq!(vm.chorus_buffer.back(), Some(&"Fiat".to_string()));

        // 5: Jump(0). (IP -> 0,0).
        // Process Organelles: Choir ticks. "Lux".
        // Trigger! IP -> (1,0). CallStack -> [(0,0)]
        vm.step();

        assert_eq!(vm.ip, (1, 0));
        assert_eq!(vm.chorus_buffer.len(), 0);

        // 6: Execute Effect (Push 100). (IP -> 1,1).
        // Organelles: "Fiat".
        vm.step();
        assert_eq!(vm.stack.last(), Some(&Value::Int(100)));

        // 7: Ret. (IP -> 0,0).
        // Organelles: "Lux". Trigger! IP -> (1,0).
        vm.step();
        assert_eq!(vm.ip, (1, 0)); // Re-triggered immediately because song matches again?
    }
}
