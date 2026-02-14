use macroquad::prelude::*;
use std::env;

mod map;
mod history;
mod erosion;

use map::{Terrain, VoxelType};
use history::HistoryLoader;
use erosion::erode;

#[macroquad::main("Fossil Canyon")]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() > 1 { &args[1] } else { "." };

    // Setup Terrain
    let mut terrain = Terrain::new(64, 64);

    // Load History
    match HistoryLoader::new(path) {
        Ok(loader) => {
            println!("Loading history from {}...", path);
            if let Err(e) = loader.load_into_terrain(&mut terrain) {
                eprintln!("Error loading history: {}", e);
            }
        },
        Err(e) => {
             eprintln!("Could not open git repo at {}: {}", path, e);
             eprintln!("Creating dummy terrain...");
             // Create dummy
             for _ in 0..1000 {
                 let x = rand::gen_range(0, terrain.width);
                 let y = rand::gen_range(0, terrain.height);
                 use map::Voxel;
                 terrain.push_voxel(x, y, Voxel::new(VoxelType::Rock, DARKGRAY));
             }
        }
    }

    // Camera State
    let mut cam_pos = vec3(32.0, 50.0, 32.0);
    let mut yaw: f32 = 0.0;
    let mut pitch: f32 = -0.5;

    let mut raining = false;
    let droplets_per_frame = 500;

    loop {
        // --- Input ---
        if is_key_down(KeyCode::W) {
            cam_pos += vec3(yaw.sin(), 0.0, yaw.cos()) * 0.5;
        }
        if is_key_down(KeyCode::S) {
            cam_pos -= vec3(yaw.sin(), 0.0, yaw.cos()) * 0.5;
        }
        if is_key_down(KeyCode::A) {
            cam_pos -= vec3(yaw.cos(), 0.0, -yaw.sin()) * 0.5;
        }
        if is_key_down(KeyCode::D) {
            cam_pos += vec3(yaw.cos(), 0.0, -yaw.sin()) * 0.5;
        }
        if is_key_down(KeyCode::Space) {
            cam_pos.y += 0.5;
        }
        if is_key_down(KeyCode::LeftShift) {
            cam_pos.y -= 0.5;
        }

        if is_key_down(KeyCode::Left) {
            yaw -= 0.05;
        }
        if is_key_down(KeyCode::Right) {
            yaw += 0.05;
        }
        if is_key_down(KeyCode::Up) {
            pitch += 0.05;
        }
        if is_key_down(KeyCode::Down) {
            pitch -= 0.05;
        }

        if is_key_pressed(KeyCode::R) {
            raining = !raining;
        }

        // --- Simulation ---
        if raining {
            erode(&mut terrain, droplets_per_frame);
        }

        // --- Rendering ---
        clear_background(LIGHTGRAY);

        set_camera(&Camera3D {
            position: cam_pos,
            up: vec3(0.0, 1.0, 0.0),
            target: cam_pos + vec3(
                    yaw.sin() * pitch.cos(),
                    pitch.sin(),
                    yaw.cos() * pitch.cos(),
                ),
            ..Default::default()
        });

        // Construct mesh
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut idx = 0;

        for y in 0..terrain.height {
            for x in 0..terrain.width {
                if let Some(voxel) = terrain.peek_voxel(x, y) {
                    let h = terrain.get_height(x, y) as f32;
                    let color = voxel.color;

                    let xf = x as f32;
                    let yf = h; // y-up in 3D is height
                    let zf = y as f32; // z is depth

                    // Top Face
                    vertices.push(vertex(xf, yf, zf, color));
                    vertices.push(vertex(xf + 1.0, yf, zf, color));
                    vertices.push(vertex(xf + 1.0, yf, zf + 1.0, color));
                    vertices.push(vertex(xf, yf, zf + 1.0, color));

                    indices.push(idx);
                    indices.push(idx + 1);
                    indices.push(idx + 2);
                    indices.push(idx);
                    indices.push(idx + 2);
                    indices.push(idx + 3);
                    idx += 4;

                    // Sides (Optional optimization: only draw if neighbor is much lower)
                    // For now, let's keep it minimal for performance.
                }
            }
        }

        let mesh = Mesh {
            vertices,
            indices,
            texture: None,
        };
        draw_mesh(&mesh);

        set_default_camera();

        draw_text("Fossil Canyon ⚛️🦴", 10.0, 30.0, 30.0, BLACK);
        draw_text(&format!("Raining: {}", raining), 10.0, 60.0, 20.0, if raining { BLUE } else { BLACK });
        draw_text("Controls: [R] Rain | [WASD] Move | [Arrows] Look", 10.0, 90.0, 20.0, DARKGRAY);

        next_frame().await;
    }
}

fn vertex(x: f32, y: f32, z: f32, color: Color) -> Vertex {
    Vertex {
        position: vec3(x, y, z),
        uv: vec2(0.0, 0.0),
        color: color.into(),
        normal: vec4(0.0, 1.0, 0.0, 0.0),
    }
}
