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
