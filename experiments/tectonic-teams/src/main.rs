use macroquad::prelude::*;
use std::env;

mod map;
mod history;
mod tectonics;

use map::WorldMap;
use history::History;

#[macroquad::main("Tectonic Teams")]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() > 1 { &args[1] } else { "." };

    // Initialize Map
    let mut map = WorldMap::new(128, 128);

    // Initialize History
    println!("Loading history from {}...", path);
    let history = match History::new(path, &mut map) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("Failed to load history: {}", e);
            // Just show empty map
            return;
        }
    };
    println!("Loaded {} commits. Found {} files.", history.commits.len(), history.files.len());

    let mut cam_dist: f32 = 200.0;
    let mut cam_angle_x: f32 = 0.5;
    let mut cam_angle_y: f32 = 0.5;
    let mut current_commit_idx = 0;
    let play_speed = 5; // Commits per frame
    let mut paused = false;

    loop {
        // Input
        if is_key_pressed(KeyCode::Escape) { break; }
        if is_key_pressed(KeyCode::Space) { paused = !paused; }
        if is_key_down(KeyCode::Right) { cam_angle_y += 0.02; }
        if is_key_down(KeyCode::Left) { cam_angle_y -= 0.02; }
        if is_key_down(KeyCode::Up) { cam_angle_x += 0.02; }
        if is_key_down(KeyCode::Down) { cam_angle_x -= 0.02; }
        if is_key_down(KeyCode::W) { cam_dist -= 2.0; }
        if is_key_down(KeyCode::S) { cam_dist += 2.0; }
        if is_key_pressed(KeyCode::R) {
            // Reset
            map.heightmap.fill(0.0);
            map.owner.fill(0);
            map.strength.fill(0.0);
            current_commit_idx = 0;
        }

        cam_angle_x = cam_angle_x.clamp(0.1, 1.5);
        cam_dist = cam_dist.max(10.0);

        // Simulation
        if !paused && current_commit_idx < history.commits.len() {
            for _ in 0..play_speed {
                if current_commit_idx < history.commits.len() {
                    let commit = &history.commits[current_commit_idx];
                    tectonics::process_commit(commit, &history, &mut map);
                    current_commit_idx += 1;
                }
            }
            // Erosion step (once per frame)
            tectonics::thermal_erosion(&mut map);
        }

        // Render
        clear_background(BLACK);

        let cam_pos = vec3(
            cam_angle_y.cos() * cam_dist,
            cam_dist * cam_angle_x.sin(),
            cam_angle_y.sin() * cam_dist,
        );
        let target = vec3(64.0, 0.0, 64.0);

        set_camera(&Camera3D {
            position: cam_pos + target,
            target,
            up: vec3(0.0, 1.0, 0.0),
            fovy: 45.0,
            ..Default::default()
        });

        draw_grid(20, 10.0, DARKGRAY, GRAY);

        // Build Mesh
        let mesh = build_mesh(&map);
        draw_mesh(&mesh);

        set_default_camera();

        // UI
        draw_text("Tectonic Teams", 10.0, 20.0, 30.0, WHITE);

        if current_commit_idx > 0 && current_commit_idx <= history.commits.len() {
            let commit = &history.commits[current_commit_idx - 1];
            draw_text(&format!("Commit: {}", commit.short_hash), 10.0, 50.0, 20.0, WHITE);
            draw_text(&format!("Author: {}", commit.author), 10.0, 70.0, 20.0, get_author_color(history.authors.get(&commit.author).copied().unwrap_or(0)));
            // draw_text(&format!("Date: {}", commit.timestamp), 10.0, 90.0, 20.0, GRAY); // Formatting datetime requires explicit dependency sometimes or just ToString
            draw_text(&format!("Msg: {}", commit.message.lines().next().unwrap_or("")), 10.0, 90.0, 20.0, LIGHTGRAY);
        }

        draw_text(&format!("Progress: {}/{}", current_commit_idx, history.commits.len()), 10.0, 120.0, 20.0, WHITE);
        draw_text("Controls: [Space] Pause | [R] Reset | [Arrows/WASD] Camera", 10.0, 150.0, 20.0, DARKGRAY);

        next_frame().await;
    }
}

fn build_mesh(map: &WorldMap) -> Mesh {
    let mut vertices = Vec::with_capacity(map.width * map.height);
    let mut indices = Vec::with_capacity((map.width - 1) * (map.height - 1) * 6);

    for y in 0..map.height {
        for x in 0..map.width {
            let idx = map.get_index(x, y).unwrap();
            let h = map.heightmap[idx];
            let owner = map.owner[idx];
            let strength = map.strength[idx];

            // Color based on owner ID
            let color = if owner == 0 {
                GRAY
            } else {
                get_author_color(owner)
            };

            // Shade by strength/height
            // Strength affects saturation/brightness?
            // Let's make "strong" ownership brighter.
            let shade = 0.2 + (strength * 0.8);

            // Height shading (fake AO)
            // Just use strength for now
            let final_color = Color::new(color.r * shade, color.g * shade, color.b * shade, 1.0);

            vertices.push(Vertex {
                position: vec3(x as f32, h, y as f32),
                uv: vec2(x as f32 / map.width as f32, y as f32 / map.height as f32),
                color: final_color.into(),
                normal: vec4(0.0, 1.0, 0.0, 0.0), // Flat shading for now
            });
        }
    }

    for y in 0..map.height - 1 {
        for x in 0..map.width - 1 {
            let i00 = (y * map.width + x) as u16;
            let i10 = (y * map.width + (x + 1)) as u16;
            let i01 = ((y + 1) * map.width + x) as u16;
            let i11 = ((y + 1) * map.width + (x + 1)) as u16;

            indices.push(i00);
            indices.push(i01);
            indices.push(i10);

            indices.push(i10);
            indices.push(i01);
            indices.push(i11);
        }
    }

    Mesh {
        vertices,
        indices,
        texture: None,
    }
}

fn get_author_color(id: u8) -> Color {
    // Generate distinct colors using golden ratio
    if id == 0 { return GRAY; }
    let golden_ratio_conjugate = 0.618033988749895;
    let h = (id as f32 * golden_ratio_conjugate) % 1.0;
    hsl_to_rgb(h, 0.8, 0.6)
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> Color {
    // Basic HSL to RGB conversion
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = if h < 1.0/6.0 {
        (c, x, 0.0)
    } else if h < 2.0/6.0 {
        (x, c, 0.0)
    } else if h < 3.0/6.0 {
        (0.0, c, x)
    } else if h < 4.0/6.0 {
        (0.0, x, c)
    } else if h < 5.0/6.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    Color::new(r + m, g + m, b + m, 1.0)
}
