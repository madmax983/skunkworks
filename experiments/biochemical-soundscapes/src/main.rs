use macroquad::prelude::*;

mod reaction;
mod shaders;
mod audio;

use audio::AudioEngine;
use shaders::{FRAGMENT_SHADER_RENDER, FRAGMENT_SHADER_SIMULATE, VERTEX_SHADER};

fn window_conf() -> Conf {
    Conf {
        window_title: "Biochemical Soundscapes".to_owned(),
        window_width: 800,
        window_height: 800,
        high_dpi: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let w = 512;
    let h = 512;

    let target1 = render_target(w, h);
    let target2 = render_target(w, h);

    target1.texture.set_filter(FilterMode::Nearest);
    target2.texture.set_filter(FilterMode::Nearest);

    // Initialize state: U=1, V=0 everywhere
    // We can do this by clearing the targets to RED.
    // RED (1, 0, 0, 1) -> r=1(u), g=0(v).
    {
        let cam = Camera2D {
            render_target: Some(target1.clone()),
            zoom: vec2(2.0 / w as f32, 2.0 / h as f32), // Map pixels to -1..1?
            // Actually, default camera maps screen pixels.
            // Let's just use default camera for target, which is pixel coords 0..w, 0..h (y down)
            // But we need to be careful.
            // Easiest is to set a camera that matches the target size.
             ..Default::default()
        };
        set_camera(&cam);
        clear_background(RED);

        // Seed center with V=1 (Yellow = R+G = U+V)
        draw_rectangle(w as f32 / 2.0 - 10.0, h as f32 / 2.0 - 10.0, 20.0, 20.0, YELLOW);

        set_default_camera();
    }

    // Copy target1 to target2 initially
    {
        let cam = Camera2D {
            render_target: Some(target2.clone()),
            ..Default::default()
        };
        set_camera(&cam);
        draw_texture(&target1.texture, 0.0, 0.0, WHITE);
        set_default_camera();
    }

    // Load Materials
    let pipeline_params = PipelineParams {
        depth_write: false,
        depth_test: Comparison::Always,
        ..Default::default()
    };

    let sim_material = load_material(
        ShaderSource::Glsl {
            vertex: VERTEX_SHADER,
            fragment: FRAGMENT_SHADER_SIMULATE,
        },
        MaterialParams {
            pipeline_params,
            uniforms: vec![
                UniformDesc::new("resolution", UniformType::Float2),
                UniformDesc::new("dt", UniformType::Float1),
                UniformDesc::new("feed", UniformType::Float1),
                UniformDesc::new("kill", UniformType::Float1),
                UniformDesc::new("diff_u", UniformType::Float1),
                UniformDesc::new("diff_v", UniformType::Float1),
                UniformDesc::new("mouse", UniformType::Float3),
            ],
            ..Default::default()
        },
    )
    .unwrap();

    let render_material = load_material(
        ShaderSource::Glsl {
            vertex: VERTEX_SHADER,
            fragment: FRAGMENT_SHADER_RENDER,
        },
        MaterialParams {
            pipeline_params,
            uniforms: vec![],
            ..Default::default()
        },
    )
    .unwrap();

    let mut feed_base = 0.055;
    let mut kill = 0.062;
    let diff_u = 1.0;
    let diff_v = 0.5;

    let mut audio_engine = AudioEngine::new();

    // Ping-pong index
    let mut flip = false;

    loop {
        let dt = get_frame_time();
        audio_engine.update(dt);
        let energy = audio_engine.get_energy(); // 0..1

        // Modulate feed with audio energy
        // Base feed 0.055. Energy adds +/- 0.005?
        // Let's make it add to feed.
        let feed = feed_base + energy * 0.01;

        let (source, dest) = if flip {
            (target2.clone(), target1.clone())
        } else {
            (target1.clone(), target2.clone())
        };

        // User Input
        if is_key_down(KeyCode::Up) { feed_base += 0.0001; }
        if is_key_down(KeyCode::Down) { feed_base -= 0.0001; }
        if is_key_down(KeyCode::Right) { kill += 0.0001; }
        if is_key_down(KeyCode::Left) { kill -= 0.0001; }

        let (mx, my) = mouse_position();
        let m_click = if is_mouse_button_down(MouseButton::Left) { 1.0 } else { 0.0 };
        // Map mouse to UV (0..1)
        // Screen size might not match render target size?
        // We render the texture to screen size.
        // Assuming texture fills screen.
        let m_uv_x = mx / screen_width();
        let m_uv_y = 1.0 - (my / screen_height()); // Flip Y if needed? Macroquad Y is down. GL Y is up?
        // Macroquad texture coords: usually 0,0 top-left.
        // But in shader with custom vertex shader?
        // VERTEX_SHADER passes texcoord.
        // Let's assume standard UV.
        // If Y is flipped, we'll see interaction mismatch.

        // Simulation Pass
        {
            let cam = Camera2D {
                render_target: Some(dest.clone()),
                zoom: vec2(1.0, 1.0), // -1..1 covers full target?
                // Wait, if zoom is 1.0, view is -1..1.
                // Standard Camera2D uses pixel coords if zoom is not set (or calculated from screen).
                // If I set render_target, it defaults to pixel coords of the target?
                // No, "The default camera maps the screen to -1..1 in Y and -aspect..aspect in X".
                // Actually `set_camera` takes a `Camera2D`.
                // `Camera2D::from_display_rect` is useful.
                // But simpler: use zoom vec2(1.0, 1.0) and draw a rect from -1 to 1?
                // Or use default pixel processing:
                // `render_target: Some(dest)`
                // `zoom: vec2(2./w, 2./h)` -> pixels map to -1..1?
                // `target: vec2(w/2, h/2)`?

                // Let's try drawing a full-screen quad in Normalized Device Coordinates (NDC) -1..1.
                // Camera zoom 1.0, target 0.0.
                ..Default::default()
            };
            set_camera(&cam);

            gl_use_material(&sim_material);
            sim_material.set_uniform("resolution", (w as f32, h as f32));
            sim_material.set_uniform("dt", 1.0f32); // Speed
            sim_material.set_uniform("feed", feed);
            sim_material.set_uniform("kill", kill);
            sim_material.set_uniform("diff_u", diff_u);
            sim_material.set_uniform("diff_v", diff_v);
            sim_material.set_uniform("mouse", (m_uv_x, 1.0 - m_uv_y, m_click)); // Trying Y-flip for shader

            sim_material.set_texture("tex", source.texture.clone());

            // Draw a quad covering -1..1
            draw_texture_ex(&source.texture, -1.0, -1.0, WHITE, DrawTextureParams {
                dest_size: Some(vec2(2.0, 2.0)),
                ..Default::default()
            });

            gl_use_default_material();
        }

        // Render Pass (to Screen)
        set_default_camera();
        clear_background(BLACK);

        gl_use_material(&render_material);
        render_material.set_texture("tex", dest.texture.clone());

        // Draw to screen
        // Use NDC coordinates (-1..1) because our vertex shader ignores camera matrices
        draw_texture_ex(&dest.texture, -1.0, -1.0, WHITE, DrawTextureParams {
            dest_size: Some(vec2(2.0, 2.0)),
            ..Default::default()
        });

        gl_use_default_material();

        // UI Debug
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 20.0, 30.0, WHITE);
        draw_text(&format!("Feed: {:.4} (Base: {:.4})", feed, feed_base), 10.0, 50.0, 30.0, WHITE);
        draw_text(&format!("Kill: {:.4}", kill), 10.0, 80.0, 30.0, WHITE);
        draw_text(&format!("Energy: {:.2}", energy), 10.0, 110.0, 30.0, WHITE);

        flip = !flip;
        next_frame().await;
    }
}
