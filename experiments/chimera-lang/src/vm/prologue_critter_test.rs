use crate::vm::prologue::critter::{CritterState, process_critter_tick, CritterAction};
use crate::vm::prologue::exec_prologue_tick;
use crate::ast::{Dna, Helix};
use crate::vm::{ChimeraVM, Value, GRID_SIZE};

fn setup_vm() -> ChimeraVM {
    let dna = Dna {
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;
    vm
}

#[test]
fn test_critter_state_parse() {
    let s = "C:100:FRL:0:1";
    let c = CritterState::parse(s).expect("Failed to parse");
    assert_eq!(c.energy, 100);
    assert_eq!(c.genes, "FRL");
    assert_eq!(c.ip, 0);
    assert_eq!(c.dir, 1);

    assert_eq!(c.to_string(), s);
}

#[test]
fn test_critter_tick_move() {
    // 16x16 grid
    let grid = vec![vec![Value::Int(0); 16]; 16];

    // Critter at 5,5 facing East (1)
    let mut c = CritterState::new(100, "F".to_string(), 0, 1);

    let action = process_critter_tick(&mut c, 5, 5, &grid);

    match action {
        CritterAction::Move(y, x) => {
            assert_eq!(y, 5);
            assert_eq!(x, 6);
        }
        _ => panic!("Expected Move action"),
    }

    assert_eq!(c.energy, 99);
}

#[test]
fn test_critter_tick_turn() {
    let grid = vec![vec![Value::Int(0); 16]; 16];

    // Critter at 5,5 facing North (0), Gene "R" (Turn Right)
    let mut c = CritterState::new(100, "R".to_string(), 0, 0);

    let action = process_critter_tick(&mut c, 5, 5, &grid);

    assert_eq!(action, CritterAction::None);
    assert_eq!(c.dir, 1); // North (0) -> East (1)
}

#[test]
fn test_critter_tick_eat() {
    let mut grid = vec![vec![Value::Int(0); 16]; 16];
    grid[5][6] = Value::Str("!".to_string()); // Food East

    // Critter at 5,5 facing East (1), Gene "E"
    let mut c = CritterState::new(100, "E".to_string(), 0, 1);

    let action = process_critter_tick(&mut c, 5, 5, &grid);

    match action {
        CritterAction::Eat(y, x) => {
            assert_eq!(y, 5);
            assert_eq!(x, 6);
        }
        _ => panic!("Expected Eat action"),
    }
}

#[test]
fn test_integration_move() {
    let mut vm = setup_vm();

    // Setup C at 5,5 with Gene F
    vm.grid[5][5] = Value::Str("C".to_string());
    let c = CritterState::new(100, "F".to_string(), 0, 1); // East
    vm.prologue_state.registers.insert((5, 5), c.to_value());

    exec_prologue_tick(&mut vm);

    // Should be at 5,6
    assert_eq!(vm.grid[5][5], Value::Int(0));
    assert_eq!(vm.grid[5][6], Value::Str("C".to_string()));

    // Check register moved
    assert!(vm.prologue_state.registers.get(&(5, 5)).is_none());
    assert!(vm.prologue_state.registers.get(&(5, 6)).is_some());
}

#[test]
fn test_integration_split() {
    let mut vm = setup_vm();

    // Setup C at 5,5 with Gene S (Split)
    // S lays egg BEHIND. Facing East (1), Behind is West (5,4).
    vm.grid[5][5] = Value::Str("C".to_string());
    let c = CritterState::new(100, "S".to_string(), 0, 1);
    vm.prologue_state.registers.insert((5, 5), c.to_value());

    exec_prologue_tick(&mut vm);

    // Parent should still be at 5,5 (Split doesn't move)
    assert_eq!(vm.grid[5][5], Value::Str("C".to_string()));

    // Child should be at 5,4
    assert_eq!(vm.grid[5][4], Value::Str("C".to_string()));

    // Parent energy reduced
    if let Value::Str(s) = vm.prologue_state.registers.get(&(5, 5)).unwrap() {
        let p_state = CritterState::parse(s).unwrap();
        assert!(p_state.energy < 100);
    } else {
        panic!("Parent register is not a string");
    }
}
