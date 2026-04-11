use resonance_audio::physics::PhysicsGrid;

#[test]
fn test_dos_overflow() {
    let grid = PhysicsGrid::new(usize::MAX, 2);
    assert_eq!(grid.width(), 0);
    assert_eq!(grid.height(), 0);
}
