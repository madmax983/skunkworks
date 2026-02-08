use macroquad::prelude::Vec3;

pub const R: f32 = 20.0;

/// Maps topological coordinates (u, v) to 3D Cartesian coordinates on a Möbius strip.
/// u: longitudinal coordinate (0 to 2*PI for one full loop, but can go beyond)
/// v: latitudinal coordinate (offset from center line)
pub fn map_to_mobius(u: f32, v: f32) -> Vec3 {
    let half_u = u * 0.5;
    let cos_u = u.cos();
    let sin_u = u.sin();
    let cos_half_u = half_u.cos();
    let sin_half_u = half_u.sin();

    // Parametric equations for Möbius strip
    let x = (R + v * cos_half_u) * cos_u;
    let y = (R + v * cos_half_u) * sin_u;
    let z = v * sin_half_u;

    // We might want to rotate it so it looks nice in default camera?
    // Current: lying in XY plane, twisting into Z.
    // Macroquad Camera3D looks at (0,0,0) usually.
    // This is fine.
    Vec3::new(x, z, y) // Swap Y and Z so it lies in XZ plane (ground), twisting into Y (up)?
}

/// Calculates the surface normal at (u, v).
/// The normal is not well-defined globally (it flips), but locally it is.
/// We compute it using cross product of partial derivatives.
pub fn get_normal(u: f32, v: f32) -> Vec3 {
    // Delta for finite difference (easier than analytic derivation error-prone)
    let du = 0.01;
    let dv = 0.01;

    let p = map_to_mobius(u, v);
    let pu = map_to_mobius(u + du, v);
    let pv = map_to_mobius(u, v + dv);

    let d_u = pu - p;
    let d_v = pv - p;

    d_u.cross(d_v).normalize()
}

/// Calculates the forward tangent at (u, v) along the strip (increasing u).
pub fn get_tangent(u: f32, v: f32) -> Vec3 {
    let du = 0.01;
    let p = map_to_mobius(u, v);
    let pu = map_to_mobius(u + du, v);
    (pu - p).normalize()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mobius_mapping() {
        let p0 = map_to_mobius(0.0, 0.0);
        assert!((p0.x - R).abs() < 1e-5);
        assert!(p0.y.abs() < 1e-5);
        assert!(p0.z.abs() < 1e-5);

        let p2pi = map_to_mobius(std::f32::consts::PI * 2.0, 0.0);
        // Note: u=2PI, cos(u/2) = cos(PI) = -1.
        // x = (R + 0 * -1) * 1 = R.
        // y = 0 * 0 = 0.
        // z = 0.
        assert!((p2pi.x - R).abs() < 1e-5);
        assert!(p2pi.y.abs() < 1e-5);
        assert!(p2pi.z.abs() < 1e-5);

        let p0_v1 = map_to_mobius(0.0, 1.0);
        // x = (R+1) * 1 = R+1.
        assert!((p0_v1.x - (R + 1.0)).abs() < 1e-5);

        let p2pi_v1 = map_to_mobius(std::f32::consts::PI * 2.0, 1.0);
        // x = (R - 1) * 1 = R - 1.
        assert!((p2pi_v1.x - (R - 1.0)).abs() < 1e-5);
    }

    #[test]
    fn test_normal_flip() {
        let n0 = get_normal(0.0, 0.0);
        let n2pi = get_normal(std::f32::consts::PI * 2.0, 0.0);
        assert!((n0 + n2pi).length() < 0.1);
    }
}
