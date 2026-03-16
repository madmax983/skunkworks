use macroquad::prelude::*;

mod graph;
use graph::Graph;
mod glitch;
use glitch::TextGlitcher;

use gray_scott::GrayScott;

#[macroquad::main("Mnemonic Diffusion")]
async fn main() {
    let mut graph = Graph::new();
    println!("Scanning directory...");

    // Attempt to scan the current crate first
    graph.scan_directory("experiments/mnem-rot/src");

    if graph.nodes.is_empty() {
        println!("Scanning current directory...");
        graph.scan_directory(".");
    }

    let width = 256;
    let height = 256;
    let mut gs = GrayScott::new(width, height);
    // Parameters that allow stable but interesting patterns
    let feed = 0.055;
    let kill = 0.062;

    let mut entropy = 0.0;
    let mut hovered_node: Option<usize>;

    // Camera
    let mut offset = vec2(0.0, 0.0);
    let mut zoom = 1.0;
    let mut dragging = false;
    let mut last_mouse = vec2(0.0, 0.0);

    let mut image = Image::gen_image_color(width as u16, height as u16, BLACK);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    loop {
        let mouse_pos = mouse_position();
        let mouse_vec = vec2(mouse_pos.0, mouse_pos.1);

        // Input: Camera
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
        }
        if is_mouse_button_down(MouseButton::Right) {
            last_mouse = mouse_vec;
        }

        let wheel = mouse_wheel();
        if wheel.1 != 0.0 {
            let old_zoom = zoom;
            zoom *= if wheel.1 > 0.0 { 1.1 } else { 0.9 };
            let center = vec2(screen_width() / 2.0, screen_height() / 2.0);
            offset = center - (center - offset) * (zoom / old_zoom);
        }

        // Logic: Decay
        entropy += 0.0001; // Slow global rot

        let dt = get_frame_time().min(0.05);

        // Physics & Decay Loop for Graph
        let mut forces = vec![vec2(0.0, 0.0); graph.nodes.len()];

        hovered_node = None;

        for (i, force) in forces.iter_mut().enumerate() {
            let mut repulsion = vec2(0.0, 0.0);
            for j in 0..graph.nodes.len() {
                if i != j {
                    let diff = graph.nodes[i].pos - graph.nodes[j].pos;
                    let dist_sq = diff.length_squared().max(1.0);
                    if dist_sq < 250000.0 {
                        repulsion += diff.normalize() * (5000.0 / dist_sq);
                    }
                }
            }
            *force += repulsion;

            // Center attraction (Gravity) to map to gray-scott grid nicely
            // We want nodes to stay roughly within the grid boundaries
            let center = vec2(width as f32 / 2.0, height as f32 / 2.0);
            let to_center = (center - graph.nodes[i].pos) * 0.05;
            *force += to_center;
        }

        for edge in &graph.edges {
            if edge.from < graph.nodes.len() && edge.to < graph.nodes.len() {
                let diff = graph.nodes[edge.to].pos - graph.nodes[edge.from].pos;
                let dist = diff.length();
                let desired = 30.0;
                let force = diff.normalize() * (dist - desired) * 0.05 * edge.strength;
                forces[edge.from] += force;
                forces[edge.to] -= force;
            }
        }

        for (i, node) in graph.nodes.iter_mut().enumerate() {
            node.vel += forces[i] * dt * 50.0;
            node.vel *= 0.85; // Damping
            node.pos += node.vel;

            // Constrain to grid
            node.pos.x = node.pos.x.clamp(0.0, width as f32 - 1.0);
            node.pos.y = node.pos.y.clamp(0.0, height as f32 - 1.0);

            // Decay
            node.health -= entropy * dt * 0.05;
            if node.health < 0.1 {
                node.health = 0.1;
            }

            // Interaction
            let screen_pos = node.pos * zoom + offset;
            if screen_pos.distance(mouse_vec) < 20.0 * zoom {
                hovered_node = Some(i);
                node.health += dt * 5.0; // Heal
                if node.health > 1.0 {
                    node.health = 1.0;
                }
            }
        }

        // --- Mnemonic Diffusion (Gray-Scott integration) ---
        // Nodes drop 'V' chemical (fungus/rot) based on their entropy (1.0 - health) and size
        for node in &graph.nodes {
            let px = node.pos.x as usize;
            let py = node.pos.y as usize;
            if px < width && py < height {
                let rot_level = 1.0 - node.health;
                // Larger/more complex files have a larger drop radius
                let radius =
                    ((node.content.len() as f32).log2().max(1.0) * 0.5 * rot_level) as isize;
                let drop_amount = rot_level * 0.5;

                for dy in -radius..=radius {
                    for dx in -radius..=radius {
                        if dx * dx + dy * dy <= radius * radius {
                            let nx = px as isize + dx;
                            let ny = py as isize + dy;
                            if nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                                let idx = ny as usize * width + nx as usize;
                                gs.v_mut()[idx] = (gs.v_mut()[idx] + drop_amount).min(1.0);
                            }
                        }
                    }
                }
            }
        }

        // Run reaction-diffusion
        gs.update(feed, kill, 1.0);

        // Render Gray-Scott to image
        let bytes = image.bytes.as_mut_slice();
        for i in 0..width * height {
            let v = gs.v()[i];

            // Color mapping:
            // V concentration represents rot/fungus (purple/green)
            // U represents clean substrate (dark)
            let intensity = (v * 3.0).clamp(0.0, 1.0);

            bytes[i * 4] = (intensity * 150.0) as u8; // R
            bytes[i * 4 + 1] = (intensity * 255.0) as u8; // G
            bytes[i * 4 + 2] = (intensity * 50.0) as u8; // B
            bytes[i * 4 + 3] = 255;
        }
        texture.update(&image);

        // Rendering
        clear_background(BLACK);

        // Draw the Gray-Scott substrate
        let draw_params = DrawTextureParams {
            dest_size: Some(vec2(width as f32 * zoom, height as f32 * zoom)),
            ..Default::default()
        };
        draw_texture_ex(&texture, offset.x, offset.y, WHITE, draw_params);

        // Draw Edges
        for edge in &graph.edges {
            if edge.from < graph.nodes.len() && edge.to < graph.nodes.len() {
                let n1 = graph.nodes[edge.from].pos * zoom + offset;
                let n2 = graph.nodes[edge.to].pos * zoom + offset;
                let avg_health =
                    (graph.nodes[edge.from].health + graph.nodes[edge.to].health) / 2.0;

                let jitter = if avg_health < 0.5 {
                    vec2(rand::gen_range(-2.0, 2.0), rand::gen_range(-2.0, 2.0)) * zoom
                } else {
                    vec2(0.0, 0.0)
                };

                draw_line(
                    n1.x + jitter.x,
                    n1.y + jitter.y,
                    n2.x + jitter.x,
                    n2.y + jitter.y,
                    1.0 * zoom,
                    Color::new(1.0, 1.0, 1.0, avg_health * 0.3),
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

            let radius = 3.0 * zoom * (0.5 + node.health);
            draw_circle(pos.x, pos.y, radius, color);

            if node.health > 0.6 || hovered_node == Some(i) {
                draw_text(&node.name, pos.x + 5.0 * zoom, pos.y, 10.0 * zoom, WHITE);
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
