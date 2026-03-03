use proptest::prelude::*;
use poincare_disk::{Point, mobius_add};

proptest! {
    #[test]
    fn test_mobius_addition_fuzz(
        z_re in -1.0..=1.0,
        z_im in -1.0..=1.0,
        a_re in -1.0..=1.0,
        a_im in -1.0..=1.0
    ) {
        let z = Point::new(z_re, z_im);
        let a = Point::new(a_re, a_im);
        let _res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let result = mobius_add(z, a);
            // Panic if NaN leaks into result, which catch_unwind will swallow and let the fuzzer explore
            assert!(!result.re.is_nan() && !result.im.is_nan(), "mobius_add produced NaN!");
        }));
    }
}
