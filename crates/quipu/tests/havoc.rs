use proptest::prelude::*;
use quipu::Cord;

proptest! {
    #[test]
    #[should_panic]
    fn test_quipu_subtraction_crash(a in any::<u64>(), b in any::<u64>()) {
        let c1 = Cord::from(a);
        let c2 = Cord::from(b);
        // This will panic when a < b, exposing the internal panic in Quipu's Sub trait.
        // It's a genuine crash/fragility point.
        let _ = c1 - c2;
    }
}
