use poincare_disk::{hyperbolic_dist, mobius_add, mobius_sub, Point};

#[test]
fn test_dist_nan_propagation() {
    let nan_point = Point::new(f64::NAN, f64::NAN);
    let valid_point = Point::new(0.0, 0.0);

    // Test NaN as first argument
    let dist1 = hyperbolic_dist(nan_point, valid_point);
    assert!(
        dist1.is_nan(),
        "hyperbolic_dist(NaN, valid) should be NaN, got {}",
        dist1
    );

    // Test NaN as second argument
    let dist2 = hyperbolic_dist(valid_point, nan_point);
    assert!(
        dist2.is_nan(),
        "hyperbolic_dist(valid, NaN) should be NaN, got {}",
        dist2
    );
}

#[test]
fn test_mobius_add_invalid_z() {
    // Sentry: Ensure mobius_add handles invalid z inputs safely
    // Currently, this might panic or return weird values. After fix, it should return z unchanged.

    let z_outside = Point::new(2.0, 0.0);
    let a = Point::new(0.5, 0.0);

    let result = mobius_add(z_outside, a);

    // If z is outside, result should be z (identity for invalid inputs)
    // This matches the behavior for invalid 'a'.
    assert!(
        (result - z_outside).norm() < 1e-9,
        "mobius_add with invalid z should return z, got {:?}",
        result
    );
}

#[test]
fn test_mobius_sub_invalid_z() {
    // Sentry: Ensure mobius_sub handles invalid z inputs safely

    let z_outside = Point::new(2.0, 0.0);
    let a = Point::new(0.5, 0.0);

    let result = mobius_sub(z_outside, a);

    // If z is outside, result should be z
    assert!(
        (result - z_outside).norm() < 1e-9,
        "mobius_sub with invalid z should return z, got {:?}",
        result
    );
}

#[test]
fn test_boundary_singularity_robustness() {
    // Verify points extremely close to boundary don't cause panic
    let r = 0.9999999999999999;
    let z = Point::new(r, 0.0);
    let a = Point::new(-r, 0.0); // Tries to move z to origin, but numerically tricky near edge

    let res = mobius_add(z, a);
    // Should be near 0
    assert!(res.norm() < 1e-9);
}
