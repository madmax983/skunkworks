use macroquad::miniquad::{
    BlendFactor, BlendState, BlendValue, Comparison, Equation, PrimitiveType,
};
use macroquad::prelude::*;
use system_attractor::audio::Synth;
use system_attractor::lyapunov::LyapunovMonitor;
use system_attractor::simulation::{Particle, Simulation};

const PARTICLE_COUNT: usize = 500_000;
const BATCH_SIZE: usize = 60_000;

const VERTEX_SHADER: &str = r#"
#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;

varying lowp vec4 color;

uniform mat4 Model;
uniform mat4 Projection;
uniform mat4 View;

void main() {
    gl_Position = Projection * View * Model * vec4(position, 1.0);
    gl_PointSize = 2.0;
    color = color0;
}
"#;

const FRAGMENT_SHADER: &str = r#"
#version 100
varying lowp vec4 color;

void main() {
    gl_FragColor = color;
}
"#;

fn to_vertex(p: &Particle) -> Vertex {
    Vertex {
        position: p.pos,
        uv: Vec2::ZERO,
        color: p.color.into(),
        normal: vec4(0.0, 0.0, 1.0, 0.0),
    }
}

#[macroquad::main("System Attractor")]
async fn main() {
    let mut sim = Simulation::new(PARTICLE_COUNT);

    // Lyapunov monitor (start at random pos near origin)
    let mut lyapunov = LyapunovMonitor::new(vec3(1.0, 1.0, 1.0), 1e-4);

    // Audio
    let mut synth = Synth::new().ok(); // Optional

    // Camera
    let mut cam_dist = 80.0;
    let mut cam_yaw: f32 = 0.0;
    let mut cam_pitch: f32 = 0.0;

    // Load Point Cloud Material
    let pipeline_params = PipelineParams {
        primitive_type: PrimitiveType::Points,
        depth_write: false,
        depth_test: Comparison::Always,
        color_blend: Some(BlendState::new(
            Equation::Add,
            BlendFactor::Value(BlendValue::SourceAlpha),
            BlendFactor::One,
        )),
        ..Default::default()
    };

    let material = load_material(
        ShaderSource::Glsl {
            vertex: VERTEX_SHADER,
            fragment: FRAGMENT_SHADER,
        },
        MaterialParams {
            pipeline_params,
            ..Default::default()
        },
    )
    .unwrap();

    // Meshes for batch rendering
    let num_batches = PARTICLE_COUNT.div_ceil(BATCH_SIZE);
    let mut meshes: Vec<Mesh> = (0..num_batches)
        .map(|_| Mesh {
            vertices: Vec::with_capacity(BATCH_SIZE),
            indices: Vec::with_capacity(BATCH_SIZE),
            texture: None,
        })
        .collect();

    // Trails Render Targets
    let mut width = screen_width() as i32;
    let mut height = screen_height() as i32;
    let mut trails_a = render_target(width as u32, height as u32);
    let mut trails_b = render_target(width as u32, height as u32);
    trails_a.texture.set_filter(FilterMode::Linear);
    trails_b.texture.set_filter(FilterMode::Linear);

    loop {
        let dt = get_frame_time().min(0.05);

        // Handle Resize
        let new_width = screen_width() as i32;
        let new_height = screen_height() as i32;
        if new_width != width || new_height != height {
            width = new_width;
            height = new_height;
            trails_a = render_target(width as u32, height as u32);
            trails_b = render_target(width as u32, height as u32);
            trails_a.texture.set_filter(FilterMode::Linear);
            trails_b.texture.set_filter(FilterMode::Linear);
        }

        sim.update(dt);
        lyapunov.update(&sim.monitor.params, dt);

        if let Some(synth) = &mut synth {
            if let Some(p) = sim.particles.first() {
                let z = p.pos.z;
                let speed = p.vel.length();
                let freq = 100.0 + (z * 20.0).clamp(0.0, 2000.0);
                let amp = (speed / 100.0).clamp(0.0, 0.5);
                synth.update(freq, amp);
            }
        }

        // Camera Input
        if is_key_down(KeyCode::Left) {
            cam_yaw -= 0.02;
        }
        if is_key_down(KeyCode::Right) {
            cam_yaw += 0.02;
        }
        if is_key_down(KeyCode::Up) {
            cam_pitch = (cam_pitch + 0.02).clamp(-1.5, 1.5);
        }
        if is_key_down(KeyCode::Down) {
            cam_pitch = (cam_pitch - 0.02).clamp(-1.5, 1.5);
        }
        if is_key_down(KeyCode::W) {
            cam_dist -= 0.5;
        }
        if is_key_down(KeyCode::S) {
            cam_dist += 0.5;
        }
        if is_key_pressed(KeyCode::R) {
            sim.reset();
            lyapunov = LyapunovMonitor::new(vec3(1.0, 1.0, 1.0), 1e-4);
        }

        // --- RENDER PASS 1: Feedback & Particles -> trails_a ---
        {
            // 1. Draw previous frame (trails_b) dimmed
            set_camera(&Camera2D {
                zoom: vec2(2.0 / width as f32, 2.0 / height as f32),
                target: vec2(0.0, 0.0),
                render_target: Some(trails_a.clone()),
                ..Default::default()
            });

            // Draw full screen quad of previous frame
            draw_texture_ex(
                &trails_b.texture,
                -width as f32 / 2.0,
                -height as f32 / 2.0,
                Color::new(0.96, 0.96, 0.96, 1.0), // Fade factor
                DrawTextureParams {
                    dest_size: Some(vec2(width as f32, height as f32)),
                    ..Default::default()
                },
            );

            // 2. Draw Particles
            let cam_pos = vec3(
                cam_dist * cam_yaw.cos() * cam_pitch.cos(),
                cam_dist * cam_pitch.sin(),
                cam_dist * cam_yaw.sin() * cam_pitch.cos(),
            );
            let target = vec3(0.0, 0.0, 25.0);

            set_camera(&Camera3D {
                position: cam_pos,
                target,
                up: vec3(0.0, 1.0, 0.0),
                render_target: Some(trails_a.clone()),
                ..Default::default()
            });

            gl_use_material(&material);

            for (i, mesh) in meshes.iter_mut().enumerate() {
                let start = i * BATCH_SIZE;
                let end = (start + BATCH_SIZE).min(sim.particles.len());
                let slice = &sim.particles[start..end];

                mesh.vertices.clear();
                mesh.indices.clear();

                for (idx, p) in slice.iter().enumerate() {
                    mesh.vertices.push(to_vertex(p));
                    mesh.indices.push(idx as u16);
                }

                draw_mesh(mesh);
            }

            gl_use_default_material();

            // Draw Lyapunov visual
            draw_sphere(lyapunov.reference, 1.0, None, YELLOW);
            draw_sphere(lyapunov.shadow, 0.5, None, ORANGE);

            draw_grid(20, 1.0, DARKGRAY, GRAY);
        }

        // --- RENDER PASS 2: Display trails_a to Screen ---
        set_default_camera();
        clear_background(BLACK);

        // Panic Mode / Shake
        let shake_intensity = if sim.monitor.params.color_shift > 0.5 {
            (sim.monitor.params.color_shift - 0.5) * 10.0
        } else {
            0.0
        };
        let shake_x = if shake_intensity > 0.0 {
            rand::gen_range(-1.0, 1.0) * shake_intensity
        } else {
            0.0
        };
        let shake_y = if shake_intensity > 0.0 {
            rand::gen_range(-1.0, 1.0) * shake_intensity
        } else {
            0.0
        };

        draw_texture_ex(
            &trails_a.texture,
            shake_x,
            shake_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                flip_y: true,
                ..Default::default()
            },
        );

        // Swap buffers for next frame
        std::mem::swap(&mut trails_a, &mut trails_b);

        draw_text("System Attractor", 10.0, 30.0, 30.0, WHITE);
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 50.0, 20.0, LIGHTGRAY);
        draw_text(
            &format!("Particles: {}", PARTICLE_COUNT),
            10.0,
            70.0,
            20.0,
            LIGHTGRAY,
        );

        let p = &sim.monitor.params;
        draw_text(
            &format!("Sigma (CPU): {:.2}", p.sigma),
            10.0,
            100.0,
            20.0,
            RED,
        );
        draw_text(&format!("Rho (RAM): {:.2}", p.rho), 10.0, 120.0, 20.0, BLUE);
        draw_text(&format!("Beta: {:.2}", p.beta), 10.0, 140.0, 20.0, GREEN);
        draw_text(
            &format!("Jitter (Swap): {:.3}", p.jitter),
            10.0,
            160.0,
            20.0,
            MAGENTA,
        );
        draw_text(
            &format!("Shift (Load): {:.2}", p.color_shift),
            10.0,
            180.0,
            20.0,
            Color::new(1.0, 0.5, 0.5, 1.0),
        );

        draw_text(
            &format!("Lyapunov Exp: {:.4}", lyapunov.get_exponent()),
            10.0,
            210.0,
            20.0,
            YELLOW,
        );

        if synth.is_some() {
            draw_text("Audio: Active", 10.0, 230.0, 20.0, GOLD);
        } else {
            draw_text("Audio: Disabled", 10.0, 230.0, 20.0, DARKGRAY);
        }

        next_frame().await
    }
}
