use macroquad::prelude::Vec3;
use system_attractor::lyapunov::LyapunovMonitor;
use system_attractor::simulation::LorenzParams;
use system_attractor::simulation::{update_particles, Particle};

#[test]
fn test_lyapunov_positive() {
    let params = LorenzParams::default();

    // Warm up phase: Evolve a particle to get onto the attractor
    let mut p = Particle::new(1.0, 1.0, 1.0);
    let dt = 0.01;
    let mut particles = vec![p];

    // Run for 1000 steps to settle
    for _ in 0..1000 {
        update_particles(&mut particles, &params, dt);
    }

    let start_pos = particles[0].pos;
    println!("Starting Lyapunov monitor at: {:?}", start_pos);

    let mut monitor = LyapunovMonitor::new(start_pos, 1e-4);

    // Run for divergence
    // 5000 steps * 0.01 = 50 seconds
    for _ in 0..5000 {
        monitor.update(&params, dt);
    }

    let lambda = monitor.get_exponent();
    println!("Lyapunov Exponent: {}", lambda);

    // For Lorenz with standard params, lambda is approx 0.9
    assert!(
        lambda > 0.0,
        "Lyapunov exponent should be positive for chaotic system, got {}",
        lambda
    );
    assert!(
        lambda < 2.0,
        "Lyapunov exponent should be reasonable, got {}",
        lambda
    );
}
