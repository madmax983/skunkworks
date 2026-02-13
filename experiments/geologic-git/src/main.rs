mod git_source;
mod layout;
mod simulation;
mod terrain;

use macroquad::prelude::*;
use simulation::Simulation;
use std::path::Path;

#[macroquad::main("Geologic Git")]
async fn main() {
    let mut sim = Simulation::new(Path::new("."));

    // Camera state
    let mut cam_dist = 300.0;
    let mut cam_angle_x: f32 = 0.5; // Radians
    let mut cam_angle_y: f32 = 0.5; // Radians

    let mut last_mouse_pos = mouse_position();

    loop {
        clear_background(LIGHTGRAY);

        // Input
        if is_key_pressed(KeyCode::Space) {
            sim.playing = !sim.playing;
        }
        if is_key_pressed(KeyCode::R) {
            sim.reset();
        }
        if is_key_down(KeyCode::Up) {
            sim.erosion_drops_per_frame += 100;
        }
        if is_key_down(KeyCode::Down) {
            if sim.erosion_drops_per_frame > 100 {
                sim.erosion_drops_per_frame -= 100;
            }
        }

        // Camera control
        let current_mouse_pos = mouse_position();
        let delta = vec2(current_mouse_pos.0 - last_mouse_pos.0, current_mouse_pos.1 - last_mouse_pos.1);
        last_mouse_pos = current_mouse_pos;

        if is_mouse_button_down(MouseButton::Left) {
            cam_angle_y -= delta.x * 0.01;
            cam_angle_x += delta.y * 0.01;
            cam_angle_x = cam_angle_x.clamp(0.1, 1.5);
        }
        let (_, wheel) = mouse_wheel();
        cam_dist -= wheel * 10.0;
        cam_dist = cam_dist.clamp(50.0, 1000.0);

        // Update Simulation
        sim.step();

        // 3D Setup
        let center = vec3(sim.terrain.width as f32 / 2.0, 0.0, sim.terrain.height as f32 / 2.0);
        let cam_pos = vec3(
            center.x + cam_dist * cam_angle_y.cos() * cam_angle_x.cos(),
            center.y + cam_dist * cam_angle_x.sin(),
            center.z + cam_dist * cam_angle_y.sin() * cam_angle_x.cos(),
        );

        let camera = Camera3D {
            position: cam_pos,
            target: center,
            up: vec3(0.0, 1.0, 0.0),
            ..Default::default()
        };
        set_camera(&camera);

        // Draw Terrain
        draw_terrain(&sim);

        // Draw Layout outlines (optional, maybe too cluttered)
        // draw_layout_outlines(&sim);

        set_default_camera();

        // UI
        draw_ui(&sim);

        next_frame().await
    }
}

fn draw_terrain(sim: &Simulation) {
    let t = &sim.terrain;
    let w = t.width;
    let h = t.height;

    // We can't rebuild mesh every frame optimally, but for 256x256 = 65k vertices, it's fine for desktop.
    // macroquad doesn't have immediate mode mesh builder that is super fast for this size maybe?
    // But draw_mesh takes a Mesh struct.

    let mut vertices = Vec::with_capacity(w * h);
    let mut indices = Vec::with_capacity((w - 1) * (h - 1) * 6);

    for y in 0..h {
        for x in 0..w {
            let height = t.get(x, y);

            // Color based on height
            // Low = Green/Sand, High = Grey/White
            // We need a height scale. Uplift is ~log(insertions). max might be around 20-50?
            let normalized = (height / 50.0).clamp(0.0, 1.0);

            let color = if height < 0.5 {
                Color::new(0.2, 0.6 + normalized * 0.4, 0.2, 1.0) // Greenish
            } else if height < 10.0 {
                Color::new(0.5, 0.5, 0.5, 1.0) // Grey
            } else {
                Color::new(0.9, 0.9, 0.9, 1.0) // Snow
            };

            // Use macroquad::models::Vertex? No, just Vertex.
            // macroquad 0.4 Vertex has normal.
            vertices.push(Vertex {
                position: vec3(x as f32, height, y as f32),
                uv: vec2(x as f32 / w as f32, y as f32 / h as f32),
                color: color.into(),
                normal: vec4(0.0, 1.0, 0.0, 1.0), // TODO: Calculate normals
            });
        }
    }

    for y in 0..h - 1 {
        for x in 0..w - 1 {
            let i = (y * w + x) as u16; // indices are u16 in macroquad usually
            let row = w as u16;

            // Triangle 1
            indices.push(i);
            indices.push(i + row);
            indices.push(i + 1);

            // Triangle 2
            indices.push(i + 1);
            indices.push(i + row);
            indices.push(i + row + 1);
        }
    }

    let mesh = Mesh {
        vertices,
        indices,
        texture: None,
    };

    draw_mesh(&mesh);
}

fn draw_ui(sim: &Simulation) {
    draw_text(
        &format!("Commit: {} / {}", sim.current_commit_index, sim.commits.len()),
        10.0,
        20.0,
        30.0,
        BLACK,
    );

    if sim.current_commit_index > 0 && sim.current_commit_index <= sim.commits.len() {
        let commit = &sim.commits[sim.current_commit_index - 1];
        draw_text(&format!("Hash: {}", commit.short_hash), 10.0, 50.0, 20.0, BLACK);
        draw_text(&format!("Msg: {}", commit.message.lines().next().unwrap_or("")), 10.0, 70.0, 20.0, BLACK);
        draw_text(&format!("Date: {}", commit.timestamp), 10.0, 90.0, 20.0, BLACK);
    }

    draw_text(
        if sim.playing { "PLAYING (Space to Pause)" } else { "PAUSED (Space to Play)" },
        10.0,
        120.0,
        20.0,
        if sim.playing { GREEN } else { RED },
    );

    draw_text(
        &format!("Erosion Drops/Frame: {}", sim.erosion_drops_per_frame),
        10.0, 140.0, 20.0, BLACK
    );

    draw_text(
        "Controls: Drag to Rotate, Scroll to Zoom, Up/Down erosion",
        10.0,
        screen_height() - 20.0,
        20.0,
        DARKGRAY,
    );
}
