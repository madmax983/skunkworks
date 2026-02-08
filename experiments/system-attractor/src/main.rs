use macroquad::prelude::*;
use macroquad::miniquad::{PrimitiveType, BlendState, BlendFactor, BlendValue, Equation, Comparison};
use system_attractor::simulation::Simulation;
use system_attractor::lyapunov::LyapunovMonitor;
use system_attractor::audio::Synth;

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
    ).unwrap();

    // Meshes for batch rendering
    let num_batches = (PARTICLE_COUNT + BATCH_SIZE - 1) / BATCH_SIZE;
    let mut meshes: Vec<Mesh> = (0..num_batches).map(|_| {
        Mesh {
            vertices: Vec::with_capacity(BATCH_SIZE),
            indices: Vec::with_capacity(BATCH_SIZE),
            texture: None,
        }
    }).collect();

    loop {
        let dt = get_frame_time().min(0.05);

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
        if is_key_down(KeyCode::Left) { cam_yaw -= 0.02; }
        if is_key_down(KeyCode::Right) { cam_yaw += 0.02; }
        if is_key_down(KeyCode::Up) { cam_pitch = (cam_pitch + 0.02).clamp(-1.5, 1.5); }
        if is_key_down(KeyCode::Down) { cam_pitch = (cam_pitch - 0.02).clamp(-1.5, 1.5); }
        if is_key_down(KeyCode::W) { cam_dist -= 0.5; }
        if is_key_down(KeyCode::S) { cam_dist += 0.5; }
        if is_key_pressed(KeyCode::R) {
            sim.reset();
            lyapunov = LyapunovMonitor::new(vec3(1.0, 1.0, 1.0), 1e-4);
        }

        clear_background(BLACK);

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
                mesh.vertices.push(Vertex {
                    position: p.pos,
                    uv: Vec2::ZERO,
                    color: p.color.into(),
                    normal: vec4(0.0, 0.0, 1.0, 0.0),
                });
                mesh.indices.push(idx as u16);
            }

            draw_mesh(mesh);
        }

        gl_use_default_material();

        // Draw Lyapunov visual
        draw_sphere(lyapunov.reference, 1.0, None, YELLOW);
        draw_sphere(lyapunov.shadow, 0.5, None, ORANGE);

        draw_grid(20, 1.0, DARKGRAY, GRAY);

        set_default_camera();

        draw_text("System Attractor", 10.0, 30.0, 30.0, WHITE);
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 50.0, 20.0, LIGHTGRAY);
        draw_text(&format!("Particles: {}", PARTICLE_COUNT), 10.0, 70.0, 20.0, LIGHTGRAY);

        let p = &sim.monitor.params;
        draw_text(&format!("Sigma (CPU): {:.2}", p.sigma), 10.0, 100.0, 20.0, RED);
        draw_text(&format!("Rho (RAM): {:.2}", p.rho), 10.0, 120.0, 20.0, BLUE);
        draw_text(&format!("Beta: {:.2}", p.beta), 10.0, 140.0, 20.0, GREEN);

        draw_text(&format!("Lyapunov Exp: {:.4}", lyapunov.get_exponent()), 10.0, 170.0, 20.0, YELLOW);

        if synth.is_some() {
             draw_text("Audio: Active", 10.0, 190.0, 20.0, GOLD);
        } else {
             draw_text("Audio: Disabled", 10.0, 190.0, 20.0, DARKGRAY);
        }

        next_frame().await
    }
}
