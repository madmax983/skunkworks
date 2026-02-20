use crate::ast::{Dna, Helix};
use crate::vm::prologue::exec_prologue_tick;
use crate::vm::{ChimeraVM, Value};

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
