//! # Poincaré Disk Model
//!
//! A library for performing calculations in the [Poincaré Disk Model](https://en.wikipedia.org/wiki/Poincar%C3%A9_disk_model) of hyperbolic geometry.
//!
//! In this model, the entire infinite hyperbolic plane is compressed into the interior of the unit disk ($|z| < 1$) in the complex plane.
//! Straight lines in hyperbolic space appear as circular arcs orthogonal to the boundary of the disk.
//!
//! ## Core Concepts
//!
//! *   **Point**: Represents a location in the hyperbolic plane as a complex number $z$ where $|z| < 1$.
//! *   **Mobius Transformations**: The "motions" (isometries) of the hyperbolic plane are represented by special Möbius transformations that map the unit disk to itself.
//! *   **Hyperbolic Distance**: Distances grow infinitely large as you approach the boundary of the disk ($|z| = 1$).
//!
//! ## Example
//!
//! ```
//! use poincare_disk::{Point, mobius_add, hyperbolic_dist};
//!
//! // Create a point near the origin
//! let p1 = Point::new(0.1, 0.0);
//!
//! // "Add" a displacement to it using Möbius addition (translation in hyperbolic space)
//! let displacement = Point::new(0.5, 0.0);
//! let p2 = mobius_add(p1, displacement);
//!
//! // Calculate the hyperbolic distance between them
//! let dist = hyperbolic_dist(p1, p2);
//! assert!(dist > 0.0);
//! ```

use num_complex::Complex;
use std::f64::consts::PI;

/// A point in the Poincaré disk ($|z| < 1$).
///
/// While this is an alias for `Complex<f64>`, all functions in this crate assume
/// that the modulus (norm) of the point is strictly less than 1.0.
pub type Point = Complex<f64>;

/// Performs Möbius addition: $ (z + a) / (1 + \bar{a}z) $.
///
/// This operation represents a hyperbolic translation that moves the origin to `a`.
/// It is non-commutative and non-associative in the Euclidean sense, but forms a group loop (gyroup).
///
/// Geometrically, this maps the origin $0$ to $a$.
///
/// # Panics
///
/// Does not panic, but returns `z` unchanged if `|a| >= 1` (invalid translation).
///
/// # Examples
///
/// ```
/// use poincare_disk::{Point, mobius_add};
///
/// let z = Point::new(0.0, 0.0); // Origin
/// let a = Point::new(0.5, 0.0); // Target
///
/// // Translating the origin by 'a' moves it to 'a'
/// let result = mobius_add(z, a);
/// assert_eq!(result, a);
/// ```
pub fn mobius_add(z: Point, a: Point) -> Point {
    if a.norm_sqr() >= 1.0 {
        return z;
    }
    (z + a) / (1.0 + a.conj() * z)
}

/// Performs Möbius subtraction: $ (z - a) / (1 - \bar{a}z) $.
///
/// This is the inverse of `mobius_add`. It represents a hyperbolic translation that moves `a` to the origin.
///
/// Geometrically, this maps $a$ to $0$.
///
/// # Examples
///
/// ```
/// use poincare_disk::{Point, mobius_sub};
///
/// let a = Point::new(0.5, 0.5);
/// // Moving 'a' "back" by 'a' should result in the origin
/// let result = mobius_sub(a, a);
/// assert!(result.norm() < 1e-10);
/// ```
pub fn mobius_sub(z: Point, a: Point) -> Point {
    if a.norm_sqr() >= 1.0 {
        return z;
    }
    (z - a) / (1.0 - a.conj() * z)
}

/// Calculates the hyperbolic distance between two points.
///
/// The distance formula is:
/// $$ d(a, b) = 2 \text{tanh}^{-1} \left( \left| \frac{a - b}{1 - \bar{a}b} \right| \right) $$
///
/// As points approach the boundary of the disk (norm -> 1), the distance approaches infinity.
///
/// # Examples
///
/// ```
/// use poincare_disk::{Point, hyperbolic_dist};
///
/// let center = Point::new(0.0, 0.0);
/// let p = Point::new(0.5, 0.0);
///
/// // Distance from 0 to 0.5 is 2 * atanh(0.5) ≈ 1.0986
/// let dist = hyperbolic_dist(center, p);
/// assert!((dist - 1.0986).abs() < 0.001);
/// ```
pub fn hyperbolic_dist(a: Point, b: Point) -> f64 {
    let num = a - b;
    let den = 1.0 - a.conj() * b;
    let modulus = (num / den).norm();
    // Clamp to avoid atanh(1.0) = infinity/NaN if floating point errors push us over
    let clamped = modulus.min(0.99999999);
    2.0 * clamped.atanh()
}

/// Represents a [Möbius transformation](https://en.wikipedia.org/wiki/M%C3%B6bius_transformation) of the form $f(z) = \frac{az + b}{cz + d}$.
///
/// In the context of the Poincaré disk, we are specifically interested in the subset of Möbius transformations
/// that map the unit disk to itself (automorphisms). These correspond to the isometries (rigid motions)
/// of the hyperbolic plane.
///
/// They can be represented as $2 \times 2$ matrices acting on homogeneous coordinates.
#[derive(Clone, Copy, Debug)]
pub struct Mobius {
    pub a: Complex<f64>,
    pub b: Complex<f64>,
    pub c: Complex<f64>,
    pub d: Complex<f64>,
}

impl Mobius {
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

    /// Composes two Möbius transformations.
    ///
    /// Returns a new transformation representing $f(g(z))$, where $f$ is `self` and $g$ is `other`.
    /// This corresponds to matrix multiplication.
    ///
    /// # Examples
    ///
    /// ```
    /// use poincare_disk::{Mobius, Point};
    /// // Translate by 0.1, then by 0.2
    /// let t1 = Mobius::translation(Point::new(0.1, 0.0));
    /// let t2 = Mobius::translation(Point::new(0.2, 0.0));
    ///
    /// // Combine: apply t2 THEN t1 ?? No, self.then(other) means self(other(z))
    /// // So it applies 'other' first, then 'self'.
    /// let combined = t1.then(&t2);
    ///
    /// let origin = Point::new(0.0, 0.0);
    /// // t2(0) = 0.2. t1(0.2) = (0.2+0.1)/(1+0.02) = 0.3 / 1.02 ≈ 0.294
    /// let result = combined.apply(origin);
    /// assert!(result.re > 0.29);
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

/// Precomputed constants for generating a hyperbolic tiling.
///
/// Specifically, this struct calculates parameters for a regular $\{p, q\}$ tiling,
/// where $p$ is the number of sides of each polygon (face) and $q$ is the number of polygons meeting at each vertex.
///
/// For a tiling to exist in the hyperbolic plane, we must have $(p-2)(q-2) > 4$.
/// For example, $\{4, 5\}$ (squares, 5 meeting at a vertex) satisfies this: $(2)(3) = 6 > 4$.
pub struct TilingConsts {
    /// The Euclidean distance from the origin to the center of an adjacent cell in the Poincaré disk model.
    ///
    /// This corresponds to the hyperbolic translation distance required to move from one tile center to the next.
    pub neighbor_offset: f64,
    /// Euclidean distance from the center of the polygon to one of its vertices.
    pub vertex_offset: f64,
}

impl TilingConsts {
    /// Calculates constants for the $\{4, 5\}$ tiling.
    ///
    /// This is an "order-5 square tiling". It consists of squares where 5 squares meet at every vertex.
    ///
    /// # Examples
    ///
    /// ```
    /// use poincare_disk::TilingConsts;
    /// let consts = TilingConsts::new_4_5();
    /// assert!(consts.neighbor_offset > 0.0);
    /// ```
    pub fn new_4_5() -> Self {
        // p = 4, q = 5
        let p = 4.0;
        let q = 5.0;

        // The hyperbolic distance 'd' between centers of adjacent p-gons sharing an edge is given by:
        // cosh(d/2) = cos(pi/q) / sin(pi/p)
        let cos_pi_q = (PI / q).cos();
        let sin_pi_p = (PI / p).sin();
        let cosh_half_d = cos_pi_q / sin_pi_p;
        let half_d = cosh_half_d.acosh();
        // Convert hyperbolic distance to Euclidean distance in the disk model
        // r_euclid = tanh(r_hyperbolic / 2)
        // Here half_d is already r_hyperbolic/2 relative to the midpoint of the edge.
        // Wait, neighbor_offset is the move from center to center. So distance is d.
        // The Mobius translation magnitude 'a' corresponds to tanh(d/2).
        let neighbor_offset = half_d.tanh();

        // Vertex distance R (circumradius)
        // Right hyperbolic triangle with angles: pi/p (center), pi/q (vertex), pi/2 (edge midpoint).
        // Let 'R' be hypotenuse (center to vertex).
        // Let 'r' be inradius (center to edge midpoint) = half_d.
        // Let 'l' be half-edge length.
        //
        // Rule: cosh(R) = cosh(r) * cosh(l)
        // We need 'l'.
        // Relation: cos(pi/p) = sin(pi/q) * cosh(l)  (from spherical law of cosines for angles... wait, dual?)
        // Let's stick to standard hyperbolic formulas.
        // cos(C) = -cos(A)cos(B) + sin(A)sin(B)cosh(c)
        // For dual triangle (edge lengths become angles):
        // cosh(R) = cot(pi/p) * cot(pi/q)
        //
        // Let's re-verify the existing implementation logic, which was:
        // cosh_r = cos(pi/q) / sin(pi/p)  <-- This matches cosh(inradius)
        // cosh_half_edge = cos(pi/p) / sin(pi/q)
        // cosh_radius = cosh_r * cosh_half_edge

        let cosh_r = cos_pi_q / sin_pi_p;

        let sin_pi_q = (PI / q).sin();
        let cos_pi_p = (PI / p).cos();
        let cosh_half_edge = cos_pi_p / sin_pi_q;

        let cosh_radius = cosh_r * cosh_half_edge;
        let radius = cosh_radius.acosh();
        // Convert R to Euclidean distance from origin
        let vertex_offset = (radius / 2.0).tanh(); // Corrected formula: Euclidean r = tanh(hyperbolic R / 2)

        Self {
            neighbor_offset,
            vertex_offset,
        }
    }
}

/// Returns the point `a` representing the center of a neighbor in the given direction.
///
/// This point `a` can be used to construct a `Mobius::translation(a)` that moves the view
/// to that neighbor.
///
/// # Arguments
///
/// *   `direction`: An index `0..4` representing the neighbor direction (Right, Up, Left, Down).
/// *   `consts`: Tiling constants (usually from `new_4_5`).
///
/// # Examples
///
/// ```
/// use poincare_disk::{neighbor_transform_a, TilingConsts};
/// let consts = TilingConsts::new_4_5();
/// let right_neighbor = neighbor_transform_a(0, &consts);
/// assert!(right_neighbor.re > 0.0);
/// ```
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

    #[test]
    fn test_mobius_origin() {
        let a = Point::new(0.5, 0.0);
        // Transforming 'a' by 'a' (via sub) should map it to the origin.
        let result = mobius_sub(a, a);
        assert!(result.norm() < 1e-9);
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
        let t = mobius_sub(z, a);
        assert!(t.norm() < 1.0);
    }

    #[test]
    fn test_havoc_singularity() {
        // 👺 HAVOC: Attempting to create a singularity at the edge of the disk.
        // We want 1 + a.conj() * z to be 0.
        // Let a = -r, z = r.
        // 1 + (-r)(r) = 1 - r^2.
        // If r -> 1, 1 - r^2 -> 0.
        // Result = (r - (-r)) / (1 - r^2) = 2r / (1 - r^2) -> Infinity.

        let r = 0.9999999999999999; // Very close to 1.0
        let z = Point::new(r, 0.0);
        let a = Point::new(-r, 0.0); // -r is a valid point inside disk (barely)

        let res = mobius_add(z, a);
        println!("Havoc Result: {:?}", res);

        // This SHOULD fail if we are properly paranoid, but the current impl doesn't check for Inf/NaN output.
        // We assert that it IS a valid point (norm < 1.0), which it mathematically isn't (it escapes to infinity).
        // If the library claims to implement the Poincaré disk, the result MUST be in the disk.
        assert!(
            res.norm() < 1.0,
            "Singularity breach! Point escaped the disk: {:?}",
            res
        );
    }
}
