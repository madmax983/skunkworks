#[cfg(test)]
mod tests {
    use locus::vec4::Vec4;
    use locus::Vec2;

    #[test]
    fn sentry_vec4_limit_overflow() {
        // 🎯 Target: Vec4::limit
        // 💣 Risk: Squared magnitude calculation overflows to Infinity for large finite vectors.
        //          If `max` is also large enough that `max*max` overflows, the check `sq_mag > max_sq`
        //          becomes `Inf > Inf` which is false, bypassing the limit.

        // f32::MAX is approx 3.4e38. sqrt(MAX) is approx 1.84e19.
        // We choose values that exceed this square root.

        let max_val: f32 = 2.0e20; // max*max = 4e40 > f32::MAX (Inf)
        let vec_val: f32 = 3.0e20; // vec_mag = 3e20. sq_mag = 9e40 (Inf)

        let v = Vec4::new(vec_val, 0.0, 0.0, 0.0);

        // Pre-condition check: ensure our assumptions about overflow are correct for this platform
        assert_eq!(v.length_squared(), f32::INFINITY, "Setup failed: Vector length squared should overflow");
        assert_eq!(max_val * max_val, f32::INFINITY, "Setup failed: Max squared should overflow");

        let limited = v.limit(max_val);

        // If the bug exists, limited.x will still be 3.0e20
        // We expect it to be clamped to 2.0e20
        assert!(limited.x <= max_val, "Vec4::limit failed to clamp! Got {}, expected <= {}", limited.x, max_val);
        assert!((limited.x - max_val).abs() < 1.0e15, "Vec4::limit result should be close to max. Got {}", limited.x);
    }

    #[test]
    fn sentry_vec2_limit_overflow() {
        // 🎯 Target: Vec2::limit (Regression test similar to havoc but explicitly finding the fix)
        // f64::MAX is approx 1.8e308. sqrt(MAX) is approx 1.34e154.

        let max_val: f64 = 2.0e155; // max*max > f64::MAX
        let vec_val: f64 = 3.0e155;

        let v = Vec2::new(vec_val, 0.0);

        assert_eq!(v.magnitude_squared(), f64::INFINITY);
        assert_eq!(max_val * max_val, f64::INFINITY);

        let limited = v.limit(max_val);

        assert!(limited.x <= max_val, "Vec2::limit failed to clamp! Got {}, expected <= {}", limited.x, max_val);
    }
}
