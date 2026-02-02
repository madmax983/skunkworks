use num_complex::Complex;

pub type Point = Complex<f64>;

/// Computes the Mobius transform (z - a) / (1 - conj(a) * z)
/// This maps a -> 0 and maps the unit disk to itself.
pub fn mobius_transform(z: Point, a: Point) -> Point {
    // Avoid division by zero if |a| >= 1, though in our model |a| < 1 always.
    if a.norm_sqr() >= 1.0 {
        return z;
    }
    // (z - a) / (1 - conj(a) * z)
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
#[allow(dead_code)]
pub fn hyperbolic_dist(a: Point, b: Point) -> f64 {
    let num = a - b;
    let den = 1.0 - a.conj() * b;
    let modulus = (num / den).norm();
    // Clamp to avoid NaN if modulus >= 1 due to float error
    let clamped_mod = modulus.min(0.999999999);
    2.0 * clamped_mod.atanh()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mobius_origin() {
        let a = Point::new(0.5, 0.0);
        let z = a;
        // Transform of a by a should be 0
        let transformed = mobius_transform(z, a);
        assert!(transformed.norm() < 1e-10);
    }

    #[test]
    fn test_mobius_inverse() {
        let a = Point::new(0.3, 0.4);
        let origin = Point::new(0.0, 0.0);
        // Inverse transform of origin should be a
        let result = inverse_mobius_transform(origin, a);
        assert!((result - a).norm() < 1e-10);
    }

    #[test]
    fn test_disk_preservation() {
        let a = Point::new(0.5, 0.0);
        let z = Point::new(0.0, 0.9);
        let t = mobius_transform(z, a);
        assert!(t.norm() < 1.0);
    }
}
