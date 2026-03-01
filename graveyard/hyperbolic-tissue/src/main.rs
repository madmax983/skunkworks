use chimera_lang::prelude::*;
use macroquad::prelude::*;
use num_complex::Complex;

mod physics;
use physics::{hyperbolic_distance, PbdSystem, Point};

const TISSUE_SIZE: usize = 6;
const INITIAL_SCALE: f32 = 0.1;

struct Cell {
    vm: ChimeraVM,
    particle_idx: usize,
    actuators: Vec<usize>, // Indices into system.constraints
    color: Color,
}

struct Tissue {
    system: PbdSystem,
    cells: Vec<Cell>,
}

impl Tissue {
    fn new() -> Self {
        let mut system = PbdSystem::new();
        let mut cells = Vec::new();
        let mut particle_indices = Vec::new();

        // Create Particles in a small grid near origin
        for y in 0..TISSUE_SIZE {
            for x in 0..TISSUE_SIZE {
                // Map grid to -0.5 to 0.5 range, then scale
                let fx = (x as f32 / (TISSUE_SIZE as f32 - 1.0)) - 0.5;
                let fy = (y as f32 / (TISSUE_SIZE as f32 - 1.0)) - 0.5;

                let pos = Complex::new(fx * INITIAL_SCALE, fy * INITIAL_SCALE);
                // Mass 1.0
                let idx = system.add_point(pos, 1.0);
                particle_indices.push(idx);
            }
        }

        // Constraints
        let mut cell_actuators = vec![Vec::new(); TISSUE_SIZE * TISSUE_SIZE];

        let add_actuator = |p1: usize, p2: usize, sys: &mut PbdSystem| -> usize {
            // Initial distance
            let dist = hyperbolic_distance(sys.points[p1], sys.points[p2]);
            // Allow expansion up to 5x, contraction to 0.5x
            // Stiffness 0.1 for soft tissue
            sys.add_actuator_constraint(p1, p2, dist * 0.5, dist * 5.0, 0.1)
        };

        for y in 0..TISSUE_SIZE {
            for x in 0..TISSUE_SIZE {
                let idx = y * TISSUE_SIZE + x;
                let p1 = particle_indices[idx];

                // Right
                if x + 1 < TISSUE_SIZE {
                    let p2 = particle_indices[y * TISSUE_SIZE + (x + 1)];
                    let c_idx = add_actuator(p1, p2, &mut system);
                    cell_actuators[idx].push(c_idx);
                    cell_actuators[y * TISSUE_SIZE + (x + 1)].push(c_idx);
                }

                // Bottom
                if y + 1 < TISSUE_SIZE {
                    let p2 = particle_indices[(y + 1) * TISSUE_SIZE + x];
                    let c_idx = add_actuator(p1, p2, &mut system);
                    cell_actuators[idx].push(c_idx);
                    cell_actuators[(y + 1) * TISSUE_SIZE + x].push(c_idx);
                }

                // Diagonals (Structural only, no actuators)
                if x + 1 < TISSUE_SIZE && y + 1 < TISSUE_SIZE {
                    let p2 = particle_indices[(y + 1) * TISSUE_SIZE + (x + 1)];
                    let dist = hyperbolic_distance(system.points[p1], system.points[p2]);
                    system.add_distance_constraint(p1, p2, dist, 0.1);

                    let p3 = particle_indices[y * TISSUE_SIZE + (x + 1)];
                    let p4 = particle_indices[(y + 1) * TISSUE_SIZE + x];
                    let dist2 = hyperbolic_distance(system.points[p3], system.points[p4]);
                    system.add_distance_constraint(p3, p4, dist2, 0.1);
                }
            }
        }

        // Cells
        let dna = create_dna();
        for (i, actuators) in cell_actuators.into_iter().enumerate() {
            let vm = ChimeraVM::new(dna.clone());
            // Random color
            let r = macroquad::rand::gen_range(0.5, 1.0);
            let g = macroquad::rand::gen_range(0.3, 0.8);
            let b = macroquad::rand::gen_range(0.5, 1.0);

            cells.push(Cell {
                vm,
                particle_idx: particle_indices[i],
                actuators,
                color: Color::new(r, g, b, 0.8),
            });
        }

        // Pin center
        let center_idx = (TISSUE_SIZE / 2) * TISSUE_SIZE + (TISSUE_SIZE / 2);
        system.add_pin_constraint(
            particle_indices[center_idx],
            system.points[particle_indices[center_idx]],
        );

        Tissue { system, cells }
    }

    fn update(&mut self) {
        // Physics
        self.system.step(0.016, 5);

        // Biology
        let system = &mut self.system;
        let cells = &mut self.cells;

        for cell in cells {
            // 1. Sense
            let mut total_strain = 0.0;
            let mut count = 0;

            for &c_idx in &cell.actuators {
                if let physics::Constraint::Actuator {
                    p1,
                    p2,
                    min_dist,
                    max_dist,
                    factor,
                    ..
                } = system.constraints[c_idx]
                {
                    let curr = hyperbolic_distance(system.points[p1], system.points[p2]);
                    let target = min_dist + (max_dist - min_dist) * factor;
                    let strain = curr / target; // > 1 stretched
                    total_strain += strain;
                    count += 1;
                }
            }

            let avg_strain = if count > 0 {
                total_strain / count as f32
            } else {
                1.0
            };

            // Sense Radius (Distance from origin)
            let radius =
                hyperbolic_distance(system.points[cell.particle_idx], Complex::new(0.0, 0.0));

            // Push inputs
            // Stack: [Radius, Strain]
            cell.vm.stack.push(Value::Int((radius * 10.0) as i64));
            cell.vm.stack.push(Value::Int((avg_strain * 100.0) as i64));

            // 2. Think
            for _ in 0..50 {
                cell.vm.step();
            }

            // 3. Act
            if let Some(val) = cell.vm.stack.pop() {
                let out = match val {
                    Value::Int(n) => (n as f32 / 100.0).clamp(0.0, 1.0),
                    _ => 0.5,
                };

                for &c_idx in &cell.actuators {
                    if let physics::Constraint::Actuator { factor, .. } =
                        &mut system.constraints[c_idx]
                    {
                        *factor = out;
                    }
                }

                // Color update
                cell.color.a = 0.5 + out * 0.5;
            }

            // Reset
            cell.vm.stack.clear();
            cell.vm.energy = 1000;
            cell.vm.ip = (0, 0);
        }
    }

    fn draw(&self) {
        // Draw Boundary
        draw_circle_lines(0.0, 0.0, 1.0, 0.01, WHITE);

        // Draw Constraints (Geodesics)
        for constraint in &self.system.constraints {
            let (p1, p2, color) = match constraint {
                physics::Constraint::Actuator { p1, p2, factor, .. } => {
                    let c = Color::new(1.0 - factor, *factor, 0.2, 0.8);
                    (p1, p2, c)
                }
                physics::Constraint::Distance { p1, p2, .. } => {
                    (p1, p2, Color::new(0.5, 0.5, 0.5, 0.3))
                }
                _ => continue,
            };

            let u = self.system.points[*p1];
            let v = self.system.points[*p2];
            draw_hyperbolic_segment(u, v, color);
        }

        // Draw Cells
        for cell in &self.cells {
            let pos = self.system.points[cell.particle_idx];
            // Radius in screen space shouldn't be constant if we want to visualize perspective
            // But for now constant is fine
            draw_circle(pos.re, pos.im, 0.02, cell.color);
        }
    }
}

fn draw_hyperbolic_segment(u: Point, v: Point, color: Color) {
    // Sample geodesic using Mobius transform
    // Map u to origin
    let one = Complex::new(1.0, 0.0);
    let den = one - u.conj() * v;
    if den.norm() < 1e-6 {
        return;
    }

    let v_prime = (v - u) / den;
    let dist = 2.0 * v_prime.norm().atanh();

    // If very close, just line
    if dist < 0.01 {
        draw_line(u.re, u.im, v.re, v.im, 0.005, color);
        return;
    }

    let steps = 10;
    let mut last_p = u;

    for i in 1..=steps {
        let t = i as f32 / steps as f32;
        // Interpolate in transformed space (line from 0 to v')
        // Distance d(t) = t * dist
        // |z| = tanh(d(t)/2)
        let d_t = t * dist;
        let mag = (d_t / 2.0).tanh();
        let p_prime = if v_prime.norm() > 1e-6 {
            (v_prime / v_prime.norm()) * mag
        } else {
            Complex::new(0.0, 0.0)
        };

        // Map back
        let num = p_prime + u;
        let den = one + u.conj() * p_prime;
        let p = num / den;

        draw_line(last_p.re, last_p.im, p.re, p.im, 0.005, color);
        last_p = p;
    }
}

fn create_dna() -> Dna {
    // Logic: Factor = (1 - (Radius > 8)) * ((Strain <= 50) ? 1.0 : 0.0)
    // Avoids Rot.
    // 1. [Radius, Strain]
    // 2. Swap -> [Strain, Radius]
    // 3. Dup -> [Strain, Radius, Radius]
    // 4. Push(8) -> [Strain, Radius, 8]
    // 5. Gt -> [Strain, Radius, IsEdge]
    // 6. Push(1), Swap, Sub -> [Strain, Radius, Mask]
    // 7. Swap, Drop -> [Strain, Mask]
    // 8. Swap -> [Mask, Strain]
    // 9. Push(50) -> [Mask, Strain, 50]
    // 10. Gt -> [Mask, IsStretched]
    // 11. Push(1), Swap, Sub -> [Mask, RelaxFlag]
    // 12. Push(100), Mul -> [Mask, Factor]
    // 13. Mul -> [Result]

    let genes = vec![
        Gene {
            op: OpCode::Swap,
            args: vec![],
        },
        Gene {
            op: OpCode::Dup,
            args: vec![],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(8)],
        },
        Gene {
            op: OpCode::Gt,
            args: vec![],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        },
        Gene {
            op: OpCode::Swap,
            args: vec![],
        },
        Gene {
            op: OpCode::Sub,
            args: vec![],
        },
        Gene {
            op: OpCode::Swap,
            args: vec![],
        },
        Gene {
            op: OpCode::Drop,
            args: vec![],
        },
        Gene {
            op: OpCode::Swap,
            args: vec![],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(50)],
        },
        Gene {
            op: OpCode::Gt,
            args: vec![],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        },
        Gene {
            op: OpCode::Swap,
            args: vec![],
        },
        Gene {
            op: OpCode::Sub,
            args: vec![],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(100)],
        },
        Gene {
            op: OpCode::Mul,
            args: vec![],
        },
        Gene {
            op: OpCode::Mul,
            args: vec![],
        },
    ];

    Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
        evolution_config: None,
    }
}

#[macroquad::main("Hyperbolic Tissue")]
async fn main() {
    let mut tissue = Tissue::new();

    loop {
        clear_background(BLACK);

        let aspect = screen_width() / screen_height();
        let scale = 1.0 / 1.2;
        set_camera(&Camera2D {
            zoom: vec2(scale / aspect, scale),
            target: vec2(0.0, 0.0),
            ..Default::default()
        });

        tissue.update();
        tissue.draw();

        set_default_camera();
        draw_text("Hyperbolic Tissue", 10.0, 30.0, 30.0, WHITE);
        draw_text("Poincare Disk Model", 10.0, 50.0, 20.0, GRAY);

        next_frame().await
    }
}
