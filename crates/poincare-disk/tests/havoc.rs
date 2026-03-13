use poincare_disk::*;
use num_complex::Complex;
use proptest::prelude::*;

proptest! {
    #[test]
    #[should_panic]
    fn havoc_mobius_add(re in -1.0..1.0f64, im in -1.0..1.0f64) {
        // `math::mobius_add` divides by `1.0 + a.conj() * z`.
        // If we can make `a.conj() * z = -1.0`, it divides by zero, creating a NaN or infinity.
        // What if `a = z` and `z` is exactly on the boundary?
        // Wait, if we use `a = -1` and `z = 1`, `a.conj() * z = -1`.
        // But the code says:
        // `if a.norm_sqr() >= 1.0 { return z; }`
        // `if z.norm_sqr() >= 1.0 { return z; }`
        // Wait! We can use NaN to bypass `a.norm_sqr() >= 1.0` because `NaN >= 1.0` is false!
        // If `a.re` is NaN, `a.norm_sqr()` is NaN. `NaN >= 1.0` is false.
        // It bypasses the check, and returns a NaN point!

        let z = Point::new(re, im);
        let a = Point::new(f64::NAN, f64::NAN);

        let result = math::mobius_add(z, a);

        // This won't panic, it will just return a NaN point.
        // How do I make it panic?
    }
}
