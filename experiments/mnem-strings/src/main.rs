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

const STRING_COUNT: usize = 12;
const STRING_SPACING: f32 = 60.0;
const BASE_FREQ: f32 = 110.0; // A2

#[macroquad::main("Mnemonic Strings")]
async fn main() {
    let (audio_handle, cmd_tx) = init_audio().expect("Failed to init audio");
    let _audio_handle = audio_handle; // Keep alive

    let mut graph = Graph::new();
    println!("Scanning directory...");
    graph.scan_directory("experiments/mnem-rot/src");
    if graph.nodes.is_empty() {
        println!("Scanning current directory...");
        graph.scan_directory(".");
    }

    let grid_w = 200;
    let grid_h = 150;
    let mut platter = Platter::new(grid_w, grid_h);
    let grid_scale = 4.0;

    let mut strings: Vec<FerrousString> = Vec::new();
    for i in 0..STRING_COUNT {
        let x = 100.0 + i as f32 * STRING_SPACING;
        let pos = vec2(x, 100.0);
        let target_freq = BASE_FREQ * (2.0f32).powf(i as f32 / 12.0);
        strings.push(FerrousString::new(pos, 400.0, target_freq));
    }

    let mut entropy = 0.0;
    let mut hovered_node: Option<usize> = None;

    let mut offset = vec2(0.0, 0.0);
    let mut zoom = 1.0;
    let mut dragging = false;
    let mut last_mouse = vec2(0.0, 0.0);

    let texture =
        Texture2D::from_image(&Image::gen_image_color(grid_w as u16, grid_h as u16, BLACK));
    texture.set_filter(FilterMode::Nearest);

    loop {
        let mouse_pos = mouse_position();
        let mouse_vec = vec2(mouse_pos.0, mouse_pos.1);
        let dt = get_frame_time();

        // --- Camera Input ---
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

        // --- Physics & Decay ---
        entropy += 0.0001;
        platter.decay(0.99);

        let mut forces = vec![vec2(0.0, 0.0); graph.nodes.len()];

        // Graph Repulsion
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
            // Gravity
            let center = vec2(screen_width() / 2.0, screen_height() / 2.0) - offset;
            let to_center = (center - graph.nodes[i].pos) * 0.01;
            forces[i] += to_center;
        }

        // Graph Springs
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

        // Apply Forces, Decay Graph Nodes
        hovered_node = None;
        let world_mouse = (mouse_vec - offset) / zoom;

        for (i, node) in graph.nodes.iter_mut().enumerate() {
            node.vel += forces[i] * dt;

            // Apply magnetic drag from platter based on health
            let gx = (node.pos.x / grid_scale) as i32;
            let gy = (node.pos.y / grid_scale) as i32;
            if gx >= 0 && gy >= 0 && gx < grid_w as i32 && gy < grid_h as i32 {
                let mag = platter.get_magnetism(gx as usize, gy as usize);
                // The more decayed a node is, the more the magnetic field affects its motion
                let magnetic_susceptibility = (1.0 - node.health) * 50.0;
                node.vel.x += mag as f32 * magnetic_susceptibility * dt;

                // Rotting nodes also deposit magnetic chaos into the platter
                let deposit = (0.5 - node.health) * node.vel.length() * 0.01;
                platter.magnetize(gx as usize, gy as usize, deposit as f64);
            }

            node.vel *= 0.90; // Damping
            node.pos += node.vel;

            node.health -= entropy * dt * 0.05;
            if node.health < 0.1 {
                node.health = 0.1;
            }

            if node.pos.distance(world_mouse) < 20.0 {
                hovered_node = Some(i);
                node.health += dt * 5.0;
                if node.health > 1.0 {
                    node.health = 1.0;
                }
            }
        }

        // --- Update Strings ---
        for s in &mut strings {
            s.update_physics(dt, &platter, grid_scale);

            // String vibration -> Platter
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

            // Graph nodes intersecting strings cause acoustic plucks based on rot!
            let string_x = s.pos.x + s.vibration;
            for node in &graph.nodes {
                // If the node is crossing the string rapidly and has low health, it strikes it violently
                let dx = (node.pos.x - string_x).abs();
                let in_range_y = node.pos.y >= s.pos.y && node.pos.y <= s.pos.y + s.length;
                if dx < 5.0 && in_range_y {
                    let rot_factor = 1.0 - node.health;
                    // Only nodes with some rot pluck heavily, healthy nodes pass silently
                    if rot_factor > 0.3 {
                        let strength = (node.vel.length() * rot_factor).clamp(5.0, 50.0);
                        let direction = if node.vel.x > 0.0 { 1.0 } else { -1.0 };

                        // To avoid continuous buzzing, only pluck if speed is high enough
                        if strength > 10.0 {
                            s.pluck(strength * direction);
                            let _ = cmd_tx.send(AudioCommand::Pluck {
                                frequency: s.frequency
                                    * (1.0 + (rand::gen_range(-0.1, 0.1) * rot_factor)),
                                decay: s.decay,
                                amplitude: (strength / 50.0).clamp(0.1, 0.8),
                            });
                        }
                    }
                }
            }
        }

        // --- Render ---
        clear_background(BLACK);

        // Draw Platter Heatmap
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

        // Center the Platter visually
        draw_texture_ex(
            &texture,
            0.0, //offset.x,
            0.0, //offset.y,
            Color::new(1.0, 1.0, 1.0, 0.3),
            DrawTextureParams {
                dest_size: Some(vec2(grid_w as f32 * grid_scale, grid_h as f32 * grid_scale)),
                ..Default::default()
            },
        );

        // Draw Strings
        for s in &strings {
            // Adjust draw pos for offset/zoom? Keeping them static for now to act as fixed instruments.
            // If we want them in world space:
            let scaled_pos = s.pos * zoom + offset;

            let steps = 20;
            let step_size = s.length / steps as f32;
            let string_color = Color::new(1.0, 1.0, 1.0, 0.5);

            for i in 0..steps {
                let ratio1 = (i as f32 * step_size) / s.length;
                let ratio2 = ((i + 1) as f32 * step_size) / s.length;

                let shape1 = (std::f32::consts::PI * ratio1).sin();
                let shape2 = (std::f32::consts::PI * ratio2).sin();

                let p1 = vec2(
                    scaled_pos.x + s.vibration * shape1 * zoom,
                    scaled_pos.y + (i as f32 * step_size) * zoom,
                );
                let p2 = vec2(
                    scaled_pos.x + s.vibration * shape2 * zoom,
                    scaled_pos.y + ((i + 1) as f32 * step_size) * zoom,
                );

                draw_line(p1.x, p1.y, p2.x, p2.y, 2.0 * zoom, string_color);
            }
        }

        // Draw Graph Edges
        for edge in &graph.edges {
            if edge.from < graph.nodes.len() && edge.to < graph.nodes.len() {
                let n1 = graph.nodes[edge.from].pos * zoom + offset;
                let n2 = graph.nodes[edge.to].pos * zoom + offset;
                let avg_health =
                    (graph.nodes[edge.from].health + graph.nodes[edge.to].health) / 2.0;

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

        // Draw Graph Nodes
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
                let display_line = if line.len() > 50 { &line[0..50] } else { line };
                draw_text(display_line, 20.0, 100.0 + j as f32 * 15.0, 14.0, LIGHTGRAY);
            }
        }

        draw_text(
            &format!("Entropy: {:.4}", entropy),
            screen_width() - 200.0,
            30.0,
            20.0,
            RED,
        );
        draw_text(
            "Right Click: Pan | Scroll: Zoom | Hover: Heal",
            10.0,
            screen_height() - 10.0,
            16.0,
            GRAY,
        );

        next_frame().await
    }
}
