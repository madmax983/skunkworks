use cgmath::*;

pub fn minkowski_dot(a: Vector4<f32>, b: Vector4<f32>) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z - a.w * b.w
}

pub fn origin() -> Vector4<f32> {
    Vector4::new(0.0, 0.0, 0.0, 1.0)
}

pub fn translation_z(d: f32) -> Matrix4<f32> {
    let ch = d.cosh();
    let sh = d.sinh();
    // Columns
    Matrix4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, ch, sh,
        0.0, 0.0, sh, ch,
    )
}

pub fn translation_x(d: f32) -> Matrix4<f32> {
    let ch = d.cosh();
    let sh = d.sinh();
    Matrix4::new(
        ch, 0.0, 0.0, sh,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        sh, 0.0, 0.0, ch,
    )
}

pub fn project_poincare(p: Vector4<f32>) -> Vector3<f32> {
    Vector3::new(p.x, p.y, p.z) / (1.0 + p.w)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translation_preserves_norm() {
        let p = origin(); // (0,0,0,1) -> norm = -1
        let t = translation_z(2.0);
        let p_prime = t * p;
        let norm = minkowski_dot(p_prime, p_prime);
        assert!((norm + 1.0).abs() < 1e-5, "Norm should be -1, got {}", norm);
    }

    #[test]
    fn test_composition() {
        let t1 = translation_z(1.0);
        let t2 = translation_z(1.0);
        let t_total = t1 * t2; // Note: cgmath mul is M * v. Matrix mul is M1 * M2.

        // Boosts along same axis add up
        let p = origin();
        let p_final = t_total * p;
        // z should be sinh(2.0), w should be cosh(2.0)
        assert!((p_final.z - 2.0f32.sinh()).abs() < 1e-5);
    }
}
