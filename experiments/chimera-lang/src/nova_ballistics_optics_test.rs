#![cfg(all(test, feature = "nova"))]

use crate::ast::{Dna, Helix, Strand};
use crate::vm::{ChimeraVM, Value};
use crate::vm::nova_ballistics::{Projectile, update_projectiles};

fn make_vm() -> ChimeraVM {
    let dna = Dna {
        helix: Helix {
            strands: vec![Strand { genes: vec![] }],
        },
    };
    ChimeraVM::new(dna)
}

fn create_projectile(x: f64, y: f64, vx: f64, vy: f64) -> Projectile {
    Projectile {
        x,
        y,
        vx,
        vy,
        power: 1,
        ttl: 10,
        owner: 0,
        last_hit: None,
    }
}

#[test]
fn test_mirror_0_vertical_reflect_x() {
    let mut vm = make_vm();
    // MIRROR:0 -> Reflect X
    vm.grid[8][8] = Value::Str("MIRROR:0".to_string());

    // Projectile moving Right (East) hits (8,8) from Left
    // Starting at (7.0, 8.0) with vx=1.0 -> becomes (8.0, 8.0).
    vm.projectiles.push(create_projectile(7.0, 8.0, 1.0, 0.0));

    update_projectiles(&mut vm);

    assert_eq!(vm.projectiles.len(), 1);
    let p = &vm.projectiles[0];
    assert_eq!(p.vx, -1.0); // Reflected X
    assert_eq!(p.vy, 0.0);
    assert_eq!(p.last_hit, Some((8, 8)));
}

#[test]
fn test_mirror_1_horizontal_reflect_y() {
    let mut vm = make_vm();
    // MIRROR:1 -> Reflect Y
    vm.grid[8][8] = Value::Str("MIRROR:1".to_string());

    // Projectile moving Down (South) hits (8,8) from Top
    // Starting at (8.0, 7.0) with vy=1.0 -> becomes (8.0, 8.0).
    vm.projectiles.push(create_projectile(8.0, 7.0, 0.0, 1.0));

    update_projectiles(&mut vm);

    assert_eq!(vm.projectiles.len(), 1);
    let p = &vm.projectiles[0];
    assert_eq!(p.vx, 0.0);
    assert_eq!(p.vy, -1.0); // Reflected Y
    assert_eq!(p.last_hit, Some((8, 8)));
}

#[test]
fn test_mirror_2_diagonal_swap_negate() {
    let mut vm = make_vm();
    // MIRROR:2 -> Swap and Negate (Forward Slash /)
    // vx' = -vy
    // vy' = -vx
    vm.grid[8][8] = Value::Str("MIRROR:2".to_string());

    // Projectile moving East (1,0) -> should go North (0,-1)
    // -vy = -0 = 0
    // -vx = -1 = -1
    vm.projectiles.push(create_projectile(7.0, 8.0, 1.0, 0.0));

    update_projectiles(&mut vm);

    assert_eq!(vm.projectiles.len(), 1);
    let p = &vm.projectiles[0];
    assert_eq!(p.vx, 0.0);
    assert_eq!(p.vy, -1.0);
    assert_eq!(p.last_hit, Some((8, 8)));
}

#[test]
fn test_mirror_3_diagonal_swap() {
    let mut vm = make_vm();
    // MIRROR:3 -> Swap (Back Slash \)
    // vx' = vy
    // vy' = vx
    vm.grid[8][8] = Value::Str("MIRROR:3".to_string());

    // Projectile moving East (1,0) -> should go South (0,1)
    // vy = 0 -> vx' = 0
    // vx = 1 -> vy' = 1
    vm.projectiles.push(create_projectile(7.0, 8.0, 1.0, 0.0));

    update_projectiles(&mut vm);

    assert_eq!(vm.projectiles.len(), 1);
    let p = &vm.projectiles[0];
    assert_eq!(p.vx, 0.0);
    assert_eq!(p.vy, 1.0);
    assert_eq!(p.last_hit, Some((8, 8)));
}

#[test]
fn test_prism_split() {
    let mut vm = make_vm();
    vm.grid[8][8] = Value::Str("PRISM:".to_string());

    // Projectile hits (8,8) moving East
    vm.projectiles.push(create_projectile(7.0, 8.0, 1.0, 0.0));

    update_projectiles(&mut vm);

    assert_eq!(vm.projectiles.len(), 3);

    // The order of pushing is Center, Left, Right

    // 1. Center (Original direction)
    let p0 = &vm.projectiles[0];
    assert!((p0.vx - 1.0).abs() < 0.001);
    assert!(p0.vy.abs() < 0.001);
    assert_eq!(p0.last_hit, Some((8, 8)));

    // 2. Left (Angle - 0.5 rad)
    // vx = cos(-0.5) ~ 0.877
    // vy = sin(-0.5) ~ -0.479
    let p1 = &vm.projectiles[1];
    let expected_vx_l = (-0.5f64).cos();
    let expected_vy_l = (-0.5f64).sin();
    assert!((p1.vx - expected_vx_l).abs() < 0.001);
    assert!((p1.vy - expected_vy_l).abs() < 0.001);
    assert_eq!(p1.last_hit, Some((8, 8)));

    // 3. Right (Angle + 0.5 rad)
    // vx = cos(0.5) ~ 0.877
    // vy = sin(0.5) ~ 0.479
    let p2 = &vm.projectiles[2];
    let expected_vx_r = (0.5f64).cos();
    let expected_vy_r = (0.5f64).sin();
    assert!((p2.vx - expected_vx_r).abs() < 0.001);
    assert!((p2.vy - expected_vy_r).abs() < 0.001);
    assert_eq!(p2.last_hit, Some((8, 8)));
}

#[test]
fn test_lens_amplify() {
    let mut vm = make_vm();
    vm.grid[8][8] = Value::Str("LENS:5".to_string());

    let mut p = create_projectile(7.0, 8.0, 1.0, 0.0);
    p.power = 1;
    p.ttl = 10;
    vm.projectiles.push(p);

    update_projectiles(&mut vm);

    assert_eq!(vm.projectiles.len(), 1);
    let p = &vm.projectiles[0];

    // Power should be max(original, lens_power) = max(1, 5) = 5
    assert_eq!(p.power, 5);

    // TTL logic:
    // 1. Decrement by 1 (10 -> 9)
    // 2. Lens adds 10 (9 -> 19)
    assert_eq!(p.ttl, 19);

    assert_eq!(p.last_hit, Some((8, 8)));
}

#[test]
fn test_loop_prevention() {
    let mut vm = make_vm();
    vm.grid[8][8] = Value::Str("MIRROR:0".to_string());

    // 1. First tick: Projectile hits mirror
    vm.projectiles.push(create_projectile(7.0, 8.0, 1.0, 0.0));
    update_projectiles(&mut vm);

    let p = &vm.projectiles[0];
    assert_eq!(p.vx, -1.0);
    assert_eq!(p.last_hit, Some((8, 8)));

    // 2. Second tick: Projectile moves away (to 7,8)
    // But let's force it to stay in (8,8) by using fractional speed < 1.0
    // If we use speed 0.1, it moves from 8.0 to 7.9, still in (8,8) grid cell (int casting)
    // Wait, (7.9 as usize) is 7. (8.0 as usize) is 8.
    // So 7.9 is in cell 7.
    // We need it to move LESS than 0.0 so it stays >= 8.0 ?
    // No, if it reflects, vx is negative.
    // If it started at 7.0, became 8.0.
    // Now x=8.0. vx=-1.0. Next x=7.0.
    // So it leaves immediately.

    // Let's manually set position to 8.5 so it stays in cell 8
    vm.projectiles[0].x = 8.5;
    vm.projectiles[0].vx = -0.1; // Moving left slowly
    // Next x = 8.4. Still in cell 8.

    update_projectiles(&mut vm);

    // It should NOT reflect again (vx should remain -0.1)
    let p = &vm.projectiles[0];
    assert_eq!(p.vx, -0.1);
    // And last_hit should remain set
    assert_eq!(p.last_hit, Some((8, 8)));
}
