#[cfg(test)]
mod tests {
    use locus::vec4::Vec4;
    use std::f32;

    // 👺 Havoc: Prove `length_squared` and `distance_squared` can overflow!
    #[test]
    #[should_panic(expected = "👺 Havoc: Vector length squared overflowed into Infinity!")]
    fn test_havoc_length_squared_overflow() {
        let v = Vec4::new(f32::MAX, f32::MAX, f32::MAX, f32::MAX);
        // This will result in an infinite f32
        let sq = v.length_squared();

        // By checking if it's infinite, we know it overflowed instead of safely clamping.
        // We assert that it *must be finite*, and when that fails, it proves the vulnerability!
        assert!(
            sq.is_finite(),
            "👺 Havoc: Vector length squared overflowed into Infinity!"
        );
    }
}
