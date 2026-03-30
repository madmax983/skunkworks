use resonance_audio::physics::{PhysicsGrid, Material};

#[test]
fn havoc_physics_nan() {
    let mut grid = PhysicsGrid::new(10, 10);
    grid.pluck(5, 5, f32::NAN);

    // We expect this to just propagate NaN.
    // If it does, does it crash anything?
    grid.step();
}
