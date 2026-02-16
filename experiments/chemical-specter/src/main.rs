use macroquad::prelude::*;

const FRAGMENT_SHADER: &'static str = include_str!("shader.glsl");

const VERTEX_SHADER: &'static str = "#version 100
attribute vec3 position;
attribute vec2 texcoord;
varying vec2 uv;
uniform mat4 Model;
uniform mat4 Projection;
void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    uv = texcoord;
}
";

struct GhostSynth {
    phase: f32,
    feed_base: f32,
    kill_base: f32,
    feed_mod: f32,
    kill_mod: f32,
}

impl GhostSynth {
    fn new() -> Self {
        Self {
            phase: 0.0,
            feed_base: 0.055,
            kill_base: 0.062,
            feed_mod: 0.0,
            kill_mod: 0.0,
        }
    }

    fn update(&mut self, dt: f32) {
        self.phase += dt;
        // Oscillate parameters slowly to explore phase space
        self.feed_mod = (self.phase * 0.1).sin() * 0.005;
        self.kill_mod = (self.phase * 0.13).cos() * 0.005;
    }

    fn get_params(&self) -> (f32, f32) {
        (
            self.feed_base + self.feed_mod,
            self.kill_base + self.kill_mod,
        )
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Chemical Specter".to_owned(),
        window_width: 800,
        window_height: 600,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut synth = GhostSynth::new();

    let w = screen_width() as i32;
    let h = screen_height() as i32;

    // Create render targets for ping-pong buffering
    let render_target_a = render_target(w as u32, h as u32);
    let render_target_b = render_target(w as u32, h as u32);

    render_target_a.texture.set_filter(FilterMode::Nearest);
    render_target_b.texture.set_filter(FilterMode::Nearest);

    // Material
    let pipeline_params = PipelineParams {
        depth_write: true,
        depth_test: Comparison::LessOrEqual,
        ..Default::default()
    };

    let material = load_material(
        ShaderSource::Glsl {
            vertex: VERTEX_SHADER,
            fragment: FRAGMENT_SHADER,
        },
        MaterialParams {
            pipeline_params,
            uniforms: vec![
                UniformDesc::new("Resolution", UniformType::Float2),
                UniformDesc::new("Time", UniformType::Float1),
                UniformDesc::new("Params", UniformType::Float4),
                UniformDesc::new("Mouse", UniformType::Float3),
            ],
            ..Default::default()
        },
    )
    .unwrap();

    let mut current_target = render_target_a.clone();
    let mut next_target = render_target_b.clone();

    // Seed the simulation
    {
        // Setup camera to map pixel coordinates 0..w, 0..h to the render target
        let mut camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, w as f32, h as f32));
        camera.render_target = Some(current_target.clone());
        set_camera(&camera);

        clear_background(BLACK); // u=0, v=0

        // Fill u=1 everywhere (Red channel)
        draw_rectangle(0.0, 0.0, w as f32, h as f32, RED);

        // Seed v=1 in center (Green channel)
        draw_rectangle(
            w as f32 / 2.0 - 10.0,
            h as f32 / 2.0 - 10.0,
            20.0,
            20.0,
            GREEN,
        );

        // Add random noise
        for _ in 0..500 {
            let x = rand::gen_range(0.0, w as f32);
            let y = rand::gen_range(0.0, h as f32);
            let size = rand::gen_range(1.0, 5.0);
            draw_rectangle(x, y, size, size, GREEN);
        }

        set_default_camera();
    }

    loop {
        // Handle input
        let mouse_pos = mouse_position();
        let mouse_x = mouse_pos.0 / screen_width();
        let mouse_y = mouse_pos.1 / screen_height();

        let mouse_click = if is_mouse_button_down(MouseButton::Left) {
            1.0
        } else {
            0.0
        };

        // Update synth
        synth.update(get_frame_time());
        let (feed, kill) = synth.get_params();

        // 1. Render Simulation Step
        {
            // Set uniforms
            material.set_uniform("Resolution", (w as f32, h as f32));
            material.set_uniform("Time", get_time() as f32);
            material.set_uniform("Params", (feed, kill, 1.0f32, 0.5f32)); // Du=1.0, Dv=0.5
            material.set_uniform("Mouse", (mouse_x, 1.0 - mouse_y, mouse_click)); // Flip Y for shader interaction

            let mut camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, w as f32, h as f32));
            camera.render_target = Some(next_target.clone());
            set_camera(&camera);

            gl_use_material(&material);

            // Draw the current texture onto the next target
            // This binds current_target.texture to TEXTURE0, which the shader uses as 'Texture' (or implied sampler)
            draw_texture_ex(
                &current_target.texture,
                0.0,
                0.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(w as f32, h as f32)),
                    ..Default::default()
                },
            );

            gl_use_default_material();
            set_default_camera();
        }

        // 2. Render Result to Screen
        clear_background(BLACK);

        // Draw the result (next_target) to screen
        draw_texture_ex(
            &next_target.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        // UI Overlay
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 20.0, 20.0, WHITE);
        draw_text(&format!("Feed: {:.4}", feed), 10.0, 40.0, 20.0, WHITE);
        draw_text(&format!("Kill: {:.4}", kill), 10.0, 60.0, 20.0, WHITE);

        // Swap ping-pong
        let temp = current_target;
        current_target = next_target;
        next_target = temp;

        next_frame().await
    }
}
