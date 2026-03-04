use poincare_disk::{Mobius, Point};
use std::f64::consts::PI;

#[test]
fn test_mobius_identity() {
    let id = Mobius::identity();
    let p = Point::new(0.5, 0.2);
    let result = id.apply(p);
    assert_eq!(result, p);

    assert_eq!(id.a(), num_complex::Complex::new(1.0, 0.0));
    assert_eq!(id.b(), num_complex::Complex::new(0.0, 0.0));
    assert_eq!(id.c(), num_complex::Complex::new(0.0, 0.0));
    assert_eq!(id.d(), num_complex::Complex::new(1.0, 0.0));
}

#[test]
fn test_mobius_translation() {
    let k = Point::new(0.5, 0.0);
    let t = Mobius::translation(k);
    let origin = Point::new(0.0, 0.0);
    assert_eq!(t.apply(origin), k);
}

#[test]
fn test_mobius_inverse_translation() {
    let k = Point::new(0.5, 0.0);
    let t = Mobius::inverse_translation(k);
    assert!(t.apply(k).norm() < 1e-10);
}

#[test]
fn test_mobius_rotation() {
    let m = Mobius::rotation(PI / 2.0);
    let z = Point::new(1.0, 0.0);
    let rotated = m.apply(z);
    assert!((rotated.im - 1.0).abs() < 1e-9);
    assert!(rotated.re.abs() < 1e-9);
}

#[test]
fn test_mobius_inverse() {
    let forward = Mobius::translation(Point::new(0.5, 0.0));
    let backward = forward.inverse();
    let p = Point::new(0.0, 0.0);
    let moved = forward.apply(p);
    let returned = backward.apply(moved);
    assert!((returned - p).norm() < 1e-9);
}

#[test]
fn test_mobius_then() {
    let rotate = Mobius::rotation(PI / 2.0);
    let translate = Mobius::translation(Point::new(0.5, 0.0));

    let step_then_turn = rotate.then(&translate);

    let origin = Point::new(0.0, 0.0);
    let result = step_then_turn.apply(origin);

    assert!((result.im - 0.5).abs() < 1e-9);
    assert!(result.re.abs() < 1e-9);
}

#[test]
fn test_mobius_new_singular() {
    // a * d - b * c = 0
    let a = num_complex::Complex::new(1.0, 0.0);
    let b = num_complex::Complex::new(1.0, 0.0);
    let c = num_complex::Complex::new(1.0, 0.0);
    let d = num_complex::Complex::new(1.0, 0.0);

    let m = Mobius::new(a, b, c, d);
    assert!(m.is_none());
}
