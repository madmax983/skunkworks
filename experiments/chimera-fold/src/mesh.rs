use crate::pbd::{Constraint, PbdSystem};
use chimera_lang::prelude::*;
use macroquad::prelude::*;

pub struct ChimeraMesh {
    pub system: PbdSystem,
    pub indices: Vec<u16>,
    pub actuators_v: Vec<usize>, // Vertical creases
    pub actuators_h: Vec<usize>, // Horizontal creases
    pub brains: Vec<ChimeraVM>,  // One brain per vertical column
    pub rows: usize,
    pub cols: usize,
}

impl ChimeraMesh {
    pub fn new(rows: usize, cols: usize, dna: Dna) -> Self {
        let (system, indices, actuators_v, actuators_h) = generate_miura_ori(rows, cols);

        // One brain for each column of vertical actuators
        // Vertical actuators exist between columns i and i+1.
        // There are `cols - 1` columns of vertical actuators.
        let num_brains = if cols > 1 { cols - 1 } else { 0 };
        let mut brains = Vec::with_capacity(num_brains);

        for i in 0..num_brains {
            let mut vm = ChimeraVM::new(dna.clone());
            // Initialize memory with "Self" coordinates
            // Row 0, Cell 2 = ID
            vm.grid[0][2] = Value::Int(i as i64);
            // Give them plenty of energy to start
            vm.energy = 100000;
            brains.push(vm);
        }

        Self {
            system,
            indices,
            actuators_v,
            actuators_h,
            brains,
            rows,
            cols,
        }
    }

    pub fn update(&mut self, dt: f32) {
        let actuators_per_col = self.rows + 1;

        for (col_idx, brain) in self.brains.iter_mut().enumerate() {
            if brain.halted { continue; }

            // 1. Sense Strain
            let start_idx = col_idx * actuators_per_col;
            let mut total_strain = 0.0;

            for j in 0..actuators_per_col {
                if let Some(&act_idx) = self.actuators_v.get(start_idx + j) {
                    if let Constraint::Actuator { p1, p2, min_len, max_len, factor, .. } = self.system.constraints[act_idx] {
                        let current_len = self.system.particles[p1].pos.distance(self.system.particles[p2].pos);
                        let target = min_len + (max_len - min_len) * factor;
                        total_strain += (current_len - target).abs();
                    }
                }
            }

            // Write Strain to Grid (0,0) scaled by 100
            brain.grid[0][0] = Value::Int((total_strain * 100.0) as i64);

            // Also write a "Clock" to (0,3) to allow rhythmic behavior
            let time = (get_time() * 10.0) as i64;
            brain.grid[0][3] = Value::Int(time);

            // 2. Think
            // Execute a few steps per frame
            for _ in 0..10 {
                brain.step();
            }

            // 3. Actuate
            // Read Target from Grid (0,1). Expects Int 0-100.
            let target_val = &brain.grid[0][1];
            let target_factor = match target_val {
                Value::Int(n) => (*n as f32 / 100.0).clamp(0.0, 1.0),
                _ => 0.5, // Default rest
            };

            // Apply to all actuators in this column
            for j in 0..actuators_per_col {
                if let Some(&act_idx) = self.actuators_v.get(start_idx + j) {
                     // Read current factor
                    let current_factor = match self.system.constraints[act_idx] {
                        Constraint::Actuator { factor, .. } => factor,
                        _ => 1.0,
                    };

                    // Smooth interpolation
                    let new_factor = current_factor + (target_factor - current_factor) * 0.1;

                    if let Constraint::Actuator { p1, p2, min_len, max_len, stiffness, .. } = self.system.constraints[act_idx] {
                        self.system.constraints[act_idx] = Constraint::Actuator {
                            p1,
                            p2,
                            min_len,
                            max_len,
                            factor: new_factor,
                            stiffness,
                        };
                    }
                }
            }
        }

        // Physics Step
        self.system.step(dt, 4);
    }

    pub fn draw(&self) {
         // Build Mesh
        let mut mesh = Mesh {
            vertices: Vec::new(),
            indices: Vec::new(),
            texture: None,
        };

        for i in (0..self.indices.len()).step_by(3) {
            let i1 = self.indices[i] as usize;
            let i2 = self.indices[i + 1] as usize;
            let i3 = self.indices[i + 2] as usize;

            let v1 = self.system.particles[i1].pos;
            let v2 = self.system.particles[i2].pos;
            let v3 = self.system.particles[i3].pos;

            let normal = (v2 - v1).cross(v3 - v1).normalize_or_zero();
            let light = vec3(1.0, 1.0, 1.0).normalize();
            let diffuse = normal.dot(light).abs();

            // Color based on brain state if available?
            // Determine which column this face belongs to.
            // Simplified: Just blue-ish
            let color = Color::new(0.2 + diffuse * 0.8, 0.4 + diffuse * 0.6, 0.8, 1.0);
            let color_bytes: [u8; 4] = color.into();

            let normal_v4 = vec4(normal.x, normal.y, normal.z, 1.0);

            let start_idx = mesh.vertices.len() as u16;

            mesh.vertices.push(Vertex {
                position: v1,
                uv: Vec2::ZERO,
                color: color_bytes,
                normal: normal_v4,
            });
            mesh.vertices.push(Vertex {
                position: v2,
                uv: Vec2::ZERO,
                color: color_bytes,
                normal: normal_v4,
            });
            mesh.vertices.push(Vertex {
                position: v3,
                uv: Vec2::ZERO,
                color: color_bytes,
                normal: normal_v4,
            });

            mesh.indices.push(start_idx);
            mesh.indices.push(start_idx + 1);
            mesh.indices.push(start_idx + 2);

            // Wireframe
            draw_line_3d(v1, v2, BLACK);
            draw_line_3d(v2, v3, BLACK);
            draw_line_3d(v3, v1, BLACK);
        }

        draw_mesh(&mesh);
    }
}

// Adapted from neuro-fold
pub fn generate_miura_ori(rows: usize, cols: usize) -> (PbdSystem, Vec<u16>, Vec<usize>, Vec<usize>) {
    let mut system = PbdSystem::new();
    let mut indices = Vec::new();
    let mut actuators_v = Vec::new();
    let mut actuators_h = Vec::new();

    let a = 2.0;
    let b = 2.0;
    let angle = 80.0f32.to_radians();

    for i in 0..=cols {
        for j in 0..=rows {
            let x = (i as f32) * a * angle.sin();
            let offset = if i % 2 == 1 { a * angle.cos() } else { 0.0 };
            let y = (j as f32) * b + offset;

            let cx = (cols as f32 * a * angle.sin()) / 2.0;
            let cy = (rows as f32 * b) / 2.0;
            let z = (i as f32 * 0.5).sin() * 2.0;

            system.add_particle(vec3(x - cx, y - cy, z), 1.0);
        }
    }

    let stiffness = 1.0;

    for i in 0..cols {
        for j in 0..rows {
            let p00 = i * (rows + 1) + j;
            let p01 = i * (rows + 1) + (j + 1);
            let p10 = (i + 1) * (rows + 1) + j;
            let p11 = (i + 1) * (rows + 1) + (j + 1);

            indices.push(p00 as u16);
            indices.push(p01 as u16);
            indices.push(p10 as u16);
            indices.push(p10 as u16);
            indices.push(p01 as u16);
            indices.push(p11 as u16);

            system.add_distance_constraint(p00, p01, stiffness);
            system.add_distance_constraint(p00, p10, stiffness);
            system.add_distance_constraint(p10, p11, stiffness);
            system.add_distance_constraint(p01, p11, stiffness);
            system.add_distance_constraint(p01, p10, stiffness);
        }
    }

    for i in 1..cols {
        for j in 0..=rows {
            let p_left = (i - 1) * (rows + 1) + j;
            let p_right = (i + 1) * (rows + 1) + j;

            let dist = system.particles[p_left].pos.distance(system.particles[p_right].pos);
            let folded_dist = dist * 0.1;

            system.add_actuator_constraint(p_left, p_right, folded_dist, dist, 0.2);
            actuators_v.push(system.constraints.len() - 1);
        }
    }

    for i in 0..=cols {
        for j in 1..rows {
            let p_top = i * (rows + 1) + (j - 1);
            let p_bottom = i * (rows + 1) + (j + 1);

            let dist = system.particles[p_top].pos.distance(system.particles[p_bottom].pos);
            let folded_dist = dist * 0.1;

            system.add_actuator_constraint(p_top, p_bottom, folded_dist, dist, 0.2);
            actuators_h.push(system.constraints.len() - 1);
        }
    }

    let head_idx = (rows + 1) / 2;
    let head_pos = system.particles[head_idx].pos;
    system.add_pin_constraint(head_idx, head_pos);

    (system, indices, actuators_v, actuators_h)
}
