use crate::ast::{Dna, Helix};
use crate::vm::prologue::epigenetics::EpigeneticMark;
use crate::vm::prologue::exec_prologue_tick;
use crate::vm::{ChimeraVM, Value};

#[test]
fn test_epigenetic_methylation() {
    let dna = Dna { evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Circuit: 42 -> ! -> ~ -> ?
    // We apply Methylation to the Wire ~ at (6,5)

    vm.grid[5][4] = Value::Int(42);
    vm.grid[5][5] = Value::Str("!".to_string());
    vm.grid[6][5] = Value::Str("~".to_string());
    vm.grid[7][5] = Value::Str("?".to_string());

    // Manual Methylation
    vm.prologue_state.epigenetic_grid[6][5] = EpigeneticMark::Methylated;

    exec_prologue_tick(&mut vm);

    // Wire should NOT have signal because apply_propagation_rune returns early
    assert!(
        vm.prologue_state.signal_grid[6][5].is_none(),
        "Methylated wire should not carry signal"
    );

    // Sink check
    let output = vm.output.join("\n");
    assert!(
        !output.contains("PROLOGUE: Sink"),
        "Sink should not trigger"
    );
}

#[test]
fn test_epigenetic_rune_application() {
    let dna = Dna { evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Setup: 1 -> ! -> .
    // South of . is (6,6).
    // (5,4)=1, (5,5)=!, (5,6)=.

    vm.grid[5][4] = Value::Int(1);
    vm.grid[5][5] = Value::Str("!".to_string());
    vm.grid[5][6] = Value::Str(".".to_string());

    // (6,6) should become Methylated.

    exec_prologue_tick(&mut vm);

    assert_eq!(
        vm.prologue_state.epigenetic_grid[6][6],
        EpigeneticMark::Methylated
    );
}

#[test]
fn test_phosphorylation_amplify() {
    let dna = Dna { evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Circuit: 42 -> ! -> ~ -> ?
    // Phosphorylate the Sink ?

    vm.grid[5][4] = Value::Int(42);
    vm.grid[5][5] = Value::Str("!".to_string());
    vm.grid[6][5] = Value::Str("~".to_string());
    vm.grid[7][5] = Value::Str("?".to_string());

    vm.prologue_state.epigenetic_grid[7][5] = EpigeneticMark::Phosphorylated;

    exec_prologue_tick(&mut vm);

    // Count occurrences in output
    let count = vm
        .output
        .iter()
        .filter(|s| s.contains("PROLOGUE: Sink"))
        .count();
    assert_eq!(count, 2, "Phosphorylated sink should trigger twice");
}
