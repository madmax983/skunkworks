use chimera_lang::compiler::compile;
use chimera_lang::vm::ChimeraVM;

#[test]
#[cfg(feature = "nova")]
fn test_crispr_syntax_compile_and_run() {
    let source = r#"
    strand target {
        push(1) push(2) add
    }

    strand main {
        crispr(target) {
            pattern: push, push, add

            # Stack: [match_index]
            # If match_index is -1, swap/cas9_cut might fail or do weird things.
            # But here we know it matches at 0.

            # Setup for cas9_cut: [strand_idx, cut_idx]
            # Stack currently: [0]

            target # Push target index (0). Stack: [0, 0]
            swap   # Stack: [0, 0]

            cas9_cut

            # Stack: [new_strand_idx]
            "Cut done" print
        }
    }
    "#;

    let dna = compile(source, None).expect("Compilation failed");
    let mut vm = ChimeraVM::new(dna);

    // Ensure we start at main (index 1)
    // By default VM starts at (0,0).
    // But index 0 is 'target'. We want to run 'main'.
    // We should jump to main or set IP.
    vm.ip = (1, 0);

    // Run until halted
    let mut steps = 0;
    while !vm.halted && steps < 100 {
        vm.step();
        steps += 1;
    }

    // Verify output
    println!("VM Output: {:?}", vm.output);
    assert!(vm.output.iter().any(|s| s.contains("Cut done")), "Did not find 'Cut done'");
    assert!(vm.output.iter().any(|s| s.contains("CRISPR_SCAN")), "Did not find 'CRISPR_SCAN'");
    assert!(vm.output.iter().any(|s| s.contains("CAS9_CUT")), "Did not find 'CAS9_CUT'");

    // Verify strand count increased (original + target + main + new_tail)
    // Target: 0
    // Main: 1
    // Guide: 2 (Anonymous)
    // New Tail: 3
    assert_eq!(vm.dna.helix.strands.len(), 4, "Expected 4 strands (Target, Main, Guide, Tail)");
}
