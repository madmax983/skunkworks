mod physics;

use chimera_lang::prelude::*;
use hyper_system::math::Vec4;
use hyper_system::monitor::SystemMonitor;
use macroquad::prelude::*;
use physics::{Constraint4D, PbdSystem4D};
use ::rand::Rng;

const GRID_SIZE: usize = 3;
const SPACING: f32 = 2.0;

struct Cell {
    vm: ChimeraVM,
    particle_idx: usize,
    actuators: Vec<usize>, // Indices into system.constraints
    color: Color,
}

struct Tissue4D {
    system: PbdSystem4D,
    cells: Vec<Cell>,
    monitor: SystemMonitor,
}

impl Tissue4D {
    fn new() -> Self {
        let mut system = PbdSystem4D::new();
        let mut cells = Vec::new();
        let mut particle_indices = Vec::new();

        // Create Particles in a 4D Grid (3x3x3x3 = 81 particles)
        // Center the grid around origin
        let offset = (GRID_SIZE as f32 - 1.0) * SPACING / 2.0;

        for w in 0..GRID_SIZE {
            for z in 0..GRID_SIZE {
                for y in 0..GRID_SIZE {
                    for x in 0..GRID_SIZE {
                        let pos = Vec4::new(
                            x as f32 * SPACING - offset,
                            y as f32 * SPACING - offset,
                            z as f32 * SPACING - offset,
                            w as f32 * SPACING - offset,
                        );
                        // Pin center?
                        // Center is at index 40 (1,1,1,1) if grid is 3x3x3x3
                        let mass = 1.0;
                        let idx = system.add_particle(pos, mass);
                        particle_indices.push(idx);
                    }
                }
            }
        }

        // Pin the center particle to anchor the simulation
        let center_coord = GRID_SIZE / 2;
        let center_idx = idx(center_coord, center_coord, center_coord, center_coord);
        system.add_pin_constraint(particle_indices[center_idx], Vec4::zero());

        // Helper to get linear index
        fn idx(x: usize, y: usize, z: usize, w: usize) -> usize {
            w * (GRID_SIZE * GRID_SIZE * GRID_SIZE)
                + z * (GRID_SIZE * GRID_SIZE)
                + y * GRID_SIZE
                + x
        }

        // Create Constraints and Cells
        let mut cell_actuators = vec![Vec::new(); particle_indices.len()];

        let add_actuator =
            |p1_idx: usize, p2_idx: usize, sys: &mut PbdSystem4D| -> usize {
                let c_idx = sys.constraints.len();
                sys.add_actuator_constraint(p1_idx, p2_idx, SPACING * 0.5, SPACING * 1.5, 0.5);
                c_idx
            };

        for w in 0..GRID_SIZE {
            for z in 0..GRID_SIZE {
                for y in 0..GRID_SIZE {
                    for x in 0..GRID_SIZE {
                        let current_linear = idx(x, y, z, w);
                        let p1 = particle_indices[current_linear];

                        // Connect to neighbors in positive direction to avoid duplicates
                        // X neighbor
                        if x + 1 < GRID_SIZE {
                            let neighbor_linear = idx(x + 1, y, z, w);
                            let p2 = particle_indices[neighbor_linear];
                            let c_idx = add_actuator(p1, p2, &mut system);
                            cell_actuators[current_linear].push(c_idx);
                            cell_actuators[neighbor_linear].push(c_idx);
                        }
                        // Y neighbor
                        if y + 1 < GRID_SIZE {
                            let neighbor_linear = idx(x, y + 1, z, w);
                            let p2 = particle_indices[neighbor_linear];
                            let c_idx = add_actuator(p1, p2, &mut system);
                            cell_actuators[current_linear].push(c_idx);
                            cell_actuators[neighbor_linear].push(c_idx);
                        }
                        // Z neighbor
                        if z + 1 < GRID_SIZE {
                            let neighbor_linear = idx(x, y, z + 1, w);
                            let p2 = particle_indices[neighbor_linear];
                            let c_idx = add_actuator(p1, p2, &mut system);
                            cell_actuators[current_linear].push(c_idx);
                            cell_actuators[neighbor_linear].push(c_idx);
                        }
                        // W neighbor
                        if w + 1 < GRID_SIZE {
                            let neighbor_linear = idx(x, y, z, w + 1);
                            let p2 = particle_indices[neighbor_linear];
                            let c_idx = add_actuator(p1, p2, &mut system);
                            cell_actuators[current_linear].push(c_idx);
                            cell_actuators[neighbor_linear].push(c_idx);
                        }
                    }
                }
            }
        }

        // Initialize Cells with DNA
        let dna = create_dna();
        for (i, actuators) in cell_actuators.into_iter().enumerate() {
            let vm = ChimeraVM::new(dna.clone());
            let r = ::rand::thread_rng().gen_range(0.4..0.9);
            let g = ::rand::thread_rng().gen_range(0.2..0.6);
            let b = ::rand::thread_rng().gen_range(0.8..1.0); // Blueish for "Hyper"

            cells.push(Cell {
                vm,
                particle_idx: particle_indices[i],
                actuators,
                color: Color::new(r, g, b, 0.8),
            });
        }

        Tissue4D {
            system,
            cells,
            monitor: SystemMonitor::new(),
        }
    }

    fn update(&mut self, dt: f32) {
        self.monitor.update();
        let cpu_load = self.monitor.cpu_usage; // 0.0 to 1.0

        // Distort Physics based on CPU Load
        // Maybe change stiffness or target length of constraints globally?
        // Or apply a "wind" force in W-axis?

        // Physics Step
        self.system.step(dt, 5);

        // VM Logic
        let constraints = &mut self.system.constraints;
        let particles = &self.system.particles;

        for cell in &mut self.cells {
            // 1. Sense: Strain
            let mut total_strain = 0.0;
            let count = cell.actuators.len();

            if count > 0 {
                for &c_idx in &cell.actuators {
                    if let Constraint4D::Actuator {
                        p1,
                        p2,
                        max_len,
                        ..
                    } = constraints[c_idx]
                    {
                        let pos1 = particles[p1].pos;
                        let pos2 = particles[p2].pos;
                        let dist = pos1.distance_squared(pos2).sqrt();
                        total_strain += dist / max_len;
                    }
                }
                let avg_strain = total_strain / count as f32;

                // Inject System Load as a "Hormone" (Input)
                cell.vm.stack.push(Value::Int((cpu_load * 100.0) as i64));
                cell.vm.stack.push(Value::Int((avg_strain * 100.0) as i64));
            } else {
                 cell.vm.stack.push(Value::Int(0));
                 cell.vm.stack.push(Value::Int(0));
            }

            // 2. Think
            for _ in 0..50 {
                cell.vm.step();
            }

            // 3. Act
            if let Some(val) = cell.vm.stack.pop() {
                let factor = match val {
                    Value::Int(n) => (n as f32 / 100.0).clamp(0.0, 1.0),
                    _ => 0.5,
                };

                // Apply to actuators
                for &c_idx in &cell.actuators {
                    if let Constraint4D::Actuator {
                        factor: ref mut f, ..
                    } = &mut constraints[c_idx]
                    {
                        *f = factor;
                    }
                }

                // Update color
                cell.color.a = 0.2 + factor * 0.8;
                // Shift hue based on W-position?
                let w_pos = particles[cell.particle_idx].pos.w;
                cell.color.g = (0.5 + w_pos * 0.1).clamp(0.0, 1.0);
            }

            // Reset
            cell.vm.stack.clear();
            cell.vm.energy = 1000;
            cell.vm.ip = (0, 0);
        }
    }

    fn draw(&self) {
        // Project all particles to 3D
        let camera_w = 15.0; // Observer position in W

        // Draw Constraints (Edges)
        for constraint in &self.system.constraints {
            match constraint {
                Constraint4D::Actuator { p1, p2, factor, .. } => {
                    let pos1_4d = self.system.particles[*p1].pos;
                    let pos2_4d = self.system.particles[*p2].pos;

                    let p1_3d = pos1_4d.project_to_3d(camera_w);
                    let p2_3d = pos2_4d.project_to_3d(camera_w);

                    // Color based on W-depth and Contraction
                    let depth_alpha = (1.0 - (pos1_4d.w + pos2_4d.w) * 0.05).clamp(0.1, 1.0);
                    let color = Color::new(1.0 - factor, 0.2, *factor, depth_alpha);

                    draw_line_3d(
                        vec3(p1_3d.x, p1_3d.y, p1_3d.z),
                        vec3(p2_3d.x, p2_3d.y, p2_3d.z),
                        color,
                    );
                }
                _ => {}
            }
        }

        // Draw Cells (Vertices)
        for cell in &self.cells {
            let pos_4d = self.system.particles[cell.particle_idx].pos;
            let p_3d = pos_4d.project_to_3d(camera_w);

            // Size depends on W distance (perspective is handled by project_to_3d coordinates,
            // but we can also scale the sprite size)
            let size = 0.2 * (2.0 / (camera_w - pos_4d.w).max(0.1));

            draw_sphere(vec3(p_3d.x, p_3d.y, p_3d.z), size, None, cell.color);
        }
    }
}

fn create_dna() -> Dna {
    // Oscillator DNA that reacts to Strain and CPU Load
    // Stack: [Strain, CpuLoad]
    let genes = vec![
        // Calculate Target = 100 - Strain + CpuLoad
        Gene { op: OpCode::Add, args: vec![] }, // [Strain + CpuLoad]
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(100)] }, // [Sum, 100]
        Gene { op: OpCode::Swap, args: vec![] }, // [100, Sum]
        Gene { op: OpCode::Sub, args: vec![] }, // [100 - Sum] -> Output Factor
        // If Sum is high (high strain + high load), Output is low (Contract)
        // If Sum is low, Output is high (Relax)
    ];

    Dna {
        helix: Helix { strands: vec![Strand { genes }] },
        evolution_config: None,
    }
}

#[macroquad::main("Hyper Tissue")]
async fn main() {
    let mut tissue = Tissue4D::new();

    let mut cam_yaw = 0.0;
    let mut cam_pitch = 0.0;
    let mut last_mouse_pos = mouse_position();

    loop {
        let dt = get_frame_time();

        // Camera Controls
        if is_mouse_button_down(MouseButton::Left) {
            let mouse_pos = mouse_position();
            let delta_x = mouse_pos.0 - last_mouse_pos.0;
            let delta_y = mouse_pos.1 - last_mouse_pos.1;
            cam_yaw += delta_x * 0.01;
            cam_pitch += delta_y * 0.01;
        }
        last_mouse_pos = mouse_position();

        let radius = 25.0;
        let cam_pos = vec3(
            cam_yaw.sin() * radius * cam_pitch.cos(),
            cam_pitch.sin() * radius,
            cam_yaw.cos() * radius * cam_pitch.cos(),
        );

        set_camera(&Camera3D {
            position: cam_pos,
            up: vec3(0.0, 1.0, 0.0),
            target: vec3(0.0, 0.0, 0.0),
            ..Default::default()
        });

        clear_background(BLACK);

        // Apply global 4D rotation to visualize the hypercube
        // Rotate all particles in XW plane over time
        let rotation_speed = 0.5 * dt;
        for p in &mut tissue.system.particles {
             if p.inv_mass > 0.0 {
                 // Rotate position relative to origin?
                 // No, just apply force? No, let's rotate the View (Projection) implicitly
                 // by rotating the particles in 4D space
                 let p_new = p.pos.rotate_xw(rotation_speed);
                 p.pos = p_new;
                 // Also rotate velocity to conserve momentum direction relative to rotation?
                 // Simple rotation is fine for visualization
             }
        }

        tissue.update(dt);
        tissue.draw();

        set_default_camera();
        draw_text("Hyper Tissue 4D", 10.0, 30.0, 30.0, WHITE);
        draw_text(
            &format!("FPS: {}", get_fps()),
            10.0,
            50.0,
            20.0,
            GRAY,
        );
        draw_text(
            &format!("CPU Load: {:.2}%", tissue.monitor.cpu_usage * 100.0),
            10.0,
            70.0,
            20.0,
            RED,
        );
         draw_text(
            "Left Click + Drag to Rotate 3D View",
            10.0,
            90.0,
            20.0,
            LIGHTGRAY,
        );
        draw_text(
            "4D Rotation (XW) is automatic",
            10.0,
            110.0,
            20.0,
            LIGHTGRAY,
        );

        next_frame().await
    }
}
