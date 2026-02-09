use macroquad::prelude::*;

mod scanner;
use scanner::{Scanner, FileMetric};
use walkdir::WalkDir;

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;

const VERTEX_SHADER: &str = r#"
#version 100
attribute vec3 position;
attribute vec2 texcoord;
varying highp vec2 uv;
uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1.0);
    uv = texcoord;
}
"#;

const FRAGMENT_SHADER: &str = r#"
#version 100
precision highp float;

varying vec2 uv;

uniform sampler2D StateTexture;
uniform sampler2D CatalystTexture;
uniform vec2 ScreenSize;

void main() {
    vec4 state = texture2D(StateTexture, uv);
    vec4 params = texture2D(CatalystTexture, uv);

    float u = state.r;
    float v = state.g;

    // Parameters from texture
    // R: Feed, G: Kill, B: Diff U, A: Diff V
    float f = params.r;
    float k = params.g;
    float du = params.b;
    float dv = params.a;

    // Laplacian
    vec2 step = 1.0 / ScreenSize;
    vec4 n = texture2D(StateTexture, uv + vec2(0.0, -step.y));
    vec4 s = texture2D(StateTexture, uv + vec2(0.0, step.y));
    vec4 e = texture2D(StateTexture, uv + vec2(step.x, 0.0));
    vec4 w = texture2D(StateTexture, uv + vec2(-step.x, 0.0));

    // Diagonal neighbors for better isotropy (optional, stick to 5-point stencil for speed)
    // float laplacian_u = (n.r + s.r + e.r + w.r - 4.0 * u);
    // float laplacian_v = (n.g + s.g + e.g + w.g - 4.0 * v);

    // 9-point stencil
    vec4 nw = texture2D(StateTexture, uv + vec2(-step.x, -step.y));
    vec4 ne = texture2D(StateTexture, uv + vec2(step.x, -step.y));
    vec4 sw = texture2D(StateTexture, uv + vec2(-step.x, step.y));
    vec4 se = texture2D(StateTexture, uv + vec2(step.x, step.y));

    float laplacian_u = (0.05 * (nw.r + ne.r + sw.r + se.r) + 0.2 * (n.r + s.r + e.r + w.r) - 1.0 * u);
    float laplacian_v = (0.05 * (nw.g + ne.g + sw.g + se.g) + 0.2 * (n.g + s.g + e.g + w.g) - 1.0 * v);

    // Reaction-Diffusion
    float uvv = u * v * v;
    float new_u = u + (du * laplacian_u - uvv + f * (1.0 - u));
    float new_v = v + (dv * laplacian_v + uvv - (f + k) * v);

    gl_FragColor = vec4(clamp(new_u, 0.0, 1.0), clamp(new_v, 0.0, 1.0), 0.0, 1.0);
}
"#;

struct State {
    state_a: RenderTarget,
    state_b: RenderTarget,
    catalyst_map: RenderTarget,
    material: Material,
    metrics: Vec<FileMetric>,
}

impl State {
    fn new() -> Self {
        let state_a = render_target(WIDTH, HEIGHT);
        let state_b = render_target(WIDTH, HEIGHT);
        let catalyst_map = render_target(WIDTH, HEIGHT);

        // Filter mode nearest for sharp pixels? Or linear for smooth reaction?
        state_a.texture.set_filter(FilterMode::Linear);
        state_b.texture.set_filter(FilterMode::Linear);
        catalyst_map.texture.set_filter(FilterMode::Linear);

        let material = load_material(
            ShaderSource::Glsl {
                vertex: VERTEX_SHADER,
                fragment: FRAGMENT_SHADER,
            },
            MaterialParams {
                uniforms: vec![
                    UniformDesc::new("StateTexture", UniformType::Int1),
                    UniformDesc::new("CatalystTexture", UniformType::Int1),
                    UniformDesc::new("ScreenSize", UniformType::Float2),
                ],
                ..Default::default()
            },
        ).unwrap();

        Self {
            state_a,
            state_b,
            catalyst_map,
            material,
            metrics: Vec::new(),
        }
    }

    fn init_catalysts(&mut self) {
        // Collect metrics
        let walker = WalkDir::new("./").max_depth(5);
        for entry in walker.into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                if let Ok(metric) = Scanner::scan_file(entry.path()) {
                    self.metrics.push(metric);
                }
            }
        }

        println!("Scanned {} files", self.metrics.len());

        // Draw to catalyst map
        set_camera(&Camera2D {
            zoom: vec2(2.0 / WIDTH as f32, 2.0 / HEIGHT as f32),
            target: vec2(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0),
            render_target: Some(self.catalyst_map.clone()),
            ..Default::default()
        });

        clear_background(Color::new(0.055, 0.062, 1.0, 0.5)); // Default GS params

        for (i, metric) in self.metrics.iter().enumerate() {
            // Map index to position (spiral or grid)
            // Let's do a random pos based on hash? Or just grid.
            // Let's do a spiral
            let angle = i as f32 * 0.5;
            let radius = (i as f32 * 2.0).sqrt() * 5.0;
            let x = WIDTH as f32 / 2.0 + radius * angle.cos();
            let y = HEIGHT as f32 / 2.0 + radius * angle.sin();

            if x < 0.0 || x > WIDTH as f32 || y < 0.0 || y > HEIGHT as f32 {
                continue;
            }

            // Map metrics to params
            // Feed: 0.01 - 0.1
            // Kill: 0.045 - 0.07
            let feed = 0.01 + (metric.size as f32 / 10000.0).clamp(0.0, 0.09);
            let kill = 0.045 + (metric.complexity / 10.0).clamp(0.0, 0.025);
            let du = 1.0; // Fixed for now
            let dv = 0.5 - (metric.lines as f32 / 1000.0).clamp(0.0, 0.4);

            draw_circle(x, y, 5.0, Color::new(feed, kill, du, dv));
        }

        set_default_camera();
    }

    fn seed(&mut self) {
         set_camera(&Camera2D {
            zoom: vec2(2.0 / WIDTH as f32, 2.0 / HEIGHT as f32),
            target: vec2(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0),
            render_target: Some(self.state_a.clone()),
            ..Default::default()
        });

        // Initial seed in center
        draw_circle(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0, 20.0, Color::new(0.0, 1.0, 0.0, 1.0)); // V = 1.0

        set_default_camera();
    }
}

#[macroquad::main("Code Reaction")]
async fn main() {
    let mut state = State::new();
    state.init_catalysts();
    state.seed();

    loop {
        // Ping Pong Simulation Steps
        for _ in 0..16 { // Speed up simulation
            let source = state.state_a.clone();
            let dest = state.state_b.clone();

            set_camera(&Camera2D {
                zoom: vec2(2.0 / WIDTH as f32, 2.0 / HEIGHT as f32),
                target: vec2(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0),
                render_target: Some(dest),
                ..Default::default()
            });

            state.material.set_texture("StateTexture", source.texture.clone());
            state.material.set_texture("CatalystTexture", state.catalyst_map.texture.clone());
            state.material.set_uniform("ScreenSize", vec2(WIDTH as f32, HEIGHT as f32));

            gl_use_material(&state.material);

            draw_texture_ex(
                &source.texture,
                0.0,
                0.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(WIDTH as f32, HEIGHT as f32)),
                    ..Default::default()
                },
            );

            gl_use_default_material();
            set_default_camera();

            // Swap
            std::mem::swap(&mut state.state_a, &mut state.state_b);
        }

        // Render to Screen
        clear_background(BLACK);

        // Just draw the V channel (chemical)
        let texture = &state.state_a.texture;

        // We need a simple shader to visualize V as color?
        // Or just draw it white?
        // Let's just draw it. Since G channel is V, and we draw WHITE, it will show up green.
        draw_texture_ex(
            texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                flip_y: true,
                ..Default::default()
            },
        );

        draw_text(format!("FPS: {}", get_fps()).as_str(), 10.0, 20.0, 30.0, WHITE);
        draw_text(format!("Files: {}", state.metrics.len()).as_str(), 10.0, 50.0, 30.0, LIGHTGRAY);

        if is_mouse_button_down(MouseButton::Left) {
             let (mx, my) = mouse_position();
             // Convert screen to render target coords
             let tx = mx / screen_width() * WIDTH as f32;
             let ty = my / screen_height() * HEIGHT as f32;

             // Inject chemical into state_a
             set_camera(&Camera2D {
                zoom: vec2(2.0 / WIDTH as f32, 2.0 / HEIGHT as f32),
                target: vec2(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0),
                render_target: Some(state.state_a.clone()),
                ..Default::default()
            });
            draw_circle(tx, ty, 10.0, Color::new(0.0, 1.0, 0.0, 1.0));
            set_default_camera();
        }

        next_frame().await
    }
}
