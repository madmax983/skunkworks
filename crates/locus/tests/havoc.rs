#[cfg(test)]
mod tests {
    use locus::vec4::Vec4;
    use std::f32;

    // 👺 Havoc: Prove `length_squared` and `distance_squared` can overflow!
    // 🔒 Warden: Fixed. It now safely clamps to MAX.
    #[test]
    fn test_havoc_length_squared_overflow() {
        let v = Vec4::new(f32::MAX, f32::MAX, f32::MAX, f32::MAX);
        let sq = v.length_squared();

        assert!(
            sq.is_finite(),
            "👺 Havoc: Vector length squared overflowed into Infinity!"
        );
    }
}
