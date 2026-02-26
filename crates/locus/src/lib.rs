//! # Locus 📍
//!
//! A lightweight 2D geometry library for TUI applications and grid-based simulations.
//!
//! `locus` provides the fundamental primitives for moving, measuring, and mapping coordinates
//! in discrete or continuous space.
//!
//! ## Features
//!
//! - **`Vec2`**: A robust 2D vector struct for physics and movement.
//! - **`Topology`**: A system for defining how your world wraps (Plane, Torus, Klein Bottle, etc.).
//! - **`serde`**: (Optional) Enables `Serialize` and `Deserialize`.
//!
//! ## Example: The Hero's Journey (Moving on a Torus)
//!
//! ```
//! use locus::{Vec2, Topology};
//!
//! # fn main() {
//!     let width = 20;
//!     let height = 10;
//!     let topo = Topology::Torus;
//!
//!     // Start at position (x=19.0, y=5.0) - at the right edge
//!     let mut position = Vec2::new(19.0, 5.0);
//!     let velocity = Vec2::new(1.0, 0.0); // Moving right
//!
//!     // Move
//!     position += velocity;
//!
//!     // Normalize using Topology to find the grid cell
//!     // Note: Topology expects (row, col) i.e. (y, x) integers
//!     let y_idx = position.y.round() as i64;
//!     let x_idx = position.x.round() as i64;
//!
//!     if let Some((ny, nx)) = topo.normalize(y_idx, x_idx, width, height) {
//!         // Should wrap to left side (x=0)
//!         assert_eq!(nx, 0);
//!         assert_eq!(ny, 5);
//!     }
//! # }
//! ```

pub mod flocking;
pub mod topology;
pub mod vec2;
pub mod vec3;
pub mod vec4;

pub use topology::*;
pub use vec2::*;
pub use vec3::*;
pub use vec4::*;
