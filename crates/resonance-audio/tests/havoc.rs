use resonance_audio::physics::PhysicsGrid;

#[test]
fn test_zero_dim() {
    let mut grid = PhysicsGrid::new(0, 0);
    grid.step();
}

#[test]
fn test_one_dim() {
    let mut grid = PhysicsGrid::new(1, 1);
    grid.step();
}

#[test]
fn havoc_resonance_overflow() {
    // 👺 Havoc: `PhysicsGrid::new` calculates `size = width * height` without checked_mul.
    // An attacker or erroneous logic providing large dimensions will cause a deterministic panic DoS.
    let _grid = PhysicsGrid::new(usize::MAX, 2);
}
