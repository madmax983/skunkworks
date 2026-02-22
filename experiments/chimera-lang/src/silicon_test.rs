#[cfg(feature = "silicon")]
use crate::ast::{Dna, Helix};
#[cfg(feature = "silicon")]
use crate::vm::{ChimeraVM, Value};

#[cfg(feature = "silicon")]
fn make_vm() -> ChimeraVM {
    let dna = Dna { evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.silicon_mode = true;
    vm
}

#[cfg(feature = "silicon")]
#[test]
fn test_wireworld_pulse_propagation() {
    let mut vm = make_vm();
    // Setup wire: Head -> Conductor -> Conductor
    // [2, 1, 1]
    vm.grid[0][0] = Value::Int(2);
    vm.grid[0][1] = Value::Int(1);
    vm.grid[0][2] = Value::Int(1);

    // Step 1
    // 0,0: Head(2) -> Tail(3)
    // 0,1: Cond(1) -> Head(2) (1 neighbor is head)
    // 0,2: Cond(1) -> Cond(1) (0 neighbors head - strictly neighbor of 0,2 is 0,1 which is 1, not 2 yet)
    // Update is simultaneous.
    crate::vm::silicon::step_circuit(&mut vm);

    assert_eq!(vm.grid[0][0], Value::Int(3));
    assert_eq!(vm.grid[0][1], Value::Int(2));
    assert_eq!(vm.grid[0][2], Value::Int(1));

    // Step 2
    // 0,0: Tail(3) -> Cond(1)
    // 0,1: Head(2) -> Tail(3)
    // 0,2: Cond(1) -> Head(2) (Neighbor 0,1 was Head)
    crate::vm::silicon::step_circuit(&mut vm);

    assert_eq!(vm.grid[0][0], Value::Int(1));
    assert_eq!(vm.grid[0][1], Value::Int(3));
    assert_eq!(vm.grid[0][2], Value::Int(2));
}

#[cfg(feature = "silicon")]
#[test]
fn test_wireworld_diode() {
    // Diode logic
    // ... -> 1 -> 1 -> 1 -> ...
    // But if we have a loop or specific geometry it acts as diode.
    // Let's just test a fork.
    // 2 (Head) at (0,0)
    // 1 (Cond) at (0,1), (1,0)

    let mut vm = make_vm();
    vm.grid[0][0] = Value::Int(2);
    vm.grid[0][1] = Value::Int(1);
    vm.grid[1][0] = Value::Int(1);

    crate::vm::silicon::step_circuit(&mut vm);

    assert_eq!(vm.grid[0][0], Value::Int(3)); // Head -> Tail
    assert_eq!(vm.grid[0][1], Value::Int(2)); // Cond -> Head
    assert_eq!(vm.grid[1][0], Value::Int(2)); // Cond -> Head
}
