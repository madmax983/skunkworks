use lensing_poetry::physics::{Body, integrate};
use macroquad::prelude::*;

#[test]
fn test_earth_orbit_stability() {
    let sun = Body {
        pos: vec2(0.0, 0.0),
        vel: vec2(0.0, 0.0),
        mass: 1.0,
        radius: 10.0,
        color: YELLOW,
    };
    let earth = Body {
        pos: vec2(1.0, 0.0),
        vel: vec2(0.0, 1.0), // v = sqrt(GM/r) for circular orbit with M=1, G=1, r=1
        mass: 0.000001,
        radius: 1.0,
        color: BLUE,
    };

    let mut bodies = vec![sun, earth];
    let dt = 0.01;
    let period = 2.0 * std::f32::consts::PI; // ~6.28

    // Run for 0.25 orbits (quarter turn)
    let steps_quarter = (period * 0.25 / dt) as usize;
    for _ in 0..steps_quarter {
        integrate(&mut bodies, dt);
    }

    // Check movement
    let pos_quarter = bodies[1].pos;
    let dist_quarter = pos_quarter.length();

    println!("Quarter Position: {:?}", pos_quarter);
    println!("Quarter Distance: {}", dist_quarter);

    // If stub, pos is (1,0).
    // If working, pos should be approx (0, 1).
    // Assert it moved significantly from (1,0)
    assert!((pos_quarter - vec2(1.0, 0.0)).length() > 0.1, "Body did not move! Stub implementation?");

    // Assert it is still in orbit (distance ~ 1.0)
    assert!((dist_quarter - 1.0).abs() < 0.05, "Orbit drifted significantly at quarter turn! r={}", dist_quarter);

    // Run for 100 orbits total (remaining 99.75)
    let total_steps = (period * 100.0 / dt) as usize;
    let remaining = total_steps - steps_quarter;

    for _ in 0..remaining {
        integrate(&mut bodies, dt);
    }

    let final_pos = bodies[1].pos;
    let final_dist = final_pos.length();

    println!("Final Position (100 years): {:?}", final_pos);
    println!("Final Distance: {}", final_dist);

    // Symplectic should be stable.
    assert!((final_dist - 1.0).abs() < 0.05, "Orbit drifted after 100 years! r={}", final_dist);
}
