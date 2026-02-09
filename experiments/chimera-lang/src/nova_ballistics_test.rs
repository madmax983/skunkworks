#![cfg(all(test, feature = "nova"))]

use crate::ast::{Dna, Helix, Strand};
use crate::vm::{ChimeraVM, Value};

fn make_vm() -> ChimeraVM {
    let dna = Dna {
        helix: Helix {
            strands: vec![Strand { genes: vec![] }],
        },
    };
    ChimeraVM::new(dna)
}

#[test]
fn test_fire_opcode() {
    let mut vm = make_vm();

    // Stack: power=1, dy=0, dx=1 (East)
    vm.stack.push(Value::Int(1)); // power
    vm.stack.push(Value::Int(0)); // dy
    vm.stack.push(Value::Int(1)); // dx

    // Set context to center
    vm.context_loc = (8, 8);

    crate::vm::nova_ballistics::exec_fire(&mut vm);

    assert_eq!(vm.projectiles.len(), 1);
    let p = &vm.projectiles[0];
    assert_eq!(p.x, 8.0);
    assert_eq!(p.y, 8.0);
    assert!((p.vx - 1.0).abs() < 0.001);
    assert!((p.vy - 0.0).abs() < 0.001);
}

#[test]
fn test_projectile_collision() {
    let mut vm = make_vm();

    // Place block at (8, 10)
    vm.grid[8][10] = Value::Int(99);

    // Spawn projectile at (8, 8) moving East
    vm.projectiles.push(crate::vm::nova_ballistics::Projectile {
        x: 8.0,
        y: 8.0,
        vx: 1.0,
        vy: 0.0,
        power: 1,
        ttl: 10,
        owner: 0,
        last_hit: None,
    });

    // Step 1: Moves to (8, 9)
    vm.step();
    assert_eq!(vm.projectiles.len(), 1);
    assert_eq!(vm.grid[8][10], Value::Int(99)); // Still there

    // Step 2: Moves to (8, 10) -> Impact!
    vm.step();
    assert_eq!(vm.projectiles.len(), 0); // Destroyed
    assert_eq!(vm.grid[8][10], Value::Int(0)); // Block destroyed

    assert!(vm.output.iter().any(|s| s.contains("IMPACT")));
}

#[test]
fn test_fire_overflow() {
    let mut vm = make_vm();

    // Stack: power=1, dy=0, dx=3037000500 (Enough to overflow i64 when squared)
    vm.stack.push(Value::Int(1)); // power
    vm.stack.push(Value::Int(0)); // dy
    vm.stack.push(Value::Int(3037000500)); // dx

    // Set context to center
    vm.context_loc = (8, 8);

    // This should NOT panic if fixed
    crate::vm::nova_ballistics::exec_fire(&mut vm);

    assert_eq!(vm.projectiles.len(), 1);
    let p = &vm.projectiles[0];
    // Direction should still be valid (normalized)
    // vx should be 1.0 (since dy=0)
    assert!((p.vx - 1.0).abs() < 0.001);
}
