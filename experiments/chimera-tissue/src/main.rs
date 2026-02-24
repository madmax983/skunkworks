use macroquad::prelude::*;
use chimera_lang::prelude::*;
use physics_pbd::{PbdSystem, Constraint};
use ::rand::Rng; // Disambiguate rand

struct Cell {
    vm: ChimeraVM,
    particle_idx: usize,
    actuators: Vec<usize>, // Indices into system.constraints
    color: Color,
}

struct Tissue {
    system: PbdSystem,
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Tissue {
    fn new(width: usize, height: usize) -> Self {
        let mut system = PbdSystem::new();
        let mut cells = Vec::new();
        let mut particle_indices = Vec::new();

        // Create Particles
        for y in 0..height {
            for x in 0..width {
                let pos = vec3(
                    x as f32 * 0.5 - (width as f32 * 0.25),
                    y as f32 * 0.5 - (height as f32 * 0.25),
                    0.0
                );
                let idx = system.add_particle(pos, 1.0);
                particle_indices.push(idx);
            }
        }

        // Create Constraints and Cells

        // Helper to add actuator
        let mut add_actuator = |p1: usize, p2: usize, sys: &mut PbdSystem| -> usize {
            let idx = sys.constraints.len();
            sys.add_actuator_constraint(p1, p2, 0.2, 0.8, 0.5); // min, max, stiffness
            idx
        };

        // Helper to add structural strut
        let mut add_strut = |p1: usize, p2: usize, sys: &mut PbdSystem| {
            sys.add_distance_constraint(p1, p2, 0.5);
        };

        // Build Connectivity
        let mut cell_actuators = vec![Vec::new(); width * height];

        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;
                let p1 = particle_indices[idx];

                // Right Neighbor
                if x + 1 < width {
                    let p2 = particle_indices[y * width + (x + 1)];
                    let c_idx = add_actuator(p1, p2, &mut system);
                    cell_actuators[idx].push(c_idx);
                    cell_actuators[y * width + (x + 1)].push(c_idx);
                }

                // Bottom Neighbor
                if y + 1 < height {
                    let p2 = particle_indices[(y + 1) * width + x];
                    let c_idx = add_actuator(p1, p2, &mut system);
                    cell_actuators[idx].push(c_idx);
                    cell_actuators[(y + 1) * width + x].push(c_idx);
                }

                // Cross Struts (Diagonals) for shear stability
                if x + 1 < width && y + 1 < height {
                    let p2 = particle_indices[(y + 1) * width + (x + 1)];
                    add_strut(p1, p2, &mut system);

                    let p3 = particle_indices[y * width + (x + 1)];
                    let p4 = particle_indices[(y + 1) * width + x];
                    add_strut(p3, p4, &mut system);
                }
            }
        }

        // Initialize Cells
        let dna = create_dna();
        for (i, actuators) in cell_actuators.into_iter().enumerate() {
            let mut vm = ChimeraVM::new(dna.clone());
            // Randomize color slightly
            let r = ::rand::thread_rng().gen_range(0.4..0.9);
            let g = ::rand::thread_rng().gen_range(0.2..0.6);
            let b = ::rand::thread_rng().gen_range(0.4..0.9);

            cells.push(Cell {
                vm,
                particle_idx: particle_indices[i],
                actuators,
                color: Color::new(r, g, b, 1.0),
            });
        }

        // Pin the center particle to keep it anchored
        let center_idx = (height / 2) * width + (width / 2);
        system.add_pin_constraint(particle_indices[center_idx], vec3(0.0, 0.0, 0.0));

        Tissue {
            system,
            cells,
            width,
            height,
        }
    }

    fn update(&mut self, dt: f32) {
        // Physics
        self.system.step(dt, 5);

        // Split borrows to avoid conflict
        let system = &mut self.system;
        let cells = &mut self.cells;

        // VM Logic
        for cell in cells {
            // 1. Sense: Calculate average strain (current_len / max_len)
            let mut total_strain = 0.0;
            let count = cell.actuators.len();

            if count > 0 {
                for &c_idx in &cell.actuators {
                    if let Constraint::Actuator { p1, p2, max_len, .. } = system.constraints[c_idx] {
                        let pos1 = system.particles[p1].pos;
                        let pos2 = system.particles[p2].pos;
                        let dist = pos1.distance(pos2);
                        total_strain += dist / max_len;
                    }
                }
                let avg_strain = total_strain / count as f32;

                // Input to VM: Strain scaled to 0-100 integer
                cell.vm.stack.push(Value::Int((avg_strain * 100.0) as i64));
            } else {
                cell.vm.stack.push(Value::Int(0));
            }

            // 2. Think: Run VM
            for _ in 0..50 {
                cell.vm.step();
            }

            // 3. Act: Read output
            if let Some(val) = cell.vm.stack.pop() {
                let factor = match val {
                    Value::Int(n) => (n as f32 / 100.0).clamp(0.0, 1.0),
                    // Value::Number does not exist in Chimera Value enum (it uses Int)
                    _ => 0.5,
                };

                // Apply to actuators
                for &c_idx in &cell.actuators {
                    if let Constraint::Actuator { factor: ref mut f, .. } = &mut system.constraints[c_idx] {
                        *f = factor;
                    }
                }

                // Update color based on contraction
                cell.color.a = 0.5 + factor * 0.5;
            }

            // Reset VM for next tick
            cell.vm.stack.clear();
            cell.vm.energy = 1000;
            cell.vm.ip = (0, 0);
        }
    }

    fn draw(&self) {
        // Draw Constraints
        for constraint in &self.system.constraints {
            match constraint {
                Constraint::Actuator { p1, p2, factor, .. } => {
                    let pos1 = self.system.particles[*p1].pos;
                    let pos2 = self.system.particles[*p2].pos;
                    // Color based on contraction factor (Red = Contracted, Blue = Extended)
                    // Dereference factor since it's &f32
                    let color = Color::new(1.0 - factor, 0.2, *factor, 0.8);
                    draw_line(pos1.x, pos1.y, pos2.x, pos2.y, 0.05, color);
                }
                Constraint::Distance { p1, p2, .. } => {
                    let pos1 = self.system.particles[*p1].pos;
                    let pos2 = self.system.particles[*p2].pos;
                    draw_line(pos1.x, pos1.y, pos2.x, pos2.y, 0.02, GRAY);
                }
                _ => {}
            }
        }

        // Draw Cells
        for cell in &self.cells {
            let pos = self.system.particles[cell.particle_idx].pos;
            draw_circle(pos.x, pos.y, 0.1, cell.color);
        }
    }
}

fn create_dna() -> Dna {
    // Simple Oscillator DNA
    // Logic:
    // Input: Strain (0-100)
    // Desired: If Strain > 50 (Stretched), Contract (Factor 0.0). If Strain < 50 (Compressed), Relax (Factor 1.0).
    // This creates a negative feedback loop -> Oscillation.

    let genes = vec![
        // Stack: [Strain]
        Gene { op: OpCode::Dup, args: vec![] }, // [Strain, Strain]
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(50)] }, // [Strain, Strain, 50]
        Gene { op: OpCode::Gt, args: vec![] }, // [Strain, Strain > 50 ? 1 : 0]

        // If > 50, we want 0 (Contract). If <= 50, we want 100 (Relax).
        // Current Stack Top: 1 (True) or 0 (False)

        // Let's implement logic: Output = (1 - IsStretched) * 100
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] }, // [Strain, IsStr, 1]
        Gene { op: OpCode::Swap, args: vec![] }, // [Strain, 1, IsStr]
        Gene { op: OpCode::Sub, args: vec![] }, // [Strain, 1 - IsStr] (Now 0 if stretched, 1 if relaxed)

        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(100)] }, // [Strain, Result, 100]
        Gene { op: OpCode::Mul, args: vec![] }, // [Strain, Result * 100]
    ];

    Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
        evolution_config: None,
    }
}

#[macroquad::main("Chimera Tissue")]
async fn main() {
    let mut tissue = Tissue::new(10, 10);

    // Camera
    let mut cam_zoom = 20.0;
    let cam_target = vec2(0.0, 0.0);

    loop {
        if is_key_down(KeyCode::Up) { cam_zoom += 1.0; }
        if is_key_down(KeyCode::Down) { cam_zoom -= 1.0; }

        set_camera(&Camera2D {
            zoom: vec2(1.0 / cam_zoom, 1.0 / cam_zoom * screen_width() / screen_height()),
            target: cam_target,
            ..Default::default()
        });

        clear_background(BLACK);

        tissue.update(0.016);
        tissue.draw();

        set_default_camera();
        draw_text("Chimera Tissue", 10.0, 30.0, 30.0, WHITE);
        draw_text("Cells: 100", 10.0, 50.0, 20.0, GRAY);

        next_frame().await
    }
}
