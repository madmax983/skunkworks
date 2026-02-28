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

    #[test]
    fn havoc_vec2_limit_overflow() {
        // [HAVOC] Vec2::limit bypass via squared overflow.
        // We choose a max value such that max is finite, but max*max overflows to Infinity.
        // max = 1e155 -> max*max = 1e310 > f64::MAX (approx 1.8e308).
        let max = 1.0e155;

        // We choose a vector with magnitude > max, but still finite.
        // v = (2.0e155, 0.0) -> magnitude = 2.0e155.
        // magnitude_squared = 4.0e310 (Infinity).
        let v = Vec2::new(2.0e155, 0.0);

        // The implementation of limit() checks:
        // if self.magnitude_squared() > max * max { ... }
        //
        // Here:
        // self.magnitude_squared() is Infinity.
        // max * max is Infinity.
        // Infinity > Infinity is FALSE.
        //
        // So the check fails, and the vector is returned AS IS.
        let limited = v.limit(max);

        // Verify that the limit SUCCEEDED (i.e., magnitude is limited to max).
        // Since the bug is fixed, this assertion should PASS.
        // We assume safe normalization handles infinity correctly, but precision might be tricky.
        // Using a generous tolerance because we are dealing with 1e155 numbers.

        // Note: limited.magnitude() might be slightly off due to float precision at this scale,
        // but it should be very close to max.
        // We handle the case where magnitude itself overflows to infinity if max is finite.

        // We use hypot directly because Vec2::magnitude() might overflow internally
        // (it uses sqrt(x*x + y*y)) even if the vector itself is finite.
        let mag = limited.x.hypot(limited.y);

        if mag.is_infinite() {
            panic!(
                "HAVOC FAILED: Limit result has infinite magnitude! Max was {}",
                max
            );
        }

        if mag > max * 1.001 {
            panic!(
                "HAVOC FAILED: Limit failed to clamp magnitude ({} > {})",
                mag, max
            );
        } else {
            println!("HAVOC BUG FIXED: Limit successfully clamped magnitude.");
        }
    }
}
