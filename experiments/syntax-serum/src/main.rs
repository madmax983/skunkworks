use macroquad::prelude::*;

const FRAGMENT_SHADER: &'static str = include_str!("reaction.glsl");

const VERTEX_SHADER: &'static str = "#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;

varying highp vec2 uv;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1.0);
    uv = texcoord;
}
";

// Bake the source code into the binary for the "Serum"
const SOURCE_CODE: &'static str = include_str!("main.rs");

#[macroquad::main("Syntax Serum")]
async fn main() {
    // Standard Gray-Scott parameters (Mitosis-like spots)
    // DA=1.0, DB=0.5
    // Feed = 0.055, Kill = 0.062
    let mut feed = 0.055;
    let mut kill = 0.062;
    let da = 1.0;
    let db = 0.5;
    let dt = 1.0;

    let w = screen_width();
    let h = screen_height();

    // Setup RenderTargets (Ping-Pong)
    let target_a = render_target(w as u32, h as u32);
    let target_b = render_target(w as u32, h as u32);

    target_a.texture.set_filter(FilterMode::Linear);
    target_b.texture.set_filter(FilterMode::Linear);

    // Initial State:
    // Fill with Chemical A (Red = 1.0, Green = 0.0)
    {
        let cam = Camera2D {
            render_target: Some(target_a.clone()),
            ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, w, h))
        };
        set_camera(&cam);

        clear_background(RED); // A=1, B=0

        // Draw the Seed (The Source Code)
        // Green = Chemical B (Activator)
        // We'll draw the text scattered or just straight up.
        let font_size = 20.0;
        let mut y = 20.0;
        for line in SOURCE_CODE.lines() {
            draw_text(line, 10.0, y, font_size, GREEN);
            y += font_size;
            if y > h {
                break;
            }
        }

        set_default_camera();
    }

    // Material Setup
    // Using the signature compatible with macroquad 0.4+
    let material = load_material(
        ShaderSource::Glsl {
            vertex: VERTEX_SHADER,
            fragment: FRAGMENT_SHADER,
        },
        MaterialParams {
            uniforms: vec![
                UniformDesc::new("Feed", UniformType::Float1),
                UniformDesc::new("Kill", UniformType::Float1),
                UniformDesc::new("DA", UniformType::Float1),
                UniformDesc::new("DB", UniformType::Float1),
                UniformDesc::new("dt", UniformType::Float1),
                UniformDesc::new("ScreenSize", UniformType::Float2),
            ],
            ..Default::default()
        },
    )
    .unwrap();

    let mut current_target = target_a;
    let mut next_target = target_b;

    loop {
        // Handle Input
        // Mouse paint: Add B (Green)
        if is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();
            // Map screen mouse pos to target coords (if different, but here same)

            // We draw onto current_target.
            // Note: drawing onto a texture that is bound as input in the next step is fine.
            let cam = Camera2D {
                render_target: Some(current_target.clone()),
                ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, screen_width(), screen_height()))
            };
            set_camera(&cam);
            draw_circle(mx, my, 20.0, GREEN);
            set_default_camera();
        }

        // Adjust Feed/Kill
        if is_key_down(KeyCode::Up) {
            feed += 0.0001;
        }
        if is_key_down(KeyCode::Down) {
            feed -= 0.0001;
        }
        if is_key_down(KeyCode::Right) {
            kill += 0.0001;
        }
        if is_key_down(KeyCode::Left) {
            kill -= 0.0001;
        }

        // Simulation Step
        {
            let cam = Camera2D {
                render_target: Some(next_target.clone()),
                ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, screen_width(), screen_height()))
            };
            set_camera(&cam);

            gl_use_material(&material);
            material.set_uniform("Feed", feed);
            material.set_uniform("Kill", kill);
            material.set_uniform("DA", da);
            material.set_uniform("DB", db);
            material.set_uniform("dt", dt);
            material.set_uniform("ScreenSize", vec2(screen_width(), screen_height()));

            // Draw the current state texture to the quad
            draw_texture_ex(
                &current_target.texture,
                0.0,
                0.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(screen_width(), screen_height())),
                    ..Default::default()
                },
            );

            gl_use_default_material();
            set_default_camera();
        }

        // Render to Screen (Visualization)
        clear_background(BLACK);

        // Draw the result
        draw_texture_ex(
            &next_target.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                flip_y: true,
                ..Default::default()
            },
        );

        // UI
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 20.0, 20.0, WHITE);
        draw_text(&format!("Feed: {:.4}", feed), 10.0, 40.0, 20.0, WHITE);
        draw_text(&format!("Kill: {:.4}", kill), 10.0, 60.0, 20.0, WHITE);

        // Swap
        let temp = current_target;
        current_target = next_target;
        next_target = temp;

        next_frame().await
    }
}
