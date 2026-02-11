use macroquad::prelude::*;
use std::sync::{Arc, RwLock};

mod audio;
mod heightmap;
mod network;
mod neuron;
mod crab_3d;

use audio::{init_audio, Oscillator, SharedState};
use heightmap::generate_text_heightmap;
use crab_3d::Crab3D;

const MAP_WIDTH: u32 = 200;
const MAP_HEIGHT: u32 = 200;
const TERRAIN_SCALE: f32 = 5.0; // Scale up to match orbital distances

fn window_conf() -> Conf {
    Conf {
        window_title: "Neuro Pathfinder".to_owned(),
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // 1. Generate Terrain
    // We use "PATHFINDER" as the seed text
    let font_data = include_bytes!("../assets/DejaVuSans.ttf");
    let heightmap = generate_text_heightmap("PATHFINDER", font_data, MAP_WIDTH, MAP_HEIGHT);

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

    // 2. Audio Setup
    let audio_state = Arc::new(RwLock::new(SharedState::new()));
    #[allow(unused_variables)]
    let audio_stream = init_audio(audio_state.clone()); // Ignoring error for sandbox

    // 3. Crab Setup
    let mut crab = Crab3D::new(vec3(0.0, 50.0, 0.0));
    let mut drive_current = 0.5;

    // Camera
    let mut cam_angle_x: f32 = 0.5;
    let mut cam_angle_y: f32 = 0.0;
    let mut cam_dist: f32 = 100.0;

    loop {
        clear_background(BLACK);

        let dt = get_frame_time();

        // Input
        if is_key_down(KeyCode::Left) {
            cam_angle_y -= 2.0 * dt;
        }
        if is_key_down(KeyCode::Right) {
            cam_angle_y += 2.0 * dt;
        }
        if is_key_down(KeyCode::Up) {
            drive_current = (drive_current + 1.0 * dt).min(5.0);
        }
        if is_key_down(KeyCode::Down) {
            drive_current = (drive_current - 1.0 * dt).max(0.0);
        }
        if is_key_down(KeyCode::W) {
            cam_dist -= 50.0 * dt;
        }
        if is_key_down(KeyCode::S) {
            cam_dist += 50.0 * dt;
        }

        // Update Crab
        crab.update(dt, drive_current, &heightmap);

        // Update Audio based on Crab
        if let Ok(mut state) = audio_state.write() {
            state.oscillators.clear();

            // Sonify Crab Height/Speed
            let height_freq = 200.0 + crab.pos.y * 10.0;
            let speed_amp = (crab.velocity.length() * 0.1).clamp(0.0, 0.5);

            state.oscillators.push(Oscillator {
                frequency: height_freq,
                amplitude: speed_amp,
            });

            // Sonify Leg Steps (Impacts?)
            // If leg is entering stance (spike), add click?
            // Simplified: Add tone for average neural activity
            let avg_activity: f32 = crab.legs.iter().map(|l| l.lift).sum::<f32>() / 6.0;
            state.oscillators.push(Oscillator {
                frequency: 100.0 + avg_activity * 500.0,
                amplitude: 0.1,
            });
        }

        // Camera Follow
        let target_pos = crab.pos;
        let cam_pos = target_pos + vec3(
            cam_dist * cam_angle_x.cos() * cam_angle_y.sin(),
            cam_dist * cam_angle_x.sin() + 20.0,
            cam_dist * cam_angle_x.cos() * cam_angle_y.cos(),
        );

        set_camera(&Camera3D {
            position: cam_pos,
            target: target_pos,
            up: vec3(0.0, 1.0, 0.0),
            ..Default::default()
        });

        draw_mesh(&mesh);

        // Draw Crab
        // Body
        draw_sphere(crab.pos, 5.0, None, RED);

        // Eyes?
        let body_rot = Quat::from_rotation_y(crab.yaw);
        let forward = body_rot * Vec3::Z;
        let eye_pos = crab.pos + forward * 4.0 + vec3(0.0, 2.0, 0.0);
        draw_sphere(eye_pos, 1.0, None, WHITE);

        // Legs
        for leg in &crab.legs {
            // Joint (Knee) logic for visualization
            // Simple IK: Midpoint lifted up
            let start = crab.pos + body_rot * leg.base_offset;
            let end = leg.current_pos;

            let mid = (start + end) * 0.5 + vec3(0.0, 10.0, 0.0); // Knee high up

            draw_line_3d(start, mid, GREEN);
            draw_line_3d(mid, end, DARKGREEN);

            draw_sphere(end, 1.5, None, BLUE); // Foot
        }

        set_default_camera();
        draw_text("Neuro Pathfinder", 10.0, 20.0, 30.0, WHITE);
        draw_text(format!("Drive: {:.2}", drive_current).as_str(), 10.0, 50.0, 20.0, WHITE);
        draw_text("Arrow Keys: Drive/Rotate Camera", 10.0, 80.0, 20.0, GRAY);

        next_frame().await
    }
}
