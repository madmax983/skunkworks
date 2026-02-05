mod math;
mod shader;

use cgmath::{InnerSpace, Vector3};
use macroquad::prelude::*;
use math::{boost, Mat4 as CgMat4};

#[macroquad::main("Hyperbolic Space")]
async fn main() {
    let mut camera_matrix = CgMat4::from_scale(1.0); // Identity

    // Compile shader
    let material = load_material(
        ShaderSource::Glsl {
            vertex: shader::VERTEX,
            fragment: shader::FRAGMENT,
        },
        MaterialParams {
            uniforms: vec![
                UniformDesc::new("iResolution", UniformType::Float3),
                UniformDesc::new("iTime", UniformType::Float1),
                UniformDesc::new("u_camera", UniformType::Mat4),
            ],
            ..Default::default()
        },
    )
    .unwrap();

    let start_time = get_time();

    loop {
        clear_background(BLACK);

        // Input Handling
        let speed = 0.05;
        let mut move_dir = Vector3::new(0.0, 0.0, 0.0);

        if is_key_down(KeyCode::W) {
            move_dir.z -= 1.0;
        }
        if is_key_down(KeyCode::S) {
            move_dir.z += 1.0;
        }
        if is_key_down(KeyCode::A) {
            move_dir.x -= 1.0;
        }
        if is_key_down(KeyCode::D) {
            move_dir.x += 1.0;
        }
        if is_key_down(KeyCode::Q) {
            move_dir.y -= 1.0;
        } // Up
        if is_key_down(KeyCode::E) {
            move_dir.y += 1.0;
        } // Down

        if move_dir.magnitude() > 0.001 {
            let b = boost(move_dir.normalize() * speed);
            // Apply boost relative to camera: NewCam = OldCam * Boost
            // Because boost is in tangent space of camera.
            camera_matrix = camera_matrix * b;
        }

        // Pass uniforms
        material.set_uniform("iResolution", (screen_width(), screen_height(), 0.0));
        material.set_uniform("iTime", (get_time() - start_time) as f32);

        material.set_uniform("u_camera", cam_matrix_from_cgmath(camera_matrix));

        // Draw Fullscreen Quad
        gl_use_material(&material);
        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), WHITE);
        gl_use_default_material();

        // UI overlay
        draw_text("WASD: Move, Q/E: Up/Down", 10.0, 20.0, 30.0, WHITE);

        next_frame().await
    }
}

// Helper to convert cgmath::Mat4 to macroquad::Mat4
fn cam_matrix_from_cgmath(m: CgMat4) -> macroquad::math::Mat4 {
    let a: [[f32; 4]; 4] = m.into();
    macroquad::math::Mat4::from_cols_array_2d(&a)
}
