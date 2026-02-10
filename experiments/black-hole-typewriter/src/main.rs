mod physics;

use macroquad::prelude::*;
use physics::{Body, Universe};

const FRAGMENT_SHADER: &'static str = include_str!("shader.glsl");
const VERTEX_SHADER: &'static str = "#version 100
attribute vec3 position;
attribute vec2 texcoord;

varying vec2 uv;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1.0);
    uv = texcoord;
}
";

#[macroquad::main("Black Hole Typewriter")]
async fn main() {
    let mut universe = Universe::new();

    let initial_text = "GENESIS";
    for (i, c) in initial_text.chars().enumerate() {
        universe.add_body(Body::new(
            vec2(200.0 + i as f32 * 40.0, 300.0),
            vec2(0.0, 0.0),
            2.0,
            c,
            GOLD,
        ));
    }

    let material = load_material(
        ShaderSource::Glsl {
            vertex: VERTEX_SHADER,
            fragment: FRAGMENT_SHADER,
        },
        MaterialParams {
            uniforms: vec![
                UniformDesc::new("Center", UniformType::Float2),
                UniformDesc::new("Aspect", UniformType::Float1),
                UniformDesc::new("Mass", UniformType::Float1),
            ],
            ..Default::default()
        },
    )
    .unwrap();

    let mut rt = render_target(screen_width() as u32, screen_height() as u32);
    rt.texture.set_filter(FilterMode::Linear);

    let mut stars = Vec::new();
    for _ in 0..200 {
        stars.push(vec2(
            rand::gen_range(0.0, screen_width()),
            rand::gen_range(0.0, screen_height()),
        ));
    }

    loop {
        if screen_width() as u32 != rt.texture.width() as u32 || screen_height() as u32 != rt.texture.height() as u32 {
             rt = render_target(screen_width() as u32, screen_height() as u32);
             rt.texture.set_filter(FilterMode::Linear);
             stars.clear();
             for _ in 0..200 {
                stars.push(vec2(
                    rand::gen_range(0.0, screen_width()),
                    rand::gen_range(0.0, screen_height()),
                ));
            }
        }

        // Input
        let (mx, my) = mouse_position();
        universe.black_hole_pos = vec2(mx, my);

        while let Some(c) = get_char_pressed() {
            let pos = vec2(rand::gen_range(0.0, screen_width()), rand::gen_range(0.0, screen_height()));
            let vel = vec2(rand::gen_range(-50.0, 50.0), rand::gen_range(-50.0, 50.0));

             let color = Color::new(
                rand::gen_range(0.5, 1.0),
                rand::gen_range(0.5, 1.0),
                rand::gen_range(0.5, 1.0),
                1.0,
            );

            universe.add_body(Body::new(pos, vel, 2.0, c, color));
        }

        // Physics
        universe.step(get_frame_time().min(0.05));

        // Render to Texture
        let camera = Camera2D {
            render_target: Some(rt.clone()),
            zoom: vec2(2.0 / screen_width(), 2.0 / screen_height()),
            target: vec2(screen_width() / 2.0, screen_height() / 2.0),
            ..Default::default()
        };
        set_camera(&camera);

        clear_background(BLACK);

        // Draw Stars
        for star in &stars {
            draw_circle(star.x, star.y, 1.0, WHITE);
        }

        // Draw Bodies
        for body in &universe.bodies {
            draw_text(&body.char.to_string(), body.pos.x, body.pos.y, 30.0, body.color);
        }

        set_default_camera();

        // Draw Lensed Scene to Screen
        clear_background(BLACK);

        gl_use_material(&material);

        let center_uv = vec2(mx / screen_width(), my / screen_height());

        material.set_uniform("Center", center_uv);
        material.set_uniform("Aspect", screen_width() / screen_height());
        material.set_uniform("Mass", 0.05);

        draw_texture_ex(
            &rt.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                flip_y: true,
                ..Default::default()
            },
        );

        gl_use_default_material();

        draw_circle_lines(mx, my, 20.0, 2.0, RED);
        draw_text("Black Hole Typewriter", 10.0, 20.0, 30.0, WHITE);
        draw_text("Type to add mass. Mouse moves the singularity.", 10.0, 50.0, 20.0, GRAY);
        draw_text(format!("Bodies: {}", universe.bodies.len()).as_str(), 10.0, 80.0, 20.0, DARKGRAY);

        next_frame().await
    }
}
