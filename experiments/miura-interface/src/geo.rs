use nalgebra::Point3;
use std::f64::consts::PI;

#[derive(Debug, Clone)]
pub struct MiuraPattern {
    pub rows: usize,
    pub cols: usize,
    pub a: f64,     // Horizontal segment length
    pub b: f64,     // Vertical segment length
    pub gamma: f64, // Sector angle (radians).
}

impl MiuraPattern {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            a: 5.0,
            b: 5.0,
            gamma: 84.0_f64.to_radians(),
        }
    }

    /// Computes vertices using the "Eggbox" parameterization.
    /// Vertices oscillate between Z = +H and Z = -H.
    /// Edge lengths a and b are preserved.
    /// Sector angle gamma is preserved.
    ///
    /// rho [0, 1] controls the folding depth H.
    pub fn compute_vertices(&self, rho: f64) -> Vec<Point3<f64>> {
        let rho = rho.clamp(0.001, 1.0);

        let limit_dim = self.a.min(self.b) * 0.99;
        let max_h = 0.45 * limit_dim;
        let h = (1.0 - rho) * max_h;
        let h_sq4 = 4.0 * h * h;

        // Projected lengths
        let u_proj = (self.a * self.a - h_sq4).sqrt();
        let v_proj = (self.b * self.b - h_sq4).sqrt();

        // Projected angle gamma_proj
        let cos_gamma_num = self.a * self.b * self.gamma.cos() - h_sq4;
        let cos_gamma_proj = (cos_gamma_num / (u_proj * v_proj)).clamp(-1.0, 1.0);
        let gamma_proj = cos_gamma_proj.acos();

        // Lattice parameters
        // theta is derived from 2*theta = 90 - gamma_proj
        let theta = (PI / 2.0 - gamma_proj) / 2.0;

        let s_x = u_proj * theta.cos();
        let o_y = u_proj * theta.sin();

        // v vector components
        // v = (o_x, s_y)
        let o_x = v_proj * theta.sin();
        let s_y = v_proj * theta.cos();

        let center_x = (self.cols as f64 * s_x) / 2.0;
        let center_y = (self.rows as f64 * s_y) / 2.0;

        let mut vertices = Vec::with_capacity(self.rows * self.cols);

        for r in 0..self.rows {
            for c in 0..self.cols {
                let x = c as f64 * s_x + (r % 2) as f64 * o_x;
                let y = r as f64 * s_y + (c % 2) as f64 * o_y;
                let z = if (r + c) % 2 == 1 { h } else { -h };

                vertices.push(Point3::new(x - center_x, -(y - center_y), z));
            }
        }

        vertices
    }

    pub fn get_quad_indices(&self, r: usize, c: usize) -> Option<[usize; 4]> {
        if r >= self.rows - 1 || c >= self.cols - 1 {
            return None;
        }
        let w = self.cols;
        let tl = r * w + c;
        let tr = r * w + (c + 1);
        let br = (r + 1) * w + (c + 1);
        let bl = (r + 1) * w + c;
        Some([tl, tr, br, bl])
    }

    pub fn export_cp_svg(&self) -> String {
        // Use flat state geometry logic
        // compute_vertices(1.0) returns the flat state coordinates relative to center.
        let verts = self.compute_vertices(1.0);

        // Find bounds
        let (min_x, max_x, min_y, max_y) = verts.iter().fold(
            (f64::MAX, f64::MIN, f64::MAX, f64::MIN),
            |(minx, maxx, miny, maxy), v| {
                (minx.min(v.x), maxx.max(v.x), miny.min(v.y), maxy.max(v.y))
            },
        );

        let width = max_x - min_x;
        let height = max_y - min_y;

        let mut svg = String::new();
        svg.push_str(&format!(
            r#"<svg viewBox="{} {} {} {}" xmlns="http://www.w3.org/2000/svg">"#,
            min_x - 10.0,
            -max_y - 10.0,
            width + 20.0,
            height + 20.0
        ));
        svg.push_str(r#"<style>line { stroke: black; stroke-width: 0.2; } .mountain { stroke: red; } .valley { stroke: blue; }</style>"#);

        for r in 0..self.rows {
            for c in 0..self.cols {
                let idx = r * self.cols + c;
                let p = verts[idx];
                let (x1, y1) = (p.x, -p.y); // Flip Y for SVG

                if c + 1 < self.cols {
                    let p2 = verts[idx + 1];
                    let (x2, y2) = (p2.x, -p2.y);
                    // Assignment for CP
                    let class = if (r + c) % 2 == 0 {
                        "mountain"
                    } else {
                        "valley"
                    };
                    svg.push_str(&format!(
                        r#"<line x1="{:.2}" y1="{:.2}" x2="{:.2}" y2="{:.2}" class="{}" />"#,
                        x1, y1, x2, y2, class
                    ));
                }

                if r + 1 < self.rows {
                    let p2 = verts[idx + self.cols];
                    let (x2, y2) = (p2.x, -p2.y);
                    let class = if (r + c) % 2 == 1 {
                        "mountain"
                    } else {
                        "valley"
                    };
                    svg.push_str(&format!(
                        r#"<line x1="{:.2}" y1="{:.2}" x2="{:.2}" y2="{:.2}" class="{}" />"#,
                        x1, y1, x2, y2, class
                    ));
                }
            }
        }

        svg.push_str("</svg>");
        svg
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_export_svg() {
        let pattern = MiuraPattern::new(5, 5);
        let svg = pattern.export_cp_svg();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("mountain"));
        assert!(svg.contains("valley"));
    }

    #[test]
    fn test_check_edge_lengths() {
        let pattern = MiuraPattern::new(5, 5);
        for rho in [0.1, 0.5, 0.9, 0.99] {
            let verts = pattern.compute_vertices(rho);

            for r in 0..pattern.rows - 1 {
                for c in 0..pattern.cols - 1 {
                    let idx = r * pattern.cols + c;
                    let p = verts[idx];

                    let p_right = verts[idx + 1];
                    let d_right = nalgebra::distance(&p, &p_right);
                    assert!(
                        (d_right - pattern.a).abs() < 1e-3,
                        "H-edge mismatch rho={}: {} vs {}",
                        rho,
                        d_right,
                        pattern.a
                    );

                    let p_down = verts[idx + pattern.cols];
                    let d_down = nalgebra::distance(&p, &p_down);
                    assert!(
                        (d_down - pattern.b).abs() < 1e-3,
                        "V-edge mismatch rho={}: {} vs {}",
                        rho,
                        d_down,
                        pattern.b
                    );
                }
            }
        }
    }
}
