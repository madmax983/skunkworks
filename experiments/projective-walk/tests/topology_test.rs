use macroquad::prelude::Vec2;
use projective_walk::topology::{Point, step};

#[test]
fn test_wrap_top() {
    let start = Point::new(0.2, 0.99, Vec2::new(0.1, 1.0).normalize());
    let delta = Vec2::new(0.0, 0.02);
    let end = step(start, delta);

    assert!((end.v - 0.01).abs() < 1e-4, "v should wrap to ~0.01, got {}", end.v);
    assert!((end.u - 0.8).abs() < 1e-4, "u should flip to ~0.8, got {}", end.u);
    assert!(end.facing.x < 0.0, "facing.x should flip sign");
    assert!(end.facing.y > 0.0, "facing.y should stay positive");
}

#[test]
fn test_wrap_right() {
    let start = Point::new(0.99, 0.2, Vec2::X);
    let delta = Vec2::new(0.02, 0.0);
    let end = step(start, delta);

    assert!((end.u - 0.01).abs() < 1e-4, "u should wrap to 0.01");
    assert!((end.v - 0.8).abs() < 1e-4, "v should flip to 0.8");
}
