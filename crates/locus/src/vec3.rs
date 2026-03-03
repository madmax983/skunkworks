//! # 3-Dimensional Vectors
//!
//! Provides the lightweight [`Vec3`] type used by locus.
//!
//! This is typically the output structure returned from reducing or projecting higher
//! dimensions (like a `Vec4`) down into a 3D coordinate.

#[cfg(feature = "macroquad")]
use macroquad::prelude::Vec3 as MacroquadVec3;

/// A simple 3D vector for projection results.
///
/// This struct primarily serves as the output type for 4D->3D projections (e.g., from [`super::Vec4`]).
/// It is intentionally minimal, avoiding the heavy machinery of full linear algebra libraries
/// like `glam` or `nalgebra` to keep the `locus` crate lightweight and fast to compile.
///
/// # Why not use `[f32; 3]`?
///
/// Using a named struct provides type safety and clearer semantics (x, y, z accessors)
/// than raw arrays or tuples.
///
/// # Interoperability
///
/// If the `macroquad` feature is enabled, this type implements `From` and `Into` for
/// `macroquad::prelude::Vec3`, allowing seamless integration with that game engine.
///
/// # Examples
///
/// ```
/// use locus::vec3::Vec3;
///
/// let v = Vec3::new(1.0, 2.0, 3.0);
/// assert_eq!(v.z, 3.0);
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Vec3 {
    /// The X component.
    pub x: f32,
    /// The Y component.
    pub y: f32,
    /// The Z component.
    pub z: f32,
}

impl Vec3 {
    /// Creates a new 3D vector.
    ///
    /// # Arguments
    ///
    /// * `x` - The X component.
    /// * `y` - The Y component.
    /// * `z` - The Z component.
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

#[cfg(feature = "macroquad")]
impl From<Vec3> for MacroquadVec3 {
    fn from(v: Vec3) -> Self {
        MacroquadVec3::new(v.x, v.y, v.z)
    }
}

#[cfg(feature = "macroquad")]
impl From<MacroquadVec3> for Vec3 {
    fn from(v: MacroquadVec3) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}
