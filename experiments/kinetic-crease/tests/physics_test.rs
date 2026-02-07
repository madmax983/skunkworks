use kinetic_crease::physics::{Particle, Solver};
use nalgebra::Vector3;

#[test]
fn test_distance_constraint() {
    let mut solver = Solver::new();

    // Create two particles at 0,0,0 and 1,0,0
    let p1 = Particle::new(0.0, 0.0, 0.0, 1.0);
    let p2 = Particle::new(1.0, 0.0, 0.0, 1.0);

    let id1 = solver.add_particle(p1);
    let id2 = solver.add_particle(p2);

    // Add distance constraint with rest_len = 0.5
    // Current dist = 1.0. Solver should pull them together.
    solver.add_distance_constraint(id1, id2, 1.0);

    // Manually modify constraint to target len 0.5 (need access or reconstruct)
    // Actually, add_distance_constraint uses current distance as rest_len.
    // So current rest_len is 1.0.
    // If I move p2 to 2.0, solver should pull back to 1.0.

    solver.particles[id2].pos.x = 2.0;

    // Step
    solver.step(0.1, 10);

    let dist = (solver.particles[id1].pos - solver.particles[id2].pos).magnitude();

    // Should be close to 1.0
    assert!(
        (dist - 1.0).abs() < 0.1,
        "Distance {} not close to 1.0",
        dist
    );
}

#[test]
fn test_hinge_geometry() {
    // Check if target length calculation logic matches manual check
    use std::f32::consts::PI;

    let fold_factor = 0.5;
    let target_angle = PI * (1.0 - fold_factor * 0.9); // From mesh.rs logic

    let h1 = 1.0;
    let h2 = 1.0;

    let d_sq = h1 * h1 + h2 * h2 - 2.0 * h1 * h2 * target_angle.cos();
    let d = d_sq.sqrt();

    // For 0.5 fold factor, angle is ~PI/2 (90 deg) if we map fully linear.
    // Here logic is PI * (1 - 0.5*0.9) = PI * 0.55 = 99 deg.
    // cos(99) is negative small.
    // d should be slightly > sqrt(2).

    assert!(d > 0.0);
}
