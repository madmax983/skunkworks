use crate::ast::{Dna, Helix};
use crate::vm::{ChimeraVM, Value};
use crate::vm::prologue::exec_prologue_tick;
#[cfg(feature = "elektra")]
use crate::vm::elektra::update_circuit;

#[test]
fn test_elektra_runes() {
    let dna = Dna {
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

    #[cfg(feature = "elektra")]
    {
        assert_eq!(vm.voltage_grid[5][5], 5.0);
        assert_eq!(vm.resistance_grid[5][5], -1.0); // Source
    }

    // Test 2: Sine (Sense)
    // Voltage at (6,6) = 10.0
    // ∿ at (6,6) -> South
    #[cfg(feature = "elektra")]
    {
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
}

#[test]
#[cfg(feature = "elektra")]
fn test_elektra_components() {
    let dna = Dna {
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // --- Test 1: Diode Forward Bias ---
    // (5,5) Source +
    // (5,6) Diode ▸
    // (5,7) Resistor ♒ (Load)
    // (5,8) Ground _
    vm.grid[5][5] = Value::Str("+".to_string());
    vm.grid[5][6] = Value::Str("▸".to_string());
    vm.grid[5][7] = Value::Str("♒".to_string());
    vm.grid[5][8] = Value::Str("_".to_string());

    // Run Prologue to set up R/V grids
    exec_prologue_tick(&mut vm);

    // Run Physics Loop (multiple times to allow propagation)
    for _ in 0..10 {
        update_circuit(&mut vm);
    }

    // Forward Bias: Diode (Low R) -> Resistor (High R)
    // V_diode (5,6) connects to Source (100) with High Cond, and Resistor (5,7) with Low Cond (0.01).
    // So V_diode ~ 100.
    // V_resistor (5,7) connects to Diode (100) with Low Cond (0.01) and Ground (0) with Low Cond (0.01).
    // So V_resistor ~ 50.
    assert!(vm.voltage_grid[5][7] >= 40.0, "Diode Forward: Resistor voltage too low ({})", vm.voltage_grid[5][7]);

    // --- Test 2: Diode Reverse Bias ---
    // (6,5) Source +
    // (6,6) Diode Left (3) - Block
    // (6,7) Resistor ♒ (Load)
    // (6,8) Ground _

    vm.grid[6][5] = Value::Str("+".to_string());
    vm.grid[6][6] = Value::Str("◂".to_string());
    vm.grid[6][7] = Value::Str("♒".to_string());
    vm.grid[6][8] = Value::Str("_".to_string());
    vm.voltage_grid[6][6] = 0.0;

    exec_prologue_tick(&mut vm);
    for _ in 0..10 {
        update_circuit(&mut vm);
    }

    // Reverse Bias: Diode (High R) -> Resistor (High R) -> Ground
    // Diode node (6,6) floats because both sides are blocked/high resistance.
    // It might float high due to asymmetric leakage.
    // However, Resistor node (6,7) connects to Diode (floating/blocked) with Extremely Low Cond (0.00001 combined)
    // and to Ground with Low Cond (0.01).
    // So Resistor should be pulled firmly to Ground.
    assert!(vm.voltage_grid[6][7] < 10.0, "Diode Reverse: Resistor voltage too high ({})", vm.voltage_grid[6][7]);

    // --- Test 3: Transistor ---
    // (8,5) Source +
    // (8,6) Transistor ¥ (Base North)
    // (8,7) Resistor ♒ (Load)
    // (8,8) Ground _
    // Base at (7,6).

    vm.grid[8][5] = Value::Str("+".to_string());
    vm.grid[8][6] = Value::Str("¥".to_string());
    vm.grid[8][7] = Value::Str("♒".to_string());
    vm.grid[8][8] = Value::Str("_".to_string());
    vm.voltage_grid[8][6] = 0.0;

    // Case A: Base Low (0V)
    vm.grid[7][6] = Value::Int(0);
    exec_prologue_tick(&mut vm);
    for _ in 0..10 { update_circuit(&mut vm); }

    // Check Resistor Node (8,7)
    let v_off = vm.voltage_grid[8][7];
    assert!(v_off < 10.0, "Transistor OFF: Resistor voltage too high ({})", v_off);

    // Case B: Base High (100V)
    vm.grid[7][6] = Value::Str("+".to_string());
    exec_prologue_tick(&mut vm);
    for _ in 0..10 { update_circuit(&mut vm); }

    // Check Resistor Node (8,7)
    let v_on = vm.voltage_grid[8][7];
    assert!(v_on >= 40.0, "Transistor ON: Resistor voltage too low ({})", v_on);
}
