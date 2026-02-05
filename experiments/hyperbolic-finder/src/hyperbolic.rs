use num_complex::Complex;

pub type Point = Complex<f64>;

/// Computes the Mobius transform (z - a) / (1 - conj(a) * z)
/// This maps a -> 0.
pub fn mobius_transform(z: Point, a: Point) -> Point {
    if a.norm_sqr() >= 1.0 {
        return z;
    }
    (z - a) / (1.0 - a.conj() * z)
}

/// Computes the inverse Mobius transform (z + a) / (1 + conj(a) * z)
/// This maps 0 -> a.
pub fn inverse_mobius_transform(z: Point, a: Point) -> Point {
    if a.norm_sqr() >= 1.0 {
        return z;
    }
    (z + a) / (1.0 + a.conj() * z)
}

/// Computes hyperbolic distance between two points in the Poincare disk.
/// d(a, b) = 2 * artanh( |(a-b)/(1-a_bar*b)| )
pub fn hyperbolic_dist(a: Point, b: Point) -> f64 {
    let num = a - b;
    let den = 1.0 - a.conj() * b;
    let modulus = (num / den).norm();
    // Clamp to avoid NaN if modulus >= 1 due to float error
    let clamped_mod = modulus.min(0.9999999999);
    2.0 * clamped_mod.atanh()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mobius_origin() {
        let a = Point::new(0.5, 0.0);
        // Transforming 'a' by 'a' should map it to the origin.
        let result = mobius_transform(a, a);
        assert!(result.norm() < 1e-9);
    }

    #[test]
    fn test_mobius_inverse() {
        let a = Point::new(0.3, 0.4);
        let origin = Point::new(0.0, 0.0);
        // The inverse transform of the origin should bring us back to 'a'.
        let result = inverse_mobius_transform(origin, a);
        assert!((result - a).norm() < 1e-9);
    }

    #[test]
    fn test_hyperbolic_dist_zero() {
        let a = Point::new(0.2, 0.2);
        assert!(hyperbolic_dist(a, a) < 1e-9);
    }

    #[test]
    fn test_disk_boundary_preservation() {
        // Points inside the disk should stay inside
        let a = Point::new(0.5, 0.0);
        let z = Point::new(0.0, 0.9);
        let t = mobius_transform(z, a);
        assert!(t.norm() < 1.0);
    }
}
