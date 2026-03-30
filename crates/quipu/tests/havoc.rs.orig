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

    /// 👺 Havoc: Proving `Cord::add` is fragile against integer overflow.
    ///
    /// The `Cord` struct implements the `Add` trait by simply doing `self.value() + rhs.value()`.
    /// Since the underlying value is a `u64`, adding two large cords (e.g., each > u64::MAX / 2)
    /// will cause a panic due to integer overflow in debug mode, or wrap silently in release mode.
    ///
    /// 🧨 **The Trigger:** Two `u64` values that sum to more than `u64::MAX`.
    #[test]
    #[should_panic(expected = "Quipu addition resulted in overflow")]
    fn test_quipu_addition_overflow(
        a in (u64::MAX / 2 + 1)..u64::MAX,
        b in (u64::MAX / 2 + 1)..u64::MAX
    ) {
        let c1 = Cord::from(a);
        let c2 = Cord::from(b);
        let _ = c1 + c2;
    }
}
