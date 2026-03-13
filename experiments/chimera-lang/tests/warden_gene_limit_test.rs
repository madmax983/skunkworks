#[cfg(feature = "nova")]
use chimera_lang::ast::{Dna, Gene, Helix, Strand};
#[cfg(feature = "nova")]
use chimera_lang::opcode::OpCode;
#[cfg(feature = "nova")]
use chimera_lang::vm::{ChimeraVM, Value, MAX_GENES_PER_STRAND};

#[cfg(feature = "nova")]
fn make_simple_dna() -> Dna {
    Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    }
}

#[cfg(feature = "nova")]
#[test]
fn test_ligase_dos_protection() {
    let mut vm = ChimeraVM::new(make_simple_dna());
    vm.energy = 1000000; // Infinite energy

    // Create two strands
    // Strand 0: Length 100
    let mut genes0 = Vec::new();
    for _ in 0..100 {
        genes0.push(Gene {
            op: OpCode::Nop,
            args: vec![],
        });
    }
    vm.dna.helix.strands.push(Strand { genes: genes0 });
    vm.telomeres.push(100);

    // Strand 1: Length MAX (almost)
    let mut genes1 = Vec::new();
    for _ in 0..MAX_GENES_PER_STRAND {
        genes1.push(Gene {
            op: OpCode::Nop,
            args: vec![],
        });
    }
    vm.dna.helix.strands.push(Strand { genes: genes1 });
    vm.telomeres.push(100);

    // Try to Ligase Strand 0 into Strand 1 (Append 0 to 1)
    // Result length would be MAX + 100 > MAX
    // stack: donor_idx (0), recipient_idx (1)
    vm.stack.push(Value::Int(1)); // recipient
    vm.stack.push(Value::Int(0)); // donor

    // Execute Ligase
    // We call the inner execution directly to simulate the op
    chimera_lang::vm::nova_genetics::exec_ligase(&mut vm);

    // Check if it failed safely
    // The recipient strand (1) should NOT have grown
    assert_eq!(
        vm.dna.helix.strands[1].genes.len(),
        MAX_GENES_PER_STRAND,
        "Ligase should not allow exceeding MAX_GENES_PER_STRAND"
    );

    // Check output for error
    assert!(
        vm.output
            .iter()
            .any(|s| s.contains("Strand limit exceeded") || s.contains("Gene Limit Exceeded")),
        "Should log limit error"
    );
}

#[cfg(feature = "nova")]
#[test]
fn test_frankenstein_dos_protection() {
    let mut vm = ChimeraVM::new(make_simple_dna());
    vm.energy = 1000000;

    // Create strands for stitching
    // Target total > MAX
    let mut genes = Vec::new();
    let half_max = MAX_GENES_PER_STRAND / 2 + 100;
    for _ in 0..half_max {
        genes.push(Gene {
            op: OpCode::Nop,
            args: vec![],
        });
    }
    vm.dna.helix.strands.push(Strand {
        genes: genes.clone(),
    }); // 0
    vm.dna.helix.strands.push(Strand { genes }); // 1
    vm.telomeres.push(100);
    vm.telomeres.push(100);

    // Frankenstein: stitches=1, strand_b=1, strand_a=0
    // Result roughly sum of both
    vm.stack.push(Value::Int(0)); // A
    vm.stack.push(Value::Int(1)); // B
    vm.stack.push(Value::Int(1)); // Stitches

    chimera_lang::vm::nova_genetics::exec_frankenstein(&mut vm);

    // Should fail. The fix should prevent creation of the new strand.
    // If successful, a new strand would be added at index 2.
    // BUT the fix actually returns None, so no new strand should be added.
    if vm.dna.helix.strands.len() > 2 {
        // If we are here, a strand WAS created.
        let new_len = vm.dna.helix.strands[2].genes.len();
        // This assertion panics if the length > MAX.
        // If the fix works, we should not be here IF the length exceeds max.
        // But in this test case, we construct inputs specifically to exceed max.
        // So if we are here, it means the check failed to prevent creation OR it truncated (which it shouldn't).
        if new_len > MAX_GENES_PER_STRAND {
            // We want this test to PASS if the implementation prevents this.
            // But if we are in this block, the implementation FAILED to prevent it.
            panic!(
                "Frankenstein result {} exceeded limit {}",
                new_len, MAX_GENES_PER_STRAND
            );
        }
    } else {
        // Failed to create strand (safe) - This is the EXPECTED outcome for this test case.
        assert!(
            vm.output.iter().any(|s| s.contains("Strand limit exceeded")
                || s.contains("Result length")
                || s.contains("Gene Limit Exceeded")),
            "Should log limit error"
        );
    }
}
