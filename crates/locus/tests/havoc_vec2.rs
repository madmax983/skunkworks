#[cfg(test)]
mod tests {
    use locus::Vec2;
    use std::f64;

    // 👺 Havoc: Prove `magnitude_squared` can overflow!
    #[test]
    #[should_panic(expected = "👺 Havoc: Vector magnitude squared overflowed into Infinity!")]
    fn test_havoc_vec2_magnitude_squared_overflow() {
        let v = Vec2::new(f64::MAX, f64::MAX);
        // This will result in an infinite f64
        let sq = v.magnitude_squared();

        // By checking if it's infinite, we know it overflowed instead of safely clamping.
        assert!(
            sq.is_finite(),
            "👺 Havoc: Vector magnitude squared overflowed into Infinity!"
        );
    }

    // 👺 Havoc: Prove `distance_squared` can overflow!
    #[test]
    #[should_panic(expected = "👺 Havoc: Vector distance squared overflowed into Infinity!")]
    fn test_havoc_vec2_distance_squared_overflow() {
        let v1 = Vec2::new(f64::MAX, f64::MAX);
        let v2 = Vec2::new(-f64::MAX, -f64::MAX);
        // This will result in an infinite f64
        let sq = v1.distance_squared(v2);

        assert!(
            sq.is_finite(),
            "👺 Havoc: Vector distance squared overflowed into Infinity!"
        );
    }
}
