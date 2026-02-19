use crate::ast::{Dna, Helix};
use crate::vm::prologue::exec_prologue_tick;
use crate::vm::prologue::critter::CritterState;
use crate::vm::{ChimeraVM, Value};

fn setup_vm() -> ChimeraVM {
    let dna = Dna {
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;
    vm
}

#[test]
fn test_critter_persistence() {
    let mut vm = setup_vm();

    // Setup: C at 5,5
    vm.grid[5][5] = Value::Str("C".to_string());

    // Set initial state
    let critter = CritterState::new(100, "R".to_string(), 0, 0); // Direction 0 (N)
    vm.prologue_state.registers.insert((5, 5), critter.to_value());

    // Run tick
    exec_prologue_tick(&mut vm);

    // After tick, Critter might have moved.
    let mut found = false;
    for y in 0..16 {
        for x in 0..16 {
            if let Value::Str(s) = &vm.grid[y][x] {
                if s == "C" {
                    found = true;
                    // Check register
                    if let Some(val) = vm.prologue_state.registers.get(&(y, x)) {
                        if let Value::Str(state_str) = val {
                            let new_critter: CritterState = state_str.parse().expect("Failed to parse critter state");
                            assert!(new_critter.energy < 100, "Energy should decay");
                        } else {
                            panic!("Register value not a string");
                        }
                    } else {
                        panic!("Register missing for Critter at {},{}", y, x);
                    }
                }
            }
        }
    }
    assert!(found, "Critter disappeared");
}

#[test]
fn test_critter_movement_legacy() {
    let mut vm = setup_vm();

    // Setup: C at 5,5 with DNA "E" (East)
    vm.grid[5][5] = Value::Str("C".to_string());

    let critter = CritterState::new(100, "E".to_string(), 0, 0);
    vm.prologue_state.registers.insert((5, 5), critter.to_value());

    exec_prologue_tick(&mut vm);

    // Should be at 5,6
    assert_eq!(vm.grid[5][5], Value::Int(0), "Old position not cleared");
    assert_eq!(vm.grid[5][6], Value::Str("C".to_string()), "New position not occupied");

    // Register should move
    assert!(vm.prologue_state.registers.get(&(5, 5)).is_none());
    assert!(vm.prologue_state.registers.get(&(5, 6)).is_some());
}

#[test]
fn test_critter_movement_relative() {
    let mut vm = setup_vm();

    // Setup: C at 5,5 facing East (1). Gene 'F' should move East (5,6).
    vm.grid[5][5] = Value::Str("C".to_string());
    let critter = CritterState::new(100, "F".to_string(), 0, 1);
    vm.prologue_state.registers.insert((5, 5), critter.to_value());

    exec_prologue_tick(&mut vm);

    assert_eq!(vm.grid[5][6], Value::Str("C".to_string()));
}

#[test]
fn test_critter_turn() {
    let mut vm = setup_vm();

    // Setup: C at 5,5 facing North (0). Gene 'R' (Turn Right -> East 1).
    vm.grid[5][5] = Value::Str("C".to_string());
    let critter = CritterState::new(100, "R".to_string(), 0, 0);
    vm.prologue_state.registers.insert((5, 5), critter.to_value());

    exec_prologue_tick(&mut vm);

    // Should still be at 5,5
    assert_eq!(vm.grid[5][5], Value::Str("C".to_string()));

    // Check direction
    if let Some(Value::Str(s)) = vm.prologue_state.registers.get(&(5, 5)) {
        let c: CritterState = s.parse().unwrap();
        assert_eq!(c.direction, 1, "Did not turn Right to East");
    }
}

#[test]
fn test_critter_collision_breed() {
    let mut vm = setup_vm();

    // Setup: C1 at 5,5 (East), C2 at 5,7 (West)
    vm.grid[5][5] = Value::Str("C".to_string());
    vm.grid[5][7] = Value::Str("C".to_string());

    let c1 = CritterState::new(100, "E".to_string(), 0, 0);
    let c2 = CritterState::new(100, "W".to_string(), 0, 0);

    vm.prologue_state.registers.insert((5, 5), c1.to_value());
    vm.prologue_state.registers.insert((5, 7), c2.to_value());

    exec_prologue_tick(&mut vm);

    // Check if we have 3 critters now
    let mut count = 0;
    for y in 0..16 {
        for x in 0..16 {
            if let Value::Str(s) = &vm.grid[y][x] {
                if s == "C" {
                    count += 1;
                }
            }
        }
    }
    assert!(count >= 3, "Breeding failed, count: {}", count);
}

#[test]
fn test_critter_eat() {
    let mut vm = setup_vm();

    // Setup: C at 5,5 (East). Food (!) at 5,6.
    vm.grid[5][5] = Value::Str("C".to_string());
    vm.grid[5][6] = Value::Str("!".to_string());

    let c1 = CritterState::new(100, "F".to_string(), 0, 1); // Facing East, move Forward
    vm.prologue_state.registers.insert((5, 5), c1.to_value());

    exec_prologue_tick(&mut vm);

    // C should be at 5,6 (Food eaten)
    assert_eq!(vm.grid[5][6], Value::Str("C".to_string()));

    // Check energy increased
    if let Some(val) = vm.prologue_state.registers.get(&(5, 6)) {
        if let Value::Str(s) = val {
             let state: CritterState = s.parse().unwrap();
             assert!(state.energy > 100, "Critter did not gain energy");
        }
    }
}

#[test]
fn test_critter_split() {
    let mut vm = setup_vm();

    // Setup: C at 5,5. Energy 100. Gene 'S'.
    vm.grid[5][5] = Value::Str("C".to_string());
    let c1 = CritterState::new(100, "S".to_string(), 0, 0);
    vm.prologue_state.registers.insert((5, 5), c1.to_value());

    exec_prologue_tick(&mut vm);

    // Should have split. One parent, one child nearby.
    let mut count = 0;
    for y in 0..16 {
        for x in 0..16 {
            if let Value::Str(s) = &vm.grid[y][x] {
                if s == "C" {
                    count += 1;
                }
            }
        }
    }
    assert!(count >= 2, "Split failed, count: {}", count);
}

#[test]
fn test_critter_attack() {
    let mut vm = setup_vm();

    // Setup: Predator at 5,5 (East). Prey at 5,6.
    vm.grid[5][5] = Value::Str("C".to_string());
    vm.grid[5][6] = Value::Str("C".to_string()); // Another critter

    let pred = CritterState::new(100, "A".to_string(), 0, 1); // Face East, Attack
    let prey = CritterState::new(100, "R".to_string(), 0, 0);

    vm.prologue_state.registers.insert((5, 5), pred.to_value());
    vm.prologue_state.registers.insert((5, 6), prey.to_value());

    exec_prologue_tick(&mut vm);

    // Predator should be at 5,5 (Attack doesn't move). Prey should be gone (0).
    assert_eq!(vm.grid[5][5], Value::Str("C".to_string()), "Predator moved?");
    assert_eq!(vm.grid[5][6], Value::Int(0), "Prey survived");

    // Check energy
    if let Some(val) = vm.prologue_state.registers.get(&(5, 5)) {
        if let Value::Str(s) = val {
             let state: CritterState = s.parse().unwrap();
             assert!(state.energy > 100, "Predator did not gain energy");
        }
    }
}
