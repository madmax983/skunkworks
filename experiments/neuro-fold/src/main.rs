use ::rand::Rng;
use macroquad::models::{Mesh, Vertex};
use macroquad::prelude::*; // Use ::rand to avoid ambiguity

mod network;
mod neuron;
mod pbd;

use network::Network;
use pbd::{Constraint, PbdSystem};

const MESH_ROWS: usize = 6;
const MESH_COLS: usize = 12; // Longer sheet to see waves
const SIM_STEPS: usize = 4;

struct Creature {
    system: PbdSystem,
    indices: Vec<u16>,
    actuators_v: Vec<usize>, // Vertical creases (controlled by neurons)
    actuators_h: Vec<usize>, // Horizontal creases
    brain: Network,
    motor_neurons: Vec<usize>, // Indices of neurons controlling vertical zones
    sensor_neurons: Vec<usize>, // Indices of neurons receiving strain feedback
}

impl Creature {
    fn new() -> Self {
        let (system, indices, actuators_v, actuators_h) = generate_miura_ori(MESH_ROWS, MESH_COLS);

        let mut brain = Network::new();
        let mut motor_neurons = Vec::new();
        let mut sensor_neurons = Vec::new();

        // Create a CPG chain for the columns
        let mut prev_neuron = None;

        for _ in 0..MESH_COLS {
            // Motor neuron for this column
            let motor = brain.add_neuron();
            motor_neurons.push(motor);

            // Sensor neuron for this column (strain feedback)
            let sensor = brain.add_neuron();
            sensor_neurons.push(sensor);

            // CPG Logic:
            if let Some(prev) = prev_neuron {
                // Forward propagation
                brain.add_synapse(prev, motor, 20.0);
                // Backward inhibition?
                brain.add_synapse(motor, prev, -5.0);
            }

            // Sensor excites motor (Reflex)
            brain.add_synapse(sensor, motor, 15.0);

            prev_neuron = Some(motor);
        }

        // Loop the chain to create a ring oscillator?
        if let Some(last) = prev_neuron {
            if !motor_neurons.is_empty() {
                brain.add_synapse(last, motor_neurons[0], 20.0);
            }
        }

        Self {
            system,
            indices,
            actuators_v,
            actuators_h,
            brain,
            motor_neurons,
            sensor_neurons,
        }
    }

    fn update(&mut self, dt: f32) {
        // 1. Calculate Strain -> Sensor Inputs
        let mut inputs = vec![0.0; self.brain.neurons.len()];

        let actuators_per_col = MESH_ROWS + 1; // Number of vertical creases per column

        for col in 0..(MESH_COLS - 1) {
            let start_idx = col * actuators_per_col;
            let mut total_strain = 0.0;

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
                        total_strain += (current_len - target).abs();
                    }
                }
            }

            // Feed strain into sensor neuron for this column
            if col < self.sensor_neurons.len() {
                inputs[self.sensor_neurons[col]] = total_strain * 5.0; // Gain
            }
        }

        // Periodic pacemaker input to the first neuron to start the wave
        if get_time() % 2.0 < 0.1 {
            if !self.motor_neurons.is_empty() {
                inputs[self.motor_neurons[0]] += 50.0;
            }
        }

        // 2. Step Brain
        self.brain.step(&inputs);

        // 3. Map Motor Output -> Actuator Targets
        for col in 0..(MESH_COLS - 1) {
            if col < self.motor_neurons.len() {
                let neuron_idx = self.motor_neurons[col];

                let target_factor = if self.brain.is_spiking(neuron_idx) {
                    0.0 // Contract (folded)
                } else {
                    1.0 // Relax (flat)
                };

                // Smooth transition (muscle dynamics)
                let start_idx = col * actuators_per_col;
                for j in 0..actuators_per_col {
                    if let Some(act_idx) = self.actuators_v.get(start_idx + j) {
                        // Read current factor
                        let current_factor = match self.system.constraints[*act_idx] {
                            Constraint::Actuator { factor, .. } => factor,
                            _ => 1.0,
                        };

                        // Smoothly interpolate
                        let new_factor = current_factor + (target_factor - current_factor) * 0.1;

                        // Update constraint
                        if let Constraint::Actuator {
                            p1,
                            p2,
                            min_len,
                            max_len,
                            stiffness,
                            ..
                        } = self.system.constraints[*act_idx]
                        {
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

            // Wireframe lines
            draw_line_3d(v1, v2, BLACK);
            draw_line_3d(v2, v3, BLACK);
            draw_line_3d(v3, v1, BLACK);
        }

        draw_mesh(&mesh);

        // Draw Actuators (Visual Debug)
        for &idx in &self.actuators_v {
            if let Constraint::Actuator { p1, p2, factor, .. } = self.system.constraints[idx] {
                let v1 = self.system.particles[p1].pos;
                let v2 = self.system.particles[p2].pos;
                let color = Color::new(1.0 - factor, factor, 0.0, 1.0);
                draw_line_3d(v1, v2, color);
            }
        }
    }
}

// Adapted from origami-constellation/src/mesh_gen.rs
fn generate_miura_ori(rows: usize, cols: usize) -> (PbdSystem, Vec<u16>, Vec<usize>, Vec<usize>) {
    let mut system = PbdSystem::new();
    let mut indices = Vec::new();
    let mut actuators_v = Vec::new();
    let mut actuators_h = Vec::new();

    let a = 2.0; // Scale up
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

            // Add some Z curvature to make it look like a creature
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
            indices.push(p00 as u16);
            indices.push(p01 as u16);
            indices.push(p10 as u16);
            indices.push(p10 as u16);
            indices.push(p01 as u16);
            indices.push(p11 as u16);

            // Structural Edges
            system.add_distance_constraint(p00, p01, stiffness);
            system.add_distance_constraint(p00, p10, stiffness);
            system.add_distance_constraint(p10, p11, stiffness);
            system.add_distance_constraint(p01, p11, stiffness);
            system.add_distance_constraint(p01, p10, stiffness);
        }
    }

    // Actuators (Creases)

    // Vertical Creases (The "V" folds) - These drive the expansion/contraction
    for i in 1..cols {
        for j in 0..=rows {
            let p_left = (i - 1) * (rows + 1) + j;
            let p_right = (i + 1) * (rows + 1) + j;

            let dist = system.particles[p_left]
                .pos
                .distance(system.particles[p_right].pos);
            let folded_dist = dist * 0.1; // Deep fold

            system.add_actuator_constraint(p_left, p_right, folded_dist, dist, 0.2); // Low stiffness for compliance
            actuators_v.push(system.constraints.len() - 1);
        }
    }

    // Horizontal Creases
    for i in 0..=cols {
        for j in 1..rows {
            let p_top = i * (rows + 1) + (j - 1);
            let p_bottom = i * (rows + 1) + (j + 1);

            let dist = system.particles[p_top]
                .pos
                .distance(system.particles[p_bottom].pos);
            let folded_dist = dist * 0.1;

            system.add_actuator_constraint(p_top, p_bottom, folded_dist, dist, 0.2);
            actuators_h.push(system.constraints.len() - 1);
        }
    }

    // Pin the head (left side) so it doesn't float away
    let head_idx = (rows + 1) / 2;
    let head_pos = system.particles[head_idx].pos;
    system.add_pin_constraint(head_idx, head_pos);

    (system, indices, actuators_v, actuators_h)
}

#[macroquad::main("Neuro-Fold")]
async fn main() {
    let mut creature = Creature::new();

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
            // Add noise to a random neuron to excite the system
            let idx = ::rand::thread_rng().gen_range(0..creature.brain.neurons.len());
            creature.brain.neurons[idx].v += 50.0;
        }

        creature.update(0.016);
        creature.draw();

        set_default_camera();

        // HUD
        draw_text("NEURO-FOLD", 10.0, 30.0, 30.0, WHITE);
        draw_text("CPG driving Origami Mesh", 10.0, 50.0, 20.0, GRAY);
        draw_text("Click to Excite Neurons", 10.0, 70.0, 20.0, RED);

        // Visualize Neural Activity
        let start_x = 10.0;
        let start_y = 100.0;
        let spacing = 15.0;

        for (i, neuron) in creature.brain.neurons.iter().enumerate() {
            let x = start_x + (i % 20) as f32 * spacing;
            let y = start_y + (i / 20) as f32 * spacing;

            let color = if neuron.v > 0.0 {
                GREEN
            } else {
                Color::new(0.2, 0.2, 0.2, 1.0)
            };

            draw_circle(x, y, 5.0, color);
        }

        next_frame().await
    }
}
