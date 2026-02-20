use ::rand::Rng;
use macroquad::prelude::*;

mod glyph;
mod starmap;

use neuro_sim::Network;
use physics_pbd::{Constraint, PbdSystem};
use starmap::StarMap;

const MESH_ROWS: usize = 8;
const MESH_COLS: usize = 16;
const SIM_STEPS: usize = 4;

struct CipherCreature {
    system: PbdSystem,
    indices: Vec<u16>,
    actuators_v: Vec<usize>,
    actuators_h: Vec<usize>,
    brain: Network,
    motor_neurons: Vec<usize>,
    sensor_neurons: Vec<usize>,
    starmap: StarMap,
    target_texture: Option<Texture2D>,
    score: f32,
    best_score: f32,
}

impl CipherCreature {
    fn new() -> Self {
        let (system, indices, actuators_v, actuators_h) = generate_miura_ori(MESH_ROWS, MESH_COLS);

        let mut brain = Network::new();
        let mut motor_neurons = Vec::new();
        let mut sensor_neurons = Vec::new();

        // Create a CPG chain for the columns
        let mut prev_neuron = None;

        for _ in 0..MESH_COLS {
            // Motor neuron
            let motor = brain.add_neuron();
            motor_neurons.push(motor);

            // Sensor neuron
            let sensor = brain.add_neuron();
            sensor_neurons.push(sensor);

            // CPG Logic
            if let Some(prev) = prev_neuron {
                brain.add_synapse(prev, motor, 20.0);
                brain.add_synapse(motor, prev, -5.0);
            }

            brain.add_synapse(sensor, motor, 15.0);
            prev_neuron = Some(motor);
        }

        if let Some(last) = prev_neuron {
            if !motor_neurons.is_empty() {
                brain.add_synapse(last, motor_neurons[0], 20.0);
            }
        }

        // Initialize StarMap with a secret message
        let starmap = StarMap::new(b"CIPHER", MESH_COLS as u32);

        Self {
            system,
            indices,
            actuators_v,
            actuators_h,
            brain,
            motor_neurons,
            sensor_neurons,
            starmap,
            target_texture: None, // Initialized in draw/main loop context
            score: 0.0,
            best_score: 0.0,
        }
    }

    fn init_texture(&mut self) {
        let img = self.starmap.generate();
        let width = img.width();
        let height = img.height();
        let bytes = img.into_raw();
        self.target_texture = Some(Texture2D::from_rgba8(width as u16, height as u16, &bytes));
    }

    fn update(&mut self, dt: f32) {
        // 1. Calculate Score (Alignment)
        // Project 3D particles to 2D XY plane and check against StarMap
        // Map mesh bounds to texture bounds
        let mesh_width = (MESH_COLS as f32) * 2.0; // Approx based on generation
        let mesh_height = (MESH_ROWS as f32) * 2.0;

        // StarMap dimensions
        let _map_w = self.starmap.width as f32;
        let _map_h = self.starmap.height as f32;

        let mut current_score = 0.0;
        let mut active_vertices = 0;

        // We use the raw starmap data to check alignment
        // Ideally we'd keep the ImageBuffer, but for now let's just use a simple distance heuristic
        // or re-generate a small buffer for checking.
        // Optimization: We won't check pixel-perfect, but "Area of Interest" perfect.
        // But for visual flair, let's just use the score to drive the "Calmness".

        // Let's say the center of the mesh is (0,0).
        // The texture is centered at (0,0) in world space for comparison.

        for p in &self.system.particles {
            // Map p.pos.x, p.pos.y to texture coordinates
            // Texture is roughly matching the mesh size
            // x: [-mesh_width/2, mesh_width/2] -> [0, map_w]

            let u = (p.pos.x / mesh_width) + 0.5;
            let v = (p.pos.y / mesh_height) + 0.5;

            if u >= 0.0 && u <= 1.0 && v >= 0.0 && v <= 1.0 {
                // Check if this UV hits a "Star"
                // This requires access to the pixel data.
                // Since we consumed the image buffer, we can't easily check.
                // Let's assume a simplified "Hotspot" model:
                // If Z is close to 0 (flat), score improves.
                // If the mesh is "Flat", the text is readable.
                current_score += (1.0 - p.pos.z.abs()).max(0.0);
                active_vertices += 1;
            }
        }

        if active_vertices > 0 {
            self.score = current_score / (active_vertices as f32);
        }

        if self.score > self.best_score {
            self.best_score = self.score;
        }

        // 2. Feedback: "Focusing"
        // High Score (Flat/Aligned) -> Low Activity (Freeze)
        // Low Score (Crumpled) -> High Activity (Search)
        let excitement = (1.0 - self.score * 0.5).clamp(0.0, 1.0) * 50.0;

        // Feed excitement into random neurons to stimulate movement
        if ::rand::thread_rng().gen_bool(0.1) {
            let idx = ::rand::thread_rng().gen_range(0..self.brain.neurons.len());
            self.brain.neurons[idx].v += excitement;
        }

        // 3. Step Brain
        // Also feed strain inputs
        let mut inputs = vec![0.0; self.brain.neurons.len()];
        let actuators_per_col = MESH_ROWS + 1;

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
            if col < self.sensor_neurons.len() {
                inputs[self.sensor_neurons[col]] = total_strain * 5.0;
            }
        }

        self.brain.step(&inputs);

        // 4. Motor Output -> Actuators
        for col in 0..(MESH_COLS - 1) {
            if col < self.motor_neurons.len() {
                let neuron_idx = self.motor_neurons[col];
                let target_factor = if self.brain.is_spiking(neuron_idx) {
                    0.0
                } else {
                    1.0
                };

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

        // 5. Physics
        self.system.step(dt, SIM_STEPS);
    }

    fn draw(&self) {
        // Draw StarMap (Background Plane)
        if let Some(tex) = &self.target_texture {
            let mesh_width = (MESH_COLS as f32) * 2.0;
            let mesh_height = (MESH_ROWS as f32) * 2.0;

            // Draw a quad with the texture below the mesh
            draw_texture_ex(
                tex,
                -mesh_width / 2.0,
                -mesh_height / 2.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(mesh_width, mesh_height)),
                    ..Default::default()
                },
            );
        }

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
            let light = vec3(0.0, 0.0, 1.0).normalize();
            let diffuse = normal.dot(light).abs();

            // Transparency based on Z (Reveal the map below)
            let alpha = 0.3 + (1.0 - diffuse) * 0.5;

            let color = Color::new(0.1, 0.8, 0.6, alpha);
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

            draw_line_3d(v1, v2, Color::new(0.0, 1.0, 0.0, 0.5));
            draw_line_3d(v2, v3, Color::new(0.0, 1.0, 0.0, 0.5));
            draw_line_3d(v3, v1, Color::new(0.0, 1.0, 0.0, 0.5));
        }

        draw_mesh(&mesh);
    }
}

// Generate Miura-Ori (Copied from neuro-fold)
fn generate_miura_ori(rows: usize, cols: usize) -> (PbdSystem, Vec<u16>, Vec<usize>, Vec<usize>) {
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

            // Initial Crumpled State
            let z = (i as f32 * 0.8).sin() * 3.0 + (j as f32 * 0.5).cos() * 3.0;

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

    // Vertical Creases
    for i in 1..cols {
        for j in 0..=rows {
            let p_left = (i - 1) * (rows + 1) + j;
            let p_right = (i + 1) * (rows + 1) + j;
            let dist = system.particles[p_left]
                .pos
                .distance(system.particles[p_right].pos);
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
            let dist = system.particles[p_top]
                .pos
                .distance(system.particles[p_bottom].pos);
            let folded_dist = dist * 0.1;
            system.add_actuator_constraint(p_top, p_bottom, folded_dist, dist, 0.2);
            actuators_h.push(system.constraints.len() - 1);
        }
    }

    // Pin Center to prevent drift
    let center_idx = (cols / 2) * (rows + 1) + (rows / 2);
    let center_pos = system.particles[center_idx].pos;
    system.add_pin_constraint(center_idx, center_pos);

    (system, indices, actuators_v, actuators_h)
}

#[macroquad::main("Neuro-Cipher")]
async fn main() {
    let mut creature = CipherCreature::new();
    creature.init_texture();

    loop {
        clear_background(BLACK);

        set_camera(&Camera3D {
            position: vec3(0.0, -30.0, 30.0),
            target: vec3(0.0, 0.0, 0.0),
            up: vec3(0.0, 0.0, 1.0),
            ..Default::default()
        });

        // Mouse interaction
        if is_mouse_button_down(MouseButton::Left) {
            let idx = ::rand::thread_rng().gen_range(0..creature.brain.neurons.len());
            creature.brain.neurons[idx].v += 50.0;
        }

        creature.update(0.016);
        creature.draw();

        set_default_camera();

        // UI
        draw_text("NEURO-CIPHER", 10.0, 30.0, 30.0, WHITE);
        draw_text(
            &format!("Focus/Score: {:.2}", creature.score),
            10.0,
            50.0,
            20.0,
            GREEN,
        );
        draw_text(
            "The Neural Network folds the mesh to reveal the StarMap",
            10.0,
            70.0,
            20.0,
            GRAY,
        );

        // Visualize Brain
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
