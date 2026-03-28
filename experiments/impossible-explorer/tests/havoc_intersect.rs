use proptest::prelude::*;

fn intersect_rect(a: (i32, i32, i32, i32), b: (i32, i32, i32, i32)) -> (i32, i32, i32, i32) {
    let ax1 = a.0 as i64;
    let ay1 = a.1 as i64;
    let ax2 = ax1 + a.2.max(0) as i64;
    let ay2 = ay1 + a.3.max(0) as i64;

    let bx1 = b.0 as i64;
    let by1 = b.1 as i64;
    let bx2 = bx1 + b.2.max(0) as i64;
    let by2 = by1 + b.3.max(0) as i64;

    let rx1 = ax1.max(bx1);
    let ry1 = ay1.max(by1);
    let rx2 = ax2.min(bx2);
    let ry2 = ay2.min(by2);

    if rx1 < rx2 && ry1 < ry2 {
        let rw = (rx2 - rx1).min(i32::MAX as i64) as i32;
        let rh = (ry2 - ry1).min(i32::MAX as i64) as i32;
        (rx1 as i32, ry1 as i32, rw, rh)
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
