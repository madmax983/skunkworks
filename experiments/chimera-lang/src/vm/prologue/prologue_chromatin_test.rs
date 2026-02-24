use crate::ast::{Dna, Helix};
use crate::vm::prologue::epigenetics::EpigeneticMark;
use crate::vm::prologue::exec_prologue_tick;
use crate::vm::{ChimeraVM, Value};

#[test]
fn test_chromatin_silencing() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Setup Circuit: 42 -> ! -> ~
    vm.grid[5][4] = Value::Int(42);
    vm.grid[5][5] = Value::Str("!".to_string());
    vm.grid[6][5] = Value::Str("~".to_string());

    // Verify Source works initially
    exec_prologue_tick(&mut vm);
    assert!(
        vm.prologue_state.signal_grid[6][5].is_some(),
        "Wire should receive signal initially"
    );

    // Spawn Chromatin Agent (χ) at (5, 6) targeted at '!' (at 5, 5)
    // State: [0, "!"] -> 0 = Methylate (Silence)
    // (5, 6) is adjacent to (5, 5)
    vm.grid[5][6] = Value::Str("χ".to_string());
    vm.prologue_state.registers.insert(
        (5, 6),
        Value::Junction(
            crate::ast::JunctionType::All,
            vec![Value::Int(0), Value::Str("!".to_string())],
        ),
    );

    exec_prologue_tick(&mut vm);

    // Check Methylation
    let mark = vm.prologue_state.epigenetic_grid[5][5];
    assert_eq!(
        mark,
        EpigeneticMark::Methylated,
        "Source '!' should be methylated"
    );

    // Run tick - Source should be silenced
    exec_prologue_tick(&mut vm);

    // Check if wire received signal
    // With '!' methylated, it should NOT emit.
    let signal = &vm.prologue_state.signal_grid[6][5];
    assert!(
        signal.is_none(),
        "Wire should NOT receive signal when source is silenced"
    );
}

#[test]
fn test_chromatin_amplifying() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Setup Agent (χ) targeting '&' with Phosphorylate (1)
    vm.grid[5][5] = Value::Str("&".to_string());
    vm.grid[5][6] = Value::Str("χ".to_string());
    vm.prologue_state.registers.insert(
        (5, 6),
        Value::Junction(
            crate::ast::JunctionType::All,
            vec![Value::Int(1), Value::Str("&".to_string())],
        ),
    );

    exec_prologue_tick(&mut vm);

    let mark = vm.prologue_state.epigenetic_grid[5][5];
    assert_eq!(
        mark,
        EpigeneticMark::Phosphorylated,
        "Gate '&' should be phosphorylated"
    );
}
