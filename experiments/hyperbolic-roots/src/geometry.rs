use num_complex::Complex;

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
#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct Mobius {
    pub a: Complex<f64>,
    pub b: Complex<f64>,
    pub c: Complex<f64>,
    pub d: Complex<f64>,
}

#[allow(dead_code)]
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
