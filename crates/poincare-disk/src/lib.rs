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

pub(crate) mod geometry;
pub(crate) mod math;
pub(crate) mod tiling;
pub(crate) mod transform;
pub(crate) mod types;

// Re-export the public API for convenience and backward compatibility
pub use geometry::Geodesic;
pub use math::{hyperbolic_dist, mobius_add, mobius_sub};
pub use tiling::{neighbor_transform_a, TilingConsts};
pub use transform::Mobius;
pub use types::Point;
