//! # Hyperbolic Tilings
//!
//! Tools for calculating and applying hyperbolic tilings.
//!
//! A regular $\{p, q\}$ tiling consists of polygons with $p$ sides, where $q$ polygons
//! meet at each vertex. In the hyperbolic plane, there are infinitely many possible
//! regular tilings because the sum of angles of a triangle can be arbitrarily small.
//!
//! This module provides [`TilingConsts`] to help compute the distances and translations
//! needed to draw and navigate these tilings.

use crate::types::Point;
use num_complex::Complex;
use std::f64::consts::PI;

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
    /// This is an "order-5 square tiling". It consists of squares ($p=4$) where 5 squares meet at every vertex ($q=5$).
    ///
    /// # Theory
    ///
    /// To calculate the dimensions, we consider a fundamental right-angled triangle formed by:
    /// *   The center of a polygon (angle $A = \pi/p$).
    /// *   A vertex of the polygon (angle $B = \pi/q$).
    /// *   The midpoint of an edge (angle $C = \pi/2$).
    ///
    /// Let the side lengths be:
    /// *   $r$ (inradius): Center to edge midpoint.
    /// *   $R$ (circumradius): Center to vertex.
    /// *   $l$ (half-edge): Vertex to edge midpoint.
    ///
    /// Using the Hyperbolic Law of Cosines for angles ($\cos C = -\cos A \cos B + \sin A \sin B \cosh c$):
    ///
    /// 1.  **For side $l$ (opposite $A$):**
    ///     $$ \cos(\pi/p) = \sin(\pi/q) \cosh(l) \implies \cosh(l) = \frac{\cos(\pi/p)}{\sin(\pi/q)} $$
    ///
    /// 2.  **For side $r$ (opposite $B$):**
    ///     $$ \cos(\pi/q) = \sin(\pi/p) \cosh(r) \implies \cosh(r) = \frac{\cos(\pi/q)}{\sin(\pi/p)} $$
    ///
    /// 3.  **For side $R$ (hypotenuse):**
    ///     $$ \cosh(R) = \cosh(r) \cosh(l) $$
    ///
    /// Finally, we convert these hyperbolic distances to Euclidean distances in the Poincaré disk using $r_{euclid} = \tanh(r_{hyperbolic} / 2)$.
    ///
    /// *   `neighbor_offset` corresponds to moving $2r$ (distance between centers), so the Mobius translation is $\tanh(r)$.
    /// *   `vertex_offset` corresponds to $R$, so Euclidean distance is $\tanh(R/2)$.
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

        let pi_p = PI / p;
        let pi_q = PI / q;

        let sin_pi_p = pi_p.sin();
        let cos_pi_p = pi_p.cos();
        let sin_pi_q = pi_q.sin();
        let cos_pi_q = pi_q.cos();

        // Calculate hyperbolic cosine of inradius (r) and half-edge (l)
        let cosh_r = cos_pi_q / sin_pi_p;
        let cosh_l = cos_pi_p / sin_pi_q;

        // Calculate hyperbolic inradius (r)
        let inradius_hyperbolic = cosh_r.acosh();

        // Calculate hyperbolic circumradius (R)
        let cosh_circumradius = cosh_r * cosh_l;
        let circumradius_hyperbolic = cosh_circumradius.acosh();

        // neighbor_offset: Euclidean translation to move 2*r (center to center)
        // displacement = tanh( (2*r) / 2 ) = tanh(r)
        let neighbor_offset = inradius_hyperbolic.tanh();

        // vertex_offset: Euclidean distance of vertex from center
        // dist = tanh( R / 2 )
        let vertex_offset = (circumradius_hyperbolic / 2.0).tanh();

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
/// *   `direction`: An index `0..4` representing the neighbor direction:
///     *   `0` = Right ($0$ rad)
///     *   `1` = Up ($\pi/2$ rad)
///     *   `2` = Left ($\pi$ rad)
///     *   `3` = Down ($3\pi/2$ rad)
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
    fn test_tiling_consts() {
        let c = TilingConsts::new_4_5();
        // Check that neighbor offset is reasonable (less than 1, greater than 0)
        assert!(c.neighbor_offset > 0.0);
        assert!(c.neighbor_offset < 1.0);
        // Approx check: ~0.485
        assert!((c.neighbor_offset - 0.485).abs() < 0.01);
    }
}
