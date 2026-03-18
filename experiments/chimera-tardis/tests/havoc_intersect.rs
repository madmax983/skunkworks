use proptest::prelude::*;

fn intersect_rect(a: (i32, i32, i32, i32), b: (i32, i32, i32, i32)) -> (i32, i32, i32, i32) {
    let ax1 = a.0;
    let ay1 = a.1;
    let ax2 = a.0.saturating_add(a.2.max(0));
    let ay2 = a.1.saturating_add(a.3.max(0));

    let bx1 = b.0;
    let by1 = b.1;
    let bx2 = b.0.saturating_add(b.2.max(0));
    let by2 = b.1.saturating_add(b.3.max(0));

    let rx1 = ax1.max(bx1);
    let ry1 = ay1.max(by1);
    let rx2 = ax2.min(bx2);
    let ry2 = ay2.min(by2);

    if rx1 < rx2 && ry1 < ry2 {
        (rx1, ry1, rx2.saturating_sub(rx1), ry2.saturating_sub(ry1))
    } else {
        (0, 0, 0, 0)
    }
}

proptest! {
    #[test]
    fn havoc_intersect_bounds_check(
        ax in any::<i32>(), ay in any::<i32>(), aw in 0..i32::MAX, ah in 0..i32::MAX,
        bx in any::<i32>(), by in any::<i32>(), bw in 0..i32::MAX, bh in 0..i32::MAX
    ) {
        let (rx, _ry, rw, _rh) = intersect_rect((ax, ay, aw, ah), (bx, by, bw, bh));

        let ax2 = ax.saturating_add(aw);
        let bx2 = bx.saturating_add(bw);
        let max_x2 = ax2.min(bx2);

        let rx2 = rx.saturating_add(rw);

        if rw > 0 {
            assert!(rx2 <= max_x2, "Resulting x2 {} exceeded max allowed x2 {}", rx2, max_x2);

            let min_x1 = ax.max(bx);
            assert!(rx >= min_x1, "Resulting rx {} was smaller than max start {}", rx, min_x1);

            assert!(rw <= aw, "Resulting width {} was larger than input a width {}", rw, aw);
            assert!(rw <= bw, "Resulting width {} was larger than input b width {}", rw, bw);
        } else {
            assert_eq!(rx, 0);
            assert_eq!(rw, 0);
        }
    }
}
