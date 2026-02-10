use macroquad::prelude::*;
use macroquad::models::{Mesh, Vertex};
use ::rand::Rng;

mod pbd;
use pbd::{PbdSystem, Constraint};

use chimera_lang::prelude::*;

const MESH_ROWS: usize = 6;
const MESH_COLS: usize = 12;
const SIM_STEPS: usize = 4;

struct ChimeraOrigami {
    system: PbdSystem,
    indices: Vec<u16>,
    actuators_v: Vec<usize>, // Vertical creases
    actuators_h: Vec<usize>, // Horizontal creases
    vm: ChimeraVM,
    generation: u64,
}

impl ChimeraOrigami {
    fn new() -> Self {
        let (system, indices, actuators_v, actuators_h) = generate_miura_ori(MESH_ROWS, MESH_COLS);

        // Create a simple DNA that oscillates
        // Program:
        // 1. Loop through columns
        // 2. Calculate contraction factor based on Time (Energy) or simple counter
        // 3. Write to Grid

        // Actually, let's just make it random/chaotic for now, or use a simple breather.
        // Or better: Use the "Energy" level to drive a sine wave?

        // Let's create a DNA that writes values to the Grid based on a sine wave pattern.
        // But Chimera is stack based.
        // OpCode::Push(N) -> Push N
        // OpCode::Store(X, Y, Val) -> Store Val at (X, Y)

        // Let's manually inject "Strain" into the VM, and let the VM react.

        let genes = vec![
            // A simple "breathing" gene
            // It just runs and consumes energy.
            // We will rely on the "Update" loop to handle the IO for now,
            // as writing a full Chimera program in Assembly is tedious without the parser.
            Gene { op: OpCode::Photosynthesize, args: vec![] }, // Gain Energy
            Gene { op: OpCode::Rest, args: vec![] },
            Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(0)] }, // Loop
        ];

        let dna = Dna { helix: Helix { strands: vec![Strand { genes }] } };
        let vm = ChimeraVM::new(dna);

        Self {
            system,
            indices,
            actuators_v,
            actuators_h,
            vm,
            generation: 0,
        }
    }

    fn update(&mut self, dt: f32) {
        // 1. Physics Step
        self.system.step(dt, SIM_STEPS);

        // 2. Calculate Strain and feed to VM (Input)
        // We will write the total strain of each column to the first row of the grid.
        let actuators_per_col = MESH_ROWS + 1;
        for col in 0..(MESH_COLS - 1) {
            let start_idx = col * actuators_per_col;
            let mut total_strain = 0.0;

            for j in 0..actuators_per_col {
                if let Some(act_idx) = self.actuators_v.get(start_idx + j) {
                    if let Constraint::Actuator { p1, p2, min_len, max_len, factor, .. } = self.system.constraints[*act_idx] {
                        let current_len = self.system.particles[p1].pos.distance(self.system.particles[p2].pos);
                        let target = min_len + (max_len - min_len) * factor;
                        total_strain += (current_len - target).abs();
                    }
                }
            }

            // Map strain to a value (0..255 or similar)
            let strain_val = (total_strain * 100.0) as i64;
            // Write to Grid (Row 0, Col = col)
            // Use safe access
            if col < 16 {
                self.vm.grid[0][col] = Value::Int(strain_val);
            }
        }

        // 3. Step VM
        // Feed it some free energy so it doesn't die immediately
        self.vm.energy = self.vm.energy.saturating_add(10);
        self.vm.step();

        // 4. Read VM Output and Apply to Actuators
        // Read from Grid (Row 1, Col = col) -> Target Factor
        // If the cell is empty/null, we default to 1.0 (Relaxed)
        for col in 0..(MESH_COLS - 1) {
             let target_factor = if col < 16 {
                match self.vm.grid[1][col] {
                    Value::Int(n) => (n as f32 / 100.0).clamp(0.0, 1.0),
                    _ => {
                        // Default behavior: Oscillate based on time + col index (Wave)
                        let time = get_time() as f32;
                        (time * 2.0 + col as f32 * 0.5).sin() * 0.5 + 0.5
                    }
                }
            } else {
                1.0
            };

            // Apply to column actuators
            let start_idx = col * actuators_per_col;
            for j in 0..actuators_per_col {
                if let Some(act_idx) = self.actuators_v.get(start_idx + j) {
                    if let Constraint::Actuator { p1, p2, min_len, max_len, factor, stiffness } = self.system.constraints[*act_idx] {
                         // Smooth interpolation
                         let new_factor = factor + (target_factor - factor) * 0.1;

                         self.system.constraints[*act_idx] = Constraint::Actuator {
                            p1, p2, min_len, max_len, factor: new_factor, stiffness
                        };
                    }
                }
            }
        }
    }

    fn draw(&self) {
        // Build Mesh
        let mut mesh = Mesh {
            vertices: Vec::new(),
            indices: Vec::new(),
            texture: None,
        };

        for i in (0..self.indices.len()).step_by(3) {
            let i1 = self.indices[i] as usize;
            let i2 = self.indices[i+1] as usize;
            let i3 = self.indices[i+2] as usize;

            let v1 = self.system.particles[i1].pos;
            let v2 = self.system.particles[i2].pos;
            let v3 = self.system.particles[i3].pos;

            let normal = (v2 - v1).cross(v3 - v1).normalize_or_zero();
            let light = vec3(1.0, 1.0, 1.0).normalize();
            let diffuse = normal.dot(light).abs();

            // Color based on VM Energy?
            let energy_factor = (self.vm.energy as f32 / 200.0).clamp(0.0, 1.0);
            let color = Color::new(0.2 + diffuse * 0.8, 0.4 + diffuse * 0.4, 0.2 + energy_factor * 0.8, 1.0);
            let color_bytes: [u8; 4] = color.into();

            let normal_v4 = vec4(normal.x, normal.y, normal.z, 1.0);

            let start_idx = mesh.vertices.len() as u16;

            mesh.vertices.push(Vertex { position: v1, uv: Vec2::ZERO, color: color_bytes, normal: normal_v4 });
            mesh.vertices.push(Vertex { position: v2, uv: Vec2::ZERO, color: color_bytes, normal: normal_v4 });
            mesh.vertices.push(Vertex { position: v3, uv: Vec2::ZERO, color: color_bytes, normal: normal_v4 });

            mesh.indices.push(start_idx);
            mesh.indices.push(start_idx + 1);
            mesh.indices.push(start_idx + 2);

            // Wireframe lines
            draw_line_3d(v1, v2, BLACK);
            draw_line_3d(v2, v3, BLACK);
            draw_line_3d(v3, v1, BLACK);
        }

        draw_mesh(&mesh);
    }
}

// Adapted from neuro-fold
fn generate_miura_ori(rows: usize, cols: usize) -> (PbdSystem, Vec<u16>, Vec<usize>, Vec<usize>) {
    let mut system = PbdSystem::new();
    let mut indices = Vec::new();
    let mut actuators_v = Vec::new();
    let mut actuators_h = Vec::new();

    let a = 2.0;
    let b = 2.0;
    let angle = 80.0f32.to_radians();

    // Create particles
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

    // Constraints & Triangles
    for i in 0..cols {
        for j in 0..rows {
            let p00 = i * (rows + 1) + j;
            let p01 = i * (rows + 1) + (j + 1);
            let p10 = (i + 1) * (rows + 1) + j;
            let p11 = (i + 1) * (rows + 1) + (j + 1);

            // Triangles
            indices.push(p00 as u16); indices.push(p01 as u16); indices.push(p10 as u16);
            indices.push(p10 as u16); indices.push(p01 as u16); indices.push(p11 as u16);

            // Structural Edges
            system.add_distance_constraint(p00, p01, stiffness);
            system.add_distance_constraint(p00, p10, stiffness);
            system.add_distance_constraint(p10, p11, stiffness);
            system.add_distance_constraint(p01, p11, stiffness);
            system.add_distance_constraint(p01, p10, stiffness);
        }
    }

    // Actuators (Creases)
    // Vertical Creases
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

    // Horizontal Creases
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

    // Pin the head
    let head_idx = (rows + 1) / 2;
    let head_pos = system.particles[head_idx].pos;
    system.add_pin_constraint(head_idx, head_pos);

    (system, indices, actuators_v, actuators_h)
}

#[macroquad::main("Chimera-Fold")]
async fn main() {
    let mut creature = ChimeraOrigami::new();

    loop {
        clear_background(BLACK);

        // Camera
        set_camera(&Camera3D {
            position: vec3(0.0, -20.0, 20.0),
            target: vec3(0.0, 0.0, 0.0),
            up: vec3(0.0, 0.0, 1.0),
            ..Default::default()
        });

        // Mouse Interaction (Perturb)
        if is_mouse_button_down(MouseButton::Left) {
            // Add massive energy to VM to simulate "Excitement"
            creature.vm.energy = creature.vm.energy.saturating_add(100);

            // Also write to Grid[0][0] to signal touch
            creature.vm.grid[0][0] = Value::Int(999);
        }

        creature.update(0.016);
        creature.draw();

        set_default_camera();

        // HUD
        draw_text("CHIMERA-FOLD", 10.0, 30.0, 30.0, WHITE);
        draw_text("VM driving Origami Mesh", 10.0, 50.0, 20.0, GRAY);
        draw_text(&format!("Energy: {}", creature.vm.energy), 10.0, 70.0, 20.0, GREEN);

        // Visualize Grid (VM Memory)
        let start_x = 10.0;
        let start_y = 100.0;
        let size = 10.0;

        for r in 0..4 { // Show first 4 rows
            for c in 0..16 {
                let x = start_x + c as f32 * size;
                let y = start_y + r as f32 * size;

                let val = &creature.vm.grid[r][c];
                let color = match val {
                    Value::Int(n) => {
                         if *n == 0 {
                             DARKGRAY
                         } else {
                             let intensity = (*n as f32 / 100.0).clamp(0.0, 1.0);
                             Color::new(intensity, 0.0, 0.0, 1.0)
                         }
                    },
                    _ => BLUE,
                };

                draw_rectangle(x, y, size - 1.0, size - 1.0, color);
            }
        }

        next_frame().await
    }
}
