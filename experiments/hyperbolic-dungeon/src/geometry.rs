use num_complex::Complex;
use std::f64::consts::PI;

pub type Point = Complex<f64>;

/// Computes the Mobius transform (z + a) / (1 + conj(a) * z)
/// This maps 0 -> a.
/// This is a translation in hyperbolic space that moves the origin to 'a'.
pub fn mobius_add(z: Point, a: Point) -> Point {
    (z + a) / (1.0 + a.conj() * z)
}

/// Computes the Mobius transform (z - a) / (1 - conj(a) * z)
/// This maps a -> 0.
/// This is the inverse of `mobius_add`.
pub fn mobius_sub(z: Point, a: Point) -> Point {
    (z - a) / (1.0 - a.conj() * z)
}

/// Distance in the Poincare disk metric
pub fn hyperbolic_dist(a: Point, b: Point) -> f64 {
    let num = a - b;
    let den = 1.0 - a.conj() * b;
    let modulus = (num / den).norm();
    let clamped = modulus.min(0.99999999);
    2.0 * clamped.atanh()
}

/// Represents a Mobius transformation (az + b) / (cz + d)
#[derive(Clone, Copy, Debug)]
pub struct Mobius {
    pub a: Complex<f64>,
    pub b: Complex<f64>,
    pub c: Complex<f64>,
    pub d: Complex<f64>,
}

impl Mobius {
    pub fn identity() -> Self {
        Self {
            a: Complex::new(1.0, 0.0),
            b: Complex::new(0.0, 0.0),
            c: Complex::new(0.0, 0.0),
            d: Complex::new(1.0, 0.0),
        }
    }

    /// Creates a transformation for (z + k) / (1 + conj(k)z)
    /// This maps 0 -> k.
    pub fn translation(k: Point) -> Self {
        Self {
            a: Complex::new(1.0, 0.0),
            b: k,
            c: k.conj(),
            d: Complex::new(1.0, 0.0),
        }
    }

    /// Creates a transformation for (z - k) / (1 - conj(k)z)
    /// This maps k -> 0.
    pub fn inverse_translation(k: Point) -> Self {
        Self {
            a: Complex::new(1.0, 0.0),
            b: -k,
            c: -k.conj(),
            d: Complex::new(1.0, 0.0),
        }
    }

    /// Compose: self(other(z))
    /// Matrix multiplication: [a b; c d] * [A B; C D]
    pub fn then(&self, other: &Mobius) -> Self {
        Self {
            a: self.a * other.a + self.b * other.c,
            b: self.a * other.b + self.b * other.d,
            c: self.c * other.a + self.d * other.c,
            d: self.c * other.b + self.d * other.d,
        }
    }

    pub fn apply(&self, z: Point) -> Point {
        let num = self.a * z + self.b;
        let den = self.c * z + self.d;
        // Check for singularity? In Disk model, denominator shouldn't be 0 for z inside disk.
        num / den
    }
}

/// Constants for {4, 5} tiling (Squares, 5 meeting at a vertex)
pub struct TilingConsts {
    /// The Euclidean distance from the origin to the center of a neighbor in the Poincare disk.
    /// This is `tanh(d/2)` where `d` is the hyperbolic distance between centers.
    pub neighbor_offset: f64,
    /// Distance from center to vertex (Euclidean in Poincare disk)
    pub vertex_offset: f64,
}

impl TilingConsts {
    pub fn new_4_5() -> Self {
        // p = 4, q = 5
        let p = 4.0;
        let q = 5.0;

        // cosh(d/2) = cos(pi/q) / sin(pi/p)
        let cos_pi_q = (PI / q).cos();
        let sin_pi_p = (PI / p).sin();
        let cosh_half_d = cos_pi_q / sin_pi_p;
        let half_d = cosh_half_d.acosh();
        let neighbor_offset = half_d.tanh();

        // Vertex distance R
        // cosh(R) = cot(pi/p) * cot(pi/q)? No.
        // Rule: cosh(R) = cos(pi/q) / sin(pi/p) ?? No, that's inradius r.
        // Wait, for {p, q}:
        // cosh(R) = cot(pi/p) * cot(pi/2 - pi/q)  <- This is what I derived, let's re-verify.
        // Triangle C-M-V. C=pi/p, M=pi/2, V=pi/q. Side m=R.
        // cos(pi/q) = sin(pi/p) * cosh(r) -> cosh(r) formula correct.
        // cos(pi/p) = sin(pi/q) * cosh(half_edge).
        // cosh(R) = cot(pi/p) * cot(pi/q) is for regular spherical?
        // Hyperbolic right triangle C-M-V:
        // cosh(R) = cosh(r) * cosh(half_edge).
        // Let's use that.
        // half_edge = acosh( cos(pi/p) / sin(pi/q) )
        // r = acosh( cos(pi/q) / sin(pi/p) )

        let cosh_r = cos_pi_q / sin_pi_p;

        let sin_pi_q = (PI / q).sin();
        let cos_pi_p = (PI / p).cos();
        let cosh_half_edge = cos_pi_p / sin_pi_q;

        let cosh_radius = cosh_r * cosh_half_edge;
        let radius = cosh_radius.acosh();
        let vertex_offset = radius.tanh();

        Self {
            neighbor_offset,
            vertex_offset,
        }
    }
}

/// Returns the translation `a` required to move from the current cell center
/// to the neighbor in the given direction (index 0..3 for square).
/// Directions: 0=Right, 1=Up, 2=Left, 3=Down
pub fn neighbor_transform_a(direction: usize, consts: &TilingConsts) -> Point {
    let angle = (direction as f64) * (PI / 2.0);
    Complex::from_polar(consts.neighbor_offset, angle)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mobius_inverses() {
        let a = Point::new(0.3, -0.2);
        let z = Point::new(0.1, 0.5);
        let t = mobius_add(z, a);
        let back = mobius_sub(t, a);
        assert!((back - z).norm() < 1e-10);
    }

    #[test]
    fn test_tiling_consts() {
        let c = TilingConsts::new_4_5();
        // Check that neighbor offset is reasonable (less than 1, greater than 0)
        assert!(c.neighbor_offset > 0.0);
        assert!(c.neighbor_offset < 1.0);
        // Approx check: ~0.485
        assert!((c.neighbor_offset - 0.485).abs() < 0.01);
    }
}
