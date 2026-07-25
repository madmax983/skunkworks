use ferrous_core::Platter;
use macroquad::prelude::*;

mod audio;
mod git;
mod particle;
mod string;

use audio::{init_audio, AudioCommand};
use particle::Particle;
use string::FerrousString;

const STRING_COUNT: usize = 8;
const STRING_SPACING: f32 = 100.0;
const BASE_FREQ: f32 = 110.0; // A2

#[macroquad::main("Git Strings")]
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
        let target_freq = BASE_FREQ * (2.0f32).powf(i as f32 / 12.0);
        strings.push(FerrousString::new(pos, 400.0, target_freq));
    }

    let commits = match git::get_commit_history() {
        Ok(c) if !c.is_empty() => c,
        _ => vec![git::Commit {
            hash: "000000".to_string(),
            author: "System".to_string(),
            message: "No history".to_string(),
        }],
    };

    let mut commit_index = 0;
    let mut spawn_timer = 0.0;
    let spawn_interval = 0.5;

    // Texture for Platter Heatmap
    let texture =
        Texture2D::from_image(&Image::gen_image_color(grid_w as u16, grid_h as u16, BLACK));
    texture.set_filter(FilterMode::Nearest);

    loop {
        let dt = get_frame_time();

        // --- Spawn Commits ---
        spawn_timer += dt;
        if spawn_timer >= spawn_interval {
            spawn_timer = 0.0;
            if commit_index < commits.len() {
                let c = commits[commit_index].clone();

                // Commits start at random heights, moving right
                let y = rand::gen_range(50.0, 550.0);
                let mut p = Particle::new(0.0, y, c);
                // initial push based on hash
                p.vel.x = 100.0 + (commit_index as f32 % 50.0);
                particles.push(p);
                commit_index += 1;
            } else {
                commit_index = 0; // loop
            }
        }

        // --- Physics Update ---

        // 1. Decay Platter
        platter.decay(0.99);

        // 2. Update Strings
        for s in &mut strings {
            s.update_physics(dt, &platter, grid_scale);

            if s.vibration.abs() > 0.1 {
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
                        let val = (s.vibration * 0.01 * shape) as f64;
                        platter.magnetize(gx as usize, gy as usize, val);
                    }
                }
            }
        }

        // 3. Update Particles & Check Collisions with Strings
        for p in &mut particles {
            let prev_x = p.pos.x;
            p.update(&platter, grid_scale, dt);
            let next_x = p.pos.x;

            for s in &mut strings {
                let string_x = s.pos.x + s.vibration;
                let crossed = (prev_x < string_x && next_x >= string_x)
                    || (prev_x > string_x && next_x <= string_x);
                let in_range = p.pos.y >= s.pos.y && p.pos.y <= s.pos.y + s.length;

                if crossed && in_range {
                    let strength = p.vel.x.clamp(5.0, 50.0);
                    let direction = if p.vel.x > 0.0 { 1.0 } else { -1.0 };
                    s.pluck(strength * direction);

                    // We perturb the particle back based on string tension
                    p.vel.x -= strength * direction * 0.5;

                    let _ = cmd_tx.send(AudioCommand {
                        frequency: s.frequency,
                        decay: s.decay,
                        amplitude: (strength / 50.0).clamp(0.1, 0.8),
                    });
                }
            }
        }

        particles.retain(|p| p.active);

        // --- Render ---
        clear_background(BLACK);

        let mut image = texture.get_texture_data();
        for y in 0..grid_h {
            for x in 0..grid_w {
                let mag = platter.get_magnetism(x, y);

                let val = mag as f32;
                let c = if val < 0.0 {
                    Color::new(0.0, 0.0, (-val).clamp(0.0, 1.0), 1.0)
                } else {
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

        for p in &particles {
            p.draw();
        }

        for s in &strings {
            s.draw();
        }

        // UI
        draw_text("Git Strings", 10.0, 30.0, 30.0, WHITE);
        draw_text(
            "Commit History -> Kinetic Agents -> Acoustic Strings",
            10.0,
            50.0,
            20.0,
            GRAY,
        );

        next_frame().await;
    }
}
