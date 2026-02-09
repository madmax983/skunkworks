use event_horizon::physics::{Body, Universe};
use macroquad::prelude::Vec2;

#[test]
fn test_stable_orbit() {
    let mut universe = Universe::new();

    // Sun at 0,0 with mass 1000
    // Earth at 100,0.
    // Circular orbit velocity v = sqrt(GM/r)
    // G is usually 1.0 or whatever constant we pick. Let's assume G=1 for now.
    // v = sqrt(1000 / 100) = sqrt(10) = 3.1622...

    let sun = Body {
        pos: Vec2::ZERO,
        vel: Vec2::ZERO,
        mass: 1000.0,
        radius: 10.0,
    };

    let earth = Body {
        pos: Vec2::new(100.0, 0.0),
        vel: Vec2::new(0.0, (1000.0f32 / 100.0).sqrt()),
        mass: 1.0,
        radius: 5.0,
    };

    universe.add_body(sun);
    universe.add_body(earth);

    // Simulate for one full orbit T = 2*pi*sqrt(r^3/GM)
    // T = 2*pi*sqrt(1000000/1000) = 2*pi*sqrt(1000) ~= 198.69

    let dt = 0.01;
    let steps = (200.0 / dt) as usize;

    for _ in 0..steps {
        universe.step(dt);
    }

    let earth_final = &universe.bodies[1];
    let dist = earth_final.pos.length();

    // Check if distance is still roughly 100.0
    // Allow small drift due to float precision, but if Euler was used, it would drift significantly.
    // With Symplectic Euler or Verlet, it should be very stable.
    assert!(
        (dist - 100.0).abs() < 1.0,
        "Orbit drifted too much: dist = {}",
        dist
    );
}
