//! # Ferrous Swarm
//!
//! **Lineage:** `ferrous-tissue` (Soft Body Physics, Piezo-Magnetism) × `luminous-flock` (Flocking Dynamics).
//!
//! **Concept:**
//! A simulation of "Jellyfish" organisms that are both:
//! 1. Soft Bodies (PBD) driven by Genetic Code (ChimeraVM).
//! 2. Boids (Flocking) driven by Separation, Alignment, and Cohesion forces.
//!
//! The organisms breathe (contract) and change magnetic polarity based on strain,
//! affecting how they cluster and swarm.

use ::rand::Rng;
use chimera_lang::prelude::*;
use flocking::{compute_force, FlockingParams};
use locus::Vec2 as LocusVec2;
use macroquad::prelude::*;
use physics_pbd::{Constraint, PbdSystem};

struct Jellyfish {
    vm: ChimeraVM,
    particle_indices: Vec<usize>, // 0 is Center, 1..N are outer rim
    actuators: Vec<usize>,        // Constraints that are muscles
    magnetism: f32,               // 0.0 (North) to 1.0 (South), 0.5 Neutral
    color: Color,
    flocking_params: FlockingParams,
}

impl Jellyfish {
    fn center_pos(&self, system: &PbdSystem) -> Vec3 {
        system.particles[self.particle_indices[0]].pos
    }

    fn center_vel(&self, system: &PbdSystem) -> Vec3 {
        system.particles[self.particle_indices[0]].vel
    }
}

struct Swarm {
    system: PbdSystem,
    jellyfish: Vec<Jellyfish>,
    width: f32,
    height: f32,
}

impl Swarm {
    fn new(width: f32, height: f32, count: usize) -> Self {
        let mut system = PbdSystem::new();
        let mut jellyfish = Vec::new();

        let dna = create_dna();

        for _ in 0..count {
            let x = ::rand::thread_rng().gen_range(-width / 2.0..width / 2.0);
            let y = ::rand::thread_rng().gen_range(-height / 2.0..height / 2.0);
            let center_pos = vec3(x, y, 0.0);

            let center_idx = system.add_particle(center_pos, 1.0); // Mass 1.0
            let mut particle_indices = vec![center_idx];
            let mut actuators = Vec::new();

            // Create a small jellyfish shape (Triangle or Diamond)
            let radius = 1.5;
            let num_tentacles = 3;
            for i in 0..num_tentacles {
                let angle = (i as f32 / num_tentacles as f32) * std::f32::consts::TAU;
                let offset = vec3(angle.cos() * radius, angle.sin() * radius, 0.0);
                let p_idx = system.add_particle(center_pos + offset, 0.5); // Lighter tentacles
                particle_indices.push(p_idx);

                // Connect to center (Muscle)
                // add_actuator_constraint(p1, p2, min_len, max_len, initial_factor)
                let c_idx = system.constraints.len();
                system.add_actuator_constraint(center_idx, p_idx, radius * 0.5, radius * 1.5, 0.5);
                actuators.push(c_idx);

                // Connect to neighbor tentacle (Structural, to keep shape)
                if i > 0 {
                    let prev = particle_indices[i]; // actually i+1 is current, i is prev in list
                    system.add_distance_constraint(
                        prev,
                        p_idx,
                        (radius * 2.0 * (std::f32::consts::PI / num_tentacles as f32).sin()),
                    );
                }
            }
            // Close loop for rim
            let first = particle_indices[1];
            let last = particle_indices[num_tentacles];
            system.add_distance_constraint(
                first,
                last,
                (radius * 2.0 * (std::f32::consts::PI / num_tentacles as f32).sin()),
            );

            let r = ::rand::thread_rng().gen_range(0.4..0.9);
            let g = ::rand::thread_rng().gen_range(0.4..0.9);
            let b = ::rand::thread_rng().gen_range(0.8..1.0); // Blue-ish base

            jellyfish.push(Jellyfish {
                vm: ChimeraVM::new(dna.clone()),
                particle_indices,
                actuators,
                magnetism: 0.5,
                color: Color::new(r, g, b, 1.0),
                flocking_params: FlockingParams {
                    view_radius: 25.0,
                    separation_radius: 5.0,
                    max_speed: 15.0,
                    max_force: 0.8,
                    separation_weight: 1.5,
                    alignment_weight: 1.0,
                    cohesion_weight: 1.0,
                },
            });
        }

        Swarm {
            system,
            jellyfish,
            width,
            height,
        }
    }

    fn update(&mut self, dt: f32) {
        // 1. Prepare Flocking Data
        let mut positions = Vec::with_capacity(self.jellyfish.len());
        let mut velocities = Vec::with_capacity(self.jellyfish.len());

        for jelly in &self.jellyfish {
            let p = jelly.center_pos(&self.system);
            let v = jelly.center_vel(&self.system);
            positions.push(LocusVec2::new(p.x as f64, p.y as f64));
            velocities.push(LocusVec2::new(v.x as f64, v.y as f64));
        }

        // 2. Compute Forces (Flocking + Magnetism)
        let mut forces = vec![Vec3::ZERO; self.jellyfish.len()];
        let mag_strength = 200.0;

        for (i, jelly) in self.jellyfish.iter().enumerate() {
            // Flocking Force
            let f_force_2d = compute_force(&positions, &velocities, i, &jelly.flocking_params);
            let f_force = vec3(f_force_2d.x as f32, f_force_2d.y as f32, 0.0);
            forces[i] += f_force;

            // Magnetic Force (Inter-Jellyfish)
            // We only check neighbors within view radius to save time? Or simple N^2 for small N.
            // Let's do N^2 for N=50 is fine.
            for (j, other) in self.jellyfish.iter().enumerate() {
                if i == j {
                    continue;
                }
                let p1 = jelly.center_pos(&self.system);
                let p2 = other.center_pos(&self.system);

                let dist_sq = p1.distance_squared(p2).max(1.0);
                if dist_sq > 2500.0 {
                    continue;
                } // Optimization: ignore far away

                let m1 = jelly.magnetism - 0.5;
                let m2 = other.magnetism - 0.5;

                // Force = k * m1 * m2 / r^2
                // Like charges repel (Positive Force pushes away)
                // North(0.0) -> -0.5, South(1.0) -> +0.5
                // (-0.5 * -0.5) = +0.25 (Repel)
                // (-0.5 * +0.5) = -0.25 (Attract)

                let dir = (p1 - p2).normalize_or_zero(); // Direction from other to me
                let mag_force_mag = (m1 * m2 * mag_strength) / dist_sq;

                // If mag_force is positive (Repel), push me away from other (dir)
                forces[i] += dir * mag_force_mag;
            }
        }

        // 3. Apply Forces & Run VM
        // We need to split borrows here because VM needs mutable access to jelly,
        // and applying forces needs mutable access to system.

        // We can iterate indices
        for i in 0..self.jellyfish.len() {
            // Apply Force to Center Particle
            let center_idx = self.jellyfish[i].particle_indices[0];
            let inv_mass = self.system.particles[center_idx].inv_mass;
            if inv_mass > 0.0 {
                self.system.particles[center_idx].vel += forces[i] * dt * inv_mass;
            }

            // Run VM
            let jelly = &mut self.jellyfish[i];

            // Sense: Strain
            let mut total_strain = 0.0;
            for &c_idx in &jelly.actuators {
                if let Constraint::Actuator {
                    p1, p2, max_len, ..
                } = self.system.constraints[c_idx]
                {
                    let pos1 = self.system.particles[p1].pos;
                    let pos2 = self.system.particles[p2].pos;
                    let dist = pos1.distance(pos2);
                    total_strain += dist / max_len;
                }
            }
            let avg_strain = if !jelly.actuators.is_empty() {
                total_strain / jelly.actuators.len() as f32
            } else {
                0.0
            };

            jelly.vm.stack.push(Value::Int((avg_strain * 100.0) as i64));
            jelly
                .vm
                .stack
                .push(Value::Int((jelly.magnetism * 100.0) as i64));

            for _ in 0..50 {
                jelly.vm.step();
            }

            // Act: Magnetism
            if let Some(val_mag) = jelly.vm.stack.pop() {
                jelly.magnetism = match val_mag {
                    Value::Int(n) => (n as f32 / 100.0).clamp(0.0, 1.0),
                    _ => jelly.magnetism,
                };
            }

            // Act: Contraction
            if let Some(val_contract) = jelly.vm.stack.pop() {
                let factor = match val_contract {
                    Value::Int(n) => (n as f32 / 100.0).clamp(0.0, 1.0),
                    _ => 0.5,
                };
                for &c_idx in &jelly.actuators {
                    if let Constraint::Actuator {
                        factor: ref mut f, ..
                    } = &mut self.system.constraints[c_idx]
                    {
                        *f = factor;
                    }
                }
            }

            // Update Color
            // North (Blue) -> South (Red)
            let t = jelly.magnetism;
            jelly.color = Color::new(t, 0.2, 1.0 - t, 1.0);

            jelly.vm.stack.clear();
            jelly.vm.energy = 1000;
            jelly.vm.ip = (0, 0);
        }

        // 4. Step Physics
        self.system.step(dt, 5);

        // 5. Wrap Around World
        for p in &mut self.system.particles {
            if p.pos.x > self.width / 2.0 {
                p.pos.x -= self.width;
            }
            if p.pos.x < -self.width / 2.0 {
                p.pos.x += self.width;
            }
            if p.pos.y > self.height / 2.0 {
                p.pos.y -= self.height;
            }
            if p.pos.y < -self.height / 2.0 {
                p.pos.y += self.height;
            }
        }
    }

    fn draw(&self) {
        for jelly in &self.jellyfish {
            // Draw Tentacles (Muscles)
            for &c_idx in &jelly.actuators {
                if let Constraint::Actuator { p1, p2, factor, .. } = self.system.constraints[c_idx]
                {
                    let pos1 = self.system.particles[p1].pos;
                    let pos2 = self.system.particles[p2].pos;
                    let thickness = 0.05 * factor + 0.05;
                    draw_line(pos1.x, pos1.y, pos2.x, pos2.y, thickness, jelly.color);
                }
            }

            // Draw Body (Center)
            let center = jelly.center_pos(&self.system);
            draw_circle(center.x, center.y, 0.3, jelly.color);

            // Draw Magnetism Indicator
            // Dot: White if magnetic (either pole)
            if (jelly.magnetism - 0.5).abs() > 0.1 {
                draw_circle(center.x, center.y, 0.1, WHITE);
            }
        }
    }
}

fn create_dna() -> Dna {
    // Logic: Piezo-Magnetic Pulsing
    // Inputs: [Strain, SelfMag]
    // Outputs: [Contraction, NewMag]

    let genes = vec![
        // Stack: [Strain, SelfMag]

        // --- Calculate NewMag ---
        Gene {
            op: OpCode::Dup,
            args: vec![],
        }, // [Strain, SelfMag, SelfMag]
        // Mag Drift: NewMag = (SelfMag + 1) % 100
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        },
        Gene {
            op: OpCode::Add,
            args: vec![],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(100)],
        },
        Gene {
            op: OpCode::Mod,
            args: vec![],
        }, // [Strain, SelfMag, NewMag]
        // Arrange Stack for Contraction
        Gene {
            op: OpCode::Swap,
            args: vec![],
        }, // [Strain, NewMag, SelfMag]
        Gene {
            op: OpCode::Drop,
            args: vec![],
        }, // [Strain, NewMag]
        Gene {
            op: OpCode::Swap,
            args: vec![],
        }, // [NewMag, Strain]
        // --- Calculate Contraction ---
        // Stack: [NewMag, Strain]
        // If Strain > 50 (Stretched), Contract (20). Else Relax (100).
        Gene {
            op: OpCode::Dup,
            args: vec![],
        }, // [NewMag, Strain, Strain]
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(50)],
        },
        Gene {
            op: OpCode::Gt,
            args: vec![],
        }, // [NewMag, Strain, IsStretched]
        // Map Bool(0/1) to Factor(100/20)
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(80)],
        },
        Gene {
            op: OpCode::Mul,
            args: vec![],
        }, // [NewMag, Strain, Offset]
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(100)],
        },
        Gene {
            op: OpCode::Swap,
            args: vec![],
        }, // [NewMag, Strain, 100, Offset]
        Gene {
            op: OpCode::Sub,
            args: vec![],
        }, // [NewMag, Strain, Contraction]
        // Cleanup Strain
        Gene {
            op: OpCode::Swap,
            args: vec![],
        }, // [NewMag, Contraction, Strain]
        Gene {
            op: OpCode::Drop,
            args: vec![],
        }, // [NewMag, Contraction]
        // Final Return Order
        Gene {
            op: OpCode::Swap,
            args: vec![],
        }, // [Contraction, NewMag]
    ];

    Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
        evolution_config: None,
    }
}

#[macroquad::main("Ferrous Swarm")]
async fn main() {
    let mut swarm = Swarm::new(80.0, 60.0, 50);

    let mut cam_zoom = 15.0;
    let cam_target = vec2(0.0, 0.0);

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
            target: cam_target,
            ..Default::default()
        });

        clear_background(Color::new(0.05, 0.05, 0.1, 1.0)); // Deep sea blue

        // Draw Grid for reference
        draw_line(-40.0, 0.0, 40.0, 0.0, 0.1, DARKGRAY);
        draw_line(0.0, -30.0, 0.0, 30.0, 0.1, DARKGRAY);

        swarm.update(0.016);
        swarm.draw();

        set_default_camera();
        draw_text("Ferrous Swarm", 10.0, 30.0, 30.0, WHITE);
        draw_text("Magnetic Soft-Body Flocking", 10.0, 50.0, 20.0, SKYBLUE);
        draw_text(
            &format!("Population: {}", swarm.jellyfish.len()),
            10.0,
            70.0,
            20.0,
            GRAY,
        );

        next_frame().await
    }
}
