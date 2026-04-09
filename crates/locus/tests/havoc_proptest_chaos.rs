use locus::vec4::Vec4;
use proptest::prelude::*;

proptest! {
    #[test]
    #[should_panic]
    fn test_havoc_distance_squared_overflow(
        x1 in (f32::MAX / 2.0)..=f32::MAX,
        y1 in (f32::MAX / 2.0)..=f32::MAX,
        z1 in (f32::MAX / 2.0)..=f32::MAX,
        w1 in (f32::MAX / 2.0)..=f32::MAX,
        x2 in f32::MIN..=(f32::MIN / 2.0),
        y2 in f32::MIN..=(f32::MIN / 2.0),
        z2 in f32::MIN..=(f32::MIN / 2.0),
        w2 in f32::MIN..=(f32::MIN / 2.0),
    ) {
        let v1 = Vec4::new(x1, y1, z1, w1);
        let v2 = Vec4::new(x2, y2, z2, w2);

        let dist_sq = v1.distance_squared(v2);
        assert!(dist_sq.is_finite(), "👺 Havoc: Vector distance squared overflowed into Infinity!");
    }
}
