use system_attractor::simulation::{update_particles, LorenzParams, Particle};

#[test]
fn test_lorenz_update() {
    // Initial state: slightly off origin to ensure movement
    let mut particles = vec![Particle::new(1.0, 1.0, 1.0)];
    let params = LorenzParams::default();

    // Store initial pos
    let initial_pos = particles[0].pos;

    // Update
    update_particles(&mut particles, &params, 0.01);

    // Check if position changed
    assert_ne!(particles[0].pos.x, initial_pos.x);
    assert_ne!(particles[0].pos.y, initial_pos.y);
    assert_ne!(particles[0].pos.z, initial_pos.z);

    // Check if velocity is non-zero
    assert!(particles[0].vel.length() > 0.0);
}
