use ferrous_core::Platter;
use macroquad::prelude::*;

mod audio;
mod string;
mod graph;
mod glitch;

use audio::{init_audio, AudioCommand};
use string::FerrousString;
use graph::Graph;
use glitch::TextGlitcher;

const STRING_COUNT: usize = 12;
const STRING_SPACING: f32 = 80.0;
const BASE_FREQ: f32 = 110.0; // A2

#[macroquad::main("Mnem Strings")]
async fn main() {
    let (audio_handle, cmd_tx) = init_audio().expect("Failed to init audio");
    // Keep handle alive
    let _audio_handle = audio_handle;

    let grid_w = 200;
    let grid_h = 150;
    let mut platter = Platter::new(grid_w, grid_h);
    let grid_scale = 4.0;

    let mut strings: Vec<FerrousString> = Vec::new();
    let mut graph = Graph::new();

    println!("Scanning directory...");
    graph.scan_directory("experiments/chimera-lang/src");
    if graph.nodes.is_empty() {
        println!("Scanning current directory...");
        graph.scan_directory(".");
    }

    // Initialize strings
    for i in 0..STRING_COUNT {
        let x = 100.0 + i as f32 * STRING_SPACING;
        let pos = vec2(x, 100.0);
        let target_freq = BASE_FREQ * (2.0f32).powf(i as f32 / 12.0);
        strings.push(FerrousString::new(pos, 400.0, target_freq));
    }

    let mut entropy = 0.0;
    let mut hovered_node: Option<usize>;

    // Camera
    let mut offset = vec2(0.0, 0.0);
    let mut zoom = 1.0;
    let mut dragging = false;
    let mut last_mouse = vec2(0.0, 0.0);

    let texture =
        Texture2D::from_image(&Image::gen_image_color(grid_w as u16, grid_h as u16, BLACK));
    texture.set_filter(FilterMode::Nearest);

    loop {
        let dt = get_frame_time();

        let mouse_pos = mouse_position();
        let mouse_vec = vec2(mouse_pos.0, mouse_pos.1);
        let world_mouse = (mouse_vec - offset) / zoom;

        // --- Camera ---
        if is_mouse_button_pressed(MouseButton::Right) {
            dragging = true;
            last_mouse = mouse_vec;
        }
        if is_mouse_button_released(MouseButton::Right) {
            dragging = false;
        }
        if dragging {
            let delta = mouse_vec - last_mouse;
            offset += delta;
            last_mouse = mouse_vec;
        }
        let wheel = mouse_wheel();
        if wheel.1 != 0.0 {
            zoom *= if wheel.1 > 0.0 { 1.1 } else { 0.9 };
        }

        // --- Logic: Decay ---
        entropy += 0.0001; // Slow global rot

        // --- Physics & Graph Forces ---
        let mut forces = vec![vec2(0.0, 0.0); graph.nodes.len()];

        // Save previous positions for string intersection
        let prev_positions: Vec<Vec2> = graph.nodes.iter().map(|n| n.pos).collect();

        // Repulsion
        for i in 0..graph.nodes.len() {
            for j in i + 1..graph.nodes.len() {
                let diff = graph.nodes[i].pos - graph.nodes[j].pos;
                let dist_sq = diff.length_squared().max(1.0);
                if dist_sq < 250000.0 {
                    let force = diff.normalize() * (5000.0 / dist_sq);
                    forces[i] += force;
                    forces[j] -= force;
                }
            }
            // Center attraction (Gravity)
            let center = vec2(screen_width() / 2.0, screen_height() / 2.0) - offset;
            let to_center = (center - graph.nodes[i].pos) * 0.01;
            forces[i] += to_center;
        }

        // Spring forces
        for edge in &graph.edges {
            if edge.from < graph.nodes.len() && edge.to < graph.nodes.len() {
                let n1 = graph.nodes[edge.from].pos;
                let n2 = graph.nodes[edge.to].pos;
                let diff = n2 - n1;
                let dist = diff.length();
                let desired = 100.0;
                let force = diff.normalize() * (dist - desired) * 0.05 * edge.strength;
                forces[edge.from] += force;
                forces[edge.to] -= force;
            }
        }

        // 1. Decay Platter
        platter.decay(0.99);

        // 2. Update Strings (Magnetize Platter)
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

        hovered_node = None;
        // 3. Apply Forces, Magnetic Field & Decay to Nodes
        for (i, node) in graph.nodes.iter_mut().enumerate() {
            // Apply magnetic field to node velocity
            let gx = (node.pos.x / grid_scale) as usize;
            let gy = (node.pos.y / grid_scale) as usize;
            if gx < grid_w && gy < grid_h {
                let mag = platter.get_magnetism(gx, gy) as f32;
                // High entropy (low health) nodes are repelled by magnetism
                // Healthy nodes are drawn to it
                let mag_force_mag = mag * 1000.0 * (0.5 - node.health);
                // We'll push them horizontally based on magnetism
                forces[i] += vec2(mag_force_mag, 0.0);
            }

            node.vel += forces[i] * dt;
            node.vel *= 0.90; // Damping

            // Jitter for decaying nodes
            if node.health < 0.5 {
                node.vel += vec2(
                    rand::gen_range(-5.0, 5.0) * (1.0 - node.health),
                    rand::gen_range(-5.0, 5.0) * (1.0 - node.health)
                );
            }

            node.pos += node.vel;

            // Decay
            node.health -= entropy * dt * 0.05;
            if node.health < 0.1 {
                node.health = 0.1;
            }

            // Interaction
            if node.pos.distance(world_mouse) < 20.0 {
                hovered_node = Some(i);
                // Heal
                node.health += dt * 5.0; // Fast heal
                if node.health > 1.0 {
                    node.health = 1.0;
                }
            }
        }

        // 4. Node <-> String Collisions
        for (i, node) in graph.nodes.iter().enumerate() {
            let prev_pos = prev_positions[i];
            let curr_pos = node.pos;
            let vel_mag = node.vel.length();

            for s in &mut strings {
                let string_x = s.pos.x + s.vibration;
                let crossed = (prev_pos.x < string_x && curr_pos.x >= string_x)
                    || (prev_pos.x > string_x && curr_pos.x <= string_x);
                let in_range = curr_pos.y >= s.pos.y && curr_pos.y <= s.pos.y + s.length;

                if crossed && in_range {
                    // Decay drives pluck strength. A rotting node hits violently.
                    let strength = (vel_mag * (1.5 - node.health)).clamp(5.0, 50.0);
                    let direction = if node.vel.x > 0.0 { 1.0 } else { -1.0 };
                    s.pluck(strength * direction);

                    // A rotting node's impact is harsher (more amplitude)
                    let amplitude = (strength / 50.0).clamp(0.1, 0.8) * (1.2 - node.health);

                    let _ = cmd_tx.send(AudioCommand::Pluck {
                        frequency: s.frequency,
                        decay: s.decay,
                        amplitude: amplitude.clamp(0.1, 1.0),
                    });
                }
            }
        }

        // --- Render ---
        clear_background(BLACK);

        let mut image = texture.get_texture_data();
        for y in 0..grid_h {
            for x in 0..grid_w {
                let mag = platter.get_magnetism(x, y); // 0.0 to 1.0
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

        // Draw Platter (offset by camera)
        draw_texture_ex(
            &texture,
            offset.x,
            offset.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(grid_w as f32 * grid_scale * zoom, grid_h as f32 * grid_scale * zoom)),
                ..Default::default()
            },
        );

        // Draw Edges
        for edge in &graph.edges {
            if edge.from < graph.nodes.len() && edge.to < graph.nodes.len() {
                let n1 = graph.nodes[edge.from].pos * zoom + offset;
                let n2 = graph.nodes[edge.to].pos * zoom + offset;
                let avg_health = (graph.nodes[edge.from].health + graph.nodes[edge.to].health) / 2.0;

                let jitter = if avg_health < 0.5 {
                    vec2(rand::gen_range(-2.0, 2.0), rand::gen_range(-2.0, 2.0))
                } else {
                    vec2(0.0, 0.0)
                };

                draw_line(
                    n1.x + jitter.x,
                    n1.y + jitter.y,
                    n2.x + jitter.x,
                    n2.y + jitter.y,
                    2.0 * zoom,
                    Color::new(0.5, 0.5, 0.5, avg_health),
                );
            }
        }

        // Draw Nodes
        for (i, node) in graph.nodes.iter().enumerate() {
            let pos = node.pos * zoom + offset;
            let color = if node.health > 0.8 {
                GREEN
            } else if node.health > 0.4 {
                YELLOW
            } else {
                RED
            };

            let radius = 5.0 * zoom * (0.5 + node.health);
            draw_circle(pos.x, pos.y, radius, color);

            if node.health > 0.6 || hovered_node == Some(i) {
                draw_text(&node.name, pos.x + 10.0, pos.y, 14.0 * zoom, WHITE);
            }
        }

        // Draw Strings
        for s in &strings {
            let start = vec2(s.pos.x, s.pos.y) * zoom + offset;
            let mut prev_pt = start;
            let segments = 20;
            let step = s.length / segments as f32;

            for i in 1..=segments {
                let ratio = (i as f32) / (segments as f32);
                let x_offset = s.vibration * (std::f32::consts::PI * ratio).sin();
                let pt = vec2(s.pos.x + x_offset, s.pos.y + (i as f32 * step)) * zoom + offset;

                // Color based on vibration
                let intensity = (s.vibration.abs() / 20.0).clamp(0.0, 1.0);
                let color = Color::new(1.0, 1.0 - intensity, 1.0 - intensity, 1.0);

                draw_line(prev_pt.x, prev_pt.y, pt.x, pt.y, 2.0 * zoom, color);
                prev_pt = pt;
            }
        }

        // UI Overlay
        if let Some(idx) = hovered_node {
            let node = &graph.nodes[idx];
            draw_rectangle(
                10.0,
                10.0,
                400.0,
                screen_height() - 20.0,
                Color::new(0.05, 0.05, 0.05, 0.95),
            );
            draw_rectangle_lines(10.0, 10.0, 400.0, screen_height() - 20.0, 2.0, WHITE);

            draw_text(&node.name, 20.0, 40.0, 30.0, GREEN);
            draw_text(
                &format!("Health: {:.0}%", node.health * 100.0),
                20.0,
                70.0,
                20.0,
                WHITE,
            );

            let intensity = 1.0 - node.health;
            let corrupted = TextGlitcher::corrupt(&node.content, intensity);

            let lines: Vec<&str> = corrupted.lines().take(35).collect();
            for (j, line) in lines.iter().enumerate() {
                let display_line: String = line.chars().take(50).collect();
                draw_text(&display_line, 20.0, 100.0 + j as f32 * 15.0, 14.0, LIGHTGRAY);
            }
        }

        draw_text("Mnem-Strings: Acoustic Manifestation of Code Decay", 10.0, 30.0, 20.0, WHITE);
        draw_text(&format!("Entropy: {:.4}", entropy), screen_width() - 200.0, 30.0, 20.0, RED);

        next_frame().await;
    }
}
