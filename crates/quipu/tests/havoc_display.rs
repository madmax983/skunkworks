use proptest::prelude::*;
use quipu::Cord;

proptest! {
    /// 👺 Havoc: Test formatting for deeply nested or arbitrary large values.
    #[test]
    fn test_display_does_not_panic(val in any::<u64>()) {
        let cord = Cord::from(val);
        let _ = format!("{}", cord);
    }
}
