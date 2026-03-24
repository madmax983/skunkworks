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
/// This struct primarily serves as the output type for 4D->3D projections (e.g., from [`crate::vec4::Vec4`]).
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
/// ## Basic Usage
///
/// ```
/// use locus::vec3::Vec3;
///
/// let v = Vec3::new(1.0, 2.0, 3.0);
/// assert_eq!(v.z, 3.0);
/// ```
///
/// ## From 4D to 3D
///
/// ```
/// use locus::vec4::Vec4;
/// use locus::vec3::Vec3;
///
/// let v4 = Vec4::new(1.0, 2.0, 3.0, 4.0);
/// let camera_w = 10.0;
/// // Projecting the 4D point into 3D view space
/// let projected: Vec3 = v4.project_to_3d(camera_w);
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Vec3 {
    /// The X component (horizontal).
    pub x: f32,
    /// The Y component (vertical).
    pub y: f32,
    /// The Z component (depth).
    pub z: f32,
}

impl Vec3 {
    /// Creates a new 3D vector.
    ///
    /// This is useful when you need to manually construct a 3D point,
    /// or when extracting components from a higher-dimensional structure.
    ///
    /// # Arguments
    ///
    /// * `x` - The X component.
    /// * `y` - The Y component.
    /// * `z` - The Z component.
    ///
    /// # Examples
    ///
    /// ```
    /// use locus::vec3::Vec3;
    ///
    /// let point = Vec3::new(1.0, 0.0, -1.0);
    /// assert_eq!(point.x, 1.0);
    /// ```
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

#[cfg(feature = "macroquad")]
/// Converts a `locus::vec3::Vec3` into a `macroquad::prelude::Vec3`.
///
/// This is a "Ghost Conversion" that is vital for drawing projected coordinates
/// using Macroquad's 3D rendering APIs without manual field mapping.
///
/// # Examples
///
/// ```ignore
/// // Requires the `macroquad` feature
/// use locus::vec3::Vec3;
/// use macroquad::prelude::Vec3 as MqVec3;
///
/// let locus_vec = Vec3::new(1.0, 2.0, 3.0);
/// let mq_vec: MqVec3 = locus_vec.into();
///
/// assert_eq!(mq_vec.z, 3.0);
/// ```
impl From<Vec3> for MacroquadVec3 {
    fn from(v: Vec3) -> Self {
        MacroquadVec3::new(v.x, v.y, v.z)
    }
}

#[cfg(feature = "macroquad")]
/// Converts a `macroquad::prelude::Vec3` into a `locus::vec3::Vec3`.
///
/// Allows returning data from Macroquad APIs (like camera targets or raycasts)
/// back into the lightweight `locus` ecosystem.
///
/// # Examples
///
/// ```ignore
/// // Requires the `macroquad` feature
/// use locus::vec3::Vec3;
/// use macroquad::prelude::Vec3 as MqVec3;
///
/// let mq_vec = MqVec3::new(10.0, 20.0, 30.0);
/// let locus_vec: Vec3 = mq_vec.into();
///
/// assert_eq!(locus_vec.y, 20.0);
/// ```
impl From<MacroquadVec3> for Vec3 {
    fn from(v: MacroquadVec3) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}
