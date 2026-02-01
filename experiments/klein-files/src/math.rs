use glam::{Quat, Vec3};

/// Generates a point on a Figure-8 Klein Bottle surface.
/// u, v are in [0, 2*PI).
/// radius is the "tube" radius scaler (e.g., 2.0).
pub fn klein_bottle(u: f32, v: f32, radius: f32) -> Vec3 {
    let cos_u = u.cos();
    let sin_u = u.sin();
    let cos_u_half = (u * 0.5).cos();
    let sin_u_half = (u * 0.5).sin();

    let sin_v = v.sin();
    let sin_2v = (2.0 * v).sin();

    // The common term
    let temp = radius + cos_u_half * sin_v - sin_u_half * sin_2v;

    let x = temp * cos_u;
    let y = temp * sin_u;
    let z = sin_u_half * sin_v + cos_u_half * sin_2v;

    Vec3::new(x, y, z)
}

/// Projects a 3D point to 2D using perspective projection.
///
/// * `point` - The 3D point to project.
/// * `rotation` - Rotation to apply to the object/world.
/// * `camera_dist` - Distance of the camera from the origin (along Z).
/// * `fov_scale` - Field of view scaler.
pub fn project(
    point: Vec3,
    rotation: Quat,
    camera_dist: f32,
    fov_scale: f32,
) -> Option<(f64, f64)> {
    let rotated = rotation * point;

    // Simple perspective projection
    // Camera is at (0, 0, -camera_dist) looking at (0, 0, 0)
    // Or simpler: Camera at (0, 0, camera_dist) looking at -Z.

    // Let's assume standard OpenGL-ish coordinate system where Z comes out of the screen,
    // or we just move the point by -camera_dist in Z.

    let z_depth = rotated.z + camera_dist;

    if z_depth <= 0.0 {
        return None; // Behind camera
    }

    let scale = fov_scale / z_depth;

    let x = rotated.x * scale;
    let y = rotated.y * scale;

    Some((x as f64, y as f64))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn test_klein_continuity() {
        // u = 0 and u = 2PI should meet (though maybe twisted?)
        // The figure-8 Klein bottle is an immersion, so it intersects itself.
        // Let's just check that 0 and 2PI produce the same point.

        let p0 = klein_bottle(0.0, 0.0, 2.0);
        let p2pi = klein_bottle(2.0 * PI, 0.0, 2.0);

        assert!(
            (p0 - p2pi).length() < 1e-5,
            "Surface should be continuous at u boundary"
        );
    }

    #[test]
    fn test_projection() {
        let p = Vec3::new(1.0, 1.0, 0.0);
        let rot = Quat::IDENTITY;
        // Camera at z=5.
        // Point is at z=0. Depth = 5.
        // fov=10. scale = 2.
        // projected x should be 1.0 * 2 = 2.0.

        let proj = project(p, rot, 5.0, 10.0);
        assert!(proj.is_some());
        let (x, y) = proj.unwrap();
        assert!((x - 2.0).abs() < 1e-5);
        assert!((y - 2.0).abs() < 1e-5);
    }
}
