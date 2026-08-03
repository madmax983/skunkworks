use ::rand::Rng;
use chimera_lang::prelude::*;
use macroquad::prelude::*;
use physics_pbd::{Constraint, PbdSystem};

struct Cell {
    vm: ChimeraVM,
    particle_idx: usize,
    actuators: Vec<usize>,
    color: Color,
    magnetism: f32, // 0.0 (North) to 1.0 (South), 0.5 Neutral
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
                    0.0,
                );
                let idx = system.add_particle(pos, 1.0);
                particle_indices.push(idx);
            }
        }

        // Build Connectivity
        let mut cell_actuators = vec![Vec::new(); width * height];

        // Helper to add actuator
        let add_actuator = |p1: usize, p2: usize, sys: &mut PbdSystem| -> usize {
            let idx = sys.constraints.len();
            sys.add_actuator_constraint(p1, p2, 0.2, 0.8, 0.5);
            idx
        };

        // Helper to add structural strut
        let add_strut = |p1: usize, p2: usize, sys: &mut PbdSystem| {
            sys.add_distance_constraint(p1, p2, 0.5);
        };

        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;
                let p1 = particle_indices[idx];

                if x + 1 < width {
                    let p2 = particle_indices[y * width + (x + 1)];
                    let c_idx = add_actuator(p1, p2, &mut system);
                    cell_actuators[idx].push(c_idx);
                    cell_actuators[y * width + (x + 1)].push(c_idx);
                }

                if y + 1 < height {
                    let p2 = particle_indices[(y + 1) * width + x];
                    let c_idx = add_actuator(p1, p2, &mut system);
                    cell_actuators[idx].push(c_idx);
                    cell_actuators[(y + 1) * width + x].push(c_idx);
                }

                // Cross Struts
                if x + 1 < width && y + 1 < height {
                    let p2 = particle_indices[(y + 1) * width + (x + 1)];
                    add_strut(p1, p2, &mut system);

                    let p3 = particle_indices[y * width + (x + 1)];
                    let p4 = particle_indices[(y + 1) * width + x];
                    add_strut(p3, p4, &mut system);
                }
            }
        }

        let dna = create_dna();
        for (i, actuators) in cell_actuators.into_iter().enumerate() {
            let vm = ChimeraVM::new(dna.clone());
            let r = ::rand::thread_rng().gen_range(0.4..0.9);
            let g = ::rand::thread_rng().gen_range(0.2..0.6);
            let b = ::rand::thread_rng().gen_range(0.4..0.9);

            cells.push(Cell {
                vm,
                particle_idx: particle_indices[i],
                actuators,
                color: Color::new(r, g, b, 1.0),
                magnetism: 0.5,
            });
        }

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
        // 1. Calculate Magnetic Forces and Apply to Velocities
        let mag_strength = 50.0;
        let count = self.cells.len();

        let mut forces = vec![Vec3::ZERO; self.system.particles.len()];

        for i in 0..count {
            for j in (i + 1)..count {
                let p1_idx = self.cells[i].particle_idx;
                let p2_idx = self.cells[j].particle_idx;

                let pos1 = self.system.particles[p1_idx].pos;
                let pos2 = self.system.particles[p2_idx].pos;

                let m1 = self.cells[i].magnetism - 0.5;
                let m2 = self.cells[j].magnetism - 0.5;

                let delta = pos2 - pos1;
                let dist_sq = delta.length_squared().max(0.01);
                let dir = delta.normalize_or_zero();

                // Force = k * m1 * m2 / r^2
                // Positive force (Repel) pushes p2 in dir, p1 in -dir

                let force_mag = (m1 * m2 * mag_strength) / dist_sq;
                let force = dir * force_mag;

                forces[p1_idx] -= force;
                forces[p2_idx] += force;
            }
        }

        // Apply forces to velocity
        for (i, force) in forces.into_iter().enumerate() {
            let inv_mass = self.system.particles[i].inv_mass;
            if inv_mass > 0.0 {
                self.system.particles[i].vel += force * dt * inv_mass;
            }
        }

        // 2. Physics Step
        self.system.step(dt, 5);

        // 3. VM Logic
        let system = &mut self.system;
        let cells = &mut self.cells;

        for cell in cells {
            // Sense: Strain
            let mut total_strain = 0.0;
            let count = cell.actuators.len();

            if count > 0 {
                for &c_idx in &cell.actuators {
                    if let Constraint::Actuator {
                        p1, p2, max_len, ..
                    } = system.constraints[c_idx]
                    {
                        let pos1 = system.particles[p1].pos;
                        let pos2 = system.particles[p2].pos;
                        let dist = pos1.distance(pos2);
                        total_strain += dist / max_len;
                    }
                }
                let avg_strain = total_strain / count as f32;
                cell.vm.stack.push(Value::Int((avg_strain * 100.0) as i64));
            } else {
                cell.vm.stack.push(Value::Int(0));
            }

            // Sense: Magnetism (Self)
            cell.vm
                .stack
                .push(Value::Int((cell.magnetism * 100.0) as i64));

            // Run VM
            for _ in 0..50 {
                cell.vm.step();
            }

            // Act: Read outputs (Magnetism, Contraction)
            // Expect stack: [..., Contraction, Magnetism] (Top is Magnetism)

            if let Some(val_mag) = cell.vm.stack.pop() {
                let mag = match val_mag {
                    Value::Int(n) => (n as f32 / 100.0).clamp(0.0, 1.0),
                    _ => 0.5,
                };
                cell.magnetism = mag;
            }

            if let Some(val_contract) = cell.vm.stack.pop() {
                let factor = match val_contract {
                    Value::Int(n) => (n as f32 / 100.0).clamp(0.0, 1.0),
                    _ => 0.5,
                };

                for &c_idx in &cell.actuators {
                    if let Constraint::Actuator {
                        factor: ref mut f, ..
                    } = &mut system.constraints[c_idx]
                    {
                        *f = factor;
                    }
                }
            }

            // Update color based on Magnetism
            // North (0.0) = Blue, South (1.0) = Red
            cell.color = Color::new(cell.magnetism, 0.2, 1.0 - cell.magnetism, 1.0);

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
                    let color = Color::new(0.5, 0.5, 0.5, 0.5); // Neutral Gray for muscles
                    draw_line(pos1.x, pos1.y, pos2.x, pos2.y, 0.05 * factor + 0.02, color);
                }
                Constraint::Distance { p1, p2, .. } => {
                    let pos1 = self.system.particles[*p1].pos;
                    let pos2 = self.system.particles[*p2].pos;
                    draw_line(pos1.x, pos1.y, pos2.x, pos2.y, 0.02, DARKGRAY);
                }
                _ => {}
            }
        }

        // Draw Cells with Halo for Magnetism
        for cell in &self.cells {
            let pos = self.system.particles[cell.particle_idx].pos;
            let radius = 0.15;
            draw_circle(pos.x, pos.y, radius, cell.color);

            // Draw field indicator (Inner dot)
            if (cell.magnetism - 0.5).abs() > 0.1 {
                draw_circle(pos.x, pos.y, 0.05, WHITE);
            }
        }
    }
}

fn create_dna() -> Dna {
    // Piezo-Magnetic DNA
    // Logic:
    // Inputs: [Strain, SelfMag]
    // Output Stack Goal: [Contraction, NewMag]

    // Logic:
    // 1. If Stretched (High Strain) -> Become Magnetic (High Mag). (Piezo)
    // 2. If Magnetic (High Mag) -> Contract (Low Factor). (Magnetostriction)

    let genes = vec![
        // Stack: [Strain, SelfMag]

        // --- Calculate Contraction based on Magnetism ---
        // Contraction = 100 - SelfMag
        Gene {
            op: OpCode::Dup,
            args: vec![],
        }, // [Strain, SelfMag, SelfMag]
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(100)],
        },
        Gene {
            op: OpCode::Swap,
            args: vec![],
        },
        Gene {
            op: OpCode::Sub,
            args: vec![],
        }, // [Strain, SelfMag, Contraction]
        // Stack: [Strain, SelfMag, Contraction]
        // Goal: [Contraction, NewMag]

        // Move Contraction deep.
        Gene {
            op: OpCode::Swap,
            args: vec![],
        }, // [Strain, Contraction, SelfMag]
        Gene {
            op: OpCode::Drop,
            args: vec![],
        }, // [Strain, Contraction]
        Gene {
            op: OpCode::Swap,
            args: vec![],
        }, // [Contraction, Strain]
        // --- Calculate NewMag based on Strain ---
        // If Strain > 50, Mag = 100, else 0.
        Gene {
            op: OpCode::Dup,
            args: vec![],
        }, // [Contraction, Strain, Strain]
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(50)],
        },
        Gene {
            op: OpCode::Gt,
            args: vec![],
        }, // [Contraction, Strain, Strain > 50 ? 1 : 0]
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(100)],
        },
        Gene {
            op: OpCode::Mul,
            args: vec![],
        }, // [Contraction, Strain, MagVal]
        Gene {
            op: OpCode::Swap,
            args: vec![],
        }, // [Contraction, MagVal, Strain]
        Gene {
            op: OpCode::Drop,
            args: vec![],
        }, // [Contraction, MagVal]
    ];

    Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
        evolution_config: None,
    }
}

#[macroquad::main("Ferrous Tissue")]
async fn main() {
    let mut tissue = Tissue::new(10, 10);

    let mut cam_zoom = 20.0;
    let cam_target = vec2(0.0, 0.0);

    loop {
        if is_key_down(KeyCode::Up) {
            cam_zoom += 1.0;
        }
        if is_key_down(KeyCode::Down) {
            cam_zoom -= 1.0;
        }

        set_camera(&Camera2D {
            zoom: vec2(
                1.0 / cam_zoom,
                1.0 / cam_zoom * screen_width() / screen_height(),
            ),
            target: cam_target,
            ..Default::default()
        });

        clear_background(BLACK);

        tissue.update(0.016);
        tissue.draw();

        set_default_camera();
        draw_text("Ferrous Tissue", 10.0, 30.0, 30.0, WHITE);
        draw_text("Piezo-Magnetic Organism", 10.0, 50.0, 20.0, GRAY);

        next_frame().await
    }
}
