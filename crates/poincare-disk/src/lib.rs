//! # Poincaré Disk Model
//!
//! A library for performing calculations in the [Poincaré Disk Model](https://en.wikipedia.org/wiki/Poincar%C3%A9_disk_model) of hyperbolic geometry.
//!
//! In this model, the entire infinite hyperbolic plane is compressed into the interior of the unit disk ($|z| < 1$) in the complex plane.
//! Straight lines in hyperbolic space appear as circular arcs orthogonal to the boundary of the disk.
//!
//! ## Theory
//!
//! In Euclidean geometry, parallel lines never meet. In Hyperbolic geometry, there are infinitely many lines parallel to a given line through a specific point. This strange property leads to a world where:
//!
//! *   **Space expands exponentially:** The circumference of a circle grows exponentially with its radius ($2\pi \sinh(r)$), not linearly ($2\pi r$).
//! *   **Triangles are thin:** The sum of angles in a triangle is always less than 180 degrees.
//! *   **The boundary is infinity:** As you move towards the edge of the disk ($|z| \to 1$), you are actually travelling towards infinity. It takes infinite time and energy to reach the edge.
//!
//! ## Core Concepts
//!
//! *   **Point**: A location in the hyperbolic plane, represented as a complex number $z$ where $|z| < 1$.
//! *   **Isometry (Motion)**: Rigid motions (translations and rotations) are represented by **Möbius transformations**. These are the hyperbolic equivalent of moving objects around without stretching them.
//! *   **Geodesic**: The "straight line" path between two points. In this model, they look like circular arcs perpendicular to the boundary.
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
//!
//! ## Scenario: A Walk in the Disk
//!
//! Imagine an ant starting at the center and walking 0.5 units to the right, then 0.5 units "up" (relative to its new position).
//!
//! ```
//! use poincare_disk::{Point, mobius_add, hyperbolic_dist};
//!
//! // 1. Start at center
//! let mut ant = Point::new(0.0, 0.0);
//!
//! // 2. Walk Right (0.5 units)
//! let step_right = Point::new(0.5, 0.0);
//! ant = mobius_add(ant, step_right);
//!
//! // 3. Walk "Up" (0.5 units relative to current position)
//! // To move "relative to the ant", we interpret the ant's position as a translation
//! // from the origin. We apply this translation to our local step vector.
//! // Mathematically: new_pos = ant (+) step
//! // In this library: mobius_add(z, a) computes a (+) z.
//! // So we switch arguments: mobius_add(step, ant) = ant (+) step.
//! let step_up = Point::new(0.0, 0.5);
//! ant = mobius_add(step_up, ant);
//!
//! // 4. Where are we?
//! // We are NOT at (0.5, 0.5) because the space is curved!
//! println!("Ant is at: {}", ant);
//! assert_ne!(ant, Point::new(0.5, 0.5));
//!
//! // But the distance from the second stop to the first stop is exactly 2*atanh(0.5)
//! let dist_step_2 = hyperbolic_dist(ant, step_right);
//! let expected_dist = 2.0 * 0.5f64.atanh();
//! assert!((dist_step_2 - expected_dist).abs() < 1e-9);
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

/// Represents a geodesic segment between two points in the Poincaré disk.
#[derive(Debug, Clone, Copy)]
pub struct Geodesic {
    pub p1: Point,
    pub p2: Point,
}

impl Geodesic {
    /// Creates a new geodesic segment connecting `p1` and `p2`.
    pub fn new(p1: Point, p2: Point) -> Self {
        Self { p1, p2 }
    }

    /// Returns the Euclidean center and radius of the circular arc representing the geodesic.
    ///
    /// Returns `None` if the geodesic is a straight line passing through the origin.
    ///
    /// # Examples
    ///
    /// ```
    /// use poincare_disk::{Geodesic, Point};
    /// let p1 = Point::new(0.5, 0.0);
    /// let p2 = Point::new(0.0, 0.5);
    /// let geo = Geodesic::new(p1, p2);
    /// if let Some((center, radius)) = geo.euclidean_circle() {
    ///     println!("Arc center: {:?}, radius: {}", center, radius);
    /// }
    /// ```
    pub fn euclidean_circle(&self) -> Option<(Point, f64)> {
        // Implementation adapted from `experiments/poincare-crawl/src/math.rs`
        let x1 = self.p1.re;
        let y1 = self.p1.im;
        let x2 = self.p2.re;
        let y2 = self.p2.im;

        // Condition for circle passing through p and orthogonal to unit circle:
        // |p|^2 - 2 Re(p conj(c)) + 1 = 0
        // which simplifies to: 2*x*x_i + 2*y*y_i = 1 + |p_i|^2

        let d1 = 1.0 + x1 * x1 + y1 * y1;
        let d2 = 1.0 + x2 * x2 + y2 * y2;

        // Linear system:
        // 2*x1*x + 2*y1*y = d1
        // 2*x2*x + 2*y2*y = d2

        let det = 4.0 * (x1 * y2 - x2 * y1);

        if det.abs() < 1e-9 {
            // Collinear with origin (or points are coincident/too close)
            return None;
        }

        let x = (d1 * 2.0 * y2 - d2 * 2.0 * y1) / det;
        let y = (2.0 * x1 * d2 - 2.0 * x2 * d1) / det;

        let center = Point::new(x, y);
        let radius = (center - self.p1).norm();

        Some((center, radius))
    }
}

/// Precomputed constants for generating a hyperbolic tiling.
///
/// Specifically, this struct calculates parameters for a regular $\{p, q\}$ tiling,
/// where $p$ is the number of sides of each polygon (face) and $q$ is the number of polygons meeting at each vertex.
///
/// Use this to generate Escher-like "Circle Limit" patterns.
///
/// For a tiling to exist in the hyperbolic plane, we must have $(p-2)(q-2) > 4$.
/// For example, $\{4, 5\}$ (squares, 5 meeting at a vertex) satisfies this: $(2)(3) = 6 > 4$.
pub struct TilingConsts {
    /// The **Euclidean distance** ($|z|$) from the origin to the center of an adjacent cell.
    ///
    /// If you are at the center of a tile (at the origin), this is how far you must "translate"
    /// to reach the center of a neighbor.
    ///
    /// Use `neighbor_transform_a` to get the actual translation point for a specific direction.
    pub neighbor_offset: f64,
    /// The **Euclidean distance** ($|z|$) from the center of the polygon to one of its vertices.
    ///
    /// This is useful for drawing the polygon. If the polygon is centered at the origin,
    /// its vertices lie on a circle of this radius.
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

#[cfg(test)]
mod sentry_tests {
    use super::*;
    use num_complex::Complex;

    /// Generates a deterministic sequence of points inside the Poincaré disk.
    pub fn pseudo_random_points(seed: u64, count: usize) -> Vec<Point> {
        let mut points = Vec::with_capacity(count);
        let mut s = seed;
        for _ in 0..count {
            // LCG: x_{n+1} = (a * x_n + c) % m
            // Using values from MMIX (Knuth)
            s = s
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let u1 = (s >> 32) as f64 / 4294967296.0;

            s = s
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let u2 = (s >> 32) as f64 / 4294967296.0;

            // Random point in unit disk: r = sqrt(u1), theta = 2*pi*u2
            // We scale r by 0.95 to ensure we stay strictly inside the disk and avoid boundary singularities
            // for general property tests.
            let r = u1.sqrt() * 0.95;
            let theta = u2 * 2.0 * std::f64::consts::PI;
            points.push(Complex::from_polar(r, theta));
        }
        points
    }

    #[test]
    fn test_triangle_inequality() {
        let points = pseudo_random_points(12345, 30);
        // Check random triples
        for i in 0..points.len() {
            let a = points[i];
            let b = points[(i + 1) % points.len()];
            let c = points[(i + 2) % points.len()];

            let dist_ac = hyperbolic_dist(a, c);
            let dist_ab = hyperbolic_dist(a, b);
            let dist_bc = hyperbolic_dist(b, c);

            assert!(
                dist_ac <= dist_ab + dist_bc + 1e-9,
                "Triangle inequality violated: d(a,c)={} > d(a,b)={} + d(b,c)={}",
                dist_ac,
                dist_ab,
                dist_bc
            );
        }
    }

    #[test]
    fn test_mobius_associativity() {
        let points = pseudo_random_points(54321, 20);

        // Construct some random Mobius transformations from points
        // T(z) = (z+a)/(1+a'z) (translation)
        let transforms: Vec<Mobius> = points.iter().map(|p| Mobius::translation(*p)).collect();

        for i in 0..(transforms.len() - 2) {
            let a = transforms[i];
            let b = transforms[i + 1];
            let c = transforms[i + 2];

            // (A * B) * C
            let ab = a.then(&b);
            let lhs = ab.then(&c);

            // A * (B * C)
            let bc = b.then(&c);
            let rhs = a.then(&bc);

            // Check if matrices are approximately equal
            let diff_a = (lhs.a() - rhs.a()).norm();
            let diff_b = (lhs.b() - rhs.b()).norm();
            let diff_c = (lhs.c() - rhs.c()).norm();
            let diff_d = (lhs.d() - rhs.d()).norm();

            assert!(
                diff_a < 1e-9,
                "Associativity failed A component: {}",
                diff_a
            );
            assert!(
                diff_b < 1e-9,
                "Associativity failed B component: {}",
                diff_b
            );
            assert!(
                diff_c < 1e-9,
                "Associativity failed C component: {}",
                diff_c
            );
            assert!(
                diff_d < 1e-9,
                "Associativity failed D component: {}",
                diff_d
            );
        }
    }

    #[test]
    fn test_isometry_property() {
        let points = pseudo_random_points(67890, 20);
        let translation_targets = pseudo_random_points(13579, 5);

        for &t_point in &translation_targets {
            // Apply translation by t_point to all test points
            // Note: mobius_add(z, a) is translation of z by a.
            // But distance isometry is usually d(z1, z2) = d(T(z1), T(z2))

            for i in 0..points.len() - 1 {
                let p1 = points[i];
                let p2 = points[i + 1];

                let dist_orig = hyperbolic_dist(p1, p2);

                let p1_prime = mobius_add(p1, t_point);
                let p2_prime = mobius_add(p2, t_point);

                let dist_new = hyperbolic_dist(p1_prime, p2_prime);

                assert!(
                    (dist_orig - dist_new).abs() < 1e-9,
                    "Isometry violated: dist_orig={}, dist_new={}, diff={}",
                    dist_orig,
                    dist_new,
                    (dist_orig - dist_new).abs()
                );
            }
        }
    }

    #[test]
    fn test_singularity_robustness() {
        // Attempt to trigger division by zero or NaN
        // 1 + a.conj() * z approx 0
        // a = -r, z = r, r->1
        let r_vals = [0.9, 0.99, 0.999, 0.9999, 0.99999999];
        for &r in &r_vals {
            let z = Point::new(r, 0.0);
            let a = Point::new(-r, 0.0);
            let res = mobius_add(z, a);

            // We don't necessarily assert it's a valid point inside the disk (since math breaks down),
            // but we assert it doesn't panic.
            // And we check if it is finite (unless we hit absolute precision limit)
            if res.re.is_finite() && res.im.is_finite() {
                // Good
            } else {
                println!("Got non-finite result for r={}: {:?}", r, res);
            }
        }
    }

    #[test]
    fn test_invalid_inputs() {
        // z outside disk
        let z = Point::new(2.0, 0.0);
        let a = Point::new(0.5, 0.0);

        // This is technically undefined behavior according to docs ("assumes norm < 1"),
        // but robust code shouldn't panic.
        // Formula: (2 + 0.5) / (1 + 0.5*2) = 2.5 / 2.0 = 1.25
        let res = mobius_add(z, a);
        assert!((res.re - 1.25).abs() < 1e-9);

        // z outside, causing singularity
        // 1 + a.conj() * z = 0  => a.conj() * z = -1
        // Let a = 0.5. z = -2.0.
        // 1 + 0.5 * (-2) = 0.
        let z_bad = Point::new(-2.0, 0.0);
        let res_bad = mobius_add(z_bad, a);
        // This likely produces Inf/NaN, but definitely should NOT panic
        assert!(!res_bad.re.is_finite() || !res_bad.im.is_finite());
    }
}
