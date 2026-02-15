#[cfg(test)]
#[cfg(feature = "nova")]
#[cfg(feature = "resonance")]
mod tests {
    use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::nova::{Organelle, OrganelleType};
    use chimera_lang::vm::nova_signals;
    use chimera_lang::vm::{ChimeraVM, Value};

    fn make_vm() -> ChimeraVM {
        // Use Push(0) so there is an argument to mutate
        let genes = vec![Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        }];
        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        let mut vm = ChimeraVM::new(dna);
        // Ensure phase allows mutation
        vm.phase = chimera_lang::vm::nova::Phase::Corporeal;
        vm
    }

    #[test]
    fn test_mutagenic_resonance() {
        let mut vm = make_vm();

        // 1. Setup Organelle at (5,5)
        let org = Organelle {
            stack: vec![],
            ip: (0, 0),
            context_loc: (5, 5),
            call_stack: vec![],
            recursion_depth: 0,
            halted: false,
            kind: OrganelleType::Worker,
            direction: (0, 0),
            ttl: None,
            name: "Victim".to_string(),
            traits: vec![],
            id: 1,
            tissue_id: None,
            genome_id: 0, energy: 100,
        };
        vm.organelles.push(org);

        // 2. Inject High Pressure at (5,5)
        // audio_snapshot.pressure is 1D vec size 256
        let idx = 5 * 16 + 5;
        if idx < vm.audio_snapshot.pressure.len() {
            vm.audio_snapshot.pressure[idx] = 1.0; // > 0.8 threshold
        }

        // 3. Process Signals
        nova_signals::process_signals(&mut vm);

        // 4. Assert Mutation Log
        let found = vm.output.iter().any(|s| s.contains("MUTATION: Resonance"));
        assert!(found, "Expected mutation log, got: {:?}", vm.output);
    }

    #[test]
    fn test_harmonic_convergence() {
        let mut vm = make_vm();

        // 1. Setup Golden Frequency at (8,8)
        // 161.8 Hz, Amp 50.0
        vm.resonance_grid[8][8] = (161.8, 50.0);

        // 2. Process
        nova_signals::process_signals(&mut vm);

        // 3. Assert Wisp Spawn
        let found = vm
            .output
            .iter()
            .any(|s| s.contains("HARMONIC: Spawned Wisp"));
        assert!(found, "Expected harmonic spawn log, got: {:?}", vm.output);

        // Check organelle count
        assert_eq!(vm.organelles.len(), 1);
        match vm.organelles[0].kind {
            OrganelleType::Wisp => {}
            _ => panic!("Expected Wisp, got {:?}", vm.organelles[0].kind),
        }
    }
}
