use macroquad::prelude::*;
use std::env;

mod erosion;
mod history;
mod map;

use erosion::ErosionParams;
use history::History;
use map::{Scanner, Terrain};

#[macroquad::main("Code Canyon")]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() > 1 { &args[1] } else { "." };

    let files = Scanner::scan(path);
    let mut terrain = Terrain::from_files(&files);

    // Initialize colors based on height
    for i in 0..terrain.colors.len() {
        terrain.colors[i] = get_color(terrain.heightmap[i]);
    }

    let params = ErosionParams::default();

    // History
    let mut history = History::new(path).ok();
    let mut replay_mode = false;
    let mut commits_processed = 0;
    let total_commits = history.as_ref().map(|h| h.total_commits()).unwrap_or(0);

    // Scale terrain for visualization
    let scale = 1.0;

    let mut cam_dist: f32 = terrain.width as f32 * 1.5;
    let mut cam_angle_x: f32 = 0.5;
    let mut cam_angle_y: f32 = 0.5;

    let mut raining = false;
    let droplets_per_frame = 1000;

    let mut mesh = build_mesh(&terrain, scale);

    loop {
        // Input Handling
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::Space) {
            raining = !raining;
            replay_mode = false;
        }
        if is_key_pressed(KeyCode::H) && history.is_some() {
            replay_mode = !replay_mode;
            raining = false;
        }
        if is_key_pressed(KeyCode::R) {
            terrain = Terrain::from_files(&files);
            // Initialize colors based on height
            for i in 0..terrain.colors.len() {
                terrain.colors[i] = get_color(terrain.heightmap[i]);
            }
            mesh = build_mesh(&terrain, scale);
            if let Some(h) = &mut history {
                h.reset();
                commits_processed = 0;
            }
        }
        if is_key_down(KeyCode::Up) {
            cam_angle_x += 0.02;
        }
        if is_key_down(KeyCode::Down) {
            cam_angle_x -= 0.02;
        }
        if is_key_down(KeyCode::Left) {
            cam_angle_y -= 0.02;
        }
        if is_key_down(KeyCode::Right) {
            cam_angle_y += 0.02;
        }
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Z) {
            cam_dist -= 1.0;
        }
        if is_key_down(KeyCode::S) {
            cam_dist += 1.0;
        }

        // Clamp camera
        cam_angle_x = cam_angle_x.clamp(0.1, 1.5);
        cam_dist = cam_dist.max(10.0);

        // Simulation
        if raining {
            erosion::erode(&mut terrain, droplets_per_frame, &params);
        }

        if replay_mode {
            if let Some(h) = &mut history {
                // Process multiple commits per frame to speed up?
                for _ in 0..5 {
                    if let Some(paths) = h.next_commit() {
                        commits_processed += 1;
                        for p in paths {
                            if let Some((x, y)) = terrain.get_coords(&p) {
                                // Rain heavily on modified file
                                for _ in 0..50 {
                                    erosion::erode_at(&mut terrain, x as f32, y as f32, &params);
                                }
                                // Flash color
                                terrain.colors[y * terrain.width + x] = RED;
                            }
                        }
                    } else {
                        replay_mode = false;
                        break;
                    }
                }
            }
        }

        // Color Decay
        for i in 0..terrain.colors.len() {
            let target = get_color(terrain.heightmap[i]);
            let current = terrain.colors[i];

            terrain.colors[i] = Color {
                r: current.r * 0.95 + target.r * 0.05,
                g: current.g * 0.95 + target.g * 0.05,
                b: current.b * 0.95 + target.b * 0.05,
                a: 1.0,
            };
        }

        // Always rebuild mesh (for colors and erosion)
        mesh = build_mesh(&terrain, scale);

        // Render
        clear_background(LIGHTGRAY);

        let cam_pos = vec3(
            cam_angle_y.cos() * cam_dist,
            cam_dist * cam_angle_x.sin(),
            cam_angle_y.sin() * cam_dist,
        );

        let target = vec3(terrain.width as f32 / 2.0, 0.0, terrain.height as f32 / 2.0);

        set_camera(&Camera3D {
            position: cam_pos + target,
            target,
            up: vec3(0.0, 1.0, 0.0),
            fovy: 45.0,
            ..Default::default()
        });

        // Draw Terrain
        draw_mesh(&mesh);

        set_default_camera();

        // UI
        draw_text(
            &format!(
                "Code Canyon: {} files mapped to {}x{} grid",
                files.len(),
                terrain.width,
                terrain.height
            ),
            10.0,
            20.0,
            30.0,
            BLACK,
        );

        let status_text = if replay_mode {
            format!("REPLAYING HISTORY: {}/{}", commits_processed, total_commits)
        } else if raining {
            "RAINING (Eroding)".to_string()
        } else {
            "PAUSED".to_string()
        };

        draw_text(
            &format!("Status: {}", status_text),
            10.0,
            50.0,
            30.0,
            if replay_mode { RED } else if raining { BLUE } else { BLACK },
        );

        draw_text(
            "Controls: [Space] Rain | [H] Replay History | [R] Reset | [W/S] Zoom",
            10.0,
            80.0,
            20.0,
            DARKGRAY,
        );

        next_frame().await;
    }
}

fn build_mesh(terrain: &Terrain, scale: f32) -> Mesh {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let w = terrain.width;
    let h = terrain.height;

    for y in 0..h {
        for x in 0..w {
            let height = terrain.heightmap[y * w + x] * scale;
            vertices.push(Vertex {
                position: vec3(x as f32, height, y as f32),
                uv: vec2(x as f32 / w as f32, y as f32 / h as f32),
                color: terrain.colors[y * w + x].into(),
                normal: vec4(0.0, 1.0, 0.0, 0.0),
            });
        }
    }

    for y in 0..h - 1 {
        for x in 0..w - 1 {
            let i00 = (y * w + x) as u16;
            let i10 = (y * w + (x + 1)) as u16;
            let i01 = ((y + 1) * w + x) as u16;
            let i11 = ((y + 1) * w + (x + 1)) as u16;

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

fn get_color(height: f32) -> Color {
    if height < 2.0 {
        BLUE // Water/Sediment
    } else if height < 5.0 {
        BEIGE // Sand
    } else if height < 15.0 {
        GREEN // Grass
    } else if height < 30.0 {
        DARKGRAY // Rock
    } else {
        WHITE // Snow
    }
}
