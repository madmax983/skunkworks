use macroquad::miniquad::{
    BlendFactor, BlendState, BlendValue, Comparison, Equation, PrimitiveType,
};
use macroquad::prelude::*;
use rayon::prelude::*;

mod rossler;
use rossler::RosslerSystem;

const PARTICLE_COUNT: usize = 100_000;
const BATCH_SIZE: usize = 60_000;

// Simple point shader
const VERTEX_SHADER: &str = r#"
#version 100
attribute vec3 position;
attribute vec4 color0;

varying lowp vec4 color;

uniform mat4 Model;
uniform mat4 Projection;
uniform mat4 View;

void main() {
    gl_Position = Projection * View * Model * vec4(position, 1.0);
    gl_PointSize = 1.5;
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

fn window_conf() -> Conf {
    Conf {
        window_title: "Genesis: Rossler Market".to_owned(),
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut particles: Vec<RosslerSystem> = (0..PARTICLE_COUNT)
        .map(|i| {
            let offset = (i as f32 / PARTICLE_COUNT as f32) * 0.1;
            RosslerSystem::new(
                Vec3::new(1.0 + offset, 1.0 - offset, 1.0 + offset),
                0.2,
                0.2,
                5.7,
            )
        })
        .collect();

    // Camera
    let mut cam_dist = 40.0;
    let mut cam_yaw: f32 = 0.0;
    let mut cam_pitch: f32 = 0.5;

    // Load Material
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

    // Initial Parameters
    let defaults = RosslerSystem::default_chaotic();
    let mut target_a = defaults.a;
    let mut target_b = defaults.b;
    let mut target_c = defaults.c;

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

        // Input
        if is_key_down(KeyCode::Left) {
            cam_yaw -= 2.0 * dt;
        }
        if is_key_down(KeyCode::Right) {
            cam_yaw += 2.0 * dt;
        }
        if is_key_down(KeyCode::Up) {
            cam_pitch = (cam_pitch + 2.0 * dt).clamp(-1.5, 1.5);
        }
        if is_key_down(KeyCode::Down) {
            cam_pitch = (cam_pitch - 2.0 * dt).clamp(-1.5, 1.5);
        }
        if is_key_down(KeyCode::W) {
            cam_dist -= 10.0 * dt;
        }
        if is_key_down(KeyCode::S) {
            cam_dist += 10.0 * dt;
        }

        // Parameter Control
        if is_key_down(KeyCode::U) {
            target_a += 0.1 * dt;
        }
        if is_key_down(KeyCode::J) {
            target_a -= 0.1 * dt;
        }
        if is_key_down(KeyCode::I) {
            target_b += 0.1 * dt;
        }
        if is_key_down(KeyCode::K) {
            target_b -= 0.1 * dt;
        }
        if is_key_down(KeyCode::O) {
            target_c += 1.0 * dt;
        }
        if is_key_down(KeyCode::L) {
            target_c -= 1.0 * dt;
        }

        if is_key_pressed(KeyCode::R) {
            let defaults = RosslerSystem::default_chaotic();
            particles.par_iter_mut().enumerate().for_each(|(i, p)| {
                let offset = (i as f32 / PARTICLE_COUNT as f32) * 0.1;
                *p = RosslerSystem::new(
                    Vec3::new(1.0 + offset, 1.0 - offset, 1.0 + offset),
                    defaults.a,
                    defaults.b,
                    defaults.c,
                );
            });
            target_a = defaults.a;
            target_b = defaults.b;
            target_c = defaults.c;
        }

        // Update Physics
        particles.par_iter_mut().for_each(|p| {
            // Apply targets smoothly
            p.a = p.a + (target_a - p.a) * dt * 2.0;
            p.b = p.b + (target_b - p.b) * dt * 2.0;
            p.c = p.c + (target_c - p.c) * dt * 2.0;
            p.step(dt);
        });

        // Render to Texture (Feedback Loop)
        {
            set_camera(&Camera2D {
                zoom: vec2(2.0 / width as f32, 2.0 / height as f32),
                target: vec2(0.0, 0.0),
                render_target: Some(trails_a.clone()),
                ..Default::default()
            });

            // Draw previous frame dimmed
            draw_texture_ex(
                &trails_b.texture,
                -width as f32 / 2.0,
                -height as f32 / 2.0,
                Color::new(0.96, 0.94, 0.92, 0.98), // Decay color (slightly warm)
                DrawTextureParams {
                    dest_size: Some(vec2(width as f32, height as f32)),
                    ..Default::default()
                },
            );

            // Draw Particles
            let cam_pos = vec3(
                cam_dist * cam_yaw.cos() * cam_pitch.cos(),
                cam_dist * cam_pitch.sin(),
                cam_dist * cam_yaw.sin() * cam_pitch.cos(),
            );

            set_camera(&Camera3D {
                position: cam_pos,
                target: vec3(0.0, 0.0, 10.0), // Look at center of attractor (roughly)
                up: vec3(0.0, 1.0, 0.0),
                render_target: Some(trails_a.clone()),
                ..Default::default()
            });

            gl_use_material(&material);

            for (i, mesh) in meshes.iter_mut().enumerate() {
                let start = i * BATCH_SIZE;
                let end = (start + BATCH_SIZE).min(particles.len());

                mesh.vertices.clear();
                mesh.indices.clear();

                for (idx, p) in particles[start..end].iter().enumerate() {
                    // Color based on velocity/instability
                    let speed = (p.pos.length() / 20.0).clamp(0.0, 1.0);
                    let color = Color::new(0.2 + speed * 0.8, 0.5 - speed * 0.3, 1.0 - speed, 0.8);

                    mesh.vertices.push(Vertex {
                        position: p.pos,
                        uv: Vec2::ZERO,
                        color: color.into(),
                        normal: Vec4::ZERO,
                    });
                    mesh.indices.push(idx as u16);
                }
                draw_mesh(mesh);
            }
            gl_use_default_material();
        }

        // Render to Screen
        set_default_camera();
        clear_background(BLACK);

        draw_texture_ex(
            &trails_a.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                flip_y: true,
                ..Default::default()
            },
        );

        // Swap buffers
        std::mem::swap(&mut trails_a, &mut trails_b);

        // HUD
        draw_text("Genesis: Rossler Market", 10.0, 30.0, 30.0, WHITE);
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 50.0, 20.0, LIGHTGRAY);
        draw_text(
            &format!("Particles: {}", PARTICLE_COUNT),
            10.0,
            70.0,
            20.0,
            LIGHTGRAY,
        );

        draw_text(
            "Market Parameters (Controls: U/J, I/K, O/L)",
            10.0,
            100.0,
            20.0,
            GOLD,
        );
        draw_text(
            &format!("Interest Rate (a): {:.3}", target_a),
            10.0,
            120.0,
            20.0,
            RED,
        );
        draw_text(
            &format!("Inflation (b): {:.3}", target_b),
            10.0,
            140.0,
            20.0,
            BLUE,
        );
        draw_text(
            &format!("Reserve Req (c): {:.3}", target_c),
            10.0,
            160.0,
            20.0,
            GREEN,
        );

        let chaos_metric = if let Some(p) = particles.first() {
            p.pos.length()
        } else {
            0.0
        };
        draw_text(
            &format!("Market Volatility: {:.2}", chaos_metric),
            10.0,
            190.0,
            20.0,
            if chaos_metric > 30.0 { RED } else { WHITE },
        );

        if chaos_metric > 50.0 {
            draw_text(
                "CRASH IMMINENT",
                screen_width() / 2.0 - 100.0,
                screen_height() / 2.0,
                40.0,
                RED,
            );
        }

        next_frame().await
    }
}
