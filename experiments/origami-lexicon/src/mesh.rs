use crate::pbd::{Constraint, PbdSystem};
use macroquad::prelude::*;

#[derive(Clone, Debug)]
pub struct Cell {
    pub indices: [usize; 4], // p0, p1, p2, p3
    pub content: Option<char>,
}

pub struct Mesh {
    pub rows: usize,
    pub cols: usize,
    pub indices: Vec<u16>,
    pub cells: Vec<Cell>,
    pub bending_indices: Vec<(usize, f32)>, // (Constraint Index, Sign)
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            rows: 0,
            cols: 0,
            indices: Vec::new(),
            cells: Vec::new(),
            bending_indices: Vec::new(),
        }
    }

    pub fn generate_miura_ori(&mut self, system: &mut PbdSystem, rows: usize, cols: usize, text: &str) {
        self.rows = rows;
        self.cols = cols;
        self.indices.clear();
        self.cells.clear();
        self.bending_indices.clear();

        let chars: Vec<char> = text.chars().collect();
        let mut char_idx = 0;

        let a = 1.0;
        let b = 1.0;
        let alpha_deg = 84.0f32;
        let alpha = alpha_deg.to_radians();
        let offset = b / alpha.tan();

        // 1. Create Particles
        for i in 0..=rows {
            for j in 0..=cols {
                let x = j as f32 * a;
                let z_offset = if j % 2 == 1 { offset } else { 0.0 };
                let z = i as f32 * b + z_offset;

                let cx = (cols as f32 * a) / 2.0;
                let cz = (rows as f32 * b) / 2.0;

                let y_bias = if (i + j) % 2 == 0 {
                    0.1
                } else {
                    -0.1
                };

                system.add_particle(vec3(x - cx, y_bias, z - cz), 1.0);
            }
        }

        let idx = |r: usize, c: usize| r * (cols + 1) + c;
        let stiffness = 1.0;

        // 2. Structural Constraints & Cells
        for i in 0..rows {
            for j in 0..cols {
                let p0 = idx(i, j);
                let p1 = idx(i, j + 1);
                let p2 = idx(i + 1, j + 1);
                let p3 = idx(i + 1, j);

                // Diagonals (Cross-bracing for rigidity)
                system.add_distance_constraint(p0, p2, stiffness);
                system.add_distance_constraint(p1, p3, stiffness);

                // Triangles for rendering
                self.indices.push(p0 as u16);
                self.indices.push(p1 as u16);
                self.indices.push(p2 as u16);

                self.indices.push(p0 as u16);
                self.indices.push(p2 as u16);
                self.indices.push(p3 as u16);

                // Cell
                let content = if char_idx < chars.len() {
                    let c = chars[char_idx];
                    char_idx += 1;
                    Some(c)
                } else {
                    None
                };

                self.cells.push(Cell {
                    indices: [p0, p1, p2, p3],
                    content,
                });
            }
        }

        // Edges (Grid lines)
         for i in 0..=rows {
            for j in 0..=cols {
                let p = idx(i, j);
                if j < cols {
                    system.add_distance_constraint(p, idx(i, j + 1), stiffness);
                }
                if i < rows {
                    system.add_distance_constraint(p, idx(i + 1, j), stiffness);
                }
            }
         }

        // 3. Bending Constraints
        let bend_stiffness = 0.2;

        // Vertical Hinges (j boundaries)
        for i in 0..rows {
            for j in 1..cols {
                let p_l = idx(i, j - 1);
                let p_r = idx(i, j + 1);
                let c_idx = system.add_bending_constraint(p_l, p_r, bend_stiffness);
                let sign = if j % 2 == 0 { 1.0 } else { -1.0 };
                self.bending_indices.push((c_idx, sign));
            }
        }

        // Horizontal Hinges (i boundaries)
        for i in 1..rows {
            for j in 0..cols {
                let p_up = idx(i - 1, j);
                let p_down = idx(i + 1, j);
                let c_idx = system.add_bending_constraint(p_up, p_down, bend_stiffness);
                self.bending_indices.push((c_idx, 1.0)); // All Mountain
            }
        }

        // Pin center
        let center = idx(rows/2, cols/2);
        let p_center = system.particles[center].pos;
        system.add_pin_constraint(center, p_center);
    }

    pub fn update_folds(&self, system: &mut PbdSystem, rho: f32) {
        for &(idx, _sign) in &self.bending_indices {
             if let Constraint::Bending { flat_length, target_length, .. } = &mut system.constraints[idx] {
                 *target_length = *flat_length * (1.0 - rho * 0.5);
             }
        }
    }
}
