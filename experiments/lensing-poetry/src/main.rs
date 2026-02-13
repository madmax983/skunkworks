use macroquad::prelude::*;

use lensing_poetry::physics::{self, Body, G};
use lensing_poetry::shader;
use lensing_poetry::text_gen;

fn window_conf() -> Conf {
    Conf {
        window_title: "Genesis: Lensing Poetry".to_string(),
        window_width: 1280,
        window_height: 720,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut width = screen_width() as u32;
    let mut height = screen_height() as u32;

    // Generate Text Texture
    let mut text_target = text_gen::generate_poetry_target(width, height);

    // Load Shader
    let frag_src = shader::get_fragment_shader();
    let material = load_material(
        ShaderSource::Glsl {
            vertex: shader::get_vertex_shader(),
            fragment: &frag_src,
        },
        MaterialParams {
            uniforms: shader::get_uniforms(),
            pipeline_params: PipelineParams {
                depth_write: false,
                depth_test: Comparison::Always,
                ..Default::default()
            },
            ..Default::default()
        },
    ).unwrap();

    // Bodies
    let mut bodies = Vec::with_capacity(shader::MAX_BODIES);

    // Initial Setup
    bodies.push(Body {
        pos: vec2(0.0, 0.0),
        vel: vec2(0.0, 0.0),
        mass: 10000.0,
        radius: 20.0,
        color: YELLOW,
    });

    bodies.push(Body {
        pos: vec2(200.0, 0.0),
        vel: vec2(0.0, (G * 10000.0 / 200.0).sqrt()),
        mass: 100.0,
        radius: 10.0,
        color: BLUE,
    });

    bodies.push(Body {
        pos: vec2(-300.0, 100.0),
        vel: vec2(2.0, -4.0),
        mass: 200.0,
        radius: 12.0,
        color: RED,
    });

    let lensing_strength = 0.5;

    loop {
        // Handle Resize
        let new_w = screen_width() as u32;
        let new_h = screen_height() as u32;
        if new_w != width || new_h != height {
            width = new_w;
            height = new_h;
            text_target = text_gen::generate_poetry_target(width, height);
        }

        // Input
        if is_mouse_button_pressed(MouseButton::Left) {
            if bodies.len() < shader::MAX_BODIES {
                let mpos = mouse_position();
                let world_pos = vec2(mpos.0 - width as f32/2.0, mpos.1 - height as f32/2.0);

                bodies.push(Body {
                    pos: world_pos,
                    vel: vec2(rand::gen_range(-10.0, 10.0), rand::gen_range(-10.0, 10.0)),
                    mass: rand::gen_range(500.0, 5000.0),
                    radius: rand::gen_range(10.0, 30.0),
                    color: Color::new(rand::gen_range(0.5, 1.0), rand::gen_range(0.5, 1.0), rand::gen_range(0.5, 1.0), 1.0),
                });
            }
        }

        if is_mouse_button_pressed(MouseButton::Right) {
             if bodies.len() < shader::MAX_BODIES {
                let mpos = mouse_position();
                let world_pos = vec2(mpos.0 - width as f32/2.0, mpos.1 - height as f32/2.0);

                bodies.push(Body {
                    pos: world_pos,
                    vel: vec2(0.0, 0.0),
                    mass: -5000.0,
                    radius: 15.0,
                    color: PURPLE,
                });
            }
        }

        if is_key_pressed(KeyCode::Space) {
            bodies.truncate(1);
            bodies[0].pos = Vec2::ZERO;
            bodies[0].vel = Vec2::ZERO;
            bodies[0].mass = 10000.0;
        }

        // Physics
        let dt = get_frame_time().min(0.05);
        let substeps = 4;
        let sdt = dt / substeps as f32;
        for _ in 0..substeps {
            physics::integrate(&mut bodies, sdt);
        }

        // Draw
        gl_use_material(&material);

        material.set_uniform("ViewSize", vec2(width as f32, height as f32));
        material.set_uniform("ViewCenter", vec2(0.0, 0.0));
        material.set_uniform("Count", bodies.len() as i32);
        material.set_uniform("LensingStrength", lensing_strength);

        for (i, body) in bodies.iter().enumerate() {
             material.set_uniform(&format!("Body{}_Pos", i), body.pos);
             material.set_uniform(&format!("Body{}_Mass", i), body.mass);
        }

        draw_texture_ex(
            &text_target.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(width as f32, height as f32)),
                ..Default::default()
            }
        );

        gl_use_default_material();

        let offset_x = width as f32 / 2.0;
        let offset_y = height as f32 / 2.0;

        for body in &bodies {
            draw_circle(body.pos.x + offset_x, body.pos.y + offset_y, body.radius, body.color);
        }

        draw_text("Genesis: Lensing Poetry", 10.0, 30.0, 30.0, WHITE);
        draw_text(&format!("Bodies: {}", bodies.len()), 10.0, 50.0, 20.0, LIGHTGRAY);
        draw_text("Left Click: Add Mass | Right Click: Add Dark Matter | Space: Reset", 10.0, height as f32 - 20.0, 20.0, LIGHTGRAY);

        next_frame().await
    }
}
