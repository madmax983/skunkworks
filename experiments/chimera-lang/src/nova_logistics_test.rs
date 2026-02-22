#[cfg(test)]
use crate::ast::{Dna, Gene, Helix, Strand};
#[cfg(test)]
use crate::opcode::OpCode;
#[cfg(test)]
use crate::vm::{ChimeraVM, Value};

#[cfg(test)]
fn make_vm() -> ChimeraVM {
    let genes = vec![Gene {
        op: OpCode::Photosynthesize,
        args: vec![],
    }];
    let dna = Dna { evolution_config: None,
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    };
    ChimeraVM::new(dna)
}

#[test]
fn test_belt_movement() {
    let mut vm = make_vm();

    // Setup: Item at 8,8
    vm.grid[8][8] = Value::Int(42);

    // Setup: Belt:E at 8,8 (Logistics OpCode is 1=Belt, Dir 1=East)
    // We can simulate the OpCode execution or just write directly to cartography_grid
    // Let's use the OpCode execution to test that too.

    // Execute: Push 8,8, 1(E), 1(Belt) -> Logistics
    vm.stack.push(Value::Int(1)); // Type: Belt
    vm.stack.push(Value::Int(1)); // Dir: East
    vm.stack.push(Value::Int(8)); // y
    vm.stack.push(Value::Int(8)); // x

    // Manually execute the op
    crate::vm::nova_logistics::exec_logistics(&mut vm, OpCode::Logistics, &[]);

    // Check map
    assert_eq!(vm.cartography_grid[8][8], Value::Str("Belt:E".to_string()));

    // Step VM (triggers process_logistics)
    vm.step();

    // Verify movement
    assert_eq!(vm.grid[8][8], Value::Int(0)); // Moved from source
    assert_eq!(vm.grid[8][9], Value::Int(42)); // Arrived at target
}

#[test]
fn test_belt_chain() {
    let mut vm = make_vm();

    // Chain: 8,8 -> 8,9 -> 8,10
    vm.cartography_grid[8][8] = Value::Str("Belt:E".to_string());
    vm.cartography_grid[8][9] = Value::Str("Belt:E".to_string());

    vm.grid[8][8] = Value::Int(99);

    // Tick 1: 8,8 -> 8,9
    vm.step();
    assert_eq!(vm.grid[8][8], Value::Int(0));
    assert_eq!(vm.grid[8][9], Value::Int(99));
    assert_eq!(vm.grid[8][10], Value::Int(0));

    // Tick 2: 8,9 -> 8,10
    vm.step();
    assert_eq!(vm.grid[8][9], Value::Int(0));
    assert_eq!(vm.grid[8][10], Value::Int(99));
}

#[test]
fn test_blocked_belt() {
    let mut vm = make_vm();

    vm.cartography_grid[8][8] = Value::Str("Belt:E".to_string());
    vm.grid[8][8] = Value::Int(10);
    vm.grid[8][9] = Value::Int(20); // Obstacle

    vm.step();

    // Should not move because target is occupied
    assert_eq!(vm.grid[8][8], Value::Int(10));
    assert_eq!(vm.grid[8][9], Value::Int(20));
}

#[test]
fn test_collision() {
    let mut vm = make_vm();

    // Two belts converging on 8,10
    // 8,9 -> E -> 8,10
    // 9,10 -> N -> 8,10

    vm.cartography_grid[8][9] = Value::Str("Belt:E".to_string());
    vm.cartography_grid[9][10] = Value::Str("Belt:N".to_string());

    vm.grid[8][9] = Value::Int(1);
    vm.grid[9][10] = Value::Int(2);

    vm.step();

    // One should succeed, one should fail (or both fail if naive, but my logic allows one)
    // The iteration order is y then x.
    // (8,9) comes before (9,10).
    // So (8,9) moves to (8,10).
    // Then (9,10) tries to move to (8,10).
    // Since (8,10) is now occupied by 1 (in the same tick logic? No, my logic writes directly to `grid`).
    // If I update `grid` in place, (8,9) writes 1 to (8,10).
    // Then (9,10) sees (8,10) has 1. So it waits.

    assert_eq!(vm.grid[8][10], Value::Int(1)); // First one made it
    assert_eq!(vm.grid[8][9], Value::Int(0)); // First one cleared
    assert_eq!(vm.grid[9][10], Value::Int(2)); // Second one blocked
}
