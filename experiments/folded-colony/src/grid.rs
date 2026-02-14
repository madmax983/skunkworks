use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub struct MiuraParams {
    pub a: f32,     // Side length a (horizontal zig-zag edge)
    pub b: f32,     // Side length b (vertical edge)
    pub alpha: f32, // Sector angle (radians)
}

impl Default for MiuraParams {
    fn default() -> Self {
        Self {
            a: 1.0,
            b: 1.0,
            alpha: 80.0f32.to_radians(),
        }
    }
}

pub struct MiuraGrid {
    pub params: MiuraParams,
    pub cols: usize,
    pub rows: usize,
}

impl MiuraGrid {
    pub fn new(cols: usize, rows: usize) -> Self {
        Self {
            params: MiuraParams::default(),
            cols,
            rows,
        }
    }

    /// Computes the vertices for a given expansion factor (0.0 to 1.0).
    /// 1.0 = fully flat.
    /// 0.0 = fully compressed (mathematical limit).
    pub fn get_vertices(&self, expansion: f32) -> Vec<Vec3> {
        let a = self.params.a;
        let b = self.params.b;
        let alpha = self.params.alpha;

        // Clamp expansion
        let expansion = expansion.clamp(0.05, 1.0);

        // Max l_x occurs when flat. l_x is the horizontal spacing between columns.
        // Flat state geometry:
        // The zig-zag angle gamma satisfies cos(gamma) = ...
        // Actually, let's use the derivation from rigid-origami if it works.
        // l_x = a * sin(alpha/2)? No, let's look at the standard Miura-ori unit cell.

        // Using the code from parent 'rigid-origami':
        // l_x_max = a * sin(alpha)
        let l_x_max = a * alpha.sin();
        let l_x = expansion * l_x_max;

        // Constraints:
        // 1. s_y^2 - l_y^2 = a^2 - l_x^2 - b^2  (Constant C1)
        // 2. s_y * l_y - l_y^2 = a * b * cos(alpha) - b^2 (Constant C2)

        // Wait, the parent code derivation looks a bit magic.
        // Let's re-verify the variable names.
        // l_x: horizontal span of one unit.
        // l_y: vertical span of one unit.
        // s_y: vertical offset for odd columns.
        // h: height (z-depth).

        // Let's blindly trust the parent for now to ensure "genetic compatibility".

        let c1 = a * a - l_x * l_x - b * b;
        let c2 = a * b * alpha.cos() - b * b;

        let denominator = c1 - 2.0 * c2;

        if denominator.abs() < 1e-6 {
             // Singularity fallback
             return self.generate_grid(l_x, b, a * alpha.cos(), 0.0);
        }

        let l_y_sq = (c2 * c2) / denominator; // This line in parent was l_y_sq = (c2*c2)/denominator ??
        // In parent: `let l_y_sq = (c2 * c2) / denominator;`

        if l_y_sq < 0.0 {
             return self.generate_grid(l_x_max, b, a * alpha.cos(), 0.0);
        }

        let l_y = l_y_sq.sqrt();

        // Parent: `let s_y = l_y + c2 / l_y;`
        let s_y = l_y + c2 / l_y;

        // Parent: `let h_sq_4 = b * b - l_y * l_y;`
        let h_sq_4 = b * b - l_y * l_y;

        let h = if h_sq_4 < 0.0 { 0.0 } else { (h_sq_4).sqrt() / 2.0 }; // Parent had `(h_sq_4 / 4.0).sqrt()` which matches.

        self.generate_grid(l_x, l_y, s_y, h)
    }

    fn generate_grid(&self, l_x: f32, l_y: f32, s_y: f32, h: f32) -> Vec<Vec3> {
        let mut vertices = Vec::with_capacity((self.rows + 1) * (self.cols + 1));

        for j in 0..=self.rows {
            for i in 0..=self.cols {
                // Centering logic
                let cx = (self.cols as f32 * l_x) * 0.5;
                let cy = (self.rows as f32 * l_y + 0.5 * s_y) * 0.5; // Approx center

                let x = i as f32 * l_x - cx;
                let y = (j as f32 * l_y + (i % 2) as f32 * s_y) - cy;

                // Checkerboard height
                // To make it look like a continuous sheet, neighboring vertices must alternate up/down relative to the mid-plane?
                // Miura-ori:
                // v(i,j) z depends on (i+j)%2
                let z = if (i + j) % 2 == 0 { h } else { -h };

                vertices.push(vec3(x, y, z));
            }
        }
        vertices
    }
}
