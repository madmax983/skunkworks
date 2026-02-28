use ferrous_core::Platter;
use macroquad::prelude::*;

mod audio;
mod particle;
mod string;

use audio::{init_audio, AudioCommand};
use particle::Particle;
use string::FerrousString;

const STRING_COUNT: usize = 8;
const PARTICLE_COUNT: usize = 500;
const STRING_SPACING: f32 = 100.0;
const BASE_FREQ: f32 = 110.0; // A2

#[macroquad::main("Ferrous Strings")]
async fn main() {
    let (audio_handle, cmd_tx) = init_audio().expect("Failed to init audio");
    // Keep handle alive
    let _audio_handle = audio_handle;

    let grid_w = 200;
    let grid_h = 150;
    let mut platter = Platter::new(grid_w, grid_h);
    let grid_scale = 4.0;

    let mut strings: Vec<FerrousString> = Vec::new();
    let mut particles: Vec<Particle> = Vec::new();

    // Initialize strings
    for i in 0..STRING_COUNT {
        let x = 100.0 + i as f32 * STRING_SPACING;
        let pos = vec2(x, 100.0);
        // Start with random frequencies around a scale (target is just a guideline for mutation range)
        let target_freq = BASE_FREQ * (2.0f32).powf(i as f32 / 12.0);
        strings.push(FerrousString::new(pos, 400.0, target_freq));
    }

    // Initialize Particles
    for _ in 0..PARTICLE_COUNT {
        let x = rand::gen_range(0.0, screen_width());
        let y = rand::gen_range(0.0, screen_height());
        particles.push(Particle::new(x, y));
    }

    let mut evolution_timer = 0.0;
    let evolution_interval = 2.0; // Evolve every 2s

    let mut prev_mouse = vec2(0.0, 0.0);

    // Texture for Platter Heatmap
    let texture =
        Texture2D::from_image(&Image::gen_image_color(grid_w as u16, grid_h as u16, BLACK));
    texture.set_filter(FilterMode::Nearest);

    loop {
        let dt = get_frame_time();
        evolution_timer += dt;

        let (mx, my) = mouse_position();
        let mouse_pos = vec2(mx, my);
        let mouse_delta = mouse_pos - prev_mouse;
        let mouse_speed = mouse_delta.length();

        // --- Physics Update ---

        // 1. Decay Platter
        platter.decay(0.99); // Slow decay to let magnetism build up

        // 2. Update Strings (Magnetize Platter)
        for s in &mut strings {
            // Apply magnetic forces from platter to string
            s.update_physics(dt, &platter, grid_scale);

            // Apply string vibration to platter
            // This is the bidirectional coupling!
            // String writes to Platter
            // Platter reads from String (in update_physics)
            // But we do it in steps.

            // To make it interesting, let's say the string ONLY writes to platter if vibrating
            if s.vibration.abs() > 0.1 {
                // Determine grid cells under the string
                // For simplicity, just a few points along the string
                let steps = 20;
                let step_size = s.length / steps as f32;

                for i in 0..=steps {
                    let y_offset = i as f32 * step_size;
                    let ratio = y_offset / s.length;
                    let shape = (std::f32::consts::PI * ratio).sin();
                    let x = s.pos.x + s.vibration * shape;
                    let y = s.pos.y + y_offset;

                    let gx = (x / grid_scale) as i32;
                    let gy = (y / grid_scale) as i32;

                    if gx >= 0 && gy >= 0 && gx < grid_w as i32 && gy < grid_h as i32 {
                        // Polarity flips based on vibration direction
                        let val = (s.vibration * 0.01 * shape) as f64;
                        platter.magnetize(gx as usize, gy as usize, val);
                    }
                }
            }

            // Mouse Interaction (Plucking)
            let string_x = s.pos.x + s.vibration; // Approximate
                                                  // Check cross
            let crossed = (prev_mouse.x < string_x && mouse_pos.x >= string_x)
                || (prev_mouse.x > string_x && mouse_pos.x <= string_x);
            let in_range = mouse_pos.y >= s.pos.y && mouse_pos.y <= s.pos.y + s.length;

            if crossed && in_range {
                let strength = mouse_speed.clamp(5.0, 50.0);
                let direction = if mouse_delta.x > 0.0 { 1.0 } else { -1.0 };
                s.pluck(strength * direction);

                // Play Audio
                let _ = cmd_tx.send(AudioCommand::Pluck {
                    frequency: s.frequency,
                    decay: s.decay,
                    amplitude: (strength / 50.0).clamp(0.1, 0.8),
                });
            }
        }

        // 3. Update Particles
        for p in &mut particles {
            p.update(&platter, grid_scale, dt);
        }

        // --- Evolution ---
        if evolution_timer > evolution_interval {
            evolution_timer = 0.0;

            // Calculate Fitness: How many particles are near the string?
            // Strings that attract particles (via magnetism) are successful.
            for s in &mut strings {
                let mut count = 0;
                let x_min = s.pos.x - 20.0;
                let x_max = s.pos.x + 20.0;
                let y_min = s.pos.y;
                let y_max = s.pos.y + s.length;

                for p in &particles {
                    if p.pos.x >= x_min && p.pos.x <= x_max && p.pos.y >= y_min && p.pos.y <= y_max
                    {
                        count += 1;
                    }
                }

                // Normalize fitness (max 50 particles expected?)
                s.fitness = (count as f32 / 50.0).clamp(0.0, 1.0);
            }

            // Apply Evolution
            for (i, s) in strings.iter_mut().enumerate() {
                let target_freq = BASE_FREQ * (2.0f32).powf(i as f32 / 12.0);
                s.evolve(target_freq);
            }
        }

        // --- Render ---
        clear_background(BLACK);

        // 1. Draw Platter (Heatmap)
        let mut image = texture.get_texture_data();
        for y in 0..grid_h {
            for x in 0..grid_w {
                let mag = platter.get_magnetism(x, y); // 0.0 to 1.0

                let val = mag as f32;
                let c = if val < 0.0 {
                    // Blue for negative
                    Color::new(0.0, 0.0, (-val).clamp(0.0, 1.0), 1.0)
                } else {
                    // Red for positive
                    Color::new(val.clamp(0.0, 1.0), 0.0, 0.0, 1.0)
                };

                image.set_pixel(x as u32, y as u32, c);
            }
        }
        texture.update(&image);

        draw_texture_ex(
            &texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(grid_w as f32 * grid_scale, grid_h as f32 * grid_scale)),
                ..Default::default()
            },
        );

        // 2. Draw Particles
        for p in &particles {
            p.draw();
        }

        // 3. Draw Strings
        for s in &strings {
            s.draw();
        }

        // UI
        draw_text("Ferrous Strings", 10.0, 30.0, 30.0, WHITE);
        draw_text(
            "Strings vibrate -> Magnetic Field -> Particles Flow",
            10.0,
            50.0,
            20.0,
            GRAY,
        );
        draw_text(
            "Particles -> String Fitness -> Evolution",
            10.0,
            70.0,
            20.0,
            GRAY,
        );
        draw_text("Pluck with Mouse!", 10.0, 90.0, 20.0, YELLOW);

        prev_mouse = mouse_pos;
        next_frame().await;
    }
}
