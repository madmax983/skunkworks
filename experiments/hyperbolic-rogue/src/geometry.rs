use num_complex::Complex;

pub type Point = Complex<f64>;

#[derive(Clone, Copy, Debug)]
pub struct Mobius {
    pub a: Point,
    pub b: Point,
    pub c: Point,
    pub d: Point,
}

impl Mobius {
    /// Identity transformation: f(z) = z
    pub fn identity() -> Self {
        Self {
            a: Point::new(1.0, 0.0),
            b: Point::new(0.0, 0.0),
            c: Point::new(0.0, 0.0),
            d: Point::new(1.0, 0.0),
        }
    }

    /// Apply the transformation to a point z
    pub fn apply(&self, z: Point) -> Point {
        let num = self.a * z + self.b;
        let den = self.c * z + self.d;
        if den.norm_sqr() < 1e-10 {
            // Should not happen for valid hyperbolic isometries applied to points in the disk
            return Point::new(0.0, 0.0);
        }
        num / den
    }

    /// Composition of two transformations (self after other)
    /// f(g(z))
    /// Matrix multiplication: M_self * M_other
    pub fn compose(&self, other: Self) -> Self {
        Self {
            a: self.a * other.a + self.b * other.c,
            b: self.a * other.b + self.b * other.d,
            c: self.c * other.a + self.d * other.c,
            d: self.c * other.b + self.d * other.d,
        }
    }

    /// Inverse transformation
    /// M^-1 = [d -b; -c a] / det
    /// Since Mobius is scale invariant, we can ignore det scaling for the mapping,
    /// but for numerical stability and composition, we might want to normalize.
    /// However, let's keep it simple.
    pub fn inverse(&self) -> Self {
        Self {
            a: self.d,
            b: -self.b,
            c: -self.c,
            d: self.a,
        }
    }

    /// Hyperbolic translation that maps origin (0) to `p`.
    /// Formula: T(z) = (z + p) / (1 + conj(p)z)
    /// Matrix: [1 p; conj(p) 1]
    pub fn translation(p: Point) -> Self {
        Self {
            a: Point::new(1.0, 0.0),
            b: p,
            c: p.conj(),
            d: Point::new(1.0, 0.0),
        }
    }

    /// Rotation around origin by theta radians
    /// f(z) = e^(i theta) * z
    /// Matrix: [e^(i theta/2) 0; 0 e^(-i theta/2)] to stay in SU(1,1)
    /// Or simply [e^(i theta) 0; 0 1] works for Mobius.
    pub fn rotation(theta: f64) -> Self {
        let rot = Point::from_polar(1.0, theta);
        Self {
            a: rot,
            b: Point::new(0.0, 0.0),
            c: Point::new(0.0, 0.0),
            d: Point::new(1.0, 0.0),
        }
    }
}
