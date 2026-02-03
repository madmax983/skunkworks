use num_complex::Complex;

/// A transformation of the form f(z) = (az + b) / (cz + d)
/// Represents an isometry of the Poincaré disk if constructed correctly.
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub struct Mobius {
    pub a: Complex<f64>,
    pub b: Complex<f64>,
    pub c: Complex<f64>,
    pub d: Complex<f64>,
}

impl Mobius {
    /// Identity transformation f(z) = z
    pub fn identity() -> Self {
        Self {
            a: Complex::new(1.0, 0.0),
            b: Complex::new(0.0, 0.0),
            c: Complex::new(0.0, 0.0),
            d: Complex::new(1.0, 0.0),
        }
    }

    /// Creates a transformation that translates the origin to `a`
    /// T_a(z) = (z + a) / (1 + conj(a)z)
    /// Note: This maps 0 -> a. The inverse maps a -> 0.
    pub fn translation(a: Complex<f64>) -> Self {
        Self {
            a: Complex::new(1.0, 0.0),
            b: a,
            c: a.conj(),
            d: Complex::new(1.0, 0.0),
        }
    }

    /// Creates a rotation by theta around the origin
    /// R_theta(z) = e^(i*theta) * z
    pub fn rotation(theta: f64) -> Self {
        let rot = Complex::from_polar(1.0, theta);
        Self {
            a: rot,
            b: Complex::new(0.0, 0.0),
            c: Complex::new(0.0, 0.0),
            d: Complex::new(1.0, 0.0),
        }
    }

    /// Apply the transformation to a point z
    pub fn apply(&self, z: Complex<f64>) -> Complex<f64> {
        let num = self.a * z + self.b;
        let den = self.c * z + self.d;
        num / den
    }

    /// Compose this transformation with another: (self o other)(z) = self(other(z))
    /// Matrix multiplication:
    /// [a1 b1] [a2 b2] = [a1a2 + b1c2, a1b2 + b1d2]
    /// [c1 d1] [c2 d2]   [c1a2 + d1c2, c1b2 + d1d2]
    pub fn compose(&self, other: &Self) -> Self {
        Self {
            a: self.a * other.a + self.b * other.c,
            b: self.a * other.b + self.b * other.d,
            c: self.c * other.a + self.d * other.c,
            d: self.c * other.b + self.d * other.d,
        }
    }

    /// Inverse of the transformation
    /// Matrix inverse for determinant 1 (or normalized) is:
    /// [ d -b]
    /// [-c  a]
    /// Since we might not be normalized, we just swap and negate.
    /// Inverse of f(z) = (az+b)/(cz+d) is g(z) = (dz-b)/(-cz+a)
    pub fn inverse(&self) -> Self {
        Self {
            a: self.d,
            b: -self.b,
            c: -self.c,
            d: self.a,
        }
    }
}

/// Represents a geodesic segment between two points in the Poincaré disk
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct Geodesic {
    pub p1: Complex<f64>,
    pub p2: Complex<f64>,
}

impl Geodesic {
    #[allow(dead_code)]
    pub fn new(p1: Complex<f64>, p2: Complex<f64>) -> Self {
        Self { p1, p2 }
    }

    /// Returns the Euclidean center and radius of the arc representing the geodesic.
    /// Returns None if the geodesic is a straight line through the origin.
    pub fn euclidean_circle(&self) -> Option<(Complex<f64>, f64)> {
        // Points p1, p2 and their inversions wrt unit circle 1/conj(p1), 1/conj(p2) define the circle.
        // Or simpler: The circle passes through p1, p2 and is orthogonal to the unit circle.
        // Let circle be |z - c|^2 = r^2.
        // Orthogonality means |c|^2 = r^2 + 1.
        // Also |p1 - c|^2 = r^2 => |p1|^2 - 2 Re(p1 conj(c)) + |c|^2 = r^2
        // => |p1|^2 - 2 Re(p1 conj(c)) + 1 = 0
        // Same for p2.
        // 2 Re(p1 conj(c)) = 1 + |p1|^2
        // 2 Re(p2 conj(c)) = 1 + |p2|^2

        // This is a linear system for Re(c) and Im(c).
        // Let c = x + iy.
        // 2(x x1 + y y1) = 1 + x1^2 + y1^2
        // 2(x x2 + y y2) = 1 + x2^2 + y2^2

        let x1 = self.p1.re;
        let y1 = self.p1.im;
        let x2 = self.p2.re;
        let y2 = self.p2.im;

        let d1 = 1.0 + x1 * x1 + y1 * y1;
        let d2 = 1.0 + x2 * x2 + y2 * y2;

        // 2*x*x1 + 2*y*y1 = d1
        // 2*x*x2 + 2*y*y2 = d2

        let det = 4.0 * (x1 * y2 - x2 * y1);

        if det.abs() < 1e-9 {
            // Collinear with origin (or close enough)
            return None;
        }

        let x = (d1 * 2.0 * y2 - d2 * 2.0 * y1) / det;
        let y = (2.0 * x1 * d2 - 2.0 * x2 * d1) / det;

        let center = Complex::new(x, y);
        let radius = (center - self.p1).norm();

        Some((center, radius))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_identity() {
        let m = Mobius::identity();
        let z = Complex::new(0.5, 0.5);
        assert!((m.apply(z) - z).norm() < 1e-9);
    }

    #[test]
    fn test_translation() {
        let a = Complex::new(0.5, 0.0);
        let m = Mobius::translation(a);
        // Maps 0 to a
        assert!((m.apply(Complex::new(0.0, 0.0)) - a).norm() < 1e-9);
        // Maps -a to 0
        assert!((m.apply(-a) - Complex::new(0.0, 0.0)).norm() < 1e-9);
    }

    #[test]
    fn test_inverse() {
        let a = Complex::new(0.3, 0.4);
        let m = Mobius::translation(a);
        let inv = m.inverse();
        let z = Complex::new(0.1, -0.2);

        let z_prime = m.apply(z);
        let z_back = inv.apply(z_prime);

        assert!((z - z_back).norm() < 1e-9);
    }

    #[test]
    fn test_rotation() {
        let m = Mobius::rotation(PI / 2.0);
        let z = Complex::new(1.0, 0.0);
        let z_rot = m.apply(z);
        assert!((z_rot - Complex::new(0.0, 1.0)).norm() < 1e-9);
    }
}
