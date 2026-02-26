#[cfg(feature = "macroquad")]
use macroquad::prelude::Vec3 as MacroquadVec3;

/// A simple 3D vector for projection results.
///
/// Used to avoid dependency on external crates for core math types like `glam` or `nalgebra`
/// in the core logic, keeping the dependency tree light.
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
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
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
