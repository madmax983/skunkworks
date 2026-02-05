use macroquad::prelude::*;
use std::sync::{Arc, RwLock};

mod audio;
mod heightmap;
mod physics;

use audio::{init_audio, Oscillator, SharedState};
use heightmap::generate_text_heightmap;
use physics::{Body, Universe, G};

const MAP_WIDTH: u32 = 200;
const MAP_HEIGHT: u32 = 200;
const TERRAIN_SCALE: f32 = 5.0; // Scale up to match orbital distances

fn window_conf() -> Conf {
    Conf {
        window_title: "Symphonic Terrain".to_owned(),
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // 1. Generate Terrain
    // We use "SYMPHONY" as the seed text
    let font_data = include_bytes!("../assets/DejaVuSans.ttf");
    let heightmap = generate_text_heightmap("SYMPHONY", font_data, MAP_WIDTH, MAP_HEIGHT);

    // Build Mesh
    let mut mesh = Mesh {
        vertices: Vec::new(),
        indices: Vec::new(),
        texture: None,
    };

    let offset_x = MAP_WIDTH as f32 / 2.0;
    let offset_z = MAP_HEIGHT as f32 / 2.0;

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let h = heightmap.get(x, y);

            // Color mapping
            let color = if h < 2.0 {
                BLUE // Void/Water
            } else if h < 5.0 {
                PURPLE // Lowlands
            } else if h < 15.0 {
                ORANGE // Mid-range
            } else {
                GOLD // Text Peaks
            };

            mesh.vertices.push(Vertex {
                position: vec3(
                    (x as f32 - offset_x) * TERRAIN_SCALE,
                    h * 2.0, // Vertical exaggeration
                    (y as f32 - offset_z) * TERRAIN_SCALE,
                ),
                uv: vec2(x as f32 / MAP_WIDTH as f32, y as f32 / MAP_HEIGHT as f32),
                color: color.into(),
                normal: vec4(0.0, 1.0, 0.0, 0.0),
            });
        }
    }

    for y in 0..MAP_HEIGHT - 1 {
        for x in 0..MAP_WIDTH - 1 {
            let i = (y * MAP_WIDTH + x) as u16;
            let next_row = ((y + 1) * MAP_WIDTH + x) as u16;
            mesh.indices.push(i);
            mesh.indices.push(next_row);
            mesh.indices.push(i + 1);
            mesh.indices.push(i + 1);
            mesh.indices.push(next_row);
            mesh.indices.push(next_row + 1);
        }
    }

    // 2. Physics Setup
    let mut universe = Universe::new();

    // Central "Conductor" Star (Invisible or bright light?)
    universe.add_body(Body::new(Vec3::new(0.0, 100.0, 0.0), 5000.0, 20.0, YELLOW));

    // Orbiting Notes
    let r1 = 200.0;
    let v1 = (G * 5000.0 / r1).sqrt();
    universe.add_body(
        Body::new(Vec3::new(r1, 100.0, 0.0), 10.0, 8.0, SKYBLUE)
            .with_velocity(Vec3::new(0.0, 0.0, v1)),
    );

    let r2 = 350.0;
    let v2 = (G * 5000.0 / r2).sqrt();
    universe.add_body(
        Body::new(Vec3::new(-r2, 120.0, 0.0), 20.0, 12.0, MAGENTA)
            .with_velocity(Vec3::new(0.0, 0.0, -v2)),
    );

    // 3. Audio Setup
    let audio_state = Arc::new(RwLock::new(SharedState::new()));
    #[allow(unused_variables)]
    let audio_stream = init_audio(audio_state.clone()); // Ignoring error for sandbox

    // Camera
    let mut cam_angle_x: f32 = 0.5;
    let mut cam_angle_y: f32 = 1.0;
    let mut cam_dist: f32 = 600.0;

    loop {
        clear_background(BLACK);

        // Input
        if is_key_down(KeyCode::Left) {
            cam_angle_y -= 0.02;
        }
        if is_key_down(KeyCode::Right) {
            cam_angle_y += 0.02;
        }
        if is_key_down(KeyCode::Up) {
            cam_angle_x = (cam_angle_x + 0.02).min(1.5);
        }
        if is_key_down(KeyCode::Down) {
            cam_angle_x = (cam_angle_x - 0.02).max(0.1);
        }
        if is_key_down(KeyCode::W) {
            cam_dist -= 5.0;
        }
        if is_key_down(KeyCode::S) {
            cam_dist += 5.0;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            // Simple spawn logic: Spawn at camera position? No, just random orbit.
            let r = rand::gen_range(150.0, 450.0);
            let angle: f32 = rand::gen_range(0.0, 6.28);
            let x = r * angle.cos();
            let z = r * angle.sin();
            let v = (G * 5000.0 / r).sqrt();
            let vx = -angle.sin() * v;
            let vz = angle.cos() * v;

            universe.add_body(
                Body::new(Vec3::new(x, 150.0, z), 5.0, 5.0, WHITE)
                    .with_velocity(Vec3::new(vx, 0.0, vz)),
            );
        }

        // Update Physics
        universe.step(0.05);

        // Update Audio & Colors based on Terrain
        if let Ok(mut state) = audio_state.write() {
            state.oscillators.clear();

            for (i, body) in universe.bodies.iter_mut().enumerate() {
                if i == 0 {
                    continue;
                } // Skip central star

                // Map world pos to grid
                let grid_x = (body.pos.x / TERRAIN_SCALE + offset_x).round() as i32;
                let grid_z = (body.pos.z / TERRAIN_SCALE + offset_z).round() as i32;

                let mut terrain_height = 0.0;
                if grid_x >= 0
                    && grid_x < MAP_WIDTH as i32
                    && grid_z >= 0
                    && grid_z < MAP_HEIGHT as i32
                {
                    terrain_height = heightmap.get(grid_x as u32, grid_z as u32);
                }

                // Audio Logic
                // Frequency based on terrain height (Text peaks = High Pitch)
                // Base 200Hz + Height * 20
                let freq = 200.0 + terrain_height * 40.0;

                // Amplitude based on vertical distance to terrain
                let ground_y = terrain_height * 2.0; // Match mesh vertical scale
                let dist_y = (body.pos.y - ground_y).abs();
                let amp = (50.0 / (dist_y + 10.0)).clamp(0.0, 0.3);

                state.oscillators.push(Oscillator {
                    frequency: freq,
                    amplitude: amp,
                });

                // Visual feedback: Body glows if over "loud" terrain (text)
                if terrain_height > 10.0 {
                    body.color = RED;
                } else {
                    body.color = body.base_color;
                }
            }
        }

        // Render
        let cam_pos = vec3(
            cam_dist * cam_angle_x.cos() * cam_angle_y.sin(),
            cam_dist * cam_angle_x.sin(),
            cam_dist * cam_angle_x.cos() * cam_angle_y.cos(),
        );

        set_camera(&Camera3D {
            position: cam_pos,
            target: vec3(0.0, 50.0, 0.0),
            up: vec3(0.0, 1.0, 0.0),
            ..Default::default()
        });

        draw_mesh(&mesh);

        // Draw bodies
        for body in &universe.bodies {
            draw_sphere(body.pos, body.radius, None, body.color);
        }

        set_default_camera();
        draw_text("Symphonic Terrain", 10.0, 20.0, 30.0, WHITE);
        draw_text("Click to spawn notes", 10.0, 40.0, 20.0, GRAY);
        draw_text(
            format!("Bodies: {}", universe.bodies.len()).as_str(),
            10.0,
            60.0,
            20.0,
            GRAY,
        );

        next_frame().await
    }
}
