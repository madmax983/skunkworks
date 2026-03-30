use ferrous_core::Platter;
use macroquad::prelude::*;

mod audio;
mod glitch;
mod graph;
mod string;

use audio::{init_audio, AudioCommand};
use glitch::TextGlitcher;
use graph::Graph;
use string::FerrousString;

const STRING_COUNT: usize = 8;
const STRING_SPACING: f32 = 100.0;
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
    graph.scan_directory("experiments/mnem-strings/src");
    if graph.nodes.is_empty() {
        println!("Scanning current directory...");
        graph.scan_directory(".");
    }

    // Init strings
    for i in 0..STRING_COUNT {
        let y = 100.0 + i as f32 * STRING_SPACING;
        let mut string = FerrousString::new(vec2(y, 100.0), 800, BASE_FREQ);
        string.base_freq = BASE_FREQ * 2.0_f32.powf(i as f32 / 12.0); // Chromatic scale up
        strings.push(string);
    }

    let mut entropy = 0.0;
    let mut offset = vec2(0.0, 0.0);
    let mut zoom = 1.0;
    let mut dragging = false;
    let mut last_mouse = vec2(0.0, 0.0);
    let mut _hovered_node: Option<usize> = None;

    loop {
        let mouse_pos = mouse_position();
        let mouse_vec = vec2(mouse_pos.0, mouse_pos.1);

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

        entropy += 0.0001;

        let dt = get_frame_time();

        // Platter logic (from ferrous-strings)
        if is_mouse_button_down(MouseButton::Left) {
            let gx = (mouse_pos.0 / grid_scale) as isize;
            let gy = (mouse_pos.1 / grid_scale) as isize;
            platter.magnetize(gx as usize, gy as usize, 5.0);
        }

        platter.decay(0.95);

        // Update nodes (graph physics)
        let mut forces = vec![vec2(0.0, 0.0); graph.nodes.len()];

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

        let world_mouse = (mouse_vec - offset) / zoom;
        _hovered_node = None;

        for (i, node) in graph.nodes.iter_mut().enumerate() {
            node.vel += forces[i] * dt;
            node.vel *= 0.90;
            node.pos += node.vel;

            let dist = (node.pos - world_mouse).length();
            if dist < 20.0 {
                _hovered_node = Some(i);
                if is_mouse_button_pressed(MouseButton::Left) {
                    node.health = 1.0;
                    entropy -= 0.01;
                }
            }

            // Decay
            node.health -= 0.001 * dt;
            if node.health < 0.0 {
                node.health = 0.0;
            }

            // High entropy/decay causes node to drop like a particle and pluck strings
            if node.health < 0.2 {
                // Gravity pulling decaying node down
                node.vel.y += 10.0 * dt * (0.2 - node.health) * 5.0;

                // Interaction with Strings
                for string in strings.iter_mut() {
                    let screen_x = node.pos.x * zoom + offset.x;
                    let screen_y = node.pos.y * zoom + offset.y;

                    if screen_x > 0.0 && screen_x < screen_width() {
                        let rel_x = screen_x / screen_width();
                        let seg_idx = (rel_x * string.segments.len() as f32) as usize;
                        if seg_idx < string.segments.len() {
                            let sy = string.segments[seg_idx].y;
                            // Check collision
                            if (screen_y - sy).abs() < 10.0 && node.vel.y > 0.0 {
                                // Pluck string!
                                let pluck_force = node.vel.y * 5.0;
                                string.pluck(pluck_force);

                                // Audio
                                let note = string.base_freq; // Could adjust based on node size
                                let _duration = 0.5 + (1.0 - node.health) * 1.5;
                                let _ = cmd_tx.send(AudioCommand::Pluck {
                                    frequency: note,
                                    decay: 0.99,
                                    amplitude: (pluck_force * 0.01).clamp(0.1, 1.0),
                                });

                                // Node bounces slightly
                                node.vel.y = -node.vel.y * 0.5;

                                // Rot causes damage to string
                                string.decay_rate += 0.001;
                            }
                        }
                    }
                }
            }

            // Interaction with magnetic platter
            let gx = ((node.pos.x * zoom + offset.x) / grid_scale) as isize;
            let gy = ((node.pos.y * zoom + offset.y) / grid_scale) as isize;
            if gx >= 0 && gx < grid_w as isize && gy >= 0 && gy < grid_h as isize {
                let mag = platter.get_magnetism(gx as usize, gy as usize);
                if mag.abs() > 0.1 {
                     node.vel.y -= mag as f32 * 100.0 * dt; // Repulsed by magnetic field
                }
            }
        }

        for string in &mut strings {
            string.update(dt);
        }

        clear_background(BLACK);

        // Draw magnetic platter
        for y in 0..grid_h {
            for x in 0..grid_w {
                let val = platter.get_magnetism(x, y);
                if val.abs() > 0.01 {
                    let color = if val > 0.0 {
                        Color::new(1.0, 0.0, 0.0, val.min(1.0) as f32)
                    } else {
                        Color::new(0.0, 0.0, 1.0, (-val).min(1.0) as f32)
                    };
                    draw_rectangle(
                        x as f32 * grid_scale,
                        y as f32 * grid_scale,
                        grid_scale,
                        grid_scale,
                        color,
                    );
                }
            }
        }

        // Draw edges
        for edge in &graph.edges {
            if edge.from < graph.nodes.len() && edge.to < graph.nodes.len() {
                let n1 = &graph.nodes[edge.from];
                let n2 = &graph.nodes[edge.to];
                let mut p1 = n1.pos * zoom + offset;
                let mut p2 = n2.pos * zoom + offset;

                // Screen bounds culling
                if (p1.x < 0.0 && p2.x < 0.0)
                    || (p1.y < 0.0 && p2.y < 0.0)
                    || (p1.x > screen_width() && p2.x > screen_width())
                    || (p1.y > screen_height() && p2.y > screen_height())
                {
                    continue;
                }

                let health_avg = (n1.health + n2.health) / 2.0;
                let alpha = (health_avg * 0.5).max(0.1);

                // Rot causes glitchy edges
                if health_avg < 0.5 {
                    let glitch_mag = (0.5 - health_avg) * 10.0;
                    p1.x += (rand::gen_range(-1.0, 1.0)) * glitch_mag;
                    p2.y += (rand::gen_range(-1.0, 1.0)) * glitch_mag;
                }

                draw_line(
                    p1.x,
                    p1.y,
                    p2.x,
                    p2.y,
                    1.0 * zoom,
                    Color::new(0.3, 0.4, 0.5, alpha),
                );
            }
        }

        // Draw Strings
        for string in &strings {
            string.draw();
        }

        // Draw nodes
        for (i, node) in graph.nodes.iter().enumerate() {
            let p = node.pos * zoom + offset;

            if p.x < -50.0 || p.y < -50.0 || p.x > screen_width() + 50.0 || p.y > screen_height() + 50.0 {
                continue;
            }

            let radius = 10.0 * zoom;

            let rot_color = Color::new(
                1.0 - node.health,
                node.health,
                0.2,
                0.8,
            );

            // Draw shadow for falling nodes
            if node.health < 0.2 {
                draw_circle(p.x, p.y + 5.0, radius * 0.8, Color::new(1.0, 0.0, 0.0, 0.5));
            }

            draw_circle(p.x, p.y, radius, rot_color);

            if Some(i) == _hovered_node {
                draw_circle_lines(p.x, p.y, radius + 2.0, 2.0, WHITE);
            }
        }

        // Draw UI
        draw_text(
            &format!("Global Entropy: {:.4}", entropy),
            10.0,
            20.0,
            20.0,
            WHITE,
        );
        draw_text(
            "Right Click & Drag: Pan | Scroll: Zoom | Click Nodes: Maintain",
            10.0,
            40.0,
            20.0,
            LIGHTGRAY,
        );

        // Hover popup
        if let Some(idx) = _hovered_node {
            let node = &graph.nodes[idx];
            let p = node.pos * zoom + offset;

            let _glitcher = TextGlitcher;


            let glitch_level = (1.0 - node.health) * 2.0;
            let display_name = TextGlitcher::corrupt(&node.name, glitch_level);

            let text_size = measure_text(&display_name, None, 20, 1.0);
            draw_rectangle(
                p.x + 15.0,
                p.y - 15.0,
                text_size.width + 10.0,
                40.0,
                Color::new(0.0, 0.0, 0.0, 0.8),
            );
            draw_text(
                &display_name,
                p.x + 20.0,
                p.y + 5.0,
                20.0,
                if node.health < 0.3 { RED } else { WHITE },
            );
            draw_text(
                &format!("Health: {:.1}%", node.health * 100.0),
                p.x + 20.0,
                p.y + 20.0,
                16.0,
                LIGHTGRAY,
            );
        }

        next_frame().await;
    }
}
