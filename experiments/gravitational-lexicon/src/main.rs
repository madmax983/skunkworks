mod physics;
mod shader;
mod text_gen;
mod phonology;

use macroquad::prelude::*;
use physics::WordBody;

fn window_conf() -> Conf {
    Conf {
        window_title: "Genesis: Gravitational Lexicon".to_string(),
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
    )
    .unwrap();

    // Bodies
    let mut bodies: Vec<WordBody> = Vec::new();

    // Initial Words
    let initial_words = vec![
        "Gravity", "Lexicon", "Orbit", "Mass", "Syntax", "Phoneme",
        "Star", "Void", "Light", "Time", "Chaos", "Order", "Flux"
    ];

    // Seed random words
    for _ in 0..8 {
        let word = initial_words[rand::gen_range(0, initial_words.len())];
        let pos = vec2(
            rand::gen_range(width as f32 * 0.2, width as f32 * 0.8),
            rand::gen_range(height as f32 * 0.2, height as f32 * 0.8)
        );
        let color = Color::new(
            rand::gen_range(0.5, 1.0),
            rand::gen_range(0.5, 1.0),
            rand::gen_range(0.5, 1.0),
            1.0,
        );
        bodies.push(WordBody::new(word, pos, color));
    }

    let lensing_strength = 5.0;

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
            let mpos = mouse_position();
            let pos = vec2(mpos.0, mpos.1);
             // Add a random word
             if bodies.len() < shader::MAX_BODIES {
                 let word = initial_words[rand::gen_range(0, initial_words.len())];
                 bodies.push(WordBody::new(word, pos, WHITE));
             }
        }

        // Mutate and Kick on Space
        if is_key_pressed(KeyCode::Space) {
             for body in &mut bodies {
                 body.mutate();
                 let force = vec2(rand::gen_range(-5000.0, 5000.0), rand::gen_range(-5000.0, 5000.0));
                 body.apply_force(force);
             }
        }

        // Reset on Enter
        if is_key_pressed(KeyCode::Enter) {
            bodies.clear();
            for _ in 0..8 {
                let word = initial_words[rand::gen_range(0, initial_words.len())];
                let pos = vec2(
                    rand::gen_range(width as f32 * 0.2, width as f32 * 0.8),
                    rand::gen_range(height as f32 * 0.2, height as f32 * 0.8)
                );
                let color = Color::new(
                    rand::gen_range(0.5, 1.0),
                    rand::gen_range(0.5, 1.0),
                    rand::gen_range(0.5, 1.0),
                    1.0,
                );
                bodies.push(WordBody::new(word, pos, color));
            }
        }

        // Physics
        let dt = get_frame_time().min(0.05);

        // Gravity between words
        physics::update_gravity(&mut bodies);

        // Internal physics
        for body in &mut bodies {
            body.update_internal(dt);

            // Boundary checks (bounce)
            let margin = 50.0;
            if body.center_of_mass.x < margin { body.apply_force(vec2(1000.0, 0.0)); }
            if body.center_of_mass.x > width as f32 - margin { body.apply_force(vec2(-1000.0, 0.0)); }
            if body.center_of_mass.y < margin { body.apply_force(vec2(0.0, 1000.0)); }
            if body.center_of_mass.y > height as f32 - margin { body.apply_force(vec2(0.0, -1000.0)); }
        }

        // Draw Background with Shader
        gl_use_material(&material);
        material.set_uniform("ViewSize", vec2(width as f32, height as f32));
        // ViewCenter at (W/2, H/2) to map UV(0..1) to World(0..W, 0..H)
        material.set_uniform("ViewCenter", vec2(width as f32 / 2.0, height as f32 / 2.0));
        material.set_uniform("Count", bodies.len() as i32);
        material.set_uniform("LensingStrength", lensing_strength);

        for (i, body) in bodies.iter().enumerate() {
            if i < shader::MAX_BODIES {
                material.set_uniform(&format!("Body{}_Pos", i), body.center_of_mass);
                material.set_uniform(&format!("Body{}_Mass", i), body.total_mass);
            }
        }

        draw_texture_ex(
            &text_target.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(width as f32, height as f32)),
                ..Default::default()
            },
        );
        gl_use_default_material();

        // Draw Foreground (Words)
        for body in &bodies {
            // Draw lines between phonemes
             for i in 0..body.nodes.len() - 1 {
                draw_line(
                    body.nodes[i].pos.x, body.nodes[i].pos.y,
                    body.nodes[i+1].pos.x, body.nodes[i+1].pos.y,
                    2.0,
                    GRAY
                );
            }
            // Draw phonemes
            for node in &body.nodes {
                let color = if node.mutated { GREEN } else { body.color };
                draw_text(
                    &node.phoneme.symbol.to_string(),
                    node.pos.x - 10.0, // Center text approx
                    node.pos.y + 10.0,
                    30.0,
                    color
                );
            }
        }

        draw_text("Genesis: Gravitational Lexicon", 10.0, 30.0, 30.0, WHITE);
        draw_text(
            "Space: Mutate & Perturb | Click: Add Word | Enter: Reset",
            10.0,
            height as f32 - 20.0,
            20.0,
            LIGHTGRAY,
        );

        next_frame().await
    }
}
