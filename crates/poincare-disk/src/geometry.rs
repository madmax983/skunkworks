//! # Hyperbolic Geometry Primitives
//!
//! Provides geometric primitives such as [`Geodesic`].
//!
//! In the Poincaré disk model, straight lines (geodesics) appear as circular arcs
//! that intersect the boundary of the unit disk orthogonally. This module provides
//! tools to work with these non-Euclidean structures.

use crate::types::Point;

/// Represents a geodesic segment between two points in the Poincaré disk.
#[derive(Debug, Clone, Copy)]
pub struct Geodesic {
    pub p1: Point,
    pub p2: Point,
}

impl Geodesic {
    /// Creates a new geodesic segment connecting `p1` and `p2`.
    ///
    /// # Examples
    ///
    /// ```
    /// use poincare_disk::{Geodesic, Point};
    /// let p1 = Point::new(0.5, 0.0);
    /// let p2 = Point::new(0.0, 0.5);
    /// let geo = Geodesic::new(p1, p2);
    /// assert_eq!(geo.p1.re, 0.5);
    /// ```
    pub fn new(p1: Point, p2: Point) -> Self {
        Self { p1, p2 }
    }

    /// Returns the Euclidean center and radius of the circular arc representing the geodesic.
    ///
    /// Returns `None` if the geodesic is a straight line passing through the origin.
    ///
    /// # Theory
    ///
    /// A geodesic in the Poincaré disk is a circular arc that intersects the boundary unit circle orthogonally.
    /// Let the center of this arc be $c = (x, y)$ and its radius be $R$.
    ///
    /// 1.  **Orthogonality condition**: Two circles are orthogonal if $d^2 = R^2 + r^2$, where $d$ is the distance between centers and $r$ is the radius of the other circle.
    ///     Here, the unit circle is centered at $(0,0)$ with radius $r=1$.
    ///     So, $|c|^2 = R^2 + 1$.
    ///
    /// 2.  **Point on circle**: For any point $p$ on the geodesic, $|p - c|^2 = R^2$.
    ///     Expanding this: $|p|^2 - 2\text{Re}(p\bar{c}) + |c|^2 = R^2$.
    ///
    /// Substituting $|c|^2 = R^2 + 1$:
    /// $$ |p|^2 - 2\text{Re}(p\bar{c}) + R^2 + 1 = R^2 $$
    /// $$ |p|^2 - 2\text{Re}(p\bar{c}) + 1 = 0 $$
    ///
    /// This gives us a linear equation for the coordinates $(x, y)$ of $c$:
    /// $$ 2x p_x + 2y p_y = 1 + |p|^2 $$
    ///
    /// With two points $p_1$ and $p_2$, we have a system of two linear equations which can be solved for $x$ and $y$.
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

        // Condition derived above: 2*x*x_i + 2*y*y_i = 1 + |p_i|^2

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
