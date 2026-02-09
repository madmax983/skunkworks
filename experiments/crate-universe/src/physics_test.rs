use crate::physics::{Body, Universe};
use macroquad::prelude::Vec2;
use std::collections::VecDeque;

#[test]
fn test_gravity_attraction() {
    let mut u = Universe::new();
    u.G = 1.0;

    let b1 = Body {
        pos: Vec2::new(0.0, 0.0),
        vel: Vec2::ZERO,
        mass: 10.0,
        radius: 1.0,
        name: "A".to_string(),
        history: VecDeque::new(),
        parent_id: None,
    };

    let b2 = Body {
        pos: Vec2::new(10.0, 0.0),
        vel: Vec2::ZERO,
        mass: 10.0,
        radius: 1.0,
        name: "B".to_string(),
        history: VecDeque::new(),
        parent_id: None,
    };

    u.bodies.push(b1);
    u.bodies.push(b2);

    // Run one step
    u.step(1.0);

    // Expect distance to decrease
    let new_dist = u.bodies[0].pos.distance(u.bodies[1].pos);
    assert!(new_dist < 10.0, "Bodies should move closer due to gravity. Current distance: {}", new_dist);
}
