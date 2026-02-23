#[cfg(feature = "nova")]
mod tests {
    use chimera_lang::ast::{Dna, Gene, Helix, Strand};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::{ChimeraVM, MidiEvent};

    fn make_vm() -> ChimeraVM {
        let dna = Dna { evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_holo_sonify() {
        let mut vm = make_vm();
        // Set a hologram point
        // Magnitude = sqrt(10^2 + 0) = 10. Threshold default 0.5.
        // x=5, y=5
        vm.hologram_grid[5][5] = (10.0, 0.0);

        // Execute HoloSonify
        let gene = Gene {
            op: OpCode::HoloSonify,
            args: vec![],
        };
        let dna = Dna { evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes: vec![gene] }],
            },
        };
        vm.dna = dna;

        vm.step();

        assert!(!vm.midi_messages.is_empty(), "Should generate MIDI");
        if let MidiEvent::NoteOn { note, velocity, .. } = vm.midi_messages[0] {
            // (5 + 5*16) % 64 + 36
            // 85 % 64 = 21. 21 + 36 = 57.
            // Wait, GRID_SIZE is 16.
            // x=5, y=5. index = 5 + 5*16 = 85.
            // 85 % 64 = 21. 21+36 = 57 (A3).
            assert_eq!(note, 57);
            assert!(velocity > 0);
        } else {
            panic!("Expected NoteOn");
        }
    }

    #[cfg(feature = "resonance")]
    #[test]
    fn test_cymatic_scan() {
        let mut vm = make_vm();
        // Set audio pressure
        // GRID_SIZE 16 -> 256 cells
        // vm.audio_snapshot.pressure is initialized to 256 zeros in new()
        if vm.audio_snapshot.pressure.len() > 0 {
            vm.audio_snapshot.pressure[0] = 10.0;
        } else {
            // Should not happen with default new(), but purely defensive
            vm.audio_snapshot.pressure = vec![0.0; 256];
            vm.audio_snapshot.pressure[0] = 10.0;
        }

        let gene = Gene {
            op: OpCode::CymaticScan,
            args: vec![],
        };
        let dna = Dna { evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes: vec![gene] }],
            },
        };
        vm.dna = dna;

        vm.step();

        // Check hologram grid
        // index 0 is (0,0)
        let (re, _) = vm.hologram_grid[0][0];
        assert!(re > 0.0, "Should contain audio energy");
        assert_eq!(re, 10.0);
    }
}
