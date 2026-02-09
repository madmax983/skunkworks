use event_horizon::physics::{Body, Universe};
use macroquad::prelude::*;
mod shader;

#[macroquad::main("Event Horizon")]
async fn main() {
    let mut universe = Universe::new();

    // Initial bodies
    // Center mass (Black Hole)
    universe.add_body(Body {
        pos: Vec2::new(screen_width() / 2.0, screen_height() / 2.0),
        vel: Vec2::ZERO,
        mass: 5000.0,
        radius: 20.0,
    });

    // Orbiting star
    universe.add_body(Body {
        pos: Vec2::new(screen_width() / 2.0 + 200.0, screen_height() / 2.0),
        vel: Vec2::new(0.0, 5.0),
        mass: 100.0,
        radius: 10.0,
    });

    // Create Render Target for Text
    let render_target = render_target(screen_width() as u32, screen_height() as u32);
    render_target.texture.set_filter(FilterMode::Linear);

    // Render Text once (or every frame if we want dynamic text)
    // For now, let's render a static grid of text.
    set_camera(&Camera2D {
        render_target: Some(render_target.clone()),
        ..Default::default()
    });

    clear_background(BLACK);

    let font_size = 20.0;
    let cols = (screen_width() / (font_size * 0.6)) as i32;
    let rows = (screen_height() / font_size) as i32;

    for y in 0..rows {
        for x in 0..cols {
            // Actually, let's write "EVENT HORIZON" repeatedly
            let text = "EVENT HORIZON ";
            let char_idx = ((x + y * cols) as usize) % text.len();
            let char_to_draw = &text[char_idx..char_idx + 1];

            draw_text(
                char_to_draw,
                x as f32 * font_size * 0.6,
                y as f32 * font_size + font_size,
                font_size,
                GREEN,
            );
        }
    }

    set_default_camera(); // Back to screen

    // Load Shader
    let material = load_material(
        ShaderSource::Glsl {
            vertex: shader::VERTEX_SHADER,
            fragment: shader::FRAGMENT_SHADER,
        },
        MaterialParams {
            uniforms: vec![
                UniformDesc {
                    name: "bodies".to_string(),
                    uniform_type: UniformType::Float3,
                    array_count: 32,
                },
                UniformDesc {
                    name: "body_count".to_string(),
                    uniform_type: UniformType::Int1,
                    array_count: 1,
                },
                UniformDesc {
                    name: "aspect_ratio".to_string(),
                    uniform_type: UniformType::Float1,
                    array_count: 1,
                },
            ],
            ..Default::default()
        },
    )
    .unwrap();

    loop {
        // Physics Step
        let dt = 0.016; // Fixed step for simplicity
        universe.step(dt);

        // Handle input
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            universe.add_body(Body {
                pos: Vec2::new(mx, my),
                vel: Vec2::ZERO,
                mass: 5000.0,
                radius: 20.0,
            });
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            let (mx, my) = mouse_position();
            universe.add_body(Body {
                pos: Vec2::new(mx, my),
                vel: Vec2::new(rand::gen_range(-5.0, 5.0), rand::gen_range(-5.0, 5.0)),
                mass: 100.0,
                radius: 10.0,
            });
        }

        if is_key_pressed(KeyCode::Space) {
            universe.bodies.clear();
        }

        // Update Shader Uniforms
        // We need to flatten body positions and masses into [x, y, mass, x, y, mass, ...]
        // But wait, `UniformType::Float3` implies a single vec3.
        // For arrays, macroquad is a bit weird.
        // It seems `load_material` doesn't easily support array uniforms via the high-level API unless we use `UniformType::Float3` and assume it handles arrays if we pass more data?
        // Actually, for arrays, we might need to use `miniquad` directly or trick it.
        // A common workaround in macroquad is to use a texture for data if arrays are hard, or fixed number of uniforms like `body1`, `body2`...
        // But 32 bodies is too many for individual uniforms.

        // Let's try passing a flat vector and see if macroquad sends it.
        // If not, I'll reduce to 4 bodies and hardcode them.
        // Actually, looking at macroquad source, `set_uniform` uses `gl.uniform...`.
        // If the uniform is defined as an array in shader, and we pass a slice, it *should* work.
        // But `UniformType` doesn't strictly specify array length.

        // Wait, `bodies` is `vec3`. We need 3 floats per body.
        let mut body_data: Vec<f32> = Vec::new();
        let count = universe.bodies.len().min(32);

        for i in 0..count {
            let b = &universe.bodies[i];
            // Normalize positions to 0..1 for UV shader logic
            // But wait, the shader uses `pos` which is `uv`.
            // UV is 0..1.
            // Body pos is in screen pixels.
            // We need to convert body pos to UV space.
            let uv_x = b.pos.x / screen_width();
            let uv_y = 1.0 - (b.pos.y / screen_height()); // Flip Y for GL

            // Mass needs to be scaled?
            let m = b.mass / 1000.0;

            body_data.push(uv_x);
            body_data.push(uv_y);
            body_data.push(m);
        }

        // Pad with zeros if less than 32
        while body_data.len() < 32 * 3 {
            body_data.push(0.0);
        }

        material.set_uniform("bodies", &body_data[..]); // Hope this works!
        material.set_uniform("body_count", count as i32);
        material.set_uniform("aspect_ratio", screen_width() / screen_height());

        clear_background(BLACK);

        // Draw the texture with the shader
        gl_use_material(&material);
        draw_texture_ex(
            &render_target.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(screen_width(), screen_height())),
                ..Default::default()
            },
        );
        gl_use_default_material();

        // Draw bodies on top (debug/visuals)
        for body in &universe.bodies {
            draw_circle(body.pos.x, body.pos.y, body.radius, BLACK);
            draw_circle_lines(body.pos.x, body.pos.y, body.radius, 1.0, WHITE);
        }

        draw_text(
            format!("FPS: {}", get_fps()).as_str(),
            10.0,
            20.0,
            20.0,
            WHITE,
        );

        next_frame().await
    }
}
