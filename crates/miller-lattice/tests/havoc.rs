use proptest::prelude::*;

proptest! {
    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn havoc_test_crystal_panic(
        val in any::<i32>(),
    ) {
        let parent_pos: i32 = i32::MAX;
        let mut x = parent_pos;
        // prevent const eval
        x += val.abs();
        x += 1;
        assert!(x > 0);
    }
}
