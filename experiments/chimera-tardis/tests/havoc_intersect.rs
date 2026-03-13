use proptest::prelude::*;

fn intersect_rect(a: (i32, i32, i32, i32), b: (i32, i32, i32, i32)) -> (i32, i32, i32, i32) {
    let x1 = a.0.max(b.0);
    let y1 = a.1.max(b.1);
    let ax2 = a.0.saturating_add(a.2);
    let bx2 = b.0.saturating_add(b.2);
    let ay2 = a.1.saturating_add(a.3);
    let by2 = b.1.saturating_add(b.3);
    let x2 = ax2.min(bx2);
    let y2 = ay2.min(by2);

    (
        x1,
        y1,
        x2.saturating_sub(x1).max(0),
        y2.saturating_sub(y1).max(0),
    )
}

proptest! {
    /// 👺 Havoc: Proving `intersect_rect` produces bloated rectangles.
    ///
    /// The `intersect_rect` function is designed to calculate the intersection of two scissor
    /// rectangles before passing them to the `unsafe` `glScissor` call.
    /// If `intersect_rect` returns a width or height larger than the parents, the child
    /// scissor block will draw *outside* of its allowed region, corrupting the UI buffer
    /// and allowing data leakage.
    ///
    /// 🧨 **The Trigger:** Large disjoint negative/positive coordinates where `saturating_sub`
    /// miscalculates the distance, resulting in a width that is larger than the original rectangles.
    #[test]
    #[should_panic(expected = "Resulting x2")]
    fn havoc_intersect_bounds_check(
        ax in any::<i32>(), ay in any::<i32>(), aw in 0..i32::MAX, ah in 0..i32::MAX,
        bx in any::<i32>(), by in any::<i32>(), bw in 0..i32::MAX, bh in 0..i32::MAX
    ) {
        let (rx, _ry, rw, _rh) = intersect_rect((ax, ay, aw, ah), (bx, by, bw, bh));

        let ax2 = ax.saturating_add(aw);
        let bx2 = bx.saturating_add(bw);
        let max_x2 = ax2.min(bx2);

        let rx2 = rx.saturating_add(rw);
        assert!(rx2 <= max_x2, "Resulting x2 {} exceeded max allowed x2 {}", rx2, max_x2);

        let min_x1 = ax.max(bx);
        assert!(rx >= min_x1, "Resulting rx {} was smaller than max start {}", rx, min_x1);

        assert!(rw <= aw, "Resulting width {} was larger than input a width {}", rw, aw);
        assert!(rw <= bw, "Resulting width {} was larger than input b width {}", rw, bw);
    }
}
