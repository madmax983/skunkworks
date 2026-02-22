#[cfg(all(test, feature = "resonance"))]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::ChimeraVM;
    use resonance_audio::audio::AudioCommand;

    fn make_vm(genes: Vec<Gene>) -> ChimeraVM {
        let dna = Dna { evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_oscillate_op_no_channel() {
        // [ push(440) push(100) oscillate() ]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(440)],
            }, // Freq
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(100)],
            }, // Strength
            Gene {
                op: OpCode::Oscillate,
                args: vec![],
            },
        ];

        let mut vm = make_vm(genes);
        let start_energy = vm.energy;

        vm.step(); // push 440
        vm.step(); // push 100
        vm.step(); // oscillate

        // Check output
        let last_out = vm.output.last().unwrap();
        assert!(last_out.contains("OSCILLATE"));
        assert!(last_out.contains("No audio channel"));

        // Check energy
        assert!(vm.energy < start_energy);
    }

    #[test]
    fn test_oscillate_with_channel() {
        use crossbeam_channel::unbounded;

        let (tx, rx) = unbounded();

        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(440)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(50)],
            },
            Gene {
                op: OpCode::Oscillate,
                args: vec![],
            },
        ];
        let mut vm = make_vm(genes);
        vm.set_audio_tx(tx);

        vm.step();
        vm.step();
        vm.step();

        assert!(
            vm.output.last().unwrap().contains("440Hz"),
            "Output was: {:?}",
            vm.output
        );

        // Check channel
        if let Ok(cmd) = rx.try_recv() {
            if let AudioCommand::Oscillate {
                frequency,
                strength,
                ..
            } = cmd
            {
                assert_eq!(frequency, 440.0);
                assert_eq!(strength, 0.5);
            } else {
                panic!("Wrong command received");
            }
        } else {
            panic!("No command received");
        }
    }
}
