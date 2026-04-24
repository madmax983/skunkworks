pub fn intersect_rect(a: (i32, i32, i32, i32), b: (i32, i32, i32, i32)) -> (i32, i32, i32, i32) {
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
fn main() {
    let p = (-16384, -16384, 32768, 32768);
    let r1 = intersect_rect((i32::MAX - 100, 0, 200, 100), p);
    println!("{:?}", r1);
}
