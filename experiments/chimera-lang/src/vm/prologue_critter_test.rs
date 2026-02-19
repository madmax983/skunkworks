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
    let critter = CritterState::new(100, "R".to_string(), 0);
    vm.prologue_state.registers.insert((5, 5), critter.to_value());

    // Run tick
    exec_prologue_tick(&mut vm);

    // After tick, Critter might have moved.
    // We scan grid to find "C".
    let mut found = false;
    for y in 0..16 {
        for x in 0..16 {
            if let Value::Str(s) = &vm.grid[y][x] {
                if s == "C" {
                    found = true;
                    // Check register
                    if let Some(val) = vm.prologue_state.registers.get(&(y, x)) {
                        if let Value::Str(state_str) = val {
                            let new_critter = CritterState::parse(state_str).expect("Failed to parse critter state");
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
fn test_critter_movement() {
    let mut vm = setup_vm();

    // Setup: C at 5,5 with DNA "E" (East)
    vm.grid[5][5] = Value::Str("C".to_string());

    let critter = CritterState::new(100, "E".to_string(), 0);
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
fn test_critter_collision_breed() {
    let mut vm = setup_vm();

    // Setup: C1 at 5,5 (East), C2 at 5,7 (West)
    // They will meet at 5,6? No, 5,5 -> 5,6. 5,7 -> 5,6.
    // If processed sequentially:
    // 5,5 moves to 5,6.
    // 5,7 sees 5,6 occupied by C. Triggers breed.

    vm.grid[5][5] = Value::Str("C".to_string());
    vm.grid[5][7] = Value::Str("C".to_string());

    let c1 = CritterState::new(100, "E".to_string(), 0);
    let c2 = CritterState::new(100, "W".to_string(), 0);

    vm.prologue_state.registers.insert((5, 5), c1.to_value());
    vm.prologue_state.registers.insert((5, 7), c2.to_value());

    exec_prologue_tick(&mut vm);

    // C1 should be at 5,6
    // C2 might have bred and stayed at 5,7? Or moved if blocked?
    // If blocked, it stays at 5,7.

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

    // Depending on spawn location, might overwrite?
    // But breeding spawns in empty neighbor.
    // Expected: C1(5,6), C2(5,7), Child(neighbor of 5,7)
    assert!(count >= 3, "Breeding failed, count: {}", count);
}

#[test]
fn test_critter_eat() {
    let mut vm = setup_vm();

    // Setup: C at 5,5 (East). Food (!) at 5,6.
    vm.grid[5][5] = Value::Str("C".to_string());
    vm.grid[5][6] = Value::Str("!".to_string());

    let c1 = CritterState::new(100, "E".to_string(), 0);
    vm.prologue_state.registers.insert((5, 5), c1.to_value());

    exec_prologue_tick(&mut vm);

    // C should be at 5,6 (Food eaten)
    assert_eq!(vm.grid[5][6], Value::Str("C".to_string()));

    // Check energy increased (started 100, cost 1, gain 20 = 119)
    if let Some(val) = vm.prologue_state.registers.get(&(5, 6)) {
        if let Value::Str(s) = val {
             let state = CritterState::parse(s).unwrap();
             assert!(state.energy > 100, "Critter did not gain energy");
        }
    }
}
