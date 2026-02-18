mod audio;
mod simulation;

use audio::{SonicEngine, Spectrum};
use simulation::HeapTerrain;
use macroquad::prelude::*;
use crossbeam_channel::unbounded;
use ::rand::Rng;

#[macroquad::main("Sonic Glacier")]
async fn main() {
    let (tx, rx) = unbounded();
    let engine = SonicEngine::new(tx);

    // Audio Setup
    start_audio(engine);

    // Simulation Setup
    let width = 64;
    let height = 64;
    let mut terrain = HeapTerrain::new(width, height);

    let mut position = vec3(32.0, 40.0, 32.0);
    let mut yaw: f32 = 0.0;
    let mut pitch: f32 = -0.5;

    let mut spectrum = Spectrum { low: 0.0, mid: 0.0, high: 0.0, raw: vec![] };
    let mut auto_mode = false;

    loop {
        // --- Input ---
        let dt = get_frame_time();

        if is_key_down(KeyCode::W) {
            position += vec3(yaw.sin(), 0.0, yaw.cos()) * 0.5;
        }
        if is_key_down(KeyCode::S) {
            position -= vec3(yaw.sin(), 0.0, yaw.cos()) * 0.5;
        }
        if is_key_down(KeyCode::A) {
            position -= vec3(yaw.cos(), 0.0, -yaw.sin()) * 0.5;
        }
        if is_key_down(KeyCode::D) {
            position += vec3(yaw.cos(), 0.0, -yaw.sin()) * 0.5;
        }
        if is_key_down(KeyCode::Space) {
            position.y += 0.5;
        }
        if is_key_down(KeyCode::LeftShift) {
            position.y -= 0.5;
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

        if is_key_pressed(KeyCode::M) {
            auto_mode = !auto_mode;
        }

        // --- Audio & Simulation ---
        // Receive latest spectrum
        while let Ok(s) = rx.try_recv() {
            spectrum = s;
        }

        // Apply Audio (Cold)
        // Low Freq: Deep freeze / Earthquakes?
        // Mid Freq: Standard freezing
        // High Freq: Sharp crystallization

        let mut rng = ::rand::thread_rng();

        // Randomly sample points to apply audio effects
        // Or apply broadly?
        // Let's apply broadly but weighted by noise/randomness
        for _ in 0..100 {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);

            // Low freq -> shifts bedrock slightly (vibration)
            if spectrum.low > 10.0 {
                 if rng.gen_bool(0.1) {
                     terrain.bedrock[y * width + x] += (rng.gen::<f32>() - 0.5) * 0.1;
                 }
            }

            // Mid freq -> Freezes water
            if spectrum.mid > 5.0 {
                 terrain.apply_cold(x, y, spectrum.mid * 0.1);
            }

            // High freq -> Creates ice spikes (adds ice directly)
            if spectrum.high > 2.0 {
                 terrain.ice[y * width + x] += spectrum.high * 0.05;
            }
        }

        // Allocations (Heat)
        // User can trigger heat with keys, or auto mode
        if is_key_down(KeyCode::H) || (auto_mode && rng.gen_bool(0.1)) {
            // Heat burst
            for _ in 0..5 {
                let x = rng.gen_range(0..width);
                let y = rng.gen_range(0..height);
                terrain.apply_heat(x, y, 5.0);
            }
        }

        terrain.tick();

        // --- Rendering ---
        clear_background(LIGHTGRAY);

        set_camera(&Camera3D {
            position,
            up: vec3(0.0, 1.0, 0.0),
            target: position
                + vec3(
                    yaw.sin() * pitch.cos(),
                    pitch.sin(),
                    yaw.cos() * pitch.cos(),
                ),
            ..Default::default()
        });

        // Build Vertex Buffers
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut idx = 0;

        for y in 0..height - 1 {
            for x in 0..width - 1 {
                let h00 = terrain.get_bedrock(x, y);
                let h10 = terrain.get_bedrock(x + 1, y);
                let h01 = terrain.get_bedrock(x, y + 1);
                let h11 = terrain.get_bedrock(x + 1, y + 1);

                let i00 = terrain.get_ice(x, y);
                let i10 = terrain.get_ice(x + 1, y);
                let i01 = terrain.get_ice(x, y + 1);
                let i11 = terrain.get_ice(x + 1, y + 1);

                let w00 = terrain.get_water(x, y);
                let w10 = terrain.get_water(x + 1, y);
                let w01 = terrain.get_water(x, y + 1);
                let w11 = terrain.get_water(x + 1, y + 1);

                // Bedrock Layer
                let bedrock_color = Color::new(0.4, 0.3, 0.2, 1.0);
                vertices.push(vertex(x as f32, h00, y as f32, bedrock_color));
                vertices.push(vertex((x + 1) as f32, h10, y as f32, bedrock_color));
                vertices.push(vertex((x + 1) as f32, h11, (y + 1) as f32, bedrock_color));
                vertices.push(vertex(x as f32, h01, (y + 1) as f32, bedrock_color));

                indices.push(idx);
                indices.push(idx + 1);
                indices.push(idx + 2);
                indices.push(idx);
                indices.push(idx + 2);
                indices.push(idx + 3);
                idx += 4;

                // Ice Layer
                if i00 > 0.1 || i10 > 0.1 || i01 > 0.1 || i11 > 0.1 {
                    let color = Color::new(0.9, 0.95, 1.0, 0.9); // White/Blueish
                    vertices.push(vertex(x as f32, h00 + i00, y as f32, color));
                    vertices.push(vertex((x + 1) as f32, h10 + i10, y as f32, color));
                    vertices.push(vertex((x + 1) as f32, h11 + i11, (y + 1) as f32, color));
                    vertices.push(vertex(x as f32, h01 + i01, (y + 1) as f32, color));

                    indices.push(idx);
                    indices.push(idx + 1);
                    indices.push(idx + 2);
                    indices.push(idx);
                    indices.push(idx + 2);
                    indices.push(idx + 3);
                    idx += 4;
                }

                // Water Layer
                if w00 > 0.01 || w10 > 0.01 || w01 > 0.01 || w11 > 0.01 {
                    let color = Color::new(0.0, 0.4, 1.0, 0.5); // Blue/Transparent
                    let b00 = h00 + i00;
                    let b10 = h10 + i10;
                    let b01 = h01 + i01;
                    let b11 = h11 + i11;

                    vertices.push(vertex(x as f32, b00 + w00, y as f32, color));
                    vertices.push(vertex((x + 1) as f32, b10 + w10, y as f32, color));
                    vertices.push(vertex((x + 1) as f32, b11 + w11, (y + 1) as f32, color));
                    vertices.push(vertex(x as f32, b01 + w01, (y + 1) as f32, color));

                    indices.push(idx);
                    indices.push(idx + 1);
                    indices.push(idx + 2);
                    indices.push(idx);
                    indices.push(idx + 2);
                    indices.push(idx + 3);
                    idx += 4;
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
        draw_text("Sonic Glacier ⚛️❄️", 20.0, 30.0, 30.0, BLACK);
        draw_text(
            format!("FPS: {}", get_fps()).as_str(),
            20.0,
            60.0,
            20.0,
            BLACK,
        );
        draw_text(
            format!("Spectrum: L:{:.1} M:{:.1} H:{:.1}", spectrum.low, spectrum.mid, spectrum.high).as_str(),
            20.0,
            80.0,
            20.0,
            BLUE,
        );
        draw_text("WASD+Arrows to move. 'H' for Heat (Melt). 'M' Toggle Auto Heat.", 20.0, 100.0, 20.0, DARKGRAY);

        next_frame().await
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

#[cfg(feature = "audio")]
fn start_audio(engine: SonicEngine) {
    std::thread::spawn(move || {
        use rodio::{OutputStream, Sink};
        // Create the stream in this thread and block
        let stream_result = OutputStream::try_default();
        if let Ok((_stream, stream_handle)) = stream_result {
             if let Ok(sink) = Sink::try_new(&stream_handle) {
                sink.append(engine);
                sink.sleep_until_end();
             }
        }
    });
}

#[cfg(not(feature = "audio"))]
fn start_audio(engine: SonicEngine) {
    std::thread::spawn(move || {
        // Just consume the iterator to generate analysis events
        // Throttle it to approx real time
        let frame_time = std::time::Duration::from_secs_f32(1.0 / 44100.0);
        let start = std::time::Instant::now();
        let mut count = 0;

        for _ in engine {
            count += 1;
            if count % 1024 == 0 {
                // Sync roughly
                let expected = frame_time * count;
                let elapsed = start.elapsed();
                if elapsed < expected {
                    std::thread::sleep(expected - elapsed);
                }
            }
        }
    });
}
