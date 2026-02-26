//! # The Map Room 🗺️
//!
//! `Topology` is the rulebook for how your world connects. It defines whether the edge of the map
//! is a cliff (Plane), a portal to the other side (Torus), or a mind-bending twist (Klein Bottle).
//!
//! Understanding your topology is crucial for:
//! - **Movement**: Knowing where an entity ends up when it crosses a boundary.
//! - **Distance**: Calculating the shortest path between two points (e.g., on a sphere).
//! - **Simulation**: Creating closed systems without artificial walls.

/// Represents the topology of a grid or space.
///
/// Determines how coordinates wrap or bound at the edges. This allows for simulating
/// different geometric surfaces (like a donut-shaped world or a Klein bottle)
/// using a simple 2D grid.
///
/// # Bridging the Gap: Vectors vs. Grids
///
/// Be mindful when converting between continuous space (like [`super::Vec2`]) and discrete grids.
/// - [`super::Vec2`] uses `(x, y)` (Cartesian coordinates).
/// - [`Topology::normalize`] uses `(y, x)` (Row-Major / Matrix indexing).
///
/// When using them together, ensure you swap the components correctly.
///
/// ```rust
/// use locus::{Vec2, Topology};
/// let pos = Vec2::new(10.5, 5.2);
/// let (y, x) = (pos.y as i64, pos.x as i64);
/// // Topology::normalize(y, x, ...);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[doc(alias = "wrapping")]
#[doc(alias = "boundary")]
pub enum Topology {
    /// **Plane**: The "Flat Earth".
    ///
    /// Edges are hard walls. Coordinates outside `[0, width)` or `[0, height)` are invalid.
    ///
    /// ```text
    /// +---+
    /// |   |
    /// +---+
    /// ```
    Plane,

    /// **Torus**: The "Arcade Loop".
    ///
    /// Wraps both X and Y. Walking off the right edge brings you to the left,
    /// and walking off the bottom brings you to the top.
    ///
    /// * `x` wraps to `x % width`
    /// * `y` wraps to `y % height`
    ///
    /// ```text
    ///    ^
    ///    |
    /// <--+--> (Wraps horizontally)
    ///    |
    ///    v (Wraps vertically)
    /// ```
    ///
    /// # The "Pac-Man Effect"
    ///
    /// ```
    /// use locus::Topology;
    /// let topo = Topology::Torus;
    /// // Walking off the right edge (x=10) of a width-10 map wraps to x=0.
    /// assert_eq!(topo.normalize(5, 10, 10, 10), Some((5, 0)));
    /// ```
    Torus,

    /// **Horizontal Cylinder**: The "Infinite Tube".
    ///
    /// Wraps X (Horizontal), Bounded Y (Vertical).
    ///
    /// * `x` wraps around.
    /// * `y` is bounded (hard walls at top/bottom).
    ///
    /// ```text
    /// +-----+
    /// |     |
    /// <--+--> (Wraps horizontally)
    /// |     |
    /// +-----+
    /// ```
    CylinderH,

    /// **Vertical Cylinder**: The "Infinite Scroll".
    ///
    /// Bounded X (Horizontal), Wraps Y (Vertical).
    ///
    /// * `x` is bounded (hard walls at left/right).
    /// * `y` wraps around.
    ///
    /// ```text
    ///    ^
    ///    |
    /// +--+--+
    /// |  |  |
    /// +--+--+
    ///    |
    ///    v (Wraps vertically)
    /// ```
    CylinderV,

    /// **Klein Bottle**: The "Twisted Tube".
    ///
    /// A non-orientable surface. Wraps X normally. Wraps Y with a twist in X.
    ///
    /// * `x` wraps normally (`x % width`).
    /// * `y` wraps (`y % height`), but if it wraps, `x` is mirrored: `x' = (width - 1) - x`.
    ///
    /// ```text
    ///    ^
    ///    |
    /// <--+--> (Wraps horizontally)
    ///    |
    ///    X (Twists vertically: x -> width - 1 - x)
    /// ```
    Klein,

    /// **Möbius Strip**: The "Twisted Path".
    ///
    /// A non-orientable surface with a boundary. Wraps X with a twist, Bounded Y.
    ///
    /// * If `x` wraps (off left/right), `y` is mirrored: `y' = (height - 1) - y`.
    /// * `y` is bounded (cannot wrap).
    ///
    /// ```text
    /// +-----+
    /// |     |
    /// X--+--X (Twists horizontally: y -> height - 1 - y)
    /// |     |
    /// +-----+
    /// ```
    Mobius,

    /// **Hyperbolic**: The "Infinite Disk".
    ///
    /// Poincaré Disk model mapping. Typically handled externally or treated as bounded.
    Hyperbolic,

    /// **Sphere**: The "Globe".
    ///
    /// Wraps X, Bounded Y with Antipodal Shift.
    ///
    /// * `x` wraps normally (`x % width`).
    /// * `y` wraps (`y % height`), but if it crosses a pole, `x` shifts by `width / 2`
    ///   and `y` is reflected.
    Sphere,

    /// **Real Projective Plane**: The "Double Twist".
    ///
    /// Wraps both X and Y with a twist.
    ///
    /// * If `x` wraps, `y` is mirrored: `y' = (height - 1) - y`.
    /// * If `y` wraps, `x` is mirrored: `x' = (width - 1) - x`.
    Projective,
}

impl Topology {
    /// Normalizes coordinates based on the topology and grid size.
    ///
    /// This function takes arbitrary signed coordinates (which may be negative or
    /// larger than the grid dimensions) and maps them to a valid `(row, col)` index
    /// within the grid, if possible.
    ///
    /// # Warning: Coordinate Systems ⚠️
    ///
    /// This function expects Matrix/Grid coordinates: `(row, col)` which corresponds to `(y, x)`.
    /// This is the reverse of standard Cartesian `(x, y)`.
    ///
    /// # Returns
    ///
    /// * `Some((row, col))` (i.e., `(y, x)`) if the coordinates are valid or successfully wrapped.
    /// * `None` if the coordinates are out of bounds (for bounded topologies).
    ///
    /// # Arguments
    ///
    /// * `y` - The Y coordinate (row/vertical).
    /// * `x` - The X coordinate (column/horizontal).
    /// * `width` - The width of the grid (number of columns).
    /// * `height` - The height of the grid (number of rows).
    ///
    /// # Examples
    ///
    /// ```
    /// use locus::Topology;
    ///
    /// // ---------------------------------------------------------
    /// // Torus: The Classic Video Game World (Wraps Both Ways)
    /// // ---------------------------------------------------------
    /// let topo = Topology::Torus;
    /// // Wrapping off top-left corner (-1, -1) -> Bottom-Right (9, 9)
    /// assert_eq!(topo.normalize(-1, -1, 10, 10), Some((9, 9)));
    ///
    /// // ---------------------------------------------------------
    /// // Plane: Hard Boundaries (No Wrapping)
    /// // ---------------------------------------------------------
    /// let plane = Topology::Plane;
    /// assert_eq!(plane.normalize(-1, 0, 10, 10), None);
    ///
    /// // ---------------------------------------------------------
    /// // Klein Bottle: A Twist in the Fabric
    /// // ---------------------------------------------------------
    /// // Wraps X normally, but twists X when wrapping Y.
    /// let klein = Topology::Klein;
    ///
    /// // Moving off the top edge (y=-1) wraps to bottom (y=9)
    /// // BUT flips the X coordinate (x=2 becomes width-1-2 = 7)
    /// assert_eq!(klein.normalize(-1, 2, 10, 10), Some((9, 7)));
    ///
    /// // ---------------------------------------------------------
    /// // CylinderV: The Infinite Scroll (Wraps Y, Bounded X)
    /// // ---------------------------------------------------------
    /// let cyl_v = Topology::CylinderV;
    ///
    /// // Walking off bottom (y=10) wraps to top (y=0)
    /// assert_eq!(cyl_v.normalize(10, 5, 10, 10), Some((0, 5)));
    ///
    /// // Walking off side (x=10) hits a wall (None)
    /// assert_eq!(cyl_v.normalize(5, 10, 10, 10), None);
    /// ```
    pub fn normalize(&self, y: i64, x: i64, width: usize, height: usize) -> Option<(usize, usize)> {
        if width == 0 || height == 0 {
            return None;
        }
        // Protect against overflow when casting to i64 (e.g. usize::MAX -> -1)
        if width > i64::MAX as usize || height > i64::MAX as usize {
            return None;
        }

        let w = width as i64;
        let h = height as i64;

        // Optimization: Fast path for in-bounds coordinates.
        // For all topologies, if the coordinates are within the grid bounds,
        // no wrapping or twisting is needed. This avoids expensive division/modulo operations.
        if x >= 0 && x < w && y >= 0 && y < h {
            return Some((y as usize, x as usize));
        }

        match self {
            Topology::Plane | Topology::Hyperbolic => {
                // If we reached here, coordinates are out of bounds.
                None
            }
            Topology::Torus => {
                let ny = y.rem_euclid(h);
                let nx = x.rem_euclid(w);
                Some((ny as usize, nx as usize))
            }
            Topology::CylinderH => {
                if y >= 0 && y < h {
                    let nx = x.rem_euclid(w);
                    Some((y as usize, nx as usize))
                } else {
                    None
                }
            }
            Topology::CylinderV => {
                if x >= 0 && x < w {
                    let ny = y.rem_euclid(h);
                    Some((ny as usize, x as usize))
                } else {
                    None
                }
            }
            Topology::Klein => {
                let ny = y.rem_euclid(h);
                let wrap_y = y.div_euclid(h);
                let mut nx = x.rem_euclid(w);
                if wrap_y % 2 != 0 {
                    nx = (w - 1) - nx;
                }
                Some((ny as usize, nx as usize))
            }
            Topology::Mobius => {
                let nx = x.rem_euclid(w);
                let wrap_x = x.div_euclid(w);
                if wrap_x % 2 != 0 {
                    // Twisted Y: map y to (h - 1) - y
                    // Use checked arithmetic to prevent panic on i64::MIN
                    if let Some(twisted_y) = (h - 1).checked_sub(y) {
                        if twisted_y >= 0 && twisted_y < h {
                            Some((twisted_y as usize, nx as usize))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else if y >= 0 && y < h {
                    Some((y as usize, nx as usize))
                } else {
                    None
                }
            }
            Topology::Sphere => {
                let wrap_y = y.div_euclid(h);
                let mut ny = y.rem_euclid(h);
                let mut nx = x.rem_euclid(w);

                if wrap_y % 2 != 0 {
                    // Crossed pole: reflect Y and shift X
                    ny = (h - 1) - ny;
                    // Avoid overflow: (nx + w/2) can exceed i64::MAX if w is large.
                    // Instead of % w, we use a conditional add/sub.
                    // Since nx < w and w/2 < w, the max value is < 2w, so one subtraction is enough.
                    let shift = w / 2;
                    if nx < w - shift {
                        nx += shift;
                    } else {
                        nx -= w - shift;
                    }
                }

                Some((ny as usize, nx as usize))
            }
            Topology::Projective => {
                let wrap_x = x.div_euclid(w);
                let wrap_y = y.div_euclid(h);

                let mut nx = x.rem_euclid(w);
                let mut ny = y.rem_euclid(h);

                if wrap_x % 2 != 0 {
                    ny = (h - 1) - ny;
                }

                if wrap_y % 2 != 0 {
                    nx = (w - 1) - nx;
                }

                Some((ny as usize, nx as usize))
            }
        }
    }
}

#[cfg(test)]
mod topology_tests {
    use super::*;

    #[test]
    fn test_klein_wrapping() {
        let topo = Topology::Klein;
        let width = 10;
        let height = 10;
        // Normal wrapping (even wrap)
        // y = 20 -> wraps to 0. No twist. x=5 -> 5.
        assert_eq!(topo.normalize(20, 5, width, height), Some((0, 5)));

        // Twisted wrapping (odd wrap)
        // y = 10 -> wraps to 0. 1 wrap (odd). Twist x.
        // x = 2 -> 9 - 2 = 7.
        assert_eq!(topo.normalize(10, 2, width, height), Some((0, 7)));

        // Twisted wrapping (odd wrap) with negative y
        // y = -1 -> wraps to 9. -1 div 10 = -1 (odd). Twist x.
        // x = 2 -> 9 - 2 = 7.
        assert_eq!(topo.normalize(-1, 2, width, height), Some((9, 7)));

        // X out of bounds + Twist
        // y = 10 (twist). x = 12.
        // x normalized: 12 % 10 = 2.
        // twist: 9 - 2 = 7.
        assert_eq!(topo.normalize(10, 12, width, height), Some((0, 7)));
    }

    #[test]
    fn test_rectangular_klein() {
        let topo = Topology::Klein;
        let width = 10;
        let height = 20;

        // y = -1 -> y wraps to 19. Twist x.
        // x = 2 -> x = 10-1-2 = 7.
        assert_eq!(topo.normalize(-1, 2, width, height), Some((19, 7)));
    }

    #[test]
    fn test_mobius_wrapping() {
        let topo = Topology::Mobius;
        let width = 10;
        let height = 10;

        // Normal wrapping (even wrap)
        // x = 20 -> 0. y=5 -> 5.
        assert_eq!(topo.normalize(5, 20, width, height), Some((5, 0)));

        // Twisted wrapping (odd wrap)
        // x = 10 -> 0. 1 wrap. Twist y.
        // y = 2 -> 9 - 2 = 7.
        assert_eq!(topo.normalize(2, 10, width, height), Some((7, 0)));

        // Twisted wrapping + Y out of bounds
        // x = 10 (twist). y = 12.
        // twist y: 9 - 12 = -3.
        // -3 out of bounds. -> None.
        assert_eq!(topo.normalize(12, 10, width, height), None);
    }

    #[test]
    fn test_sphere_wrapping() {
        let topo = Topology::Sphere;
        let width = 10;
        let height = 10;

        // Normal wrapping in X
        // x = 10 -> 0
        assert_eq!(topo.normalize(5, 10, width, height), Some((5, 0)));

        // Crossing North Pole (y = -1)
        // wrap_y = -1 (odd).
        // ny = -1 % 10 = 9. Reflected: 9 - 9 = 0.
        // nx = 2 + 5 = 7.
        // Expected: (0, 7)
        assert_eq!(topo.normalize(-1, 2, width, height), Some((0, 7)));

        // Crossing South Pole (y = 10)
        // wrap_y = 1 (odd).
        // ny = 10 % 10 = 0. Reflected: 9 - 0 = 9.
        // nx = 2 + 5 = 7.
        // Expected: (9, 7)
        assert_eq!(topo.normalize(10, 2, width, height), Some((9, 7)));
    }

    #[test]
    fn test_projective_wrapping() {
        let topo = Topology::Projective;
        let width = 10;
        let height = 10;

        // Wrap X (twist Y)
        // x = 10 -> 0. wrap_x = 1 (odd).
        // y = 2 -> 9 - 2 = 7.
        assert_eq!(topo.normalize(2, 10, width, height), Some((7, 0)));

        // Wrap Y (twist X)
        // y = 10 -> 0. wrap_y = 1 (odd).
        // x = 2 -> 9 - 2 = 7.
        assert_eq!(topo.normalize(10, 2, width, height), Some((0, 7)));

        // Wrap Both (double twist)
        // x = 10 -> 0. wrap_x = 1.
        // y = 10 -> 0. wrap_y = 1.
        // nx = 0 -> 9 - 0 = 9.
        // ny = 0 -> 9 - 0 = 9.
        assert_eq!(topo.normalize(10, 10, width, height), Some((9, 9)));
    }
}
