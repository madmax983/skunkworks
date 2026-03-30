use proptest::prelude::*;
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

proptest! {
    /// 👺 Havoc: Proving that `PhysicsGrid::new` causes a panic on large inputs due to unchecked multiplication.
    ///
    /// 🧨 **The Trigger:** A width and height such that `width * height > usize::MAX`.
    #[test]
    #[should_panic(expected = "attempt to multiply with overflow")]
    fn test_havoc_overflow_proptest(
        width in (usize::MAX / 2 + 1)..=usize::MAX,
        height in 2..=10usize
    ) {
        let _grid = PhysicsGrid::new(width, height);
    }
}
