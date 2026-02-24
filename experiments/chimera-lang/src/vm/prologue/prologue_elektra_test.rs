use crate::ast::{Dna, Helix};
use crate::vm::prologue::exec_prologue_tick;
use crate::vm::{ChimeraVM, Value};

#[test]
fn test_elektra_runes() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Test 1: Bolt (Source)
    // 5 -> ! -> ⚡
    vm.grid[5][3] = Value::Int(5);
    vm.grid[5][4] = Value::Str("!".to_string());
    vm.grid[5][5] = Value::Str("⚡".to_string());

    exec_prologue_tick(&mut vm);

    assert_eq!(vm.voltage_grid[5][5], 5.0);
    assert_eq!(vm.resistance_grid[5][5], -1.0); // Source

    // Test 2: Sine (Sense)
    // Voltage at (6,6) = 10.0
    // ∿ at (6,6) -> South
    vm.voltage_grid[6][6] = 10.0;
    vm.grid[6][6] = Value::Str("∿".to_string());

    exec_prologue_tick(&mut vm);

    // Should output 10 to South (7,6)
    if let Some(val) = &vm.prologue_state.signal_grid[7][6] {
        assert_eq!(*val, Value::Int(10));
    } else {
        panic!("Sine rune did not sense voltage");
    }
}

#[test]
fn test_bio_voltaics() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;
    vm.energy = 50;
    vm.tick_counter = 1;

    // Test 🔌 Bio-Generator
    // Energy 50 -> Consumes 1 -> Voltage 100.0
    vm.grid[5][5] = Value::Str("🔌".to_string());

    exec_prologue_tick(&mut vm);

    assert_eq!(vm.energy, 49); // Consumed 1
    assert_eq!(vm.voltage_grid[5][5], 100.0);
    assert_eq!(vm.resistance_grid[5][5], -1.0); // Source

    // Verify register was set
    let reg = vm.prologue_state.registers.get(&(5, 5));
    assert_eq!(reg, Some(&Value::Int(1)));

    // Advance tick, add Light 💡
    vm.tick_counter = 2;
    vm.grid[5][6] = Value::Str("💡".to_string());
    // Manually propagate voltage for this test setup (as wire simulation runs separately usually)
    vm.voltage_grid[5][6] = 100.0;

    exec_prologue_tick(&mut vm);

    // Generator consumes 1 (49 -> 48)
    // Light adds 5 (48 -> 53)
    // Note: Generator runs first or second depending on scan order, but both run in same tick.
    // Register prevents double charging if iterated.
    assert_eq!(vm.energy, 53);
    assert_eq!(vm.resistance_grid[5][6], 100.0); // Load

    // Verify registers
    assert_eq!(
        vm.prologue_state.registers.get(&(5, 5)),
        Some(&Value::Int(2))
    );
    assert_eq!(
        vm.prologue_state.registers.get(&(5, 6)),
        Some(&Value::Int(2))
    );
}

#[test]
fn test_advanced_components() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Test Capacitor 🔋
    vm.grid[5][5] = Value::Str("🔋".to_string());
    exec_prologue_tick(&mut vm);
    assert_eq!(vm.capacitance_grid[5][5], 100.0);

    // Test Memristor ♒
    vm.grid[6][6] = Value::Str("♒".to_string());
    exec_prologue_tick(&mut vm);
    assert_eq!(vm.resistance_grid[6][6], 50.0);
}
