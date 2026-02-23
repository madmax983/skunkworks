#[cfg(feature = "nova")]
#[test]
fn test_egregore_evolution() {
    use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::ChimeraVM;

    // Setup: Create a VM with a strand that Prays
    // Initial energy is 50. Praying 10 costs 10.
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(10)],
        }, // Amount
        Gene {
            op: OpCode::Pray,
            args: vec![],
        },
    ];

    let dna = Dna { evolution_config: None,
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    };

    let mut vm = ChimeraVM::new(dna);

    // Initial state
    assert_eq!(vm.egregore.alignment, 0);
    assert_eq!(vm.egregore.faith, 0);

    // Run
    vm.step(); // Push
    vm.step(); // Pray

    // Verify
    assert!(
        vm.egregore.alignment > 0,
        "Alignment should increase (Order)"
    );
    assert_eq!(vm.egregore.faith, 10, "Faith should increase");
    // Start 50 - 1 (step) - 1 (step) - 10 (pray) = 38
    assert!(vm.energy > 0, "Energy should remain");
}

#[cfg(feature = "nova")]
#[test]
fn test_egregore_sacrifice() {
    use chimera_lang::ast::{Dna, Gene, Helix, Strand};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::ChimeraVM;

    // Setup: Strand that Sacrifices itself
    let genes = vec![Gene {
        op: OpCode::Sacrifice,
        args: vec![],
    }];

    let dna = Dna { evolution_config: None,
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    };

    let mut vm = ChimeraVM::new(dna);

    vm.step(); // Sacrifice

    assert!(
        vm.egregore.alignment < 0,
        "Alignment should decrease (Chaos)"
    );
    assert!(
        vm.halted,
        "VM should halt after sacrifice (if last strand died)"
    );
    assert!(
        vm.dna.helix.strands[0].genes.is_empty(),
        "Strand should be empty"
    );
}
