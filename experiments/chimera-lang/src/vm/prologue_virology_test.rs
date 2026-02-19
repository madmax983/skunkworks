use crate::ast::{Dna, Helix};
use crate::vm::prologue::exec_prologue_tick;
use crate::vm::{ChimeraVM, Value};

fn make_vm() -> ChimeraVM {
    let dna = Dna {
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;
    vm
}

#[test]
fn test_virus_replication() {
    let mut vm = make_vm();
    // Setup:
    // Trigger "!" at (5,4) -> emits signal to (5,4)
    // Virus "v" at (5,5) -> reads signal at (5,4)
    // Target at (5,6) is empty.

    vm.grid[5][4] = Value::Str("!".to_string());
    vm.grid[5][3] = Value::Int(1); // Source for ! (West)
    vm.grid[5][5] = Value::Str("v".to_string());

    exec_prologue_tick(&mut vm);

    // Check if v replicated to (5,6)
    match &vm.grid[5][6] {
        Value::Str(s) => assert_eq!(s, "v"),
        _ => panic!("Virus did not replicate"),
    }
}

#[test]
fn test_infect_injection() {
    let mut vm = make_vm();
    // Setup:
    // Trigger "!" at (5,4) -> emits signal to (5,4)
    // Payload "P" at (4,5)
    // Direction 2 (South) at (6,5)
    // Infect "i" at (5,5)

    vm.grid[5][4] = Value::Str("!".to_string());
    vm.grid[5][3] = Value::Int(1); // Source for ! (West)

    vm.grid[4][5] = Value::Str("P".to_string()); // Payload (North)
    vm.grid[6][5] = Value::Int(2); // Direction: South (South)
    vm.grid[5][5] = Value::Str("i".to_string());

    // Target (South of i) is (6,5). i overwrites Direction rune.

    exec_prologue_tick(&mut vm);

    // Check if (6,5) became "P"
    match &vm.grid[6][5] {
        Value::Str(s) => assert_eq!(s, "P"),
        _ => panic!("Infect did not inject payload"),
    }
}

#[test]
fn test_antibody_cleaning() {
    let mut vm = make_vm();
    // Setup:
    // Trigger "!" at (5,4) -> emits signal to (5,4)
    // Antibody "a" at (5,5)
    // Virus "v" at (5,6) (East neighbor)

    vm.grid[5][4] = Value::Str("!".to_string());
    vm.grid[5][3] = Value::Int(1); // Source for ! (West)
    vm.grid[5][5] = Value::Str("a".to_string());
    vm.grid[5][6] = Value::Str("v".to_string());

    exec_prologue_tick(&mut vm);

    // Check if (5,6) became 0 (Empty)
    match &vm.grid[5][6] {
        Value::Int(n) => assert_eq!(*n, 0),
        _ => panic!("Antibody did not clean virus"),
    }
}
