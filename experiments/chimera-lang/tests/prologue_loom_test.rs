use chimera_lang::prelude::*;
use chimera_lang::vm::prologue;

#[test]
fn test_loom_push_to_warp() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Setup:
    // (5, 5) -> Shuttle (Payload 10)
    // (5, 6) -> ( (Tension Up)
    // (5, 7) -> 8 (Knot)
    // (2, 7) -> ║ (Warp)

    vm.grid[5][5] = Value::Str("ð".to_string());
    // Initial State: [0, 1, Payload=10, Underfoot=0, Tension=0]
    vm.prologue_state.registers.insert(
        (5, 5),
        Value::Junction(
            JunctionType::All,
            vec![
                Value::Int(0),
                Value::Int(1),
                Value::Int(10),
                Value::Int(0),
                Value::Int(0),
            ],
        ),
    );

    vm.grid[5][6] = Value::Str("(".to_string());
    vm.grid[5][7] = Value::Str("8".to_string());
    vm.grid[2][7] = Value::Str("║".to_string());
    // Warp starts empty (0)
    vm.prologue_state.registers.insert((2, 7), Value::Int(0));

    // Tick 1: Shuttle moves to (5, 6). Picks up '('.
    prologue::exec_prologue_tick(&mut vm);

    // Verify Shuttle at (5, 6)
    assert_eq!(vm.grid[5][6], Value::Str("ð".to_string()));

    // Check Agent State (Underfoot='(', Tension=0)
    let state = vm.prologue_state.registers.get(&(5, 6)).unwrap();
    if let Value::Junction(_, list) = state {
        assert_eq!(list[3], Value::Str("(".to_string())); // Underfoot
        assert_eq!(list[4], Value::Int(0)); // Tension is not applied until processed
    } else {
        panic!("Invalid state format");
    }

    // Tick 2: Shuttle processes '(', Tension -> 1. Moves to (5, 7). Picks up '8'.
    prologue::exec_prologue_tick(&mut vm);

    // Verify Shuttle at (5, 7)
    assert_eq!(vm.grid[5][7], Value::Str("ð".to_string()));

    // Check Agent State (Underfoot='8', Tension=1)
    let state = vm.prologue_state.registers.get(&(5, 7)).unwrap();
    if let Value::Junction(_, list) = state {
        assert_eq!(list[3], Value::Str("8".to_string())); // Underfoot
        assert_eq!(list[4], Value::Int(1)); // Tension applied
    }

    // Tick 3: Shuttle processes '8'. Push Payload (10). Moves to (5, 8).
    prologue::exec_prologue_tick(&mut vm);

    // Verify Warp at (2, 7) has 10
    let warp_val = vm.prologue_state.registers.get(&(2, 7)).unwrap();
    assert_eq!(*warp_val, Value::Int(10));
}

#[test]
fn test_loom_pull_from_warp() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Setup:
    // (5, 5) -> Shuttle (Payload 0)
    // (5, 6) -> ) (Tension Down)
    // (5, 7) -> 8 (Knot)
    // (2, 7) -> ║ (Warp with 42)

    vm.grid[5][5] = Value::Str("ð".to_string());
    // Initial State: [0, 1, Payload=0, Underfoot=0, Tension=0]
    vm.prologue_state.registers.insert(
        (5, 5),
        Value::Junction(
            JunctionType::All,
            vec![
                Value::Int(0),
                Value::Int(1),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ],
        ),
    );

    vm.grid[5][6] = Value::Str(")".to_string());
    vm.grid[5][7] = Value::Str("8".to_string());
    vm.grid[2][7] = Value::Str("║".to_string());
    vm.prologue_state.registers.insert((2, 7), Value::Int(42));

    // Tick 1: Shuttle moves to (5, 6). Picks up ')'.
    prologue::exec_prologue_tick(&mut vm);

    // Verify Shuttle at (5, 6)
    assert_eq!(vm.grid[5][6], Value::Str("ð".to_string()));

    // Check Agent State (Underfoot=')', Tension=0)
    let state = vm.prologue_state.registers.get(&(5, 6)).unwrap();
    if let Value::Junction(_, list) = state {
        assert_eq!(list[4], Value::Int(0));
    }

    // Tick 2: Shuttle processes ')', Tension -> -1. Moves to (5, 7). Picks up '8'.
    prologue::exec_prologue_tick(&mut vm);

    // Verify Shuttle at (5, 7)
    assert_eq!(vm.grid[5][7], Value::Str("ð".to_string()));

    // Check Agent State (Underfoot='8', Tension=-1)
    let state = vm.prologue_state.registers.get(&(5, 7)).unwrap();
    if let Value::Junction(_, list) = state {
        assert_eq!(list[4], Value::Int(-1));
    }

    // Tick 3: Shuttle processes '8'. Pulls Warp (42). Moves to (5, 8).
    prologue::exec_prologue_tick(&mut vm);

    // Check Shuttle Payload (should be 42) at (5, 8)
    let state = vm.prologue_state.registers.get(&(5, 8)).unwrap();
    if let Value::Junction(_, list) = state {
        assert_eq!(list[2], Value::Int(42)); // Payload
    } else {
        panic!("Invalid state format");
    }
}
