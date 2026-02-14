use macroquad::miniquad::{PipelineParams, UniformDesc, UniformType};
use macroquad::prelude::*;

mod audio;
mod shader;

use audio::Synthesizer;

fn window_conf() -> Conf {
    Conf {
        window_title: "Rhythm Diffusion".to_string(),
        window_width: 800,
        window_height: 800,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut width = screen_width() as u32;
    let mut height = screen_height() as u32;

    // Parameters
    let mut synthesizer = Synthesizer::new();
    let da = 1.0;
    let db = 0.5;
    let dt = 1.0;

    // Render Targets for Ping-Pong
    let mut target_a = render_target(width, height);
    let mut target_b = render_target(width, height);

    target_a.texture.set_filter(FilterMode::Nearest);
    target_b.texture.set_filter(FilterMode::Nearest);

    // Initial State
    // Function to reseed
    let reseed = |target: &RenderTarget, w: f32, h: f32| {
        let mut cam = Camera2D::from_display_rect(Rect::new(0.0, 0.0, w, h));
        cam.render_target = Some(target.clone());
        set_camera(&cam);

        // Fill with U=1 (Red channel), V=0 (Green channel)
        clear_background(RED);

        // Add seed (U=0, V=1) -> Green
        // Actually, if we draw Green (0, 1, 0), R becomes 0, G becomes 1.
        // Reaction consumes U (Red) and produces V (Green).
        draw_rectangle(w / 2.0 - 10.0, h / 2.0 - 10.0, 20.0, 20.0, GREEN);

        set_default_camera();
    };

    reseed(&target_a, width as f32, height as f32);

    // Material
    let material = load_material(
        ShaderSource::Glsl {
            vertex: shader::VERTEX_SHADER,
            fragment: shader::FRAGMENT_SHADER,
        },
        MaterialParams {
            uniforms: shader::get_uniforms(), // Use the function from shader.rs
            pipeline_params: PipelineParams {
                depth_write: false,
                depth_test: Comparison::Always,
                ..Default::default()
            },
            ..Default::default()
        },
    )
    .unwrap();

    loop {
        // Handle Resize
        let new_w = screen_width() as u32;
        let new_h = screen_height() as u32;
        if new_w != width || new_h != height {
            width = new_w;
            height = new_h;
            target_a = render_target(width, height);
            target_b = render_target(width, height);
            target_a.texture.set_filter(FilterMode::Nearest);
            target_b.texture.set_filter(FilterMode::Nearest);
            reseed(&target_a, width as f32, height as f32);
        }

        // Handle Input (Mouse Painting)
        if is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();
            // We need to draw into the CURRENT source texture (target_a) so it persists?
            // No, we draw into the SOURCE, which is then fed to the shader to produce DEST.
            // If we draw to A, and A is source for B, B gets the reaction of A (plus our drawing?).
            // Actually, best to draw into the source texture before the simulation step.

            let mut cam =
                Camera2D::from_display_rect(Rect::new(0.0, 0.0, width as f32, height as f32));
            cam.render_target = Some(target_a.clone()); // A is source
            set_camera(&cam);

            // Draw Green (V)
            draw_circle(mx, my, 10.0, GREEN);

            set_default_camera();
        }
        if is_mouse_button_down(MouseButton::Right) {
            let (mx, my) = mouse_position();
            let mut cam =
                Camera2D::from_display_rect(Rect::new(0.0, 0.0, width as f32, height as f32));
            cam.render_target = Some(target_a.clone());
            set_camera(&cam);

            // Draw Red (U) - erasing V
            draw_circle(mx, my, 20.0, RED);

            set_default_camera();
        }

        if is_key_pressed(KeyCode::Space) {
            reseed(&target_a, width as f32, height as f32);
        }

        if is_key_pressed(KeyCode::R) {
            synthesizer.base_feed = rand::gen_range(0.01, 0.09);
            synthesizer.base_kill = rand::gen_range(0.045, 0.07);
            synthesizer.frequency = rand::gen_range(0.5, 2.0);
        }

        // Update Audio
        synthesizer.update(get_frame_time());
        let (feed, kill) = synthesizer.get_params();

        // Simulation Steps
        let steps = 12;
        for _ in 0..steps {
            // Ping: Read A, Write B
            {
                let mut cam =
                    Camera2D::from_display_rect(Rect::new(0.0, 0.0, width as f32, height as f32));
                cam.render_target = Some(target_b.clone()); // Render to B
                set_camera(&cam);

                gl_use_material(&material);
                material.set_uniform("Resolution", vec2(width as f32, height as f32));
                material.set_uniform("Feed", feed);
                material.set_uniform("Kill", kill);
                material.set_uniform("Da", da);
                material.set_uniform("Db", db);
                material.set_uniform("dt", dt);

                // Draw Texture A
                draw_texture_ex(
                    &target_a.texture,
                    0.0,
                    0.0,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(width as f32, height as f32)),
                        ..Default::default()
                    },
                );

                gl_use_default_material();
                set_default_camera();
            }

            // Swap
            std::mem::swap(&mut target_a, &mut target_b);
        }

        // Render to Screen
        clear_background(BLACK);

        // Draw the final texture (Target A)
        // Flip Y if needed.
        draw_texture_ex(
            &target_a.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                flip_y: true,
                ..Default::default()
            },
        );

        draw_text("Genesis: Rhythm Diffusion", 10.0, 30.0, 30.0, WHITE);
        draw_text(
            &format!("Feed: {:.4} Kill: {:.4}", feed, kill),
            10.0,
            50.0,
            20.0,
            LIGHTGRAY,
        );
        draw_text(
            "LMB: Add | RMB: Erase | Space: Reset",
            10.0,
            screen_height() - 20.0,
            20.0,
            LIGHTGRAY,
        );

        next_frame().await
    }
}
