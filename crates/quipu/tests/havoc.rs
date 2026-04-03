use proptest::prelude::*;
use quipu::Cord;

proptest! {
    #[test]
    fn test_quipu_checked_subtraction(a in any::<u64>(), b in any::<u64>()) {
        let c1 = Cord::from(a);
        let c2 = Cord::from(b);

        let result = c1.checked_sub(&c2);

        if a < b {
            assert!(result.is_none(), "Expected underflow to return None");
        } else {
            assert_eq!(result.unwrap().value(), a - b);
        }
    }

    /// 👺 Havoc: Proving `Cord::add` overflow is now handled safely.
    ///
    /// The unsafe `Add` and `Sub` trait implementations have been removed,
    /// so this test now confirms that `checked_add` correctly returns None
    /// instead of panicking or wrapping on overflow.
    #[test]
    fn test_quipu_checked_addition_overflow(
        a in (u64::MAX / 2 + 1)..u64::MAX,
        b in (u64::MAX / 2 + 1)..u64::MAX
    ) {
        let c1 = Cord::from(a);
        let c2 = Cord::from(b);

        let result = c1.checked_add(&c2);
        assert!(result.is_none(), "Expected overflow to return None");
    }
}
