mod monitor;
mod scavenger;
mod sediment;

use macroquad::prelude::*;
use monitor::ProcessMonitor;
use scavenger::{create_scavenger_dna, MagneticScavenger};
use sediment::SedimentParticle;
use physics_pbd::{Constraint, PbdSystem};
use flocking::{compute_force, FlockingParams};
use locus::Vec2 as LocusVec2;
use chimera_lang::prelude::*;

#[macroquad::main("Magnetic Sediment")]
async fn main() {
    let mut system = PbdSystem::new();
    let mut monitor = ProcessMonitor::new();
    let mut scavengers: Vec<MagneticScavenger> = Vec::new();
    let mut sediment: Vec<SedimentParticle> = Vec::new();

    let width = screen_width();
    let height = screen_height();
    let dna = create_scavenger_dna();

    // Spawn Initial Scavengers
    for _ in 0..20 {
        let pos = vec2(rand::gen_range(0.0, width), rand::gen_range(0.0, height));
        scavengers.push(MagneticScavenger::new(&mut system, pos, dna.clone()));
    }

    loop {
        let dt = 0.016;

        clear_background(Color::new(0.05, 0.05, 0.08, 1.0)); // Deep dark blue

        // 1. Monitor Processes & Spawn Sediment
        let new_sediment = monitor.update();
        sediment.extend(new_sediment);

        // Limit sediment to prevent lag
        if sediment.len() > 200 {
            sediment.remove(0);
        }

        // Draw Monitor (Living Processes)
        monitor.draw();

        // Draw Sediment
        for p in &sediment {
            draw_circle(p.pos.x, p.pos.y, p.mass.clamp(2.0, 8.0), p.color);
        }

        // 2. Prepare Flocking Data
        let mut positions = Vec::with_capacity(scavengers.len());
        let mut velocities = Vec::with_capacity(scavengers.len());

        for scavenger in &scavengers {
            let p = scavenger.center_pos(&system);
            let v = scavenger.center_vel(&system);
            positions.push(LocusVec2::new(p.x as f64, p.y as f64));
            velocities.push(LocusVec2::new(v.x as f64, v.y as f64));
        }

        // 3. Compute Forces
        let mut forces = vec![Vec3::ZERO; scavengers.len()];
        let mag_strength = 200.0;
        let sediment_attraction = 500.0;

        for (i, scavenger) in scavengers.iter().enumerate() {
            // Flocking
            let f_force_2d = compute_force(&positions, &velocities, i, &scavenger.flocking_params);
            forces[i] += vec3(f_force_2d.x as f32, f_force_2d.y as f32, 0.0);

            let my_pos = scavenger.center_pos(&system);
            let my_mag = scavenger.magnetism - 0.5; // -0.5 to 0.5

            // Magnetic Repulsion/Attraction with other Scavengers
            for (j, other) in scavengers.iter().enumerate() {
                 if i == j { continue; }
                 let other_pos = other.center_pos(&system);
                 let dist_sq = my_pos.distance_squared(other_pos).max(1.0);
                 if dist_sq > 2500.0 { continue; } // Optimization

                 let other_mag = other.magnetism - 0.5;

                 // Like poles repel, Opposites attract?
                 // Force = k * m1 * m2 / r^2
                 // If m1, m2 have same sign, product is positive. If positive force means repel:
                 let force_mag = (my_mag * other_mag * mag_strength) / dist_sq;
                 let dir = (my_pos - other_pos).normalize_or_zero();
                 forces[i] += dir * force_mag;
            }

            // Attraction to Sediment
            // Scavengers are attracted to sediment regardless of polarity (ferromagnetic attraction)
            for s in &sediment {
                let s_pos = vec3(s.pos.x, s.pos.y, 0.0);
                let dist_sq = my_pos.distance_squared(s_pos).max(1.0);
                if dist_sq < 10000.0 {
                     // Attraction force proportional to sediment mass and scavenger magnetism magnitude
                     let attraction = (s.magnetic_charge * my_mag.abs() * sediment_attraction) / dist_sq;
                     let dir = (s_pos - my_pos).normalize_or_zero();
                     forces[i] += dir * attraction;
                }
            }
        }

        // 4. Update Scavengers & VM
        let mut eaten_indices = Vec::new(); // indices of sediment to remove

        for (i, scavenger) in scavengers.iter_mut().enumerate() {
             // Apply Force
             let center_idx = scavenger.particle_indices[0];
             let inv_mass = system.particles[center_idx].inv_mass;
             if inv_mass > 0.0 {
                 system.particles[center_idx].vel += forces[i] * dt * inv_mass;
             }

             // VM & Physics Logic (same as ferrous-swarm)
            // Sense: Strain
            let mut total_strain = 0.0;
            for &c_idx in &scavenger.actuators {
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
            let avg_strain = if !scavenger.actuators.is_empty() {
                total_strain / scavenger.actuators.len() as f32
            } else {
                0.0
            };

            scavenger.vm.stack.push(chimera_lang::vm::Value::Int((avg_strain * 100.0) as i64));
            scavenger.vm.stack.push(chimera_lang::vm::Value::Int((scavenger.magnetism * 100.0) as i64));

            for _ in 0..50 {
                scavenger.vm.step();
            }

            // Act: Magnetism
            if let Some(val_mag) = scavenger.vm.stack.pop() {
                scavenger.magnetism = match val_mag {
                    chimera_lang::vm::Value::Int(n) => (n as f32 / 100.0).clamp(0.0, 1.0),
                    _ => scavenger.magnetism,
                };
            }

            // Act: Contraction
            if let Some(val_contract) = scavenger.vm.stack.pop() {
                let factor = match val_contract {
                    chimera_lang::vm::Value::Int(n) => (n as f32 / 100.0).clamp(0.0, 1.0),
                    _ => 0.5,
                };
                for &c_idx in &scavenger.actuators {
                    if let Constraint::Actuator {
                        factor: ref mut f, ..
                    } = &mut system.constraints[c_idx]
                    {
                        *f = factor;
                    }
                }
            }

            // Update Color based on magnetism
            let t = scavenger.magnetism;
            scavenger.color = Color::new(t, 0.2, 1.0 - t, 1.0);

            scavenger.vm.stack.clear();
            scavenger.vm.energy = 1000;
            scavenger.vm.ip = (0, 0);

            // Eat Sediment
            let my_pos = system.particles[center_idx].pos;
            for (s_idx, s) in sediment.iter().enumerate() {
                let s_pos = vec3(s.pos.x, s.pos.y, 0.0);
                if my_pos.distance(s_pos) < 10.0 { // Eat radius
                     if !eaten_indices.contains(&s_idx) {
                         eaten_indices.push(s_idx);
                         scavenger.energy += s.mass * 10.0;
                     }
                }
            }
        }

        // Remove eaten sediment
        // Sort descending to remove efficiently
        eaten_indices.sort_by(|a, b| b.cmp(a));
        eaten_indices.dedup(); // just in case
        for idx in eaten_indices {
            if idx < sediment.len() {
                sediment.remove(idx);
            }
        }

        // 5. Step Physics
        system.step(dt, 5);

        // 6. Draw Scavengers
        for scavenger in &scavengers {
             // Draw Tentacles
            for &c_idx in &scavenger.actuators {
                if let Constraint::Actuator { p1, p2, factor, .. } = system.constraints[c_idx]
                {
                    let pos1 = system.particles[p1].pos;
                    let pos2 = system.particles[p2].pos;
                    let thickness = 0.05 * factor + 0.05;
                    draw_line(pos1.x, pos1.y, pos2.x, pos2.y, thickness, scavenger.color);
                }
            }
            // Draw Body
            let center = scavenger.center_pos(&system);
            draw_circle(center.x, center.y, 2.0, scavenger.color);
        }

        // Wrap Around
        for p in &mut system.particles {
            if p.pos.x > width { p.pos.x -= width; }
            if p.pos.x < 0.0 { p.pos.x += width; }
            if p.pos.y > height { p.pos.y -= height; }
            if p.pos.y < 0.0 { p.pos.y += height; }
        }

        draw_text("Magnetic Sediment", 10.0, 30.0, 30.0, WHITE);
        draw_text(&format!("Processes: {}", monitor.known_pids.len()), 10.0, 50.0, 20.0, GREEN);
        draw_text(&format!("Sediment: {}", sediment.len()), 10.0, 70.0, 20.0, GRAY);
        draw_text(&format!("Scavengers: {}", scavengers.len()), 10.0, 90.0, 20.0, RED);

        next_frame().await
    }
}
