use glam::{Mat4, Vec3, Vec4};

/// Represents a rotation in 4D space.
/// In 4D, rotations occur in planes (XY, XZ, XW, YZ, YW, ZW).
pub struct Rotor4 {
    // We could use a full 4x4 matrix, or a Bivector representation.
    // For simplicity, we'll just compose rotation matrices.
    matrix: Mat4,
}

impl Rotor4 {
    #[allow(dead_code)]
    pub fn identity() -> Self {
        Self {
            matrix: Mat4::IDENTITY,
        }
    }

    pub fn new(xw: f32, yw: f32, zw: f32, xy: f32, xz: f32, yz: f32) -> Self {
        let mut m = Mat4::IDENTITY;

        // XW plane rotation (mixes X and W)
        // x' = x cos - w sin
        // w' = x sin + w cos
        if xw != 0.0 {
            let (s, c) = xw.sin_cos();
            // Mat4 is column-major.
            // Row 0: c, 0, 0, -s
            // Row 3: s, 0, 0, c
            let rot = Mat4::from_cols_array(&[
                c, 0.0, 0.0, s, // Col 0
                0.0, 1.0, 0.0, 0.0, // Col 1
                0.0, 0.0, 1.0, 0.0, // Col 2
                -s, 0.0, 0.0, c, // Col 3
            ]);
            m = m * rot;
        }

        // YW plane rotation
        if yw != 0.0 {
            let (s, c) = yw.sin_cos();
            let rot = Mat4::from_cols_array(&[
                1.0, 0.0, 0.0, 0.0, 0.0, c, 0.0, s, 0.0, 0.0, 1.0, 0.0, 0.0, -s, 0.0, c,
            ]);
            m = m * rot;
        }

        // ZW plane rotation
        if zw != 0.0 {
            let (s, c) = zw.sin_cos();
            let rot = Mat4::from_cols_array(&[
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, c, s, 0.0, 0.0, -s, c,
            ]);
            m = m * rot;
        }

        // XY (Z-axis rotation in 3D)
        if xy != 0.0 {
            m = m * Mat4::from_rotation_z(xy);
        }

        // XZ (Y-axis rotation in 3D, but sign might differ)
        if xz != 0.0 {
            // In 3D, rotation around Y mixes X and Z.
            // x' = x cos + z sin
            // z' = -x sin + z cos
            // Mat4::from_rotation_y does this.
            m = m * Mat4::from_rotation_y(xz);
        }

        // YZ (X-axis rotation in 3D)
        if yz != 0.0 {
            m = m * Mat4::from_rotation_x(yz);
        }

        Self { matrix: m }
    }

    pub fn transform(&self, v: Vec4) -> Vec4 {
        self.matrix * v
    }
}

/// Project 4D point to 3D.
/// camera_w: The position of the camera along the W axis (usually negative or positive).
/// We assume the camera is at (0,0,0, camera_w) looking towards origin.
pub fn project_4d_to_3d(v: Vec4, camera_w: f32) -> Vec3 {
    // Perspective projection:
    // w' = 1 / (w - camera_w) ? No, standard perspective division.
    // If camera is at W=C, and we look at W=0.
    // scale = 1 / (1 - v.w / camera_w) if C is "distance".
    // Let's assume camera is at W=-distance. Looking at +W.
    // scale = distance / (distance + v.w).

    let distance = camera_w.abs();
    // Avoid division by zero
    let scale = if (distance + v.w).abs() < 1e-4 {
        1000.0 // Clipping or massive scale
    } else {
        distance / (distance + v.w)
    };

    Vec3::new(v.x * scale, v.y * scale, v.z * scale)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xw_rotation() {
        let v = Vec4::new(1.0, 0.0, 0.0, 0.0);
        let rotor = Rotor4::new(std::f32::consts::FRAC_PI_2, 0.0, 0.0, 0.0, 0.0, 0.0);
        let rotated = rotor.transform(v);
        // X -> W.
        // x' = 1*0 - 0*1 = 0
        // w' = 1*1 + 0*0 = 1
        assert!((rotated.x).abs() < 1e-6);
        assert!((rotated.w - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_projection() {
        let v = Vec4::new(1.0, 1.0, 1.0, 0.0);
        let p = project_4d_to_3d(v, -5.0);
        // w=0, so scale should be 1.0.
        assert!((p.x - 1.0).abs() < 1e-6);

        let v2 = Vec4::new(1.0, 1.0, 1.0, 2.5);
        let p2 = project_4d_to_3d(v2, -5.0); // distance = 5.0. scale = 5 / (5 + 2.5) = 5/7.5 = 2/3.
        assert!((p2.x - 0.666666).abs() < 1e-5);
    }
}
