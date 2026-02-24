use ::rand::Rng;
use chimera_lang::prelude::*;
use macroquad::prelude::*;
use physics_pbd::{Constraint, PbdSystem};
use quipu::{Cord, Knot};

const SEGMENT_LENGTH: f32 = 0.5;
const KNOT_MASS_SCALE: f32 = 0.5;

struct QuipuKnot {
    particle_idx: usize,
    vm: ChimeraVM,
    #[allow(dead_code)]
    knot_type: Knot,
    original_value: u8,
    color: Color,
}

struct QuipuWorld {
    physics: PbdSystem,
    knots: Vec<QuipuKnot>,
    cords: Vec<Vec<usize>>, // Lists of particle indices forming cords
    main_cord: Vec<usize>,  // The horizontal main cord
}

impl QuipuWorld {
    fn new() -> Self {
        let mut physics = PbdSystem::new();
        let knots = Vec::new();
        let cords = Vec::new();
        let mut main_cord = Vec::new();

        // 1. Create Main Cord (Horizontal Anchor)
        // Spans from x=-10 to x=10 at y=10
        let main_cord_segments = 20;
        let start_pos = vec3(-10.0, 10.0, 0.0);

        for i in 0..=main_cord_segments {
            let x = start_pos.x + i as f32;
            let pos = vec3(x, start_pos.y, 0.0);
            // Main cord is static (infinite mass) to hold the weight
            let idx = physics.add_particle(pos, 0.0);
            main_cord.push(idx);

            if i > 0 {
                let prev = main_cord[i - 1];
                physics.add_distance_constraint(prev, idx, 1.0); // Stiff main cord
            }
        }

        let mut world = QuipuWorld {
            physics,
            knots,
            cords,
            main_cord,
        };

        // 2. Add Pendant Cords
        // Generate some random numbers to represent
        let mut rng = ::rand::thread_rng();
        for i in 0..15 {
            let val = rng.gen_range(10..999);
            let cord_data = Cord::from(val);

            // Attach to random position on main cord
            // Use indices 1 to len-1 to avoid ends? No, just distribute.
            // Map i to main cord index.
            let attachment_idx = (i * main_cord_segments / 15) as usize;
            let anchor_particle = world.main_cord[attachment_idx];

            world.add_pendant_cord(anchor_particle, cord_data);
        }

        world
    }

    fn add_pendant_cord(&mut self, anchor_idx: usize, data: Cord) {
        let mut cord_particles = Vec::new();
        let mut prev_idx = anchor_idx;

        // Starting position just below anchor
        let mut current_pos = self.physics.particles[anchor_idx].pos;
        current_pos.y -= SEGMENT_LENGTH;

        // Iterate through clusters (Thousands -> Units)
        // Quipu Cord clusters are stored Units first (index 0).
        // But physically, they hang Top-Down (Thousands -> Units).
        // So we iterate in reverse order.

        for (_cluster_idx, cluster) in data.clusters.iter().enumerate().rev() {
            // Gap between clusters
            let gap_idx = self.physics.add_particle(current_pos, 0.1); // Light string
            self.physics.add_distance_constraint(prev_idx, gap_idx, 0.8); // Loose gap
            cord_particles.push(gap_idx);
            prev_idx = gap_idx;
            current_pos.y -= SEGMENT_LENGTH;

            if cluster.is_empty() {
                continue;
            }

            // Add Knots in this cluster
            for knot in cluster {
                let mass = 1.0 + (knot.value() as f32 * KNOT_MASS_SCALE);
                let p_idx = self.physics.add_particle(current_pos, mass);

                // Add Actuator Constraint (Muscle) to previous particle
                // This allows the knot to "climb" or "contract" the cord
                self.physics.add_actuator_constraint(
                    prev_idx,
                    p_idx,
                    0.1,
                    SEGMENT_LENGTH * 1.5,
                    0.5,
                );

                // Create VM for this knot
                let vm = ChimeraVM::new(create_knot_dna(knot.value()));

                let color = match knot {
                    Knot::Simple => RED,
                    Knot::Long(_) => GREEN,
                    Knot::FigureEight => BLUE,
                };

                self.knots.push(QuipuKnot {
                    particle_idx: p_idx,
                    vm,
                    knot_type: *knot,
                    original_value: knot.value(),
                    color,
                });

                cord_particles.push(p_idx);
                prev_idx = p_idx;
                current_pos.y -= SEGMENT_LENGTH;
            }
        }

        self.cords.push(cord_particles);
    }

    fn update(&mut self, dt: f32) {
        // Step Physics
        // Gravity
        for p in &mut self.physics.particles {
            if p.inv_mass > 0.0 {
                p.vel.y -= 9.8 * dt; // Gravity down
            }
        }
        self.physics.step(dt, 5);

        // Update VMs
        // Split borrows
        let physics = &mut self.physics;
        let knots = &mut self.knots;

        for knot in knots {
            let p_idx = knot.particle_idx;
            let particle = physics.particles[p_idx];

            // 1. Sense
            // Input: Velocity Y (Falling/Rising), Tension (implied by distance to prev?)
            // We need to find the constraint connecting to above.
            // Simplified: Input = Velocity Y * 10.0
            let input_vel = (particle.vel.y * 10.0) as i64;
            knot.vm.stack.push(Value::Int(input_vel));

            // 2. Think
            for _ in 0..20 {
                knot.vm.step();
            }

            // 3. Act
            // Output 1: Contraction (0-100)
            if let Some(val) = knot.vm.stack.pop() {
                let contract_factor = match val {
                    Value::Int(n) => (n as f32 / 100.0).clamp(0.0, 1.0),
                    _ => 0.5,
                };

                // Apply to relevant actuator(s)
                // Find constraints attached to this particle
                for c in &mut physics.constraints {
                    if let Constraint::Actuator { p2, factor, .. } = c {
                        if *p2 == p_idx {
                            // If this is the bottom particle of the segment
                            *factor = contract_factor;
                        }
                    }
                }

                // Visual feedback: Color based on contraction
                knot.color.a = 0.5 + (contract_factor * 0.5);
            }

            // Reset
            knot.vm.stack.clear();
            knot.vm.energy = 100; // Refuel
            knot.vm.ip = (0, 0);
        }
    }

    fn draw(&self) {
        // Draw Main Cord
        for i in 0..self.main_cord.len() - 1 {
            let p1 = self.physics.particles[self.main_cord[i]].pos;
            let p2 = self.physics.particles[self.main_cord[i + 1]].pos;
            draw_line(p1.x, p1.y, p2.x, p2.y, 0.1, BEIGE);
        }

        // Draw Pendant Cords (Constraints)
        for c in &self.physics.constraints {
            match c {
                Constraint::Distance { p1, p2, .. } | Constraint::Actuator { p1, p2, .. } => {
                    let pos1 = self.physics.particles[*p1].pos;
                    let pos2 = self.physics.particles[*p2].pos;
                    draw_line(pos1.x, pos1.y, pos2.x, pos2.y, 0.05, BROWN);
                }
                _ => {}
            }
        }

        // Draw Knots
        for knot in &self.knots {
            let pos = self.physics.particles[knot.particle_idx].pos;
            let radius = 0.1 + (knot.original_value as f32 * 0.02);
            draw_circle(pos.x, pos.y, radius, knot.color);

            // Draw Value
            // draw_text(&format!("{}", knot.original_value), pos.x + 0.2, pos.y, 0.02, WHITE);
        }
    }
}

fn create_knot_dna(_value: u8) -> Dna {
    // DNA that reacts to velocity (gravity)
    // If falling (negative velocity), contract (pull up).
    // If rising, relax.
    // Tries to stabilize against gravity.

    let genes = vec![
        // Stack: [Velocity]
        Gene {
            op: OpCode::Dup,
            args: vec![],
        }, // [Vel, Vel]
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        }, // [Vel, Vel, 0]
        Gene {
            op: OpCode::Lt,
            args: vec![],
        }, // [Vel, Vel < 0 ? 1 : 0]
        // If falling (< 0), push 0 (Contract). Else push 100 (Relax).
        // Wait, Actuator factor 0.0 = min_len (Contracted). 1.0 = max_len (Relaxed).
        // If falling, we want to contract to pull up? Or relax to fall?
        // Let's say we want to fight gravity: Contract (0.0).

        // Logic: If Vel < 0 (Falling), Result is 1.
        // We want Factor 0.0.
        // If Vel >= 0 (Rising/Stable), Result is 0.
        // We want Factor 1.0 (100).

        // Map 1 -> 0, 0 -> 100
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        }, // [Vel, IsFalling, 1]
        Gene {
            op: OpCode::Swap,
            args: vec![],
        }, // [Vel, 1, IsFalling]
        Gene {
            op: OpCode::Sub,
            args: vec![],
        }, // [Vel, 1 - IsFalling] (0 if falling, 1 if stable)
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(100)],
        }, // [Vel, Res, 100]
        Gene {
            op: OpCode::Mul,
            args: vec![],
        }, // [Vel, Res * 100]
    ];

    Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
        evolution_config: None,
    }
}

#[macroquad::main("Quipu Tissue")]
async fn main() {
    let mut world = QuipuWorld::new();

    // Camera
    let mut cam_zoom = 20.0;
    let mut cam_target = vec2(0.0, 5.0);

    loop {
        if is_key_down(KeyCode::Up) {
            cam_zoom += 0.5;
        }
        if is_key_down(KeyCode::Down) {
            cam_zoom -= 0.5;
        }
        if is_key_down(KeyCode::Left) {
            cam_target.x -= 0.5;
        }
        if is_key_down(KeyCode::Right) {
            cam_target.x += 0.5;
        }
        if is_key_down(KeyCode::W) {
            cam_target.y += 0.5;
        }
        if is_key_down(KeyCode::S) {
            cam_target.y -= 0.5;
        }

        // Reset
        if is_key_pressed(KeyCode::R) {
            world = QuipuWorld::new();
        }

        set_camera(&Camera2D {
            zoom: vec2(
                1.0 / cam_zoom,
                1.0 / cam_zoom * screen_width() / screen_height(),
            ),
            target: cam_target,
            ..Default::default()
        });

        clear_background(Color::new(0.1, 0.1, 0.1, 1.0));

        // Physics Step
        world.update(0.016);

        // Draw
        world.draw();

        set_default_camera();
        draw_text(
            "Quipu Tissue: Biological Data Structure",
            10.0,
            30.0,
            30.0,
            WHITE,
        );
        draw_text(
            &format!("Knots: {}", world.knots.len()),
            10.0,
            50.0,
            20.0,
            GRAY,
        );
        draw_text(
            "Controls: Arrows/WASD to move, R to reset",
            10.0,
            70.0,
            20.0,
            GRAY,
        );

        next_frame().await
    }
}
