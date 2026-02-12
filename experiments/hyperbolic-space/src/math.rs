use cgmath::{InnerSpace, Matrix4, SquareMatrix, Vector3, Vector4, Zero};

pub type Mat4 = Matrix4<f32>;
pub type Vec4 = Vector4<f32>;
pub type Vec3 = Vector3<f32>;

/// Returns a boost matrix for a displacement `delta` in tangent space.
/// In H3 model (hyperboloid w^2 - x^2 - y^2 - z^2 = 1), origin is (0,0,0,1).
/// A boost by `d` along `v` direction:
/// w' = w cosh d + (v.x x + v.y y + v.z z) sinh d
/// x' = x + v.x (w sinh d + (cosh d - 1)(v.x x + ...)) ?
///
/// Standard formula for boost by velocity vector `v` (where |v| = tanh(rapidity)):
/// But here `delta` is the displacement vector in the tangent space at origin.
/// Direction `n = delta.normalize()`, distance `rho = delta.magnitude()`.
/// Boost B(n, rho).
pub fn boost(delta: Vec3) -> Mat4 {
    let rho = delta.magnitude();
    if rho < 1e-6 {
        return Mat4::identity();
    }
    let n = delta / rho;
    let c = rho.cosh();
    let s = rho.sinh();

    // Lorentz boost matrix in Minkowski space (-+++ metric or +---? We want w^2 - x^2 ... = 1).
    // Let's use coordinates (x, y, z, w).
    // Metric diag(-1, -1, -1, 1).
    // Origin is (0,0,0,1).
    // Boost moves origin to (n*s, c).

    // Matrix M such that M * [0,0,0,1] = [nx*s, ny*s, nz*s, c]
    // And M preserves Minkowski metric.

    // Rows of M:
    // col 3 (input w) must be output column for w input.
    // M =
    // [ 1 + (c-1)nx*nx   (c-1)nx*ny     (c-1)nx*nz     nx*s ]
    // [ (c-1)ny*nx       1 + (c-1)ny*ny (c-1)ny*nz     ny*s ]
    // [ (c-1)nz*nx       (c-1)nz*ny     1 + (c-1)nz*nz nz*s ]
    // [ nx*s             ny*s           nz*s           c    ]

    // If we use (x,y,z,w) vector.

    let nx = n.x;
    let ny = n.y;
    let nz = n.z;

    Mat4::new(
        1.0 + (c - 1.0) * nx * nx,
        (c - 1.0) * nx * ny,
        (c - 1.0) * nx * nz,
        nx * s,
        (c - 1.0) * ny * nx,
        1.0 + (c - 1.0) * ny * ny,
        (c - 1.0) * ny * nz,
        ny * s,
        (c - 1.0) * nz * nx,
        (c - 1.0) * nz * ny,
        1.0 + (c - 1.0) * nz * nz,
        nz * s,
        nx * s,
        ny * s,
        nz * s,
        c,
    )
}

/// Minkowski dot product: w1*w2 - x1*x2 - y1*y2 - z1*z2
/// We use (x,y,z,w) layout, so w is index 3.
pub fn minkowski_dot(a: Vec4, b: Vec4) -> f32 {
    a.w * b.w - a.x * b.x - a.y * b.y - a.z * b.z
}

#[cfg(test)]
mod tests {
    use super::*;
    use cgmath::assert_relative_eq;

    #[test]
    fn test_identity_boost() {
        let b = boost(Vec3::zero());
        assert_relative_eq!(b.w.w, 1.0);
        assert_relative_eq!(b.x.x, 1.0);
    }

    #[test]
    fn test_metric_preservation() {
        let v = Vec4::new(0.0, 0.0, 0.0, 1.0); // Origin
        let displacement = Vec3::new(0.5, -0.2, 0.8);
        let m = boost(displacement);
        let v_prime = m * v;

        let mag_sq = minkowski_dot(v_prime, v_prime);
        // Should be 1.0
        assert_relative_eq!(mag_sq, 1.0, epsilon = 1e-5);
    }

    #[test]
    fn test_boost_overflow() {
        // Boost magnitude > 88.0 causes overflow in f32 exp/cosh/sinh
        let displacement = Vec3::new(100.0, 0.0, 0.0);
        let m = boost(displacement);

        // Havoc: We expect this to fail (produce Infinity)
        // Proving the system is fragile to large inputs.
        assert!(!m.w.w.is_finite(), "Expected overflow/infinity for large input, but got finite value");
    }
}
