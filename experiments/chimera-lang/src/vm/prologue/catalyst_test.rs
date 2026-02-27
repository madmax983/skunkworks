use crate::ast::Dna;
use crate::ast::Helix;
use crate::vm::prologue::exec_prologue_tick;
use crate::vm::ChimeraVM;
use crate::vm::Value;
#[cfg(feature = "nova")]
use crate::vm::catalyst::Catalyst;

#[test]
fn test_heat_rune() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Setup:
    // (5,5) = 10 (Heat Source)
    // (5,6) = ♨ (Heat Rune)
    // Expect: (4,6), (5,7), (6,6) to heat up

    vm.grid[5][5] = Value::Int(10);
    vm.grid[5][6] = Value::Str("♨".to_string()); // Source ! is implicit for runes usually, but let's use signal grid directly via Source '!'

    // Correct Circuit:
    // 10 ! ♨
    //  ^  ^
    //(5,4)(5,5)(5,6)

    vm.grid[5][4] = Value::Int(10);
    vm.grid[5][5] = Value::Str("!".to_string());
    vm.grid[5][6] = Value::Str("♨".to_string());

    exec_prologue_tick(&mut vm);
    // Tick 1: ! reads 10, emits to self (5,5)
    // Tick 2 (propagation): ♨ reads 5,5 (West).
    // Emits 10 to N(4,6), E(5,7), S(6,6).

    // Check North
    if let Some(Value::Int(v)) = &vm.prologue_state.signal_grid[4][6] {
        assert_eq!(*v, 10);
    } else {
        panic!("Heat did not propagate North");
    }

    // Check East
    if let Some(Value::Int(v)) = &vm.prologue_state.signal_grid[5][7] {
        assert_eq!(*v, 10);
    } else {
        panic!("Heat did not propagate East");
    }

    // Check South
    if let Some(Value::Int(v)) = &vm.prologue_state.signal_grid[6][6] {
        assert_eq!(*v, 10);
    } else {
        panic!("Heat did not propagate South");
    }
}

#[cfg(feature = "nova")]
#[test]
fn test_catalyze_rune_trigger() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Register a catalyst
    let cat = Catalyst {
        id: 1,
        recipe: vec![],
        charge: 100,
        stability: 1.0,
    };
    vm.catalysts.push(cat);

    // Setup:
    // West (Source): 1 (Catalyst ID)
    // North (Ing): 1
    // South (Ing): 1
    // Rune: Ϡ

    // Grid:
    //   1 (4,6)
    // 1 ! Ϡ (5,4 5,5 5,6)
    //   1 (6,6)

    vm.grid[4][6] = Value::Int(1); // North Ing
    vm.grid[5][4] = Value::Int(1); // Cat ID
    vm.grid[5][5] = Value::Str("!".to_string());
    vm.grid[5][6] = Value::Str("Ϡ".to_string());
    vm.grid[6][6] = Value::Int(1); // South Ing

    // We also need ! sources for North and South to ensure signal is present?
    // Or does Catalyze read GRID?
    // catalyst.rs logic:
    // let n_sig = current_signals[ny][nx]
    // So it reads SIGNALS. We need sources for ingredients too.

    // Revised Setup:
    //    1 (3,6)
    //    ! (4,6)
    // 1 ! Ϡ (5,4 5,5 5,6)
    //    ! (6,6)
    //    1 (7,6)

    vm.grid[3][6] = Value::Int(1);
    vm.grid[4][6] = Value::Str("!".to_string());

    vm.grid[7][6] = Value::Int(1);
    vm.grid[6][6] = Value::Str("!".to_string());

    exec_prologue_tick(&mut vm);

    // Check Trigger Output (East of Ϡ => 5,7)
    // Wait, Apply Sinks runs AFTER prop.
    // Prop emits 1 to East if conditions met.

    if let Some(Value::Int(v)) = &vm.prologue_state.signal_grid[5][7] {
        assert_eq!(*v, 1); // Success signal
    } else {
        panic!("Catalyst trigger failed to emit success signal");
    }

    // Check Sink Effect (Log message)
    let output = vm.output.join("\n");
    assert!(output.contains("CATALYST: Triggered #1"));
}
