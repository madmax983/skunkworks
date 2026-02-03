use proptest::prelude::*;
use tui_shared::math::Vec2;

proptest! {
    #[test]
    fn test_vec2_limit_finite(
        x in prop_oneof![
            Just(f64::MAX),
            Just(f64::MIN),
            Just(1.7e308),
            Just(-1.7e308),
            any::<f64>()
        ],
        y in prop_oneof![
            Just(f64::MAX),
            Just(f64::MIN),
            Just(1.7e308),
            Just(-1.7e308),
            any::<f64>()
        ]
    ) {
        if x.is_finite() && y.is_finite() {
             let v = Vec2::new(x, y);
             let limit_val = 1.0;
             let limited = v.limit(limit_val);

             prop_assert!(!limited.x.is_nan(), "Limited X is NaN for input {:?}", v);
             prop_assert!(!limited.y.is_nan(), "Limited Y is NaN for input {:?}", v);

             let mag = limited.magnitude();

             // If original is huge, result should be capped at limit_val.
             // But if `magnitude()` overflows to Infinity, `normalize()` divides by Infinity -> 0.
             // So we get 0 vector. Which has magnitude 0.
             // 0 != 1.0.

             // If v is (MAX, 0), mag is Inf. Inf > 1.0.
             // limited should be (1.0, 0). mag should be 1.0.

             if v.magnitude() > limit_val {
                 prop_assert!((mag - limit_val).abs() < 1e-4, "Magnitude {} != {} for input {:?}", mag, limit_val, v);
             }
        }
    }
}
