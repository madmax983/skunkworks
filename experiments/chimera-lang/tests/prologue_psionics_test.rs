use chimera_lang::ast::{Dna, Helix};
use chimera_lang::vm::prologue::exec_prologue_tick;
use chimera_lang::vm::{ChimeraVM, Value};

#[test]
fn test_psionics_telepathy() {
    // Θ (Theta) = Telepathy: Reads West (Y), North (X) -> Output Self (Value at grid[Y][X])
    let dna = Dna {
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Setup:
    // Target at (1, 1) = 42
    // Theta at (5, 5)

    vm.grid[1][1] = Value::Int(42);

    // West Input: Provide Y=1 at (5, 4)
    // Source ! at (5, 4) reads West (5, 3)
    vm.grid[5][3] = Value::Int(1);
    vm.grid[5][4] = Value::Str("!".to_string());

    // North Input: Provide X=1 at (4, 5)
    // Source ! at (4, 5) reads West (4, 4)
    vm.grid[4][4] = Value::Int(1);
    vm.grid[4][5] = Value::Str("!".to_string());

    vm.grid[5][5] = Value::Str("Θ".to_string());
    vm.grid[5][6] = Value::Str("~".to_string());

    exec_prologue_tick(&mut vm);

    if let Some(val) = &vm.prologue_state.signal_grid[5][6] {
        if let Value::Int(v) = val {
            assert_eq!(*v, 42);
        } else {
            panic!("Expected Int(42), got {:?}", val);
        }
    } else {
        panic!("Signal did not propagate from Theta");
    }
}

#[test]
fn test_psionics_suggestion() {
    // Σ (Sigma) = Suggestion: West (Val), North (Y), East (X) -> Write Val to grid[Y][X]
    let dna = Dna {
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Setup:
    // Target at (1, 1) = Empty
    // Sigma at (5, 5)

    // West Input: Val=99 at (5, 4)
    vm.grid[5][3] = Value::Int(99);
    vm.grid[5][4] = Value::Str("!".to_string());

    // North Input: Y=1 at (4, 5)
    vm.grid[4][4] = Value::Int(1);
    vm.grid[4][5] = Value::Str("!".to_string());

    // East Input: X=1 at (5, 6)
    // Wire at (5, 6) fed by Source at (6, 6) reading (6, 5)
    vm.grid[5][6] = Value::Str("~".to_string());
    vm.grid[6][6] = Value::Str("!".to_string());
    vm.grid[6][5] = Value::Int(1);

    vm.grid[5][5] = Value::Str("Σ".to_string());

    exec_prologue_tick(&mut vm);

    // Check if grid[1][1] became 99
    match &vm.grid[1][1] {
        Value::Int(v) => assert_eq!(*v, 99),
        val => panic!("Expected Int(99) at (1,1), got {:?}", val),
    }
}

#[test]
fn test_psionics_telekinesis() {
    // Ξ (Xi) = Telekinesis: West (Dir), North (Y), East (X) -> Move grid[Y][X] in Dir
    // Dir: 0=N, 1=E, 2=S, 3=W
    let dna = Dna {
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Setup:
    // Object at (2, 2) = 100
    // Move it East (Dir=1) to (2, 3)
    // Xi at (5, 5)

    vm.grid[2][2] = Value::Int(100);

    // West Input (Dir=1)
    vm.grid[5][3] = Value::Int(1);
    vm.grid[5][4] = Value::Str("!".to_string());

    // North Input (Y=2)
    vm.grid[4][4] = Value::Int(2);
    vm.grid[4][5] = Value::Str("!".to_string());

    // East Input (X=2)
    vm.grid[5][6] = Value::Str("~".to_string());
    vm.grid[6][6] = Value::Str("!".to_string());
    vm.grid[6][5] = Value::Int(2);

    vm.grid[5][5] = Value::Str("Ξ".to_string());

    exec_prologue_tick(&mut vm);

    // Object should move from (2,2) to (2,3) (East)
    match &vm.grid[2][2] {
        Value::Int(0) => {}, // Should be empty/0
        val => panic!("Expected (2,2) to be empty, got {:?}", val),
    }

    match &vm.grid[2][3] {
        Value::Int(100) => {},
        val => panic!("Expected Int(100) at (2,3), got {:?}", val),
    }
}
