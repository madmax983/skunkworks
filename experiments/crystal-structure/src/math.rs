use nalgebra::{Point3, Rotation3};

pub fn rotate_point(point: Point3<f64>, rotation: Rotation3<f64>) -> Point3<f64> {
    rotation * point
}

pub fn project_point(point: Point3<f64>, scale: f64, offset_x: f64, offset_y: f64) -> (f64, f64) {
    // Simple orthographic projection for now? Or perspective?
    // Let's do simple perspective: x / (z + dist)

    let dist = 10.0; // Distance from camera
    let z = point.z + dist;

    if z <= 0.1 {
        // Too close or behind
        return (offset_x, offset_y);
    }

    let factor = scale / z;
    let x = point.x * factor + offset_x;
    let y = point.y * factor + offset_y; // Invert Y? Terminal Y is down. Math Y is usually up.

    (x, y)
}

pub fn distance_to_plane(point: Point3<f64>, h: f64, k: f64, l: f64, d: f64) -> f64 {
    // Distance from point (x,y,z) to plane hx + ky + lz = d
    // D = |ax + by + cz - d| / sqrt(a^2 + b^2 + c^2)
    // Here a=h, b=k, c=l.

    let num = (h * point.x + k * point.y + l * point.z - d).abs();
    let den = (h*h + k*k + l*l).sqrt();

    if den == 0.0 {
        return 0.0; // Undefined plane, everything is on it?
    }

    num / den
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_distance_to_plane() {
        let p = Point3::new(1.0, 1.0, 1.0);
        // Plane x=0 (h=1, k=0, l=0, d=0)
        let dist = distance_to_plane(p, 1.0, 0.0, 0.0, 0.0);
        assert!((dist - 1.0).abs() < 1e-6);

        // Plane z=2 (h=0, k=0, l=1, d=2)
        let dist = distance_to_plane(p, 0.0, 0.0, 1.0, 2.0);
        assert!((dist - 1.0).abs() < 1e-6); // |1 - 2| = 1
    }

    #[test]
    fn test_rotation() {
         use nalgebra::Vector3;
         let p = Point3::new(1.0, 0.0, 0.0);
         let rot = Rotation3::from_axis_angle(&Vector3::z_axis(), PI / 2.0);
         let p_rot = rotate_point(p, rot);

         // Should be (0, 1, 0)
         assert!((p_rot.x).abs() < 1e-6);
         assert!((p_rot.y - 1.0).abs() < 1e-6);
    }
}
