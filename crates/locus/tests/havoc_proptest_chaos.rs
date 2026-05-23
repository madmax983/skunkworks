use locus::Vec2;
use locus::Vec4;
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_havoc_length_squared_overflow_proptest(
        x in (f32::MAX / 2.0)..=f32::MAX,
        y in (f32::MAX / 2.0)..=f32::MAX,
        z in (f32::MAX / 2.0)..=f32::MAX,
        w in (f32::MAX / 2.0)..=f32::MAX,
    ) {
        let v = Vec4::new(x, y, z, w);
        let sq = v.length_squared();
        assert!(sq.is_finite(), "👺 Havoc: Vector length squared overflowed into Infinity!");
    }

    #[test]
    fn test_havoc_vec2_magnitude_squared_overflow_proptest(
        x in (f64::MAX / 2.0)..=f64::MAX,
        y in (f64::MAX / 2.0)..=f64::MAX,
    ) {
        let v = Vec2::new(x, y);
        let sq = v.magnitude_squared();
        assert!(sq.is_finite(), "👺 Havoc: Vector magnitude squared overflowed into Infinity!");
    }
}
