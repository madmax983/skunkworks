//! # Ferrous Choreography
//!
//! **Lineage:** `ferrous-swarm` (Magnetic Physics) × `chimera-choreography` (Laban Effort).
//!
//! **Concept:**
//! Magnetic particles ("Dancers") whose physical properties (Magnetism, Mass, Friction, Steering)
//! are driven by Laban Effort parameters derived from their ChimeraVM DNA.
//!
//! **Laban Mapping:**
//! - **Weight (Strong/Light):** Controls Magnetism strength and Mass.
//!   - Strong: High Magnetism, Heavy (High Inertia).
//!   - Light: Low Magnetism, Light (Low Inertia).
//! - **Time (Sudden/Sustained):** Controls Movement Impulse frequency.
//!   - Sudden: Rare but powerful bursts of force.
//!   - Sustained: Constant, gentle force.
//! - **Space (Direct/Indirect):** Controls Alignment/Targeting.
//!   - Direct: Moves straight towards target/center.
//!   - Indirect: Adds noise/wander to trajectory.
//! - **Flow (Bound/Free):** Controls Damping/Friction.
//!   - Bound: High damping (movement stops quickly).
//!   - Free: Low damping (movement glides).

mod laban;

use ::rand::Rng;
use chimera_lang::prelude::*;
use laban::LabanEffort;
use locus::Vec3;
use macroquad::prelude::*;
use physics_pbd::PbdSystem;

struct MagneticDancer {
    vm: ChimeraVM,
    particle_idx: usize,
    effort: LabanEffort,
    magnetism: f32, // -1.0 (North) to 1.0 (South)
    color: Color,
    impulse_timer: f32,
}

struct Stage {
    system: PbdSystem,
    dancers: Vec<MagneticDancer>,
    width: f32,
    height: f32,
}

impl Stage {
    fn new(width: f32, height: f32, count: usize) -> Self {
        let mut system = PbdSystem::new();
        let mut dancers = Vec::new();

        let dna = create_dna(); // Common DNA for now, evolves later

        for _ in 0..count {
            let x = ::rand::thread_rng().gen_range(-width / 2.0..width / 2.0);
            let y = ::rand::thread_rng().gen_range(-height / 2.0..height / 2.0);
            let pos = vec3(x, y, 0.0);

            // Default Mass 1.0, modified by Weight later
            let idx = system.add_particle(pos, 1.0);

            let mut vm = ChimeraVM::new(dna.clone());
            // Seed grid with random Laban values [0..100]
            for i in 0..4 {
                vm.grid[0][i] = Value::Int(::rand::thread_rng().gen_range(0..100));
            }

            dancers.push(MagneticDancer {
                vm,
                particle_idx: idx,
                effort: LabanEffort::new(),
                magnetism: 0.0,
                color: WHITE,
                impulse_timer: 0.0,
            });
        }

        Stage {
            system,
            dancers,
            width,
            height,
        }
    }

    fn update(&mut self, dt: f32) {
        // 1. Run VM & Update Laban Effort
        for dancer in &mut self.dancers {
            // Step VM
            dancer.vm.energy = 100; // Unlimited energy for dance
            if !dancer.vm.halted {
                dancer.vm.step();
            }

            // Read Laban from Grid
            let get_val = |vm: &ChimeraVM, idx: usize| -> f32 {
                match &vm.grid[0][idx] {
                    Value::Int(n) => (*n as f32 / 100.0).clamp(0.0, 1.0),
                    _ => 0.5,
                }
            };

            dancer.effort.weight = get_val(&dancer.vm, 0);
            dancer.effort.time = get_val(&dancer.vm, 1);
            dancer.effort.space = get_val(&dancer.vm, 2);
            dancer.effort.flow = get_val(&dancer.vm, 3);

            // Update Physical Properties based on Laban
            // Weight -> Mass & Magnetism
            // Strong (0.0) -> Heavy (Mass 5.0), High Mag (1.0)
            // Light (1.0) -> Light (Mass 0.5), Low Mag (0.1)
            let w = dancer.effort.weight;
            let mass = 5.0 - (w * 4.5);
            let mag_strength = 1.0 - (w * 0.9);

            // Update Particle Mass
            if mass > 0.0 {
                self.system.particles[dancer.particle_idx].inv_mass = 1.0 / mass;
            }

            // Update Magnetism (Oscillate based on ID/Time for interest, modulated by strength)
            let time = get_time() as f32;
            let base_mag = (time + dancer.particle_idx as f32).sin();
            dancer.magnetism = base_mag * mag_strength;

            // Color: Red (North) to Blue (South), Brightness by Flow
            let t = (dancer.magnetism + 1.0) / 2.0; // 0..1
            let alpha = 0.5 + dancer.effort.flow * 0.5;
            dancer.color = Color::new(t, 0.2, 1.0 - t, alpha);
        }

        // 2. Calculate Forces
        let mut forces = vec![Vec3::new(0.0, 0.0, 0.0); self.system.particles.len()];
        let mag_constant = 1000.0;

        for i in 0..self.dancers.len() {
            let p1 = self.system.particles[self.dancers[i].particle_idx].pos;
            let m1 = self.dancers[i].magnetism;
            let effort = self.dancers[i].effort;

            // A. Magnetic Force (N^2 but simple)
            for j in 0..self.dancers.len() {
                if i == j {
                    continue;
                }
                let p2 = self.system.particles[self.dancers[j].particle_idx].pos;
                let m2 = self.dancers[j].magnetism;

                let diff = p1 - p2;
                let dist_sq = diff.length_squared().max(1.0);
                if dist_sq > 2500.0 {
                    continue;
                } // View radius

                // Force = k * m1 * m2 / r^2
                // Like charges repel (+), Opposite attract (-)
                let force_mag = (mag_constant * m1 * m2) / dist_sq;
                let dir = diff.normalize_or_zero();

                let force = dir * force_mag;
                let p_idx = self.dancers[i].particle_idx;
                forces[p_idx].x += force.x;
                forces[p_idx].y += force.y;
                forces[p_idx].z += force.z;
            }

            // B. Laban Steering (Self-Propulsion)
            // Time: Sudden vs Sustained
            self.dancers[i].impulse_timer += dt;
            let impulse_interval = if effort.is_sudden() { 2.0 } else { 0.1 };

            if self.dancers[i].impulse_timer > impulse_interval {
                self.dancers[i].impulse_timer = 0.0;

                // Space: Direct vs Indirect
                let mut target_dir = -p1.normalize_or_zero(); // Center seeking
                if effort.is_indirect() {
                    // Add noise
                    let noise = vec3(
                        ::rand::thread_rng().gen_range(-1.0..1.0),
                        ::rand::thread_rng().gen_range(-1.0..1.0),
                        0.0,
                    );
                    target_dir = (target_dir + noise).normalize_or_zero();
                }

                // Force magnitude based on Sudden (Burst) vs Sustained (Gentle)
                let strength = if effort.is_sudden() { 500.0 } else { 50.0 };
                let force = target_dir * strength;
                let p_idx = self.dancers[i].particle_idx;
                forces[p_idx].x += force.x;
                forces[p_idx].y += force.y;
                forces[p_idx].z += force.z;
            }

            // Flow: Damping applied directly to velocity later
        }

        // 3. Apply Forces & Damping
        for (i, dancer) in self.dancers.iter().enumerate() {
            let p_idx = dancer.particle_idx;
            let inv_mass = self.system.particles[p_idx].inv_mass;

            if inv_mass > 0.0 {
                let f = forces[p_idx];
                let f_vec = vec3(f.x, f.y, f.z);
                self.system.particles[p_idx].vel += f_vec * dt * inv_mass;
            }

            // Flow: Bound (High Friction) vs Free (Low Friction)
            let damping = if dancer.effort.is_bound() {
                0.90
            } else {
                0.99
            };
            self.system.particles[p_idx].vel *= damping;

            // Bounds check
            let p = &mut self.system.particles[p_idx];
            let margin = 5.0;
            if p.pos.x > self.width / 2.0 - margin {
                p.vel.x -= 10.0;
            }
            if p.pos.x < -self.width / 2.0 + margin {
                p.vel.x += 10.0;
            }
            if p.pos.y > self.height / 2.0 - margin {
                p.vel.y -= 10.0;
            }
            if p.pos.y < -self.height / 2.0 + margin {
                p.vel.y += 10.0;
            }
        }

        // 4. Physics Step
        self.system.step(dt, 2);
    }

    fn draw(&self) {
        for dancer in &self.dancers {
            let pos = self.system.particles[dancer.particle_idx].pos;
            // Size based on Mass (Weight)
            let radius = (1.0 / self.system.particles[dancer.particle_idx].inv_mass).sqrt() * 0.5;

            // Draw Body
            draw_circle(pos.x, pos.y, radius, dancer.color);

            // Draw Orientation/Velocity
            let vel = self.system.particles[dancer.particle_idx].vel;
            if vel.length_squared() > 1.0 {
                let end = pos + vel.normalize() * radius * 2.0;
                draw_line(pos.x, pos.y, end.x, end.y, 0.1, WHITE);
            }
        }
    }
}

fn create_dna() -> Dna {
    // Evolve Laban Parameters randomly over time
    let mut genes = Vec::new();
    // Random Walk on Grid[0..3]
    for _ in 0..16 {
        // Pick random index 0..3
        genes.push(Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(::rand::thread_rng().gen_range(0..4))],
        });
        // Pick random delta -10..10
        genes.push(Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(::rand::thread_rng().gen_range(0..21))],
        });
        genes.push(Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(10)],
        });
        genes.push(Gene {
            op: OpCode::Sub,
            args: vec![],
        });
        // Read current value
        genes.push(Gene {
            op: OpCode::Dup,
            args: vec![],
        }); // [Idx, Delta, Delta] (Oops, need Swap/Dup logic, simplifying)
            // Just write random value 0..100
    }

    // Simplified DNA: Write Random Value to Random Laban Param
    let mut genes = Vec::new();
    for _ in 0..10 {
        genes.push(Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(::rand::thread_rng().gen_range(0..4))],
        }); // Index
        genes.push(Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        }); // Y
        genes.push(Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(::rand::thread_rng().gen_range(0..100))],
        }); // Value
        genes.push(Gene {
            op: OpCode::GWrite,
            args: vec![],
        });
    }

    Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
        evolution_config: None,
    }
}

#[macroquad::main("Ferrous Choreography")]
async fn main() {
    let mut stage = Stage::new(80.0, 60.0, 100);

    let mut cam_zoom = 15.0;

    loop {
        if is_key_down(KeyCode::Up) {
            cam_zoom += 0.5;
        }
        if is_key_down(KeyCode::Down) {
            cam_zoom -= 0.5;
        }

        set_camera(&Camera2D {
            zoom: vec2(
                1.0 / cam_zoom,
                1.0 / cam_zoom * screen_width() / screen_height(),
            ),
            target: vec2(0.0, 0.0),
            ..Default::default()
        });

        clear_background(Color::new(0.02, 0.02, 0.05, 1.0));

        // Draw Stage
        draw_rectangle_lines(-40.0, -30.0, 80.0, 60.0, 0.2, DARKGRAY);

        stage.update(0.016);
        stage.draw();

        set_default_camera();
        draw_text("Ferrous Choreography", 10.0, 30.0, 30.0, WHITE);
        draw_text("Laban-Driven Magnetic Physics", 10.0, 50.0, 20.0, LIGHTGRAY);

        next_frame().await
    }
}
