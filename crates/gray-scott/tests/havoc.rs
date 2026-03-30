use gray_scott::GrayScott;
use proptest::prelude::*;

proptest! {
    /// 👺 Havoc: Proving GrayScott is fragile when explicitly attempting to bypass the capacity limit.
    #[test]
    #[should_panic(expected = "capacity overflow")]
    fn havoc_gray_scott_init_overflow(width in usize::MAX/2..=usize::MAX, height in 2..10usize) {
        let _ = GrayScott::new(width, height);
    }

    /// 👺 Havoc: Proving `get_index` is vulnerable to Y-coordinate overflow because it lacks explicit width bounding when tested blindly.
    #[test]
    #[should_panic(expected = "coordinate out of bounds")]
    fn havoc_gray_scott_scanline_wrapping(x in 10..=100usize, y in 0..=100usize) {
        let gs = GrayScott::new(10, 10);
        let _ = gs.get_index(x, y);
    }
}
