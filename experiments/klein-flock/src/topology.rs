use glam::Vec3;

/// Calculates a point on the Figure-8 immersion of the Klein bottle.
///
/// # Arguments
/// * `u` - The longitudinal parameter [0, 2*PI)
/// * `v` - The cross-sectional parameter [0, 2*PI)
/// * `radius` - The radius of the tube.
pub fn figure_8_klein(u: f32, v: f32, radius: f32) -> Vec3 {
    let cos_u = u.cos();
    let sin_u = u.sin();
    let cos_u_half = (u * 0.5).cos();
    let sin_u_half = (u * 0.5).sin();
    let sin_v = v.sin();
    let sin_2v = (2.0 * v).sin();

    // The cross-section radius term
    let r_term = radius + cos_u_half * sin_v - sin_u_half * sin_2v;

    let x = r_term * cos_u;
    let y = r_term * sin_u;
    let z = sin_u_half * sin_v + cos_u_half * sin_2v;

    Vec3::new(x, y, z)
}
