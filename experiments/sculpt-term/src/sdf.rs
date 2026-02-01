use crate::math::Vec3;

pub fn sd_sphere(p: Vec3, r: f64) -> f64 {
    p.length() - r
}

pub fn sd_box(p: Vec3, b: Vec3) -> f64 {
    let q = p.abs() - b;
    let outside = q.max(Vec3::ZERO).length();
    let inside = q.x.max(q.y).max(q.z).min(0.0);
    outside + inside
}

pub fn sd_torus(p: Vec3, t: Vec3) -> f64 {
    // t.x is major radius, t.y is minor radius
    // We need 2D length for xz.
    let q_xz = (p.x * p.x + p.z * p.z).sqrt() - t.x;
    let q_y = p.y;
    (q_xz * q_xz + q_y * q_y).sqrt() - t.y
}

pub fn op_union(d1: f64, d2: f64) -> f64 {
    d1.min(d2)
}

pub fn op_smooth_union(d1: f64, d2: f64, k: f64) -> f64 {
    let h = (0.5 + 0.5 * (d2 - d1) / k).clamp(0.0, 1.0);
    // mix(d2, d1, h) = d2 * (1-h) + d1 * h
    let mix = d2 * (1.0 - h) + d1 * h;
    mix - k * h * (1.0 - h)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::Vec3;

    #[test]
    fn test_sphere() {
        let p = Vec3::new(0.0, 2.0, 0.0);
        assert_eq!(sd_sphere(p, 1.0), 1.0); // 2 - 1 = 1
    }

    #[test]
    fn test_box() {
        let p = Vec3::new(2.0, 0.0, 0.0);
        let b = Vec3::new(1.0, 1.0, 1.0);
        // p is (2,0,0), box is (-1..1, -1..1, -1..1).
        // q = (2,0,0) - (1,1,1) = (1, -1, -1).
        // max(q, 0) = (1, 0, 0). length = 1.
        assert_eq!(sd_box(p, b), 1.0);
    }
}
