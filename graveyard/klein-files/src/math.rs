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

    #[test]
    fn test_klein_topology_gluing() {
        let a = 3.0;
        let epsilon = 1e-4;

        // Check that (u=0, v) connects to (u=2PI, 2PI-v)
        // Note: 2PI is numerically approximate, so we use close values.
        let u_start = 0.0;
        let u_end = 2.0 * PI;

        for i in 0..20 {
            let v = (i as f32 / 20.0) * 2.0 * PI;
            let p_start = figure_8_klein(u_start, v, a);

            // The gluing condition: (2PI, 2PI - v)
            let v_flipped = 2.0 * PI - v;
            let p_end = figure_8_klein(u_end, v_flipped, a);

            let dist_sq = (p_start[0] - p_end[0]).powi(2)
                + (p_start[1] - p_end[1]).powi(2)
                + (p_start[2] - p_end[2]).powi(2);

            assert!(
                dist_sq < epsilon,
                "Gluing failed at v={}: dist_sq={} (expected 0). p_start={:?}, p_end={:?}",
                v,
                dist_sq,
                p_start,
                p_end
            );
        }
    }

    #[test]
    fn test_klein_discontinuity() {
        let a = 3.0;
        // Check that (u=0, v) is NOT generally (u=2PI, v)
        // This confirms the Möbius twist is necessary.
        let u_start = 0.0;
        let u_end = 2.0 * PI;
        let v = PI / 2.0; // sin(v) = 1

        let p_start = figure_8_klein(u_start, v, a);
        let p_end = figure_8_klein(u_end, v, a);

        let dist_sq = (p_start[0] - p_end[0]).powi(2)
            + (p_start[1] - p_end[1]).powi(2)
            + (p_start[2] - p_end[2]).powi(2);

        // At v=PI/2:
        // r(0) = a + sin(PI/2) = a + 1
        // r(2PI) = a - sin(PI/2) = a - 1
        // x(0) = a + 1
        // x(2PI) = a - 1
        // dist should be roughly 2.0
        assert!(
            dist_sq > 1.0,
            "Expected discontinuity at v={}, got dist_sq={}",
            v,
            dist_sq
        );
    }
}
