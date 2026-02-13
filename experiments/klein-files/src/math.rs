/// Figure-8 Immersion of the Klein Bottle
/// u in [0, 2*PI], v in [0, 2*PI]
/// Scale factor `a` controls the major radius (tube distance from origin).
pub fn figure_8_klein(u: f32, v: f32, a: f32) -> [f32; 3] {
    let cu = u.cos();
    let su = u.sin();
    let _cv = v.cos(); // Unused in this formulation?
    let sv = v.sin();
    let _cv2 = (2.0 * v).cos(); // Unused
    let sv2 = (2.0 * v).sin();
    let cu2 = (u / 2.0).cos();
    let su2 = (u / 2.0).sin();

    // Gray's parameterization for Figure-8 immersion
    // x = (a + cos(u/2)sin(v) - sin(u/2)sin(2v)) * cos(u)
    // y = (a + cos(u/2)sin(v) - sin(u/2)sin(2v)) * sin(u)
    // z = sin(u/2)sin(v) + cos(u/2)sin(2v)

    let r = a + cu2 * sv - su2 * sv2;
    let x = r * cu;
    let y = r * su;
    let z = su2 * sv + cu2 * sv2;

    [x, y, z]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn test_klein_points() {
        let a = 3.0;
        for i in 0..20 {
            let u = (i as f32 / 20.0) * 2.0 * PI;
            for j in 0..20 {
                let v = (j as f32 / 20.0) * 2.0 * PI;
                let p = figure_8_klein(u, v, a);
                assert!(!p[0].is_nan(), "x is NaN at u={}, v={}", u, v);
                assert!(!p[1].is_nan(), "y is NaN at u={}, v={}", u, v);
                assert!(!p[2].is_nan(), "z is NaN at u={}, v={}", u, v);

                // Ensure coordinates are within reasonable bounds
                assert!(p[0].abs() < 10.0);
                assert!(p[1].abs() < 10.0);
                assert!(p[2].abs() < 10.0);
            }
        }
    }
}
