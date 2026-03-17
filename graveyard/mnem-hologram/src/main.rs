use macroquad::prelude::*;

mod glitch;
use glitch::TextGlitcher;
mod graph;
use graph::Graph;
mod hologram;
use hologram::Hologram;

/// 🧬 Lineage: experiments/mnem-hologram
///
/// This experiment crosses the codebase graph decay visualization of `mnem-rot`
/// with the FFT-based optical interference rendering of `hologram-text`.
///
/// Novel trait: Entropy-driven Holography. The decaying nodes (high entropy) in
/// a codebase graph act as spatial objects that perturb an optical interference
/// pattern (the hologram). The hologram's frequency domain representation and
/// spatial reconstruction dynamically morph based on the graph's entropy/rot.
#[macroquad::main("Mnemonic Hologram")]
async fn main() {
    let mut graph = Graph::new();
    println!("Scanning directory...");

    // Try scanning the source directory
    graph.scan_directory("experiments/mnem-hologram/src");
    if graph.nodes.is_empty() {
        println!("Scanning current directory...");
        graph.scan_directory(".");
    }

    let mut entropy = 0.0;
    let mut hovered_node: Option<usize> = None;

    // Camera for graph
    let mut offset = vec2(0.0, 0.0);
    let mut zoom = 1.0;
    let mut dragging = false;
    let mut last_mouse = vec2(0.0, 0.0);

    // Hologram resolution
    let holo_w = 256;
    let holo_h = 128;
    let mut image_holo = Image::gen_image_color(holo_w as u16, holo_h as u16, BLACK);
    let mut image_recon = Image::gen_image_color(holo_w as u16, holo_h as u16, BLACK);
    let texture_holo = Texture2D::from_image(&image_holo);
    let texture_recon = Texture2D::from_image(&image_recon);

    texture_holo.set_filter(FilterMode::Nearest);
    texture_recon.set_filter(FilterMode::Nearest);

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
            last_mouse = mouse_vec;
        }

        let wheel = mouse_wheel();
        if wheel.1 != 0.0 {
            zoom *= if wheel.1 > 0.0 { 1.1 } else { 0.9 };
        }

        // Logic: Decay
        entropy += 0.0001; // Slow global rot

        // Physics & Decay Loop
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
            let center = vec2(screen_width() / 4.0, screen_height() / 2.0) - offset;
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

        let dt = get_frame_time();
        hovered_node = None;
        let world_mouse = (mouse_vec - offset) / zoom;

        for (i, node) in graph.nodes.iter_mut().enumerate() {
            node.vel += forces[i] * dt;
            node.vel *= 0.90; // Damping
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

        // Generate Hologram based on the current graph positions and health
        let mut density_grid = vec![0.0_f64; holo_w * holo_h];

        // Mapping graph to hologram grid
        // The graph logic centers around (screen_width() / 4.0, screen_height() / 2.0)
        let graph_center_x = screen_width() / 4.0;
        let graph_center_y = screen_height() / 2.0;

        for node in &graph.nodes {
            let rx = node.pos.x - graph_center_x;
            let ry = node.pos.y - graph_center_y;
            // Map rx, ry to holo_w, holo_h
            let hx = (holo_w as f32 / 2.0 + rx * 0.1) as i32;
            let hy = (holo_h as f32 / 2.0 + ry * 0.1) as i32;

            if hx >= 0 && hx < holo_w as i32 && hy >= 0 && hy < holo_h as i32 {
                let idx = (hy as usize) * holo_w + (hx as usize);
                // Higher entropy = more intense perturbation
                density_grid[idx] += (1.0 - node.health) as f64;
            }
        }

        let holo = Hologram::from_density_field(holo_w, holo_h, &density_grid);
        let recon = holo.reconstruct(-20, -10);
        let mag = holo.get_magnitude();

        let max_mag = mag.iter().cloned().fold(0.0_f64, f64::max).max(1.0);
        let max_recon = recon.iter().cloned().fold(0.0_f64, f64::max).max(0.01);

        for y in 0..holo_h {
            for x in 0..holo_w {
                let idx = y * holo_w + x;
                // Hologram visual
                let v_holo = (mag[idx] / max_mag) as f32;
                image_holo.set_pixel(x as u32, y as u32, Color::new(0.0, 0.0, v_holo, 1.0));

                // Reconstruction visual
                let v_recon = (recon[idx] / max_recon) as f32;
                image_recon.set_pixel(x as u32, y as u32, Color::new(0.0, v_recon, 0.0, 1.0));
            }
        }

        texture_holo.update(&image_holo);
        texture_recon.update(&image_recon);

        // Render
        clear_background(BLACK);

        // Left split: Graph
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

        // Right split: Hologram visuals
        let panel_x = screen_width() / 2.0;
        let panel_w = screen_width() / 2.0;
        let scale = (panel_w / holo_w as f32).min((screen_height() / 2.0) / holo_h as f32) * 0.9;

        let draw_w = holo_w as f32 * scale;
        let draw_h = holo_h as f32 * scale;

        // Hologram (Frequency) top right
        let cx1 = panel_x + (panel_w - draw_w) / 2.0;
        let cy1 = screen_height() * 0.25 - draw_h / 2.0;

        draw_texture_ex(
            &texture_holo,
            cx1,
            cy1,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(draw_w, draw_h)),
                ..Default::default()
            },
        );
        draw_rectangle_lines(cx1, cy1, draw_w, draw_h, 2.0, BLUE);
        draw_text("Hologram (Frequency Domain)", cx1, cy1 - 10.0, 20.0, WHITE);

        // Reconstruction (Spatial) bottom right
        let cy2 = screen_height() * 0.75 - draw_h / 2.0;
        draw_texture_ex(
            &texture_recon,
            cx1,
            cy2,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(draw_w, draw_h)),
                ..Default::default()
            },
        );
        draw_rectangle_lines(cx1, cy2, draw_w, draw_h, 2.0, GREEN);
        draw_text("Reconstruction (Spatial Domain)", cx1, cy2 - 10.0, 20.0, WHITE);

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
