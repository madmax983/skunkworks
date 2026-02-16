use crate::pbd::Solver;
use macroquad::prelude::*;

pub struct OrigamiMesh {
    pub solver: Solver,
    pub width: usize,
    pub height: usize,
    pub triangles: Vec<[usize; 3]>,
    pub actuators: Vec<Actuator>,
}

#[derive(Clone, Copy)]
pub struct Actuator {
    pub constraint_idx: usize,
    pub min_len: f32,
    pub max_len: f32,
    pub target_factor: f32, // 0..1
}

impl OrigamiMesh {
    pub fn new_miura(width: usize, height: usize) -> Self {
        let mut solver = Solver::new();
        let mut triangles = Vec::new();
        let mut actuators = Vec::new();

        let dx = 10.0;
        let dy = 10.0;
        let offset_x = 3.0; // The zig-zag offset

        // Create vertices
        for y in 0..height {
            for x in 0..width {
                let mut pos_x = x as f32 * dx;
                // Zig-zag every other row
                if y % 2 == 1 {
                    pos_x += offset_x;
                }

                let pos_y = y as f32 * dy;
                // Slight perturbation in Z to help folding direction
                let pos_z = if (x + y) % 2 == 0 { 2.0 } else { -2.0 };

                solver.add_particle(vec3(pos_x, pos_y, pos_z), 1.0);
            }
        }

        // Pin the center for stability
        let mid_y = height / 2;
        solver.particles[mid_y * width].pinned = true;
        solver.particles[mid_y * width + 1].pinned = true; // Pin a segment to prevent rotation

        // Create Faces and Constraints
        for y in 0..height - 1 {
            for x in 0..width - 1 {
                let p0 = y * width + x;
                let p1 = y * width + x + 1;
                let p2 = (y + 1) * width + x;
                let p3 = (y + 1) * width + x + 1;

                // Quad p0-p1-p3-p2
                // Edges (Constraints)
                solver.add_distance_constraint(p0, p1, 0.0); // Top
                solver.add_distance_constraint(p2, p3, 0.0); // Bottom
                solver.add_distance_constraint(p0, p2, 0.0); // Left
                solver.add_distance_constraint(p1, p3, 0.0); // Right

                // Triangulate (Split quad into two triangles)
                // We use p0-p3 as the rigid diagonal.
                solver.add_distance_constraint(p0, p3, 0.0);

                // Add triangles for rendering
                triangles.push([p0, p3, p1]);
                triangles.push([p0, p2, p3]);
            }
        }

        // Add Bending Actuators across the vertical folds
        // Connect (x, y) to (x+2, y)
        for y in 0..height {
            for x in 0..width - 2 {
                let p_left = y * width + x;
                let p_right = y * width + x + 2;

                // Initial length
                let p1 = solver.particles[p_left].pos;
                let p2 = solver.particles[p_right].pos;
                let dist = p1.distance(p2);

                let idx = solver.add_distance_constraint(p_left, p_right, 0.001); // Compliant
                actuators.push(Actuator {
                    constraint_idx: idx,
                    min_len: dist * 0.4, // Folded
                    max_len: dist * 1.0, // Flat
                    target_factor: 1.0,
                });
            }
        }

        Self {
            solver,
            width,
            height,
            triangles,
            actuators,
        }
    }

    pub fn update(&mut self, dt: f32) {
        // Update actuators
        for a in &self.actuators {
            let target = a.min_len + (a.max_len - a.min_len) * a.target_factor;
            self.solver.set_rest_length(a.constraint_idx, target);
        }
        self.solver.update(dt);
    }
}
