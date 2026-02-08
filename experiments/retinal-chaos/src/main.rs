mod neuron;
mod retina;
mod simulation;

use macroquad::prelude::*;
use macroquad::miniquad::{PrimitiveType, BlendState, BlendFactor, BlendValue, Equation, Comparison};
use simulation::{Particle, Simulation};
use retina::Retina;

const PARTICLE_COUNT: usize = 50_000;
const BATCH_SIZE: usize = 10_000;
const EYE_RES: usize = 128;

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

#[macroquad::main("Retinal Chaos")]
async fn main() {
    let mut sim = Simulation::new(PARTICLE_COUNT);
    let mut retina = Retina::new(EYE_RES, EYE_RES);

    // Render Target for the Eye
    let render_target = render_target(EYE_RES as u32, EYE_RES as u32);
    render_target.texture.set_filter(FilterMode::Nearest);

    // Textures for debug visualization
    let bipolar_texture = Texture2D::from_image(&Image::gen_image_color(EYE_RES as u16, EYE_RES as u16, BLACK));
    bipolar_texture.set_filter(FilterMode::Nearest);
    let ganglion_texture = Texture2D::from_image(&Image::gen_image_color(EYE_RES as u16, EYE_RES as u16, BLACK));
    ganglion_texture.set_filter(FilterMode::Nearest);

    let mut bipolar_image = Image::gen_image_color(EYE_RES as u16, EYE_RES as u16, BLACK);
    let mut ganglion_image = Image::gen_image_color(EYE_RES as u16, EYE_RES as u16, BLACK);

    // Camera
    let mut cam_dist = 60.0;
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
    let num_batches = PARTICLE_COUNT.div_ceil(BATCH_SIZE);
    let mut meshes: Vec<Mesh> = (0..num_batches).map(|_| {
        Mesh {
            vertices: Vec::with_capacity(BATCH_SIZE),
            indices: Vec::with_capacity(BATCH_SIZE),
            texture: None,
        }
    }).collect();

    loop {
        let dt = get_frame_time().min(0.05);

        // 1. Update Simulation
        sim.update(dt);

        // 2. Render to Eye Texture
        {
            let cam_pos = vec3(
                cam_dist * cam_yaw.cos() * cam_pitch.cos(),
                cam_dist * cam_pitch.sin(),
                cam_dist * cam_yaw.sin() * cam_pitch.cos(),
            );
            let target = vec3(0.0, 0.0, 25.0); // Focus on center of attractor (approx z=25)

            let camera = Camera3D {
                position: cam_pos,
                target,
                up: vec3(0.0, 1.0, 0.0),
                fovy: 45.0,
                aspect: Some(1.0), // Square aspect for retina
                projection: Projection::Perspective,
                render_target: Some(render_target.clone()),
                viewport: None,
                z_near: 0.1,
                z_far: 1000.0,
            };

            set_camera(&camera);
            clear_background(BLACK);

            // Draw simplified particles for the eye (just white dots)
             gl_use_material(&material);
             for (i, mesh) in meshes.iter_mut().enumerate() {
                let start = i * BATCH_SIZE;
                let end = (start + BATCH_SIZE).min(sim.particles.len());
                let slice = &sim.particles[start..end];

                mesh.vertices.clear();
                mesh.indices.clear();

                for (idx, p) in slice.iter().enumerate() {
                    // Eye sees structure, not velocity color? Or keep it?
                    // Let's keep it but maybe brighter for contrast
                    mesh.vertices.push(to_vertex(p));
                    mesh.indices.push(idx as u16);
                }
                draw_mesh(mesh);
            }
            gl_use_default_material();
            set_default_camera();
        }

        // 3. Readback and Vision Update
        let eye_image = render_target.texture.get_texture_data();
        // Convert to grayscale f32 buffer
        let mut input_buffer = vec![0.0; EYE_RES * EYE_RES];
        for (i, pixel) in eye_image.bytes.chunks(4).enumerate() {
            if i < input_buffer.len() {
                // Simple luminance: R+G+B / 3.0 / 255.0
                let r = pixel[0] as f32;
                let g = pixel[1] as f32;
                let b = pixel[2] as f32;
                input_buffer[i] = (r + g + b) / (3.0 * 255.0);
            }
        }

        let spikes = retina.update(&input_buffer);

        // 4. Feedback Logic
        let spike_count = spikes.len();
        let excitement = (spike_count as f32 / (EYE_RES * EYE_RES) as f32) * 100.0; // % of firing neurons

        // Modulate Rho (Rayleigh number)
        // Base Rho = 28.0
        // Target Rho = 28.0 + Excitement * 2.0 (High excitement -> High Chaos)
        let target_rho = 28.0 + excitement * 50.0;

        // Smoothly interpolate
        sim.params.rho += (target_rho - sim.params.rho) * dt * 2.0;

        // 5. Update Debug Textures
        for y in 0..EYE_RES {
            for x in 0..EYE_RES {
                let idx = y * EYE_RES + x;

                // Bipolar
                let b = retina.bipolar[idx];
                let b_norm = (b * 5.0).tanh();
                let r = if b_norm > 0.0 { (b_norm * 255.0) as u8 } else { 0 };
                let bl = if b_norm < 0.0 { (-b_norm * 255.0) as u8 } else { 0 };
                bipolar_image.set_pixel(x as u32, y as u32, Color::from_rgba(r, 0, bl, 255));

                // Ganglion Background (Voltage)
                let v = retina.ganglion[idx].v;
                let v_norm = ((v + 70.0) / 100.0).clamp(0.0, 1.0);
                let g_val = (v_norm * 100.0) as u8; // Dim green
                ganglion_image.set_pixel(x as u32, y as u32, Color::from_rgba(0, g_val, 0, 255));
            }
        }
        // Draw spikes
        for (sx, sy) in &spikes {
             ganglion_image.set_pixel(*sx as u32, *sy as u32, WHITE);
        }
        bipolar_texture.update(&bipolar_image);
        ganglion_texture.update(&ganglion_image);

        // 6. Render Main View
        // Camera controls
        if is_key_down(KeyCode::Left) { cam_yaw -= 0.02; }
        if is_key_down(KeyCode::Right) { cam_yaw += 0.02; }
        if is_key_down(KeyCode::Up) { cam_pitch = (cam_pitch + 0.02).clamp(-1.5, 1.5); }
        if is_key_down(KeyCode::Down) { cam_pitch = (cam_pitch - 0.02).clamp(-1.5, 1.5); }
        if is_key_down(KeyCode::W) { cam_dist -= 0.5; }
        if is_key_down(KeyCode::S) { cam_dist += 0.5; }
        if is_key_pressed(KeyCode::R) { sim.reset(); }

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
        for mesh in &meshes {
            draw_mesh(mesh);
        }
        gl_use_default_material();

        set_default_camera();

        // 7. Render UI and PIP
        let sw = screen_width();
        let sh = screen_height();

        let pip_size = 150.0;
        let pad = 10.0;

        // Retina Input
        draw_texture_ex(&render_target.texture, sw - pip_size - pad, pad, WHITE, DrawTextureParams {
            dest_size: Some(vec2(pip_size, pip_size)),
            flip_y: true, // RenderTargets are flipped
            ..Default::default()
        });
        draw_rectangle_lines(sw - pip_size - pad, pad, pip_size, pip_size, 2.0, GRAY);
        draw_text("Retina Input", sw - pip_size - pad, pad - 5.0, 20.0, WHITE);

        // Bipolar
        draw_texture_ex(&bipolar_texture, sw - pip_size - pad, pad * 2.0 + pip_size, WHITE, DrawTextureParams {
            dest_size: Some(vec2(pip_size, pip_size)),
            ..Default::default()
        });
        draw_rectangle_lines(sw - pip_size - pad, pad * 2.0 + pip_size, pip_size, pip_size, 2.0, GRAY);
        draw_text("Bipolar (Edges)", sw - pip_size - pad, pad * 2.0 + pip_size - 5.0, 20.0, WHITE);

        // Ganglion
        draw_texture_ex(&ganglion_texture, sw - pip_size - pad, pad * 3.0 + pip_size * 2.0, WHITE, DrawTextureParams {
            dest_size: Some(vec2(pip_size, pip_size)),
            ..Default::default()
        });
        draw_rectangle_lines(sw - pip_size - pad, pad * 3.0 + pip_size * 2.0, pip_size, pip_size, 2.0, GRAY);
        draw_text("Ganglion (Spikes)", sw - pip_size - pad, pad * 3.0 + pip_size * 2.0 - 5.0, 20.0, WHITE);

        // Stats
        draw_text("Retinal Chaos", 20.0, 30.0, 40.0, WHITE);
        draw_text(&format!("FPS: {}", get_fps()), 20.0, 60.0, 20.0, LIGHTGRAY);
        draw_text(&format!("Spikes: {}", spike_count), 20.0, 90.0, 20.0, GREEN);
        draw_text(&format!("Excitement: {:.2}%", excitement), 20.0, 110.0, 20.0, YELLOW);
        draw_text(&format!("Rho (Chaos): {:.2}", sim.params.rho), 20.0, 140.0, 20.0, RED);
        draw_text("The Eye watches the Chaos.", 20.0, sh - 40.0, 20.0, LIGHTGRAY);
        draw_text("The Chaos reacts to the Eye.", 20.0, sh - 20.0, 20.0, LIGHTGRAY);

        next_frame().await;
    }
}
