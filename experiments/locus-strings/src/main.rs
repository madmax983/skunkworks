#![allow(
    clippy::too_many_lines,
    clippy::future_not_send,
    clippy::expect_used,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation
)]
use locus::flocking::{compute_force, FlockingParams};
use locus::Vec2 as LocusVec2;
use macroquad::prelude::*;

use crate::audio::{init_audio, AudioCommand};
use crate::string::FerrousString;

mod audio;
mod string;

pub struct Boid {
    pub position: LocusVec2,
    pub velocity: LocusVec2,
}

const STRING_COUNT: usize = 8;
const BOID_COUNT: usize = 150;
const STRING_SPACING: f32 = 100.0;
const BASE_FREQ: f32 = 110.0; // A2

#[macroquad::main("Locus Strings")]

async fn main() {
    let (audio_handle, cmd_tx) = init_audio().expect("Failed to init audio");
    let _audio_handle = audio_handle;

    let mut strings: Vec<FerrousString> = Vec::new();
    let mut boids: Vec<Boid> = Vec::new();

    // Initialize strings
    for i in 0..STRING_COUNT {
        #[allow(clippy::cast_precision_loss)]
        let x = (i as f32).mul_add(STRING_SPACING, 100.0);
        let pos = vec2(x, 200.0);
        #[allow(clippy::cast_precision_loss)]
        let target_freq = BASE_FREQ * (i as f32 / 12.0).exp2();
        strings.push(FerrousString::new(pos, 300.0, target_freq));
    }

    // Initialize Boids
    for _ in 0..BOID_COUNT {
        let pos = LocusVec2::new(
            rand::gen_range(0.0, f64::from(screen_width())),
            rand::gen_range(0.0, f64::from(screen_height())),
        );
        let angle = rand::gen_range(0.0, std::f64::consts::PI * 2.0);
        let vel = LocusVec2::new(angle.cos(), angle.sin()) * 2.0;
        boids.push(Boid {
            position: pos,
            velocity: vel,
        });
    }

    let params = FlockingParams {
        view_radius: 50.0,
        separation_radius: 20.0,
        max_speed: 4.0,
        max_force: 0.1,
        separation_weight: 1.5,
        alignment_weight: 1.0,
        cohesion_weight: 1.0,
    };

    loop {
        let dt = get_frame_time();

        let width = f64::from(screen_width());
        let height = f64::from(screen_height());

        // 1. Update Flock
        let mut positions = Vec::with_capacity(BOID_COUNT);
        let mut velocities = Vec::with_capacity(BOID_COUNT);
        for b in &boids {
            positions.push(b.position);
            velocities.push(b.velocity);
        }

        let mut forces = Vec::with_capacity(BOID_COUNT);
        for i in 0..BOID_COUNT {
            forces.push(compute_force(&positions, &velocities, i, &params));
        }

        for (i, boid) in boids.iter_mut().enumerate() {
            boid.velocity += forces[i];

            // Limit speed
            let speed = boid.velocity.magnitude();
            if speed > params.max_speed {
                boid.velocity = (boid.velocity / speed) * params.max_speed;
            }

            boid.position += boid.velocity;

            // Wrap around screen
            if boid.position.x < 0.0 {
                boid.position.x += width;
            } else if boid.position.x >= width {
                boid.position.x -= width;
            }

            if boid.position.y < 0.0 {
                boid.position.y += height;
            } else if boid.position.y >= height {
                boid.position.y -= height;
            }
        }

        // 2. Update Strings and Interactions
        for s in &mut strings {
            s.update_physics(dt);

            let string_x = s.pos.x + s.vibration;
            let mut pluck_force: f32 = 0.0;

            for boid in &mut boids {
                // Check if boid crossed string
                let boid_pos = boid.position;
                let boid_vel = boid.velocity;
                let boid_prev = boid_pos - boid_vel * f64::from(dt);

                let crossed = (boid_prev.x < f64::from(string_x)
                    && boid_pos.x >= f64::from(string_x))
                    || (boid_prev.x > f64::from(string_x) && boid_pos.x <= f64::from(string_x));
                let in_range =
                    boid_pos.y >= f64::from(s.pos.y) && boid_pos.y <= f64::from(s.pos.y + s.length);

                if crossed && in_range {
                    // Pluck!
                    #[allow(clippy::cast_possible_truncation)]
                    let boid_speed = boid_vel.magnitude() as f32;
                    let strength = boid_speed.clamp(2.0, 10.0) * 2.0;
                    let direction = if boid_vel.x > 0.0 { 1.0 } else { -1.0 };
                    pluck_force += strength * direction;

                    // Play Audio
                    let _ = cmd_tx.send(AudioCommand::Pluck {
                        frequency: s.frequency,
                        decay: s.decay,
                        amplitude: (strength / 20.0).clamp(0.1, 0.8),
                    });
                }

                // Apply Magnetic force from string vibration to boids
                let dist_to_string = (boid_pos.x - f64::from(string_x)).abs();
                if dist_to_string < 150.0 && in_range {
                    let force_mag = f64::from(s.vibration) * 0.1 / dist_to_string.mul_add(0.1, 1.0);
                    // Magnetic force repels or attracts based on vibration polarity
                    let force_dir = LocusVec2::new(
                        if boid_pos.x > f64::from(string_x) {
                            1.0
                        } else {
                            -1.0
                        },
                        0.0,
                    );
                    boid.velocity += force_dir * force_mag * f64::from(dt) * 50.0;
                }
            }

            if pluck_force.abs() > 0.0 {
                s.pluck(pluck_force);
            }
        }

        // --- Render ---
        clear_background(BLACK);

        for s in &strings {
            s.draw();
        }

        for boid in &boids {
            let color = if boid.velocity.magnitude() > 3.0 {
                RED
            } else {
                WHITE
            };
            #[allow(clippy::cast_possible_truncation)]
            draw_circle(boid.position.x as f32, boid.position.y as f32, 2.0, color);
        }

        draw_text("Locus Strings", 10.0, 30.0, 30.0, WHITE);
        draw_text(
            "Boids swarm and pluck resonant strings",
            10.0,
            50.0,
            20.0,
            GRAY,
        );
        draw_text(
            "String vibrations create magnetic fields",
            10.0,
            70.0,
            20.0,
            GRAY,
        );
        draw_text("that physically perturb the flock", 10.0, 90.0, 20.0, GRAY);

        next_frame().await;
    }
}
