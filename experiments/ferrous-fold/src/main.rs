use ::rand::Rng;
use chimera_lang::prelude::*;
use glam::Vec3;
use macroquad::prelude::*;
use origami::{MiuraOri, MiuraParams, Orientation, OrigamiMesh}; // Removed OrigamiVertex
use physics_pbd::PbdSystem;

struct Cell {
    vm: ChimeraVM,
    particle_idx: usize,
    color: Color,
    magnetism: f32, // 0.0 (North/Blue) to 1.0 (South/Red), 0.5 Neutral
}

struct FoldedTissue {
    system: PbdSystem,
    cells: Vec<Cell>,
    mesh: OrigamiMesh,
}

impl FoldedTissue {
    fn new(cols: usize, rows: usize) -> Self {
        let mut system = PbdSystem::new();
        let mut cells = Vec::new();

        // 1. Generate Origami Mesh
        let params = MiuraParams {
            a: 1.0,
            b: 1.0,
            gamma: 80.0f32.to_radians(), // Sharp fold angle
            orientation: Orientation::Horizontal,
        };
        let origami = MiuraOri::new(params, (cols, rows));
        let initial_mesh = origami.generate_mesh(0.5); // Start half-folded

        // 2. Create Particles from Vertices
        let mut particle_indices = Vec::new();
        for v in &initial_mesh.vertices {
            // Add particle with mass 1.0
            let idx = system.add_particle(v.pos, 1.0);
            particle_indices.push(idx);
        }

        // 3. Create Constraints from Mesh Indices (Triangles)
        let mut edges = std::collections::HashSet::new();

        for i in (0..initial_mesh.indices.len()).step_by(3) {
            let i0 = initial_mesh.indices[i] as usize;
            let i1 = initial_mesh.indices[i + 1] as usize;
            let i2 = initial_mesh.indices[i + 2] as usize;

            let p0 = particle_indices[i0];
            let p1 = particle_indices[i1];
            let p2 = particle_indices[i2];

            // Add edges if not already added
            let mut add_edge = |a: usize, b: usize| {
                let (min, max) = if a < b { (a, b) } else { (b, a) };
                if edges.insert((min, max)) {
                    let p_a: Vec3 = system.particles[min].pos;
                    let p_b: Vec3 = system.particles[max].pos;
                    let dx = p_a.x - p_b.x;
                    let dy = p_a.y - p_b.y;
                    let dz = p_a.z - p_b.z;
                    let dist = (dx * dx + dy * dy + dz * dz).sqrt();
                    system.add_distance_constraint(min, max, dist);
                }
            };

            add_edge(p0, p1);
            add_edge(p1, p2);
            add_edge(p2, p0);
        }

        // 4. Initialize Cells (VM)
        let dna = create_dna();
        for (i, &p_idx) in particle_indices.iter().enumerate() {
            let vm = ChimeraVM::new(dna.clone());

            // Random initial magnetism
            let magnetism = ::rand::thread_rng().gen_range(0.0..1.0);

            cells.push(Cell {
                vm,
                particle_idx: p_idx,
                color: Color::new(0.5, 0.5, 0.5, 1.0),
                magnetism,
            });
        }

        // Pin the center vertex to prevent drifting away
        if !particle_indices.is_empty() {
            let center_idx = particle_indices[particle_indices.len() / 2];
            system.add_pin_constraint(center_idx, vec3(0.0, 0.0, 0.0));
        }

        FoldedTissue {
            system,
            cells,
            mesh: initial_mesh,
        }
    }

    fn update(&mut self, dt: f32) {
        // 1. Calculate Magnetic Forces
        let mag_strength = 200.0; // Stronger for folding
        let count = self.cells.len();
        let mut forces = vec![Vec3::ZERO; self.system.particles.len()];

        for i in 0..count {
            for j in (i + 1)..count {
                let p1_idx = self.cells[i].particle_idx;
                let p2_idx = self.cells[j].particle_idx;

                let pos1 = self.system.particles[p1_idx].pos;
                let pos2 = self.system.particles[p2_idx].pos;

                // Normalize magnetism to -0.5 to 0.5 range (Polarity)
                let m1 = self.cells[i].magnetism - 0.5;
                let m2 = self.cells[j].magnetism - 0.5;

                let delta = pos2 - pos1;
                let dist_sq = delta.length_squared().max(0.1); // Avoid singularity
                let dir = delta.normalize_or_zero();

                // F = (k * q1 * q2) / r^2
                // If m1, m2 have same sign -> product positive -> Repel (Force pushes p2 away)
                // If opposite sign -> product negative -> Attract
                let force_mag = (m1 * m2 * mag_strength) / dist_sq;
                let force = dir * force_mag;

                forces[p1_idx] -= force;
                forces[p2_idx] += force;
            }
        }

        // Apply forces
        for (i, force) in forces.into_iter().enumerate() {
            let inv_mass = self.system.particles[i].inv_mass;
            if inv_mass > 0.0 {
                self.system.particles[i].vel += force * dt * inv_mass;
            }
        }

        // 2. Physics Step
        self.system.step(dt, 5);

        // 3. VM Logic & Update Visuals
        for cell in &mut self.cells {
            cell.vm
                .stack
                .push(Value::Int((cell.magnetism * 100.0) as i64));

            // Run VM
            for _ in 0..10 {
                cell.vm.step();
            }

            // Act: Update Magnetism
            if let Some(val) = cell.vm.stack.pop() {
                let target = match val {
                    Value::Int(n) => (n as f32 / 100.0).clamp(0.0, 1.0),
                    _ => 0.5,
                };
                // Smooth transition
                cell.magnetism = cell.magnetism * 0.9 + target * 0.1;
            }

            // Update Color
            // Blue (North/0.0) <-> Grey <-> Red (South/1.0)
            let t = cell.magnetism;
            cell.color = Color::new(t, 0.2, 1.0 - t, 1.0);

            // Reset VM for next frame
            cell.vm.stack.clear();
            cell.vm.energy = 100;
            cell.vm.ip = (0, 0);
        }

        // 4. Update Mesh Vertices from Physics
        for (i, v) in self.mesh.vertices.iter_mut().enumerate() {
            v.pos = self.system.particles[i].pos;
        }
    }

    fn draw(&self) {
        // Draw Mesh
        // We convert to Macroquad Mesh manually
        let mut mq_vertices = Vec::with_capacity(self.mesh.vertices.len());

        for (i, v) in self.mesh.vertices.iter().enumerate() {
            let color = self.cells[i].color;
            let r = (color.r * 255.0) as u8;
            let g = (color.g * 255.0) as u8;
            let b = (color.b * 255.0) as u8;
            let a = (color.a * 255.0) as u8;

            mq_vertices.push(Vertex {
                position: v.pos,
                uv: v.uv,
                color: [r, g, b, a],
                normal: vec4(0.0, 0.0, 1.0, 0.0), // Default normal
            });
        }

        let mesh = Mesh {
            vertices: mq_vertices,
            indices: self.mesh.indices.clone(),
            texture: None,
        };

        draw_mesh(&mesh);
    }
}

fn create_dna() -> Dna {
    // Magnetic Oscillator DNA
    // Logic: Target = 100 - Mag
    // Causes value to flip-flop over time.

    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(100)],
        },
        Gene {
            op: OpCode::Swap,
            args: vec![],
        }, // [100, Mag]
        Gene {
            op: OpCode::Sub,
            args: vec![],
        }, // [100 - Mag]
    ];

    Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
        evolution_config: None,
    }
}

#[macroquad::main("Ferrous Fold")]
async fn main() {
    let mut tissue = FoldedTissue::new(8, 8);

    let mut cam_dist = 20.0;
    let mut cam_angle_x = 0.0f32;
    let mut cam_angle_y = 0.5f32;

    loop {
        // Camera Controls
        if is_key_down(KeyCode::Up) {
            cam_angle_y += 0.02;
        }
        if is_key_down(KeyCode::Down) {
            cam_angle_y -= 0.02;
        }
        if is_key_down(KeyCode::Left) {
            cam_angle_x -= 0.02;
        }
        if is_key_down(KeyCode::Right) {
            cam_angle_x += 0.02;
        }
        if is_key_down(KeyCode::Z) {
            cam_dist -= 0.5;
        }
        if is_key_down(KeyCode::X) {
            cam_dist += 0.5;
        }

        let rot = Quat::from_rotation_y(cam_angle_x) * Quat::from_rotation_x(cam_angle_y);
        let pos = rot * vec3(0.0, 0.0, cam_dist);

        set_camera(&Camera3D {
            position: pos,
            target: vec3(0.0, 0.0, 0.0),
            up: vec3(0.0, 1.0, 0.0),
            ..Default::default()
        });

        clear_background(BLACK);

        // Grid for reference
        draw_grid(20, 1.0, DARKGRAY, GRAY);

        tissue.update(0.016);
        tissue.draw();

        set_default_camera();
        draw_text("Ferrous Fold", 10.0, 30.0, 30.0, WHITE);
        draw_text("Magnetic Miura-ori", 10.0, 50.0, 20.0, GRAY);
        draw_text("Arrows/ZX: Camera", 10.0, 70.0, 20.0, DARKGRAY);

        next_frame().await
    }
}
