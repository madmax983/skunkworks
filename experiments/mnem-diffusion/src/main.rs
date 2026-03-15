use gray_scott::GrayScott;
use macroquad::prelude::*;

mod graph;
use graph::Graph;

const GRID_WIDTH: usize = 256;
const GRID_HEIGHT: usize = 256;

#[macroquad::main("Mnemonic Diffusion")]
async fn main() {
    let mut graph = Graph::new();
    println!("Scanning directory...");

    // Attempt to scan the current crate first
    graph.scan_directory("experiments/mnem-diffusion/src");

    if graph.nodes.is_empty() {
        println!("Scanning current directory...");
        graph.scan_directory(".");
    }

    let mut gs = GrayScott::new(GRID_WIDTH, GRID_HEIGHT);

    // Initial seed for Gray-Scott
    gs.add_chemical(GRID_WIDTH / 2, GRID_HEIGHT / 2, 1.0);

    // Camera
    let mut offset = vec2(GRID_WIDTH as f32 / 2.0, GRID_HEIGHT as f32 / 2.0);
    let mut zoom = 1.0;
    let mut dragging = false;
    let mut last_mouse = vec2(0.0, 0.0);

    // Texture for rendering Gray-Scott
    let mut image = Image::gen_image_color(GRID_WIDTH as u16, GRID_HEIGHT as u16, BLACK);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    loop {
        clear_background(BLACK);

        let dt = get_frame_time();

        // Input Handling
        let mouse_pos = vec2(mouse_position().0, mouse_position().1);
        if is_mouse_button_pressed(MouseButton::Right) {
            dragging = true;
            last_mouse = mouse_pos;
        }
        if is_mouse_button_released(MouseButton::Right) {
            dragging = false;
        }
        if dragging {
            let delta = mouse_pos - last_mouse;
            offset += delta / zoom;
            last_mouse = mouse_pos;
        }
        let (_, wheel_y) = mouse_wheel();
        if wheel_y > 0.0 {
            zoom *= 1.1;
        } else if wheel_y < 0.0 {
            zoom /= 1.1;
        }

        // --- Physics (from mnem-rot) ---
        let mut forces = vec![vec2(0.0, 0.0); graph.nodes.len()];
        for (i, force_i) in forces.iter_mut().enumerate() {
            for j in 0..graph.nodes.len() {
                if i != j {
                    let d = graph.nodes[i].pos - graph.nodes[j].pos;
                    let dist = d.length().max(1.0);
                    let force = d.normalize() * (100.0 / dist);
                    *force_i += force;
                }
            }
        }
        for edge in &graph.edges {
            let p1 = graph.nodes[edge.from].pos;
            let p2 = graph.nodes[edge.to].pos;
            let d = p2 - p1;
            let dist = d.length();
            let force = d.normalize() * (dist - 100.0) * edge.strength * 0.1;
            forces[edge.from] += force;
            forces[edge.to] -= force;
        }
        for (i, node) in graph.nodes.iter_mut().enumerate() {
            node.vel += forces[i] * dt;
            node.vel *= 0.9;
            node.pos += node.vel;
        }

        // --- Gray-Scott Injection ---
        for node in &graph.nodes {
            let gx = (node.pos.x / 5.0 + GRID_WIDTH as f32 / 2.0).round() as isize;
            let gy = (node.pos.y / 5.0 + GRID_HEIGHT as f32 / 2.0).round() as isize;

            if gx >= 0 && gx < GRID_WIDTH as isize && gy >= 0 && gy < GRID_HEIGHT as isize {
                let rot_amount = 1.0 - node.health;
                let injection_amount = 0.5 + rot_amount * 2.0;
                gs.add_chemical(gx as usize, gy as usize, injection_amount);
            }
        }

        // Standard Gray-Scott update
        for _ in 0..10 {
            gs.update(0.055, 0.062, 1.0);
        }

        // Render Gray-Scott to texture
        let u_vals = gs.u();
        let v_vals = gs.v();
        let bytes = image.bytes.as_mut_slice();
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let idx = y * GRID_WIDTH + x;
                let u = u_vals[idx];
                let v = v_vals[idx];

                let v_color = (v.clamp(0.0, 1.0) * 255.0) as u8;
                let u_color = (u.clamp(0.0, 1.0) * 100.0) as u8;

                let img_idx = idx * 4;
                bytes[img_idx] = v_color; // Red: V chemical (rot)
                bytes[img_idx + 1] = u_color; // Green: U chemical
                bytes[img_idx + 2] = v_color.saturating_mul(2); // Blue: Some V glow
                bytes[img_idx + 3] = 255;
            }
        }
        texture.update(&image);

        // Draw everything with camera transform
        set_camera(&Camera2D {
            target: offset,
            zoom: vec2(zoom / screen_width() * 2.0, -zoom / screen_height() * 2.0),
            ..Default::default()
        });

        // Draw the Gray-Scott substrate
        draw_texture_ex(
            &texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(GRID_WIDTH as f32, GRID_HEIGHT as f32)),
                ..Default::default()
            },
        );

        // Draw the graph overlay
        for edge in &graph.edges {
            let p1 = graph.nodes[edge.from].pos;
            let p2 = graph.nodes[edge.to].pos;
            let alpha = edge.strength * 0.5;
            let scaled_p1 = p1 / 5.0 + vec2(GRID_WIDTH as f32 / 2.0, GRID_HEIGHT as f32 / 2.0);
            let scaled_p2 = p2 / 5.0 + vec2(GRID_WIDTH as f32 / 2.0, GRID_HEIGHT as f32 / 2.0);
            draw_line(
                scaled_p1.x,
                scaled_p1.y,
                scaled_p2.x,
                scaled_p2.y,
                1.0,
                Color::new(0.5, 0.5, 0.5, alpha),
            );
        }

        for node in &graph.nodes {
            let rot = 1.0 - node.health;
            let radius = 0.5;

            // Healthy = White/Cyan, Rot = Magenta/Red
            let color = Color::new(0.5 + rot * 0.5, 1.0 - rot, 1.0 - rot * 0.5, 1.0);

            let scaled_p = node.pos / 5.0 + vec2(GRID_WIDTH as f32 / 2.0, GRID_HEIGHT as f32 / 2.0);
            draw_circle(scaled_p.x, scaled_p.y, radius, color);
        }

        set_default_camera();

        // UI
        draw_text("Mnemonic Diffusion", 10.0, 20.0, 20.0, WHITE);
        draw_text(
            "Codebase Graph driving Reaction-Diffusion Morphogenesis",
            10.0,
            40.0,
            20.0,
            LIGHTGRAY,
        );
        draw_text(
            &format!("Nodes: {}", graph.nodes.len()),
            10.0,
            60.0,
            20.0,
            GRAY,
        );
        draw_text("Drag: Pan | Scroll: Zoom", 10.0, 80.0, 20.0, GRAY);

        next_frame().await;
    }
}
