use macroquad::prelude::*;

pub const R: f32 = 2.0;

/// Returns the 3D position on a Figure-8 Klein Bottle for parameters (u, v).
/// u and v are expected to be in [0, 2*PI].
pub fn klein_bottle(u: f32, v: f32) -> Vec3 {
    let cu = u.cos();
    let su = u.sin();
    let sv = v.sin();

    // Half angle terms
    let cu2 = (u / 2.0).cos();
    let su2 = (u / 2.0).sin();

    // Double angle terms for v
    let s2v = (2.0 * v).sin();

    // Figure-8 immersion equations
    let r_term = R + cu2 * sv - su2 * s2v;

    let x = r_term * cu;
    let y = r_term * su;
    let z = su2 * sv + cu2 * s2v;

    vec3(x, y, z)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn test_periodicity() {
        // The Figure-8 immersion is periodic in v with period 2*PI
        let p0 = klein_bottle(PI, 0.0);
        let p1 = klein_bottle(PI, 2.0 * PI);

        let diff = p0 - p1;
        assert!(diff.length() < 1e-4, "Expected v-periodicity: {:?} vs {:?}", p0, p1);

        // For u, it connects with a twist. u=0 and u=2*PI should meet but maybe with sign flip in some terms?
        // At v=0, sin(v)=0, sin(2v)=0.
        // x = (R + 0 - 0) * cos(u) = R * cos(u)
        // y = (R + 0 - 0) * sin(u) = R * sin(u)
        // z = 0 + 0 = 0
        // So for v=0, it's a circle of radius R in xy plane.
        // u=0 -> (R, 0, 0)
        // u=2*PI -> (R, 0, 0)

        let p_start = klein_bottle(0.0, 0.0);
        let p_end = klein_bottle(2.0 * PI, 0.0);
        assert!((p_start - p_end).length() < 1e-4);
    }
}
