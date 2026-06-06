use poincare_disk::{Point, mobius_add, mobius_sub};
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig { failure_persistence: None, cases: 1000, .. ProptestConfig::default() })]

    #[test]
    fn havoc_poincare_disk_mobius_add(
        z_re in prop_oneof![Just(f64::INFINITY), Just(f64::NEG_INFINITY), Just(f64::NAN), proptest::num::f64::ANY],
        z_im in prop_oneof![Just(f64::INFINITY), Just(f64::NEG_INFINITY), Just(f64::NAN), proptest::num::f64::ANY],
        a_re in prop_oneof![Just(f64::INFINITY), Just(f64::NEG_INFINITY), Just(f64::NAN), proptest::num::f64::ANY],
        a_im in prop_oneof![Just(f64::INFINITY), Just(f64::NEG_INFINITY), Just(f64::NAN), proptest::num::f64::ANY],
    ) {
        let z = Point::new(z_re, z_im);
        let a = Point::new(a_re, a_im);

        let _ = mobius_add(z, a);
    }

    #[test]
    fn havoc_poincare_disk_mobius_sub(
        z_re in prop_oneof![Just(f64::INFINITY), Just(f64::NEG_INFINITY), Just(f64::NAN), proptest::num::f64::ANY],
        z_im in prop_oneof![Just(f64::INFINITY), Just(f64::NEG_INFINITY), Just(f64::NAN), proptest::num::f64::ANY],
        a_re in prop_oneof![Just(f64::INFINITY), Just(f64::NEG_INFINITY), Just(f64::NAN), proptest::num::f64::ANY],
        a_im in prop_oneof![Just(f64::INFINITY), Just(f64::NEG_INFINITY), Just(f64::NAN), proptest::num::f64::ANY],
    ) {
        let z = Point::new(z_re, z_im);
        let a = Point::new(a_re, a_im);

        let _ = mobius_sub(z, a);
    }
}
