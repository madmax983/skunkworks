use harmony_of_spheres::physics::{update, Body, G};
use macroquad::prelude::{Vec2, BLACK};

#[test]
fn test_energy_conservation() {
    let star_mass = 50000.0;
    let planet_mass = 1.0;
    let planet_dist = 200.0;

    // Circular orbit velocity: v = sqrt(GM/r)
    let v_mag = (G * star_mass / planet_dist).sqrt();

    let mut bodies = vec![
        Body::new(
            Vec2::new(0.0, 0.0),
            Vec2::new(0.0, 0.0),
            star_mass,
            20.0,
            BLACK,
        ),
        Body::new(
            Vec2::new(planet_dist, 0.0),
            Vec2::new(0.0, v_mag),
            planet_mass,
            5.0,
            BLACK,
        ),
    ];

    // Initial Energy
    let initial_energy = calculate_energy(&bodies);
    println!("Initial Energy: {}", initial_energy);

    // Simulate for 1000 steps
    let dt = 0.016; // 60 FPS
    for _ in 0..10000 {
        update(&mut bodies, dt);
    }

    let final_energy = calculate_energy(&bodies);
    println!("Final Energy: {}", final_energy);

    let drift = (final_energy - initial_energy).abs() / initial_energy.abs();
    println!("Energy Drift: {}%", drift * 100.0);

    // Symplectic Euler is first-order (or second depending on implementation),
    // but it should be bounded. 1% drift over 10000 steps is acceptable for a game.
    // Actually, Verlet/Symplectic Euler should be very stable.
    assert!(drift < 0.01, "Energy drift too high: {}%", drift * 100.0);
}

fn calculate_energy(bodies: &[Body]) -> f32 {
    let star = &bodies[0];
    let planet = &bodies[1];

    let r = star.pos.distance(planet.pos);
    let v_sq = planet.vel.length_squared();

    let kinetic = 0.5 * planet.mass * v_sq;
    let potential = -G * star.mass * planet.mass / r;

    kinetic + potential
}
