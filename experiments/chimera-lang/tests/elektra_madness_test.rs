#[cfg(all(feature = "elektra", feature = "nova"))]
mod elektra_madness {
    use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::{ChimeraVM, Value};

    fn make_vm(genes: Vec<Gene>) -> ChimeraVM {
        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_tesla_coil_fries_organelles() {
        // [ push(50) push(2) tesla_coil ]
        // Power 50, Radius 2.
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(50)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(2)],
            },
            Gene {
                op: OpCode::TeslaCoil,
                args: vec![],
            },
        ];

        let mut vm = make_vm(genes);

        // Setup: Place an organelle at (8, 9) (Radius 2 from default (8,8))
        // We need to manually add an organelle since Spawn is complex to setup via genes in a short test
        // But vm.organelles is pub.

        // We need to use the Organelle struct. It's likely in `chimera_lang::vm::nova::Organelle`.
        // But `nova` module might not be fully pub re-exported?
        // `use chimera_lang::vm::nova::Organelle` should work if `pub mod nova` in `lib.rs` / `vm/mod.rs`.

        use chimera_lang::vm::nova::{Organelle, OrganelleType};

        let target_loc = (9, 8); // y=9, x=8. Distance 1.

        let org = Organelle {
            stack: vec![],
            ip: (0, 0),
            context_loc: target_loc,
            call_stack: vec![],
            recursion_depth: 0,
            halted: false,
            kind: OrganelleType::Worker,
            direction: (0, 1),
            ttl: None,
            name: "TestSubject".to_string(),
            traits: vec![],
            id: 999,
            tissue_id: None,
            genome_id: 0,
            energy: 100,
        };

        vm.organelles.push(org);

        // Charge the battery!
        // Default context is (8, 8).
        vm.voltage_grid[8][8] = 100.0;
        vm.resistance_grid[8][8] = -1.0; // Mark as Battery to prevent decay

        vm.step(); // Execute Push
        vm.step(); // Execute Push
        vm.step(); // Execute TeslaCoil

        // Check output
        let output = vm.output.join("\n");
        println!("{}", output);
        assert!(output.contains("TESLA COIL: Discharging 50"));
        assert!(output.contains("TESLA COIL: Fried 1 organelles"));

        // Check organelle status
        // Since it halted, process_organelles removes it from the list!
        assert!(vm.organelles.is_empty());

        // Check voltage consumed
        // Should be 100.0 - 50.0 = 50.0
        assert_eq!(vm.voltage_grid[8][8], 50.0);
    }

    #[test]
    fn test_galvanize_resurrection() {
        // [ push(0) galvanize ]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                op: OpCode::Galvanize,
                args: vec![],
            },
        ];

        let mut vm = make_vm(genes);

        // Setup Graveyard
        let dead_gene = Gene {
            op: OpCode::Print,
            args: vec![Nucleotide::String("BRAINS".to_string())],
        };
        let corpse = Strand {
            genes: vec![dead_gene],
        };
        vm.graveyard.push(corpse);

        // Apply High Voltage
        vm.voltage_grid[8][8] = 200.0;
        vm.resistance_grid[8][8] = -1.0; // Mark as Battery to prevent decay

        vm.step(); // Push 0
        vm.step(); // Galvanize

        let output = vm.output.join("\n");
        println!("{}", output);
        assert!(output.contains("GALVANIZE: IT'S ALIVE!"));

        // Check DNA
        // Original 1 strand + 1 resurrected = 2
        assert_eq!(vm.dna.helix.strands.len(), 2);

        // Check Voltage Discharge
        // Since we marked it as battery, it might NOT discharge to 0.0 unless Galvanize forces it?
        // In my implementation: vm.voltage_grid[y][x] = 0.0; // Discharge
        // But update_circuit runs AFTER gene execution in step().
        // If resistance is -1.0, update_circuit will KEEP it at what it was at start of loop?
        // No, update_circuit copies old to new if R < 0.
        // If Galvanize sets V to 0, but update_circuit runs next step...
        // Wait, Galvanize runs in step(), THEN update_circuit runs.
        // If Galvanize sets V=0.
        // Then update_circuit runs.
        // If R=-1.0, update_circuit takes V from *current grid*.
        // So if Galvanize set it to 0, update_circuit sees 0 and keeps it 0?
        // Yes.
        assert_eq!(vm.voltage_grid[8][8], 0.0);

        // Check Stack: Should have new strand index (1)
        assert_eq!(vm.stack.pop(), Some(Value::Int(1)));

        // Graveyard should be empty
        assert!(vm.graveyard.is_empty());
    }
}
