mod shaders;

use macroquad::prelude::*;
use shaders::{FRAGMENT_SHADER_RENDER, FRAGMENT_SHADER_SIM, VERTEX_SHADER};

fn window_conf() -> Conf {
    Conf {
        window_title: "Chemical Specter ⚛️".to_string(),
        window_width: 800,
        window_height: 800,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let width = 512;
    let height = 512;

    let target_a = render_target(width as u32, height as u32);
    target_a.texture.set_filter(FilterMode::Nearest);

    let target_b = render_target(width as u32, height as u32);
    target_b.texture.set_filter(FilterMode::Nearest);

    // Setup Materials
    let pipeline_params = PipelineParams {
        depth_write: false,
        ..Default::default()
    };

    let material_sim = load_material(
        ShaderSource::Glsl {
            vertex: VERTEX_SHADER,
            fragment: FRAGMENT_SHADER_SIM,
        },
        MaterialParams {
            pipeline_params,
            uniforms: vec![
                UniformDesc::new("feed", UniformType::Float1),
                UniformDesc::new("kill", UniformType::Float1),
                UniformDesc::new("beat", UniformType::Float1),
                UniformDesc::new("resolution", UniformType::Float2),
            ],
            ..Default::default()
        },
    )
    .unwrap();

    let material_render = load_material(
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

    // Initial State
    let mut feed = 0.0545;
    let mut kill = 0.0620;

    let mut active_target = target_a.clone();
    let mut next_target = target_b.clone();

    // Initialize with U=1 (Red), V=0 (Green)
    {
        let cam = Camera2D {
            render_target: Some(active_target.clone()),
            zoom: vec2(1.0, 1.0), // -1..1 coordinates
            target: vec2(0.0, 0.0),
            ..Default::default()
        };
        set_camera(&cam);

        // Base state: U=1.0 (Red)
        clear_background(Color::new(1.0, 0.0, 0.0, 1.0));

        // Seed: Rectangle of V=1.0 (Green) in center
        // In -1..1 coords
        draw_rectangle(-0.1, -0.1, 0.2, 0.2, Color::new(0.0, 1.0, 0.0, 1.0));

        set_default_camera();
    }

    loop {
        // --- Ghost Audio ---
        let time = get_time();
        // LFO: 120 BPM = 2Hz
        let beat_raw = (time * 4.0).sin() as f32;
        let beat = ((beat_raw + 1.0) * 0.5).powf(8.0); // Spiky beat

        // --- Input ---
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

        if is_key_pressed(KeyCode::R) {
            let cam = Camera2D {
                render_target: Some(active_target.clone()),
                zoom: vec2(1.0, 1.0),
                target: vec2(0.0, 0.0),
                ..Default::default()
            };
            set_camera(&cam);
            clear_background(Color::new(1.0, 0.0, 0.0, 1.0));
            draw_rectangle(-0.1, -0.1, 0.2, 0.2, Color::new(0.0, 1.0, 0.0, 1.0));
            set_default_camera();
        }

        // --- Simulation Pass ---
        {
            let cam = Camera2D {
                render_target: Some(next_target.clone()),
                zoom: vec2(1.0, 1.0),
                target: vec2(0.0, 0.0),
                ..Default::default()
            };
            set_camera(&cam);

            material_sim.set_uniform("feed", feed);
            material_sim.set_uniform("kill", kill);
            material_sim.set_uniform("beat", beat);
            material_sim.set_uniform("resolution", vec2(width as f32, height as f32));

            gl_use_material(&material_sim);

            // Draw previous frame
            // Note: drawing a texture that is flipped?
            // Macroquad render targets are usually upside down compared to screen?
            // But if we just pass through UVs, it should be consistent.
            // Draw a quad covering -1..1
            draw_texture_ex(
                &active_target.texture,
                -1.0,
                -1.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(2.0, 2.0)),
                    ..Default::default()
                },
            );

            gl_use_default_material();

            // Mouse Interaction (Adding Chemical V)
            if is_mouse_button_down(MouseButton::Left) {
                let (mx, my) = mouse_position();
                let sw = screen_width();
                let sh = screen_height();

                // Map screen mouse to simulation space (-1..1)
                // Note: Mouse Y is 0 at top, Screen H at bottom.
                // Camera Y is -1 at bottom, 1 at top.
                let nx = (mx / sw) * 2.0 - 1.0;
                let ny = 1.0 - (my / sh) * 2.0;

                draw_circle(nx, ny, 0.05, Color::new(0.0, 1.0, 0.0, 1.0));
            }

            set_default_camera();
        }

        // Swap
        let temp = active_target;
        active_target = next_target;
        next_target = temp;

        // --- Render Pass ---
        clear_background(BLACK);

        gl_use_material(&material_render);

        // Draw to screen (Upside down issue?)
        // RenderTargets are usually inverted in Y when drawn to screen compared to internal.
        // Let's draw normally and see.
        draw_texture_ex(
            &active_target.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        gl_use_default_material();

        // UI
        draw_text("Chemical Specter ⚛️", 20.0, 30.0, 30.0, WHITE);
        draw_text(
            &format!("Feed: {:.4} | Kill: {:.4}", feed, kill),
            20.0,
            60.0,
            20.0,
            LIGHTGRAY,
        );
        draw_text(
            &format!("Beat: {:.2}", beat),
            20.0,
            80.0,
            20.0,
            if beat > 0.5 { RED } else { GRAY },
        );
        draw_text(
            "Arrows: Modulate | Click: Add Catalyst | R: Reset",
            20.0,
            screen_height() - 20.0,
            20.0,
            GRAY,
        );

        next_frame().await
    }
}
