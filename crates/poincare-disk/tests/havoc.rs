use poincare_disk::{mobius_add, mobius_sub, Point};
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig { failure_persistence: None, cases: 10000, .. ProptestConfig::default() })]

    // 👺 Havoc: We must find inputs where Poincare math returns a NaN or Infinity!
    // Since `norm_sqr() < 1.0` avoids division by zero, let's explicitly inject NaNs
    // or Infinities as inputs to see if it gracefully returns them or panics elsewhere.
    // Also, we can use `f64::ANY` to test ALL float values (including NaNs/Infs).
    #[test]
    #[should_panic]
    fn test_mobius_add_fuzzing(
        z_re in proptest::num::f64::ANY,
        z_im in proptest::num::f64::ANY,
        a_re in proptest::num::f64::ANY,
        a_im in proptest::num::f64::ANY,
    ) {
        let z = Point::new(z_re, z_im);
        let a = Point::new(a_re, a_im);

        let res = mobius_add(z, a);

        // Let's assert that the result is NEVER NaN.
        // The current implementation probably propagates NaNs because `norm_sqr() >= 1.0`
        // is false if it's NaN! Wait, `NaN >= 1.0` is false, so it falls through to division
        // and produces NaN.
        prop_assert!(!res.re.is_nan(), "Havoc 👺: Math failed! NaN produced.");
        prop_assert!(!res.im.is_nan(), "Havoc 👺: Math failed! NaN produced.");
    }

    #[test]
    #[should_panic]
    fn test_mobius_sub_fuzzing(
        z_re in proptest::num::f64::ANY,
        z_im in proptest::num::f64::ANY,
        a_re in proptest::num::f64::ANY,
        a_im in proptest::num::f64::ANY,
    ) {
        let z = Point::new(z_re, z_im);
        let a = Point::new(a_re, a_im);

        let res = mobius_sub(z, a);

        prop_assert!(!res.re.is_nan(), "Havoc 👺: Math failed! NaN produced.");
        prop_assert!(!res.im.is_nan(), "Havoc 👺: Math failed! NaN produced.");
    }
}
