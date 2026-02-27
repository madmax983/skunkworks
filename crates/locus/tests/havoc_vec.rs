#[cfg(test)]
mod tests {
    use locus::Vec2;
    use std::f64;

    #[test]
    fn havoc_vec2_nan_propagation() {
        // [HAVOC] Vec2 operations with NaN propagate silently and corrupt game state.
        let v_nan = Vec2::new(f64::NAN, 0.0);
        let v_normal = Vec2::new(10.0, 10.0);

        // Addition corrupts (X component only, since Y is 0.0)
        let sum = v_normal + v_nan;
        assert!(sum.x.is_nan());
        assert!(!sum.y.is_nan()); // Y is 10.0 + 0.0 = 10.0

        // Normalize corrupts
        // Magnitude = sqrt(NAN^2 + 0^2) = NAN
        // x / mag = NAN / NAN = NAN
        // y / mag = 0 / NAN = NAN
        let norm = v_nan.normalize();
        assert!(norm.x.is_nan());
        assert!(norm.y.is_nan());

        // Reflect corrupts
        let reflected = v_normal.reflect(v_nan);
        // n_sq = NAN
        // dot = NAN
        // factor = NAN
        // r.x = 10 - NAN * NAN = NAN
        // r.y = 10 - NAN * 0 = NAN
        assert!(reflected.x.is_nan());
        assert!(reflected.y.is_nan());

        println!("HAVOC: Vec2 successfully corrupted by NaN propagation.");
    }

    #[test]
    fn havoc_vec2_infinity_overflow() {
        // [HAVOC] Operations on large coordinates overflow to Infinity.
        let huge = Vec2::new(f64::MAX / 2.0, 0.0);

        // Magnitude squared overflows
        // (MAX/2)^2 = MAX^2 / 4 -> Infinity
        let mag_sq = huge.magnitude_squared();
        assert_eq!(mag_sq, f64::INFINITY);

        // Distance calculation overflows intermediate squared distance
        let huge2 = Vec2::new(0.0, f64::MAX / 2.0);
        let dist_sq = huge.distance_squared(huge2);
        assert_eq!(dist_sq, f64::INFINITY);

        println!("HAVOC: Vec2 overflowed to Infinity.");
    }

    #[test]
    fn havoc_vec2_precision_loss() {
        // [HAVOC] Large coordinates lose small precision, causing entities to "stuck".
        let large = Vec2::new(1e17, 0.0);
        let small = Vec2::new(0.1, 0.0);

        let sum = large + small;

        // Check if addition had any effect
        if sum.x == large.x {
             println!("HAVOC: Precision loss detected. 1e17 + 0.1 == 1e17.");
        } else {
             panic!("HAVOC FAILED: Precision was preserved unexpectedly?");
        }
    }
}
