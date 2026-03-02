use ferrous_core::Platter;
use macroquad::prelude::*;

mod audio;
mod particle;
mod physics;
mod string;

use audio::{init_audio, AudioCommand};
use particle::Particle;
use physics::PendulumSystem;
use string::FerrousString;

const STRING_COUNT: usize = 8;
const PARTICLE_COUNT: usize = 500;
const STRING_SPACING: f32 = 100.0;
const BASE_FREQ: f32 = 110.0; // A2

#[macroquad::main("Chaos Strings")]
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

    // Initialize Particles
    for _ in 0..PARTICLE_COUNT {
        let x = rand::gen_range(0.0, screen_width());
        let y = rand::gen_range(0.0, screen_height());
        particles.push(Particle::new(x, y));
    }

    // Initialize Pendulum System
    let mut pendulum = PendulumSystem::new();
    pendulum.gravity = vec2(0.0, 400.0); // Stronger gravity for chaotic motion
    pendulum.friction = 0.999;

    // Add nodes for a double pendulum
    let cx = screen_width() / 2.0;
    pendulum.nodes.push(physics::Node {
        pos: vec2(cx, 50.0),
        prev_pos: vec2(cx, 50.0),
        mass: 1.0,
        fixed: true,
        name: "Root".to_string(),
    });
    pendulum.nodes.push(physics::Node {
        pos: vec2(cx + 100.0, 50.0),
        prev_pos: vec2(cx + 100.0, 50.0),
        mass: 2.0,
        fixed: false,
        name: "Joint 1".to_string(),
    });
    pendulum.nodes.push(physics::Node {
        pos: vec2(cx + 200.0, 50.0),
        prev_pos: vec2(cx + 200.0, 50.0),
        mass: 1.5,
        fixed: false,
        name: "Joint 2".to_string(),
    });
    pendulum.nodes.push(physics::Node {
        pos: vec2(cx + 300.0, 50.0),
        prev_pos: vec2(cx + 300.0, 50.0),
        mass: 1.0,
        fixed: false,
        name: "Stylus".to_string(),
    });

    pendulum.add_link(0, 1, 150.0);
    pendulum.add_link(1, 2, 150.0);
    pendulum.add_link(2, 3, 100.0);

    let texture =
        Texture2D::from_image(&Image::gen_image_color(grid_w as u16, grid_h as u16, BLACK));
    texture.set_filter(FilterMode::Nearest);

    loop {
        let dt = get_frame_time();

        // --- Physics Update ---

        // 1. Decay Platter
        platter.decay(0.99);

        // 2. Update Pendulum
        let prev_stylus_pos = pendulum.nodes[3].pos;
        pendulum.step(dt);
        let stylus_pos = pendulum.nodes[3].pos;

        let stylus_delta = stylus_pos - prev_stylus_pos;
        let stylus_speed = stylus_delta.length() / dt.max(0.001);

        // Magnetic Feedback: Platter -> Pendulum
        for node in &mut pendulum.nodes {
            if node.fixed {
                continue;
            }
            let gx = (node.pos.x / grid_scale) as i32;
            let gy = (node.pos.y / grid_scale) as i32;

            if gx >= 0 && gy >= 0 && gx < grid_w as i32 && gy < grid_h as i32 {
                let mag = platter.get_magnetism(gx as usize, gy as usize) as f32;
                // Add magnetic force to node (modify prev_pos for verlet integration to simulate force)
                // Magnetic field pulls or pushes the node
                let force = vec2(0.0, mag * 50.0 * dt);
                node.prev_pos -= force;
            }
        }

        // 3. Update Strings
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
                        let val = (s.vibration * 0.05 * shape) as f64;
                        platter.magnetize(gx as usize, gy as usize, val);
                    }
                }
            }

            // Pendulum Stylus Plucking
            let string_x = s.pos.x + s.vibration;
            let crossed = (prev_stylus_pos.x < string_x && stylus_pos.x >= string_x)
                || (prev_stylus_pos.x > string_x && stylus_pos.x <= string_x);
            let in_range = stylus_pos.y >= s.pos.y && stylus_pos.y <= s.pos.y + s.length;

            if crossed && in_range {
                let strength = (stylus_speed * 0.1).clamp(5.0, 50.0);
                let direction = if stylus_delta.x > 0.0 { 1.0 } else { -1.0 };
                s.pluck(strength * direction);

                let _ = cmd_tx.send(AudioCommand::Pluck {
                    frequency: s.frequency,
                    decay: s.decay,
                    amplitude: (strength / 50.0).clamp(0.1, 0.8),
                });
            }
        }

        // 4. Update Particles
        for p in &mut particles {
            p.update(&platter, grid_scale, dt);
        }

        // Interaction (Mouse Interaction to push pendulum)
        if is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let mouse_pos = vec2(mx, my);

            for node in &mut pendulum.nodes {
                if !node.fixed {
                    if node.pos.distance(mouse_pos) < 50.0 {
                        node.pos = mouse_pos;
                        node.prev_pos = mouse_pos;
                    }
                }
            }
        }

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

        // Draw Pendulum Links
        for link in &pendulum.links {
            let pos_a = pendulum.nodes[link.a].pos;
            let pos_b = pendulum.nodes[link.b].pos;
            draw_line(pos_a.x, pos_a.y, pos_b.x, pos_b.y, 4.0, LIGHTGRAY);
        }

        // Draw Pendulum Nodes
        for node in &pendulum.nodes {
            let color = if node.fixed { RED } else { BLUE };
            let size = (node.mass * 4.0).clamp(5.0, 20.0);
            draw_circle(node.pos.x, node.pos.y, size, color);
        }

        // UI
        draw_text("Chaos Strings", 10.0, 30.0, 30.0, WHITE);
        draw_text("Chaotic Pendulum plucks strings", 10.0, 50.0, 20.0, GRAY);
        draw_text(
            "Strings magnetize Platter -> Drag on Pendulum",
            10.0,
            70.0,
            20.0,
            GRAY,
        );
        draw_text("Drag pendulum nodes with Mouse", 10.0, 90.0, 20.0, YELLOW);

        next_frame().await;
    }
}
