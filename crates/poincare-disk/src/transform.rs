use crate::types::Point;
use num_complex::Complex;

/// Represents a [Möbius transformation](https://en.wikipedia.org/wiki/M%C3%B6bius_transformation) of the form $f(z) = \frac{az + b}{cz + d}$.
///
/// In the context of the Poincaré disk, we are specifically interested in the subset of Möbius transformations
/// that map the unit disk to itself (automorphisms). These correspond to the isometries (rigid motions)
/// of the hyperbolic plane.
///
/// They can be represented as $2 \times 2$ matrices acting on homogeneous coordinates.
#[derive(Clone, Copy, Debug)]
#[doc(alias = "Isometry")]
#[doc(alias = "Automorphism")]
#[doc(alias = "Transform")]
pub struct Mobius {
    a: Complex<f64>,
    b: Complex<f64>,
    c: Complex<f64>,
    d: Complex<f64>,
}

impl Mobius {
    /// Creates a new Möbius transformation if the determinant is non-zero.
    ///
    /// The transformation is defined as $f(z) = \frac{az + b}{cz + d}$.
    ///
    /// Returns `None` if $ad - bc \approx 0$ (singular matrix).
    pub fn new(a: Complex<f64>, b: Complex<f64>, c: Complex<f64>, d: Complex<f64>) -> Option<Self> {
        let det = a * d - b * c;
        if det.norm_sqr() < 1e-12 {
            return None;
        }
        Some(Self { a, b, c, d })
    }

    /// Returns the coefficient `a`.
    pub fn a(&self) -> Complex<f64> {
        self.a
    }

    /// Returns the coefficient `b`.
    pub fn b(&self) -> Complex<f64> {
        self.b
    }

    /// Returns the coefficient `c`.
    pub fn c(&self) -> Complex<f64> {
        self.c
    }

    /// Returns the coefficient `d`.
    pub fn d(&self) -> Complex<f64> {
        self.d
    }

    /// Returns the identity transformation $f(z) = z$.
    ///
    /// Corresponds to the matrix $\begin{pmatrix} 1 & 0 \\ 0 & 1 \end{pmatrix}$.
    ///
    /// # Examples
    ///
    /// ```
    /// use poincare_disk::{Mobius, Point};
    /// let id = Mobius::identity();
    /// let p = Point::new(0.5, 0.2);
    /// assert_eq!(id.apply(p), p);
    /// ```
    pub fn identity() -> Self {
        Self {
            a: Complex::new(1.0, 0.0),
            b: Complex::new(0.0, 0.0),
            c: Complex::new(0.0, 0.0),
            d: Complex::new(1.0, 0.0),
        }
    }

    /// Creates a transformation that maps the origin $0$ to the point $k$.
    ///
    /// Form: $f(z) = \frac{z + k}{1 + \bar{k}z}$
    ///
    /// This is equivalent to `mobius_add(z, k)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use poincare_disk::{Mobius, Point};
    /// let k = Point::new(0.5, 0.0);
    /// let t = Mobius::translation(k);
    ///
    /// // Maps origin to k
    /// assert_eq!(t.apply(Point::new(0.0, 0.0)), k);
    /// ```
    pub fn translation(k: Point) -> Self {
        Self {
            a: Complex::new(1.0, 0.0),
            b: k,
            c: k.conj(),
            d: Complex::new(1.0, 0.0),
        }
    }

    /// Creates a transformation that maps the point $k$ to the origin $0$.
    ///
    /// Form: $f(z) = \frac{z - k}{1 - \bar{k}z}$
    ///
    /// This is equivalent to `mobius_sub(z, k)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use poincare_disk::{Mobius, Point};
    /// let k = Point::new(0.5, 0.0);
    /// let t = Mobius::inverse_translation(k);
    ///
    /// // Maps k to origin
    /// assert!(t.apply(k).norm() < 1e-10);
    /// ```
    pub fn inverse_translation(k: Point) -> Self {
        Self {
            a: Complex::new(1.0, 0.0),
            b: -k,
            c: -k.conj(),
            d: Complex::new(1.0, 0.0),
        }
    }

    /// Creates a rotation by theta around the origin.
    ///
    /// Form: $f(z) = e^{i\theta} z$
    ///
    /// # Examples
    ///
    /// ```
    /// use poincare_disk::{Mobius, Point};
    /// use std::f64::consts::PI;
    /// let m = Mobius::rotation(PI / 2.0);
    /// let z = Point::new(1.0, 0.0); // Technically on boundary, but valid for rotation
    /// let rotated = m.apply(z);
    /// assert!((rotated.im - 1.0).abs() < 1e-9);
    /// ```
    pub fn rotation(theta: f64) -> Self {
        let rot = Complex::from_polar(1.0, theta);
        Self {
            a: rot,
            b: Complex::new(0.0, 0.0),
            c: Complex::new(0.0, 0.0),
            d: Complex::new(1.0, 0.0),
        }
    }

    /// Returns the inverse of the transformation.
    ///
    /// The inverse $f^{-1}$ undoes the effect of $f$.
    /// If you move forward with `f`, you can move back with `inverse()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use poincare_disk::{Mobius, Point};
    ///
    /// // A transformation that moves the origin to (0.5, 0.0)
    /// let forward = Mobius::translation(Point::new(0.5, 0.0));
    ///
    /// // The inverse moves (0.5, 0.0) back to the origin
    /// let backward = forward.inverse();
    ///
    /// let p = Point::new(0.0, 0.0);
    /// let moved = forward.apply(p);
    /// let returned = backward.apply(moved);
    ///
    /// assert!((returned - p).norm() < 1e-9);
    /// ```
    pub fn inverse(&self) -> Self {
        Self {
            a: self.d,
            b: -self.b,
            c: -self.c,
            d: self.a,
        }
    }

    /// Composes two Möbius transformations.
    ///
    /// Returns a new transformation that applies `other` first, and then `self`.
    /// mathematically: $(f \circ g)(z) = f(g(z))$.
    ///
    /// Note: This is equivalent to multiplying the matrices $M_f \times M_g$.
    ///
    /// # Examples
    ///
    /// ```
    /// use poincare_disk::{Mobius, Point};
    ///
    /// // 1. Rotate by 90 degrees
    /// let rotate = Mobius::rotation(std::f64::consts::PI / 2.0);
    ///
    /// // 2. Translate by 0.5 to the right
    /// let translate = Mobius::translation(Point::new(0.5, 0.0));
    ///
    /// // Combined: Translate FIRST, then Rotate.
    /// // Imagine holding a camera: you step right, then turn 90 degrees left.
    /// let step_then_turn = rotate.then(&translate);
    ///
    /// let origin = Point::new(0.0, 0.0);
    /// let result = step_then_turn.apply(origin);
    ///
    /// // Origin -> (0.5, 0.0) -> (0.0, 0.5)
    /// assert!((result.im - 0.5).abs() < 1e-9);
    /// assert!(result.re.abs() < 1e-9);
    /// ```
    pub fn then(&self, other: &Mobius) -> Self {
        // Matrix mul: self * other
        Self {
            a: self.a * other.a + self.b * other.c,
            b: self.a * other.b + self.b * other.d,
            c: self.c * other.a + self.d * other.c,
            d: self.c * other.b + self.d * other.d,
        }
    }

    /// Applies the transformation to a point $z$.
    ///
    /// $$ w = \frac{az + b}{cz + d} $$
    pub fn apply(&self, z: Point) -> Point {
        let num = self.a * z + self.b;
        let den = self.c * z + self.d;
        // In the Disk model, the denominator (cz + d) is never zero for |z| < 1
        // (unless the transformation maps the disk to infinity, which these shouldn't).
        num / den
    }
}
