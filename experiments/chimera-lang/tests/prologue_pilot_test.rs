use chimera_lang::ast::{Dna, Helix};
use chimera_lang::vm::prologue::exec_prologue_tick;
use chimera_lang::vm::{ChimeraVM, Value};

fn create_vm() -> ChimeraVM {
    let dna = Dna {
        helix: Helix { strands: vec![] },
        evolution_config: None,
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;
    vm
}

#[test]
fn test_pilot_movement_east() {
    let mut vm = create_vm();
    vm.grid[5][5] = Value::Str("⚓".to_string());

    exec_prologue_tick(&mut vm);

    // After 1 tick, should have moved to (5, 6)
    let agent = &vm.prologue_state.agents[0];
    assert_eq!(agent.x, 6, "Agent X should be 6");
    assert_eq!(agent.y, 5, "Agent Y should be 5");
    assert_eq!(vm.grid[5][5], Value::Int(0), "Old pos should be cleared");
    assert_eq!(vm.grid[5][6], Value::Str("⚓".to_string()), "New pos should be occupied");
}

#[test]
fn test_pilot_direction_change() {
    let mut vm = create_vm();
    vm.grid[5][5] = Value::Str("⚓".to_string());
    vm.grid[5][6] = Value::Str("S".to_string());

    // Tick 1: Move to 5,6. Captures 'S' as underfoot.
    exec_prologue_tick(&mut vm);

    assert_eq!(vm.grid[5][6], Value::Str("⚓".to_string()));
    let agent = &vm.prologue_state.agents[0];
    assert_eq!(agent.x, 6);

    // Tick 2: Execute 'S'. Move South to 6,6. Restores 'S' to 5,6.
    exec_prologue_tick(&mut vm);

    assert_eq!(vm.grid[5][6], Value::Str("S".to_string()));
    assert_eq!(vm.grid[6][6], Value::Str("⚓".to_string()));
}

#[test]
fn test_pilot_arithmetic() {
    let mut vm = create_vm();

    // 5 -> 3 -> + -> !
    vm.grid[5][5] = Value::Str("⚓".to_string());
    vm.grid[5][6] = Value::Int(5);
    vm.grid[5][7] = Value::Int(3);
    vm.grid[5][8] = Value::Str("+".to_string());
    vm.grid[5][9] = Value::Str("!".to_string());

    // Tick 1: Move to 5,6 (Capture 5)
    exec_prologue_tick(&mut vm);

    // Tick 2: Exec 5. Move to 5,7 (Capture 3)
    exec_prologue_tick(&mut vm);

    // Tick 3: Exec 3. Move to 5,8 (Capture +)
    exec_prologue_tick(&mut vm);

    // Tick 4: Exec +. Move to 5,9 (Capture !)
    exec_prologue_tick(&mut vm);

    // Tick 5: Exec !. Emit 8. Move to 5,10.
    exec_prologue_tick(&mut vm);

    if let Some(Value::Int(sig)) = &vm.prologue_state.signal_grid[5][9] {
        assert_eq!(*sig, 8);
    } else {
         panic!("Signal Grid at 5,9 is {:?}", vm.prologue_state.signal_grid[5][9]);
    }
}

#[test]
fn test_pilot_collision() {
    let mut vm = create_vm();
    vm.grid[5][5] = Value::Str("⚓".to_string());
    vm.grid[5][6] = Value::Str("@".to_string()); // Blocked by Seeker

    exec_prologue_tick(&mut vm);

    // Should be blocked. Still at (5, 5).
    assert_eq!(vm.grid[5][5], Value::Str("⚓".to_string()));
    let agent = &vm.prologue_state.agents[0];
    assert_eq!(agent.x, 5);
}
