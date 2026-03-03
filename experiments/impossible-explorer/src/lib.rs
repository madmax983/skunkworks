pub mod safe_gl;

pub fn intersect_rect(a: (i32, i32, i32, i32), b: (i32, i32, i32, i32)) -> (i32, i32, i32, i32) {
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
