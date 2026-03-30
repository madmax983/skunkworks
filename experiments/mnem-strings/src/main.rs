use ferrous_core::Platter;
use macroquad::prelude::*;

mod audio;
mod glitch;
mod graph;
mod particle;
mod string;

use audio::{init_audio, AudioCommand};
use graph::Graph;
use particle::Particle;
use string::FerrousString;

const STRING_COUNT: usize = 12;
const PARTICLE_COUNT: usize = 200;
const STRING_SPACING: f32 = 60.0;
const BASE_FREQ: f32 = 55.0; // A1 (lower for more ominous rumble)

#[macroquad::main("Mnemonic Strings")]
async fn main() {
    let (audio_handle, cmd_tx) = init_audio().expect("Failed to init audio");
    let _audio_handle = audio_handle; // Keep alive

    let grid_w = 200;
    let grid_h = 150;
    let mut platter = Platter::new(grid_w, grid_h);
    let grid_scale = 4.0;

    // 1. Initialize Graph (from mnem-rot)
    let mut graph = Graph::new();
    println!("Scanning directory...");
    graph.scan_directory("experiments/mnem-strings/src");
    if graph.nodes.is_empty() {
        graph.scan_directory(".");
    }

    // 2. Initialize Strings (from ferrous-strings)
    let mut strings: Vec<FerrousString> = Vec::new();
    for i in 0..STRING_COUNT {
        let x = 50.0 + i as f32 * STRING_SPACING;
        let pos = vec2(x, 100.0);
        let target_freq = BASE_FREQ * (2.0f32).powf(i as f32 / 12.0);
        strings.push(FerrousString::new(pos, 400.0, target_freq));
    }

    // 3. Initialize Particles
    let mut particles: Vec<Particle> = Vec::new();
    for _ in 0..PARTICLE_COUNT {
        let x = rand::gen_range(0.0, screen_width());
        let y = rand::gen_range(0.0, screen_height());
        particles.push(Particle::new(x, y));
    }

    let mut entropy = 0.0;
    let mut offset = vec2(0.0, 0.0);
    let mut zoom = 1.0;
    let mut dragging = false;
    let mut last_mouse = vec2(0.0, 0.0);
    let mut hovered_node: Option<usize> = None;

    // Platter texture
    let texture =
        Texture2D::from_image(&Image::gen_image_color(grid_w as u16, grid_h as u16, BLACK));
    texture.set_filter(FilterMode::Nearest);

    loop {
        let dt = get_frame_time();

        // --- Camera & Input ---
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

        // --- Physics & Logic Update ---

        // 1. Graph Decay & Forces
        entropy += 0.0001; // Slow global rot
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
            let center = vec2(screen_width() / 2.0, screen_height() / 2.0) - offset;
            let to_center = (center - graph.nodes[i].pos) * 0.01;
            forces[i] += to_center;
        }

        // Spring forces
        for edge in &graph.edges {
            if edge.from < graph.nodes.len() && edge.to < graph.nodes.len() {
                let diff = graph.nodes[edge.to].pos - graph.nodes[edge.from].pos;
                let force = diff.normalize() * (diff.length() - 100.0) * 0.05 * edge.strength;
                forces[edge.from] += force;
                forces[edge.to] -= force;
            }
        }

        hovered_node = None;
        let world_mouse = (mouse_vec - offset) / zoom;

        // 2. The Splice: Codebase Graph interacts with Platter and Strings
        for (i, node) in graph.nodes.iter_mut().enumerate() {
            node.vel += forces[i] * dt;
            node.vel *= 0.90;
            node.pos += node.vel;

            node.health -= entropy * dt * 0.05;
            if node.health < 0.1 {
                node.health = 0.1;
            }

            if node.pos.distance(world_mouse) < 20.0 {
                hovered_node = Some(i);
                node.health = (node.health + dt * 5.0).min(1.0);
            }

            // Magnetic Pole Deposition: Rotting code acts as a strong repulsive pole
            let world_x = node.pos.x * zoom + offset.x;
            let world_y = node.pos.y * zoom + offset.y;
            let gx = (world_x / grid_scale) as i32;
            let gy = (world_y / grid_scale) as i32;

            if gx >= 0 && gy >= 0 && gx < grid_w as i32 && gy < grid_h as i32 {
                let rot_intensity = 1.0 - node.health;
                if rot_intensity > 0.5 {
                    // Deposition of negative magnetism (repulsive to N poles) based on entropy
                    platter.magnetize(gx as usize, gy as usize, -(rot_intensity as f64) * 0.1);
                } else if node.health > 0.9 {
                    // Healthy code deposits positive magnetism
                    platter.magnetize(gx as usize, gy as usize, 0.05);
                }
            }
        }

        // 3. String & Particle Updates
        platter.decay(0.95); // Faster decay so history fades quickly

        for s in &mut strings {
            s.update_physics(dt, &platter, grid_scale);

            // The Splice: Nodes violently pluck strings based on proximity and rot
            for node in &graph.nodes {
                let rot = 1.0 - node.health;
                if rot > 0.6 {
                    // Only severely rotting nodes pluck strings
                    let world_x = node.pos.x * zoom + offset.x;
                    let world_y = node.pos.y * zoom + offset.y;

                    // Check if node is colliding with the string's bounding box roughly
                    if world_x > s.pos.x - 20.0
                        && world_x < s.pos.x + 20.0
                        && world_y > s.pos.y
                        && world_y < s.pos.y + s.length
                    {
                        let pluck_strength = (rot - 0.5) * 20.0 * dt;
                        let direction = if world_x > s.pos.x { -1.0 } else { 1.0 };

                        // Apply pluck
                        s.vibration += pluck_strength * direction;

                        // Play Audio if pluck is strong enough
                        if s.vibration.abs() > 5.0 && rand::gen_range(0.0, 1.0) < 0.1 {
                            let _ = cmd_tx.send(AudioCommand::Pluck {
                                frequency: s.frequency,
                                decay: s.decay,
                                amplitude: (pluck_strength.abs() / 10.0).clamp(0.1, 0.5),
                            });
                        }
                    }
                }
            }
        }

        for p in &mut particles {
            p.update(&platter, grid_scale, dt);
        }

        // --- Render ---
        clear_background(BLACK);

        // 1. Draw Platter (Heatmap)
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

        // 2. Draw Graph Edges
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
                    Color::new(0.5, 0.5, 0.5, avg_health * 0.5), // Lower opacity to let strings shine
                );
            }
        }

        // 3. Draw Strings & Particles
        for s in &strings {
            s.draw();
        }
        for p in &particles {
            p.draw();
        }

        // 4. Draw Graph Nodes (on top)
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

        // UI & Glitch text overlay
        if let Some(idx) = hovered_node {
            let node = &graph.nodes[idx];
            draw_rectangle(10.0, 10.0, 400.0, 200.0, Color::new(0.05, 0.05, 0.05, 0.95));
            draw_rectangle_lines(10.0, 10.0, 400.0, 200.0, 2.0, WHITE);
            draw_text(&node.name, 20.0, 40.0, 30.0, GREEN);
            draw_text(
                &format!("Health: {:.0}%", node.health * 100.0),
                20.0,
                70.0,
                20.0,
                WHITE,
            );

            let intensity = 1.0 - node.health;
            let corrupted = glitch::TextGlitcher::corrupt(&node.content, intensity);
            let lines: Vec<&str> = corrupted.lines().take(5).collect();
            for (j, line) in lines.iter().enumerate() {
                let display_line = if line.len() > 50 { &line[0..50] } else { line };
                draw_text(display_line, 20.0, 100.0 + j as f32 * 15.0, 14.0, LIGHTGRAY);
            }
        }

        draw_text(
            "Mnemonic Strings",
            10.0,
            screen_height() - 70.0,
            30.0,
            WHITE,
        );
        draw_text(
            "Code rot plucks acoustic strings and shapes magnetic fields.",
            10.0,
            screen_height() - 40.0,
            20.0,
            GRAY,
        );
        draw_text(
            "Right Click: Pan | Scroll: Zoom | Hover: Heal",
            10.0,
            screen_height() - 10.0,
            16.0,
            GRAY,
        );

        next_frame().await;
    }
}
