use crate::ast::{Dna, Helix};
use crate::vm::{ChimeraVM, Value};
use crate::vm::prologue::exec_prologue_tick;

#[test]
fn test_workbench_prototyper() {
    let dna = Dna { evolution_config: None, helix: Helix { strands: vec![] } };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // 1. Define Schematic "Gate" in Library
    // Pattern:
    // 1 0 1
    // 0 & 0
    // 0 1 0
    let schematic = vec![
        vec![Value::Int(1), Value::Int(0), Value::Int(1)],
        vec![Value::Int(0), Value::Str("&".to_string()), Value::Int(0)],
        vec![Value::Int(0), Value::Int(1), Value::Int(0)],
    ];
    vm.prologue_state.schematic_library.insert("Gate".to_string(), schematic);

    // 2. Setup Grid
    // "Gate" -> Π
    vm.grid[5][4] = Value::Str("Gate".to_string());
    vm.grid[5][5] = Value::Str("Π".to_string());
    // Signal Source to trigger Π
    vm.grid[5][6] = Value::Int(1); // Not needed? Prototyper reads West.
    // Wait, Construct Logic:
    // Prototyper: West (Blueprint/String) -> Grid (East)
    // So "Gate" is at West.
    // Logic: apply_construct_sinks -> reads signal_grid.
    // Need a Source to push "Gate" into signal_grid.

    // Circuit: "Gate" -> ! -> ~ -> Π
    vm.grid[5][2] = Value::Str("Gate".to_string());
    vm.grid[5][3] = Value::Str("!".to_string());
    vm.grid[5][4] = Value::Str("~".to_string());
    vm.grid[5][5] = Value::Str("Π".to_string());

    // 3. Execute
    // Tick 1: ! emits "Gate"
    exec_prologue_tick(&mut vm);
    assert!(vm.prologue_state.signal_grid[5][3].is_some());

    // Tick 2: ~ propagates "Gate" to Π
    exec_prologue_tick(&mut vm);
    assert_eq!(vm.prologue_state.signal_grid[5][4], Some(Value::Str("Gate".to_string())));

    // Tick 3: Π consumes "Gate" and pastes schematic to East (Start X+1 = 6)
    // Schematic is 3x3. Center Y is 5.
    // Top-Left of schematic relative to center: Y-1, X+0?
    // construct.rs:
    // let h = rows.len() as i64; (3)
    // let start_y = (y as i64) - (h / 2); (5 - 1 = 4)
    // let start_x = (x as i64) + 1; (5 + 1 = 6)
    // So pastes at (4,6) to (6,8).

    exec_prologue_tick(&mut vm);

    // 4. Verify Paste
    // (4,6) should be 1
    // (5,7) should be &
    assert_eq!(vm.grid[4][6], Value::Int(1));
    assert_eq!(vm.grid[5][7], Value::Str("&".to_string()));
    assert_eq!(vm.grid[6][7], Value::Int(1));
}
