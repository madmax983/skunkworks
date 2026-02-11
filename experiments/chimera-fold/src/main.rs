use macroquad::prelude::*;

mod controller;
mod mesh;
mod pbd;

use controller::SwarmController;
use mesh::generate_miura_ori;
use pbd::{Constraint, PbdSystem};

const MESH_ROWS: usize = 6;
const MESH_COLS: usize = 12;
const SIM_STEPS: usize = 4;

struct Creature {
    system: PbdSystem,
    indices: Vec<u16>,
    actuators_v: Vec<usize>, // Vertical creases (controlled by VMs)
    actuators_h: Vec<usize>, // Horizontal creases
    controller: SwarmController,
}

impl Creature {
    fn new() -> Self {
        let (system, indices, actuators_v, actuators_h) = generate_miura_ori(MESH_ROWS, MESH_COLS);

        // One VM per column
        // We have MESH_COLS columns, but the mesh generation loop goes to `cols`.
        // The actuators logic in `mesh.rs` iterates `1..cols`.
        // Let's check `mesh.rs` logic again.
        // `for i in 1..cols { ... actuators_v.push(...) }`
        // So we have `cols - 1` columns of vertical actuators.
        let vm_count = MESH_COLS;

        let controller = SwarmController::init(vm_count);

        Self {
            system,
            indices,
            actuators_v,
            actuators_h,
            controller,
        }
    }

    fn update(&mut self, dt: f32) {
        // 1. Calculate Inputs (Strain)
        let mut inputs = Vec::new();
        let actuators_per_col = MESH_ROWS + 1;

        // We assume actuators_v is ordered by column (i) then row (j)
        // In mesh.rs: `for i in 1..cols { for j in 0..=rows { ... } }`
        // So indices are [col 0 start, col 0 end, col 1 start ...]
        // Wait, loop is `1..cols`. So i=1 is the first set of V-folds.
        // There are `cols - 1` columns of V-folds.

        // The controller expects `vm_count` inputs.
        // Let's just feed the average strain of each column to the corresponding VM.
        // And maybe the first VM gets a pacemaker signal?

        // The controller.init created `MESH_COLS` VMs.
        // But we only have `MESH_COLS - 1` columns of actuators.
        // Let's just use `MESH_COLS - 1` inputs.

        let num_actuator_cols = MESH_COLS - 1;

        for col in 0..num_actuator_cols {
            let start_idx = col * actuators_per_col;
            let mut total_strain = 0.0;
            let mut count = 0.0;

            for j in 0..actuators_per_col {
                if let Some(act_idx) = self.actuators_v.get(start_idx + j) {
                    if let Constraint::Actuator {
                        p1,
                        p2,
                        min_len,
                        max_len,
                        factor,
                        ..
                    } = self.system.constraints[*act_idx]
                    {
                        let current_len = self.system.particles[p1]
                            .pos
                            .distance(self.system.particles[p2].pos);
                        let target = min_len + (max_len - min_len) * factor;
                        let strain = (current_len - target).abs();
                        total_strain += strain;
                        count += 1.0;
                    }
                }
            }

            let avg_strain = if count > 0.0 { total_strain / count } else { 0.0 };
            inputs.push(avg_strain);
        }

        // Add a pacemaker input to the first VM (index 0)
        // If the array is empty, this does nothing.
        // But the input vector should match VM count if possible.
        // If inputs < vms, the controller handles it (0.0).

        // Let's add a manual "kick" if mouse is pressed, to input 0.
        if is_mouse_button_down(MouseButton::Left) {
            if !inputs.is_empty() {
                inputs[0] += 1.0; // Huge signal
            }
        }

        // Periodic signal
        if get_time() % 2.0 < 0.1 {
             if !inputs.is_empty() {
                inputs[0] += 0.5;
            }
        }

        // 2. Step Controller
        let outputs = self.controller.step(&inputs);

        // 3. Apply Outputs
        for col in 0..num_actuator_cols {
            if col < outputs.len() {
                let target_factor = outputs[col]; // 0.0 (contract) to 1.0 (relax) usually
                // Inverting might be interesting?
                // Let's just use it directly.

                // Smooth transition
                let start_idx = col * actuators_per_col;
                for j in 0..actuators_per_col {
                    if let Some(act_idx) = self.actuators_v.get(start_idx + j) {
                        if let Constraint::Actuator {
                            p1,
                            p2,
                            min_len,
                            max_len,
                            factor,
                            stiffness,
                        } = self.system.constraints[*act_idx]
                        {
                            let new_factor = factor + (target_factor - factor) * 0.1;
                             self.system.constraints[*act_idx] = Constraint::Actuator {
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
        }

        // 4. Step Physics
        self.system.step(dt, SIM_STEPS);
    }

    fn draw(&self) {
        // Draw Mesh
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

#[macroquad::main("Chimera-Fold")]
async fn main() {
    let mut creature = Creature::new();

    loop {
        clear_background(BLACK);

        set_camera(&Camera3D {
            position: vec3(0.0, -20.0, 20.0),
            target: vec3(0.0, 0.0, 0.0),
            up: vec3(0.0, 0.0, 1.0),
            ..Default::default()
        });

        creature.update(0.016);
        creature.draw();

        set_default_camera();

        // UI Overlay
        draw_text("CHIMERA-FOLD", 10.0, 30.0, 30.0, WHITE);
        draw_text("Genetic Programs driving Origami Mesh", 10.0, 50.0, 20.0, GRAY);
        draw_text("Click to Excite VM 0", 10.0, 70.0, 20.0, RED);

        // Visualize VMs
        let start_x = 10.0;
        let start_y = 100.0;
        let spacing = 20.0;

        for (i, vm) in creature.controller.vms.iter().enumerate() {
            let x = start_x + (i as f32) * spacing;
            let y = start_y;

            // Color based on Output (grid[0][1])
            let val = if let Some(row) = vm.grid.get(0) {
                 if let Some(cell) = row.get(1) {
                     match cell {
                         chimera_lang::vm::Value::Int(v) => (*v as f32 / 100.0).clamp(0.0, 1.0),
                         _ => 0.0,
                     }
                 } else { 0.0 }
            } else { 0.0 };

            let color = Color::new(val, 1.0 - val, 0.0, 1.0);
            draw_rectangle(x, y, 15.0, 15.0, color);
            draw_text(&format!("{}", i), x + 2.0, y + 12.0, 10.0, BLACK);
        }

        next_frame().await
    }
}
