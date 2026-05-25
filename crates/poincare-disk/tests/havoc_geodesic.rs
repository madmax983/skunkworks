use poincare_disk::{Geodesic, Point};
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig { failure_persistence: None, cases: 10000, .. ProptestConfig::default() })]

    // 👺 Havoc: Test `euclidean_circle` for panics when inputs are collinear with origin!
    // Or when division by zero produces NaNs.
    // The Red phase: Write a test that fails natively without any subprocess/exit tricks.
    #[test]
    #[should_panic(expected = "Havoc WRECKAGE: NaN propagated to")]
    fn havoc_test_geodesic_nan_poison(
        p1_re in prop_oneof![Just(f64::NAN), proptest::num::f64::ANY],
        p1_im in proptest::num::f64::ANY,
        p2_re in proptest::num::f64::ANY,
        p2_im in proptest::num::f64::ANY,
    ) {
        let p1 = Point::new(p1_re, p1_im);
        let p2 = Point::new(p2_re, p2_im);
        let geo = Geodesic::new(p1, p2);

        let res = geo.euclidean_circle();
        if let Some((center, radius)) = res {
            // Panic if NaN propagates to radius!
            assert!(!radius.is_nan(), "Havoc WRECKAGE: NaN propagated to");
            assert!(!center.re.is_nan(), "Havoc WRECKAGE: NaN propagated to");
            assert!(!center.im.is_nan(), "Havoc WRECKAGE: NaN propagated to");
        }
    }
}
