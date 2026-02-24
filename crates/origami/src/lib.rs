//! # Origami 🦢
//!
//! A library for generating procedural origami meshes, specifically the **Miura-ori** fold.
//!
//! The Miura-ori is a method of folding a flat surface such as a sheet of paper into a smaller area.
//! It consists of a tessellated pattern of parallelograms and is known for its property of
//! having a single degree of freedom—meaning the entire structure can be expanded or contracted
//! with a single motion.
//!
//! ## Core Concepts
//!
//! *   **[`MiuraOri`]**: The main generator struct. It takes parameters and a grid size, and produces meshes.
//! *   **[`MiuraParams`]**: Configuration for the geometric properties of the fold (unit cell dimensions, angle).
//! *   **Extension Factor**: A value from 0.0 (collapsed) to 1.0 (fully expanded) that drives the simulation.
//!
//! ## Example
//!
//! ```
//! use origami::{MiuraOri, MiuraParams, Orientation};
//!
//! // Define the fold parameters
//! let params = MiuraParams {
//!     a: 1.0,      // Side length A
//!     b: 1.0,      // Side length B
//!     gamma: 1.4,  // Fold angle (radians)
//!     orientation: Orientation::Horizontal,
//! };
//!
//! // Create a 10x10 grid generator
//! let origami = MiuraOri::new(params, (10, 10));
//!
//! // Generate the mesh at 50% expansion
//! let mesh = origami.generate_mesh(0.5);
//!
//! assert_eq!(mesh.vertices.len(), 11 * 11); // (cols+1) * (rows+1)
//! ```

use macroquad::prelude::*;

/// Orientation of the fold pattern.
///
/// The Miura-ori pattern is anisotropic. The "zig-zag" happens along one axis,
/// creating different structural properties and visual appearances.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Orientation {
    /// Zig-zag along X axis (rows shift). Height map forms stripes along Y.
    ///
    /// Commonly used when the compression is desired along the horizontal axis.
    Horizontal,
    /// Zig-zag along Y axis (cols shift). Height map forms a checkerboard pattern.
    ///
    /// Used when compression is desired along the vertical axis.
    Vertical,
}

impl Default for Orientation {
    fn default() -> Self {
        Self::Horizontal
    }
}

/// Geometric parameters for the Miura-ori unit cell.
///
/// A Miura-ori pattern is defined by a repeating parallelogram unit cell.
#[derive(Clone, Copy, Debug)]
pub struct MiuraParams {
    /// Length of the edge `a` of the unit cell.
    pub a: f32,
    /// Length of the edge `b` of the unit cell.
    pub b: f32,
    /// The angle `gamma` of the unit cell (in radians).
    ///
    /// This defines the "shear" of the pattern.
    pub gamma: f32,
    /// The orientation of the zig-zag pattern.
    pub orientation: Orientation,
}

/// A single vertex in the generated origami mesh.
pub struct OrigamiVertex {
    /// 3D position of the vertex.
    pub pos: Vec3,
    /// UV Texture coordinates (0.0 - 1.0).
    pub uv: Vec2,
}

/// A complete mesh generated from the origami pattern.
///
/// Contains vertices and indices suitable for rendering with `macroquad` or other engines.
pub struct OrigamiMesh {
    /// List of vertices.
    pub vertices: Vec<OrigamiVertex>,
    /// List of indices forming triangles.
    pub indices: Vec<u16>,
}

/// The generator for Miura-ori patterns.
///
/// Use this struct to calculate the geometry of a folded sheet at various stages of expansion.
pub struct MiuraOri {
    /// The geometric parameters of the fold.
    pub params: MiuraParams,
    /// The dimensions of the grid (cols, rows).
    pub grid_size: (usize, usize),
}

impl MiuraOri {
    /// Creates a new Miura-ori generator.
    ///
    /// # Arguments
    ///
    /// * `params` - The geometric parameters ([`MiuraParams`]).
    /// * `grid_size` - A tuple `(cols, rows)` defining the number of unit cells in X and Y.
    pub fn new(params: MiuraParams, grid_size: (usize, usize)) -> Self {
        Self { params, grid_size }
    }

    /// Generates the full mesh (vertices with UVs + indices).
    ///
    /// # Arguments
    ///
    /// * `extension_factor` - A value between `0.0` (fully collapsed) and `1.0` (fully expanded).
    ///   Clamped to `[0.001, 1.0]` internally to avoid singularities.
    ///
    /// # Returns
    ///
    /// An [`OrigamiMesh`] containing the vertex data and index buffer for rendering.
    pub fn generate_mesh(&self, extension_factor: f32) -> OrigamiMesh {
        let (vertices_pos, _, _) = self.calculate_positions(extension_factor);
        let (cols, rows) = self.grid_size;

        let mut vertices = Vec::with_capacity(vertices_pos.len());
        for (idx, pos) in vertices_pos.iter().enumerate() {
            let i = idx % (cols + 1);
            let j = idx / (cols + 1);
            vertices.push(OrigamiVertex {
                pos: *pos,
                uv: vec2(i as f32 / cols as f32, j as f32 / rows as f32),
            });
        }

        let mut indices = Vec::new();
        for j in 0..rows {
            for i in 0..cols {
                let v_cols = cols + 1;
                let p00 = (j * v_cols + i) as u16;
                let p10 = (j * v_cols + (i + 1)) as u16;
                let p01 = ((j + 1) * v_cols + i) as u16;
                let p11 = ((j + 1) * v_cols + (i + 1)) as u16;

                // Two triangles
                indices.push(p00);
                indices.push(p10);
                indices.push(p01);

                indices.push(p10);
                indices.push(p11);
                indices.push(p01);
            }
        }

        OrigamiMesh { vertices, indices }
    }

    /// Generates just the grid of vertex positions (row-major order).
    ///
    /// Useful if you only need the physics/geometry points and not a renderable mesh.
    pub fn generate_grid(&self, extension_factor: f32) -> Vec<Vec3> {
        self.calculate_positions(extension_factor).0
    }

    /// Internal helper to calculate positions
    fn calculate_positions(&self, extension_factor: f32) -> (Vec<Vec3>, f32, f32) {
        match self.params.orientation {
            Orientation::Horizontal => self.calculate_horizontal(extension_factor),
            Orientation::Vertical => self.calculate_vertical(extension_factor),
        }
    }

    fn calculate_horizontal(&self, extension_factor: f32) -> (Vec<Vec3>, f32, f32) {
        let (cols, rows) = self.grid_size;
        let a = self.params.a;
        let b = self.params.b;
        let gamma = self.params.gamma;

        let cos_gamma = gamma.cos();
        let theta_min = cos_gamma.asin();
        let theta_max = std::f32::consts::FRAC_PI_2;

        let theta = theta_min + (theta_max - theta_min) * extension_factor.clamp(0.001, 1.0);

        let sin_theta = theta.sin();
        let cos_theta = theta.cos();

        let sx = a * sin_theta;
        let h_amp = a * cos_theta;

        let x_off = b * cos_gamma / sin_theta;
        let sy_sq = b * b - x_off * x_off;
        let sy = if sy_sq > 0.0 { sy_sq.sqrt() } else { 0.0 };

        let mut positions = Vec::with_capacity((rows + 1) * (cols + 1));

        let total_w = (cols as f32) * sx + x_off;
        let total_h = (rows as f32) * sy;
        let cx = total_w / 2.0;
        let cy = total_h / 2.0;

        for j in 0..=rows {
            for i in 0..=cols {
                let x = (i as f32) * sx + ((j % 2) as f32) * x_off;
                let y = (j as f32) * sy;
                let z = ((i % 2) as f32) * h_amp;

                positions.push(vec3(x - cx, y - cy, z));
            }
        }

        (positions, total_w, total_h)
    }

    fn calculate_vertical(&self, extension_factor: f32) -> (Vec<Vec3>, f32, f32) {
        let (cols, rows) = self.grid_size;
        let a = self.params.a;
        let b = self.params.b;
        let alpha = self.params.gamma; // gamma acts as alpha here

        // Logic ported from rigid-origami/src/kinematics.rs

        let expansion = extension_factor.clamp(0.01, 1.0);
        let l_x_max = a * alpha.sin();
        let l_x = expansion * l_x_max;

        let c1 = a * a - l_x * l_x - b * b;
        let c2 = a * b * alpha.cos() - b * b;
        let denominator = c1 - 2.0 * c2;

        let (l_y, s_y, h) = if denominator.abs() < 1e-6 {
            // Fallback/Flat
            (b, a * alpha.cos(), 0.0)
        } else {
            let l_y_sq = (c2 * c2) / denominator;
            if l_y_sq < 0.0 {
                (b, a * alpha.cos(), 0.0)
            } else {
                let l_y = l_y_sq.sqrt();
                let s_y = l_y + c2 / l_y;
                let h_sq_4 = b * b - l_y * l_y;
                let h = if h_sq_4 < 0.0 {
                    0.0
                } else {
                    (h_sq_4 / 4.0).sqrt()
                };
                (l_y, s_y, h)
            }
        };

        let mut positions = Vec::with_capacity((rows + 1) * (cols + 1));

        let total_w = (cols as f32) * l_x;
        // Approximation for centering, ignoring the zig-zag offset s_y
        let total_h = (rows as f32) * l_y;
        let cx = total_w / 2.0;
        let cy = total_h / 2.0;

        for j in 0..=rows {
            for i in 0..=cols {
                let x = i as f32 * l_x;
                let y = j as f32 * l_y + (i % 2) as f32 * s_y;
                let z = if (i + j) % 2 == 0 { h } else { -h };
                positions.push(vec3(x - cx, y - cy, z));
            }
        }

        (positions, total_w, total_h)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_horizontal_edge_lengths() {
        let params = MiuraParams {
            a: 1.0,
            b: 1.0,
            gamma: 80.0f32.to_radians(),
            orientation: Orientation::Horizontal,
        };
        let origami = MiuraOri::new(params, (2, 2));
        let grid = origami.generate_grid(0.5);
        let width = 3; // cols + 1

        // Horizontal edges (i to i+1) should be `a`
        for j in 0..=2 {
            for i in 0..2 {
                let idx1 = j * width + i;
                let idx2 = j * width + i + 1;
                let d = grid[idx1].distance(grid[idx2]);
                assert!(
                    (d - 1.0).abs() < 1e-4,
                    "Horizontal edge length mismatch: {}",
                    d
                );
            }
        }

        // Vertical edges (j to j+1) should be `b`
        for j in 0..2 {
            for i in 0..=2 {
                let idx1 = j * width + i;
                let idx2 = (j + 1) * width + i;
                let d = grid[idx1].distance(grid[idx2]);
                assert!(
                    (d - 1.0).abs() < 1e-4,
                    "Vertical edge length mismatch: {}",
                    d
                );
            }
        }
    }

    #[test]
    fn test_vertical_edge_lengths() {
        let params = MiuraParams {
            a: 1.0,
            b: 1.0,
            gamma: 80.0f32.to_radians(),
            orientation: Orientation::Vertical,
        };
        let origami = MiuraOri::new(params, (2, 2));
        let grid = origami.generate_grid(0.5);
        let width = 3;

        // Horizontal edges (i to i+1) should be `a`
        for j in 0..=2 {
            for i in 0..2 {
                let idx1 = j * width + i;
                let idx2 = j * width + i + 1;
                let d = grid[idx1].distance(grid[idx2]);
                assert!(
                    (d - 1.0).abs() < 1e-4,
                    "Horizontal edge length mismatch: {}",
                    d
                );
            }
        }

        // Vertical edges (j to j+1) should be `b`
        for j in 0..2 {
            for i in 0..=2 {
                let idx1 = j * width + i;
                let idx2 = (j + 1) * width + i;
                let d = grid[idx1].distance(grid[idx2]);
                assert!(
                    (d - 1.0).abs() < 1e-4,
                    "Vertical edge length mismatch: {}",
                    d
                );
            }
        }
    }
}
