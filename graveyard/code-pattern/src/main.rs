use macroquad::prelude::*;

mod hilbert;
mod scanner;

use hilbert::d2xy;
use scanner::Scanner;

const WIDTH: u32 = 512;
const HEIGHT: u32 = 512;

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
uniform sampler2D ParamTexture;
uniform vec2 ScreenSize;

void main() {
    vec4 state = texture2D(StateTexture, uv);
    vec4 params = texture2D(ParamTexture, uv);

    float u = state.r;
    float v = state.g;

    // Decode params from [0, 1] range to simulation range
    // R channel -> Feed rate. Map 0.0-1.0 to 0.01-0.1
    // G channel -> Kill rate. Map 0.0-1.0 to 0.045-0.07

    // Default Gray-Scott spots: F=0.035, K=0.065 roughly?
    // Let's allow wide range for "Wild Mode".

    // If params.r (Feed) is 0, we use a default base rate.
    // Base F = 0.0545, K = 0.062

    float feed = 0.01 + params.r * 0.09;
    float kill = 0.045 + params.g * 0.025;

    // If no file (black params), use default "Mitosis" spot parameters
    if (params.r < 0.001 && params.g < 0.001) {
        feed = 0.0367;
        kill = 0.06418;
    }

    // Diffusion rates
    // Stable simulation requires D_u * dt / dx^2 < 0.25
    // Let's say dt=1.0, dx=1.0. D_u = 0.2, D_v = 0.1
    float diff_u = 0.2097;
    float diff_v = 0.105;

    // Laplacian (5-point)
    vec2 step = 1.0 / ScreenSize;
    float val_u = 0.0;
    float val_v = 0.0;

    // Center
    val_u += -4.0 * u;
    val_v += -4.0 * v;

    // Neighbors
    val_u += texture2D(StateTexture, uv + vec2(0.0, -step.y)).r;
    val_u += texture2D(StateTexture, uv + vec2(0.0, step.y)).r;
    val_u += texture2D(StateTexture, uv + vec2(step.x, 0.0)).r;
    val_u += texture2D(StateTexture, uv + vec2(-step.x, 0.0)).r;

    val_v += texture2D(StateTexture, uv + vec2(0.0, -step.y)).g;
    val_v += texture2D(StateTexture, uv + vec2(0.0, step.y)).g;
    val_v += texture2D(StateTexture, uv + vec2(step.x, 0.0)).g;
    val_v += texture2D(StateTexture, uv + vec2(-step.x, 0.0)).g;

    // Reaction-Diffusion
    float uvv = u * v * v;
    float du = diff_u * val_u - uvv + feed * (1.0 - u);
    float dv = diff_v * val_v + uvv - (feed + kill) * v;

    float nu = clamp(u + du, 0.0, 1.0);
    float nv = clamp(v + dv, 0.0, 1.0);

    gl_FragColor = vec4(nu, nv, 0.0, 1.0);
}
"#;

// Render shader to colorize the output
const RENDER_FRAGMENT_SHADER: &str = r#"
#version 100
precision highp float;

varying vec2 uv;
uniform sampler2D Texture;

void main() {
    vec4 state = texture2D(Texture, uv);
    float v = state.g;
    float u = state.r;

    // Visualization
    // U is background (1.0 usually), V is the pattern (0.0 usually, grows to 1.0)
    // We want to visualize V.

    // Color palette based on V
    vec3 col = vec3(0.0);
    if (v > 0.0) {
        // gradient
        col = mix(vec3(0.1, 0.1, 0.2), vec3(0.2, 0.8, 0.5), v * 2.0);
        col = mix(col, vec3(0.9, 0.9, 1.0), smoothstep(0.4, 0.6, v));
    } else {
        col = vec3(0.05, 0.05, 0.1) * u;
    }

    gl_FragColor = vec4(col, 1.0);
}
"#;

struct SimulationState {
    texture_a: RenderTarget,
    texture_b: RenderTarget,
    param_texture: Texture2D,
    material: Material,
    render_material: Material,
    step_count: u64,
}

impl SimulationState {
    fn new() -> Self {
        let texture_a = render_target(WIDTH, HEIGHT);
        let texture_b = render_target(WIDTH, HEIGHT);

        texture_a.texture.set_filter(FilterMode::Nearest);
        texture_b.texture.set_filter(FilterMode::Nearest);

        // Param texture (static)
        let param_texture = Texture2D::from_image(&Image::gen_image_color(
            WIDTH as u16,
            HEIGHT as u16,
            Color::new(0.0, 0.0, 0.0, 1.0),
        ));
        param_texture.set_filter(FilterMode::Nearest);

        let material = load_material(
            ShaderSource::Glsl {
                vertex: VERTEX_SHADER,
                fragment: FRAGMENT_SHADER,
            },
            MaterialParams {
                uniforms: vec![
                    UniformDesc::new("StateTexture", UniformType::Int1),
                    UniformDesc::new("ParamTexture", UniformType::Int1),
                    UniformDesc::new("ScreenSize", UniformType::Float2),
                ],
                ..Default::default()
            },
        )
        .unwrap();

        let render_material = load_material(
            ShaderSource::Glsl {
                vertex: VERTEX_SHADER,
                fragment: RENDER_FRAGMENT_SHADER,
            },
            MaterialParams {
                uniforms: vec![UniformDesc::new("Texture", UniformType::Int1)],
                ..Default::default()
            },
        )
        .unwrap();

        Self {
            texture_a,
            texture_b,
            param_texture,
            material,
            render_material,
            step_count: 0,
        }
    }

    fn init_params(&mut self) {
        // Scan current directory
        let metrics = Scanner::scan_directory(".");
        println!("Scanned {} files", metrics.len());

        // Create image buffer for params
        let mut image = Image::gen_image_color(WIDTH as u16, HEIGHT as u16, BLACK);

        // Map files to grid
        // We use Hilbert curve to map index to x,y
        // Grid size is 512x512 = 2^18 pixels.
        // We have N files. We can map them sequentially.
        // If N < 512*512, we fill a part of the screen.
        // To make it look cooler, we might want to scale the Hilbert curve or just fill it.

        // Let's assume we map each file to a block of pixels or just one pixel?
        // 1 pixel is too small to influence diffusion much.
        // Let's map each file to a 4x4 block?

        // Better: Map file i to position (x,y) via Hilbert.
        // Then draw a circle or splash of parameters there.

        let n = 9; // 2^9 = 512.
                   // Wait, d2xy(n, d) returns x,y in range [0, 2^n - 1].

        for (i, metric) in metrics.iter().enumerate() {
            // Hilbert mapping
            // We scale 'i' to cover the space?
            // Or just use 'i' directly. If we have 1000 files, we only use 1000 pixels.
            // That's very sparse.
            // Let's map i * STRIDE.
            let stride = 50;
            let d = (i as u32 * stride) % (WIDTH * HEIGHT);
            let (x, y) = d2xy(n, d);

            // Map params
            // Feed = Size
            // Kill = Depth/Complexity

            // Normalize size: 0-1MB -> 0.0-1.0
            let norm_size = (metric.size as f32 / 10000.0).clamp(0.0, 1.0);

            // Normalize depth: 0-10 -> 0.0-1.0
            let norm_depth = (metric.depth as f32 / 10.0).clamp(0.0, 1.0);
            let norm_complexity = metric.complexity;
            let hash_val = metric.hash_val;

            if i % 100 == 0 {
                println!(
                    "Mapping file {}: Size={} Depth={}",
                    metric.path, metric.size, metric.depth
                );
            }

            // Draw a blob in the param image
            // We draw a circle at x,y
            let radius = 2.0 + norm_size * 5.0;

            // Manually draw to image pixel data
            // (Image doesn't have draw_circle, we iterate)

            let r_u8 = (norm_size * 255.0) as u8;
            let g_u8 = ((norm_depth * 0.6 + norm_complexity * 0.3 + hash_val * 0.1) * 255.0) as u8;
            let col = Color::from_rgba(r_u8, g_u8, 0, 255); // R=Feed, G=Kill

            for dy in -(radius as i32)..=(radius as i32) {
                for dx in -(radius as i32)..=(radius as i32) {
                    let px = x as i32 + dx;
                    let py = y as i32 + dy;

                    if px >= 0 && px < WIDTH as i32 && py >= 0 && py < HEIGHT as i32 {
                        let dist = (dx * dx + dy * dy) as f32;
                        if dist <= radius * radius {
                            image.set_pixel(px as u32, py as u32, col);
                        }
                    }
                }
            }
        }

        self.param_texture = Texture2D::from_image(&image);
        self.param_texture.set_filter(FilterMode::Nearest);
    }

    fn seed(&mut self) {
        set_camera(&Camera2D {
            zoom: vec2(1.0, 1.0),   // Maps -1..1 to screen.
            target: vec2(0.0, 0.0), // Center
            render_target: Some(self.texture_a.clone()),
            ..Default::default()
        });

        // Initialize with U=1, V=0
        // We use clear_background. R=1, G=0.
        clear_background(Color::new(1.0, 0.0, 0.0, 1.0));

        // Add some random V seeds
        // We can use draw_rectangle or similar.
        // Coordinate system:
        // Camera zoom 1.0 means viewport is 2.0 units wide (-1 to 1).
        // Render target is WIDTH x HEIGHT.
        // It's easier to use pixel coordinates with custom camera.

        set_camera(&Camera2D {
            zoom: vec2(2.0 / WIDTH as f32, 2.0 / HEIGHT as f32),
            target: vec2(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0),
            render_target: Some(self.texture_a.clone()),
            ..Default::default()
        });

        // Random seeds
        for _ in 0..50 {
            let x = macroquad::rand::gen_range(0.0, WIDTH as f32);
            let y = macroquad::rand::gen_range(0.0, HEIGHT as f32);
            let r = macroquad::rand::gen_range(5.0, 20.0);
            draw_circle(x, y, r, Color::new(1.0, 1.0, 0.0, 1.0)); // G=1 means V=1
        }

        set_default_camera();
    }

    fn add_catalyst(&mut self, x: f32, y: f32) {
        // Draw to texture_a (or current active one)
        set_camera(&Camera2D {
            zoom: vec2(2.0 / WIDTH as f32, 2.0 / HEIGHT as f32),
            target: vec2(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0),
            render_target: Some(self.texture_a.clone()), // Ideally check which is active but writing to one is enough if we swap fast
            ..Default::default()
        });

        draw_circle(x, y, 10.0, Color::new(0.5, 1.0, 0.0, 1.0)); // Add V

        set_default_camera();

        // Also write to B to avoid flickering if it's the read one
        set_camera(&Camera2D {
            zoom: vec2(2.0 / WIDTH as f32, 2.0 / HEIGHT as f32),
            target: vec2(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0),
            render_target: Some(self.texture_b.clone()),
            ..Default::default()
        });
        draw_circle(x, y, 10.0, Color::new(0.5, 1.0, 0.0, 1.0));
        set_default_camera();
    }
}

#[macroquad::main("Code Pattern")]
async fn main() {
    let mut sim = SimulationState::new();
    sim.init_params();
    sim.seed();

    loop {
        // Physics Steps (Ping Pong)
        // 16 steps per frame
        for _ in 0..16 {
            let source = sim.texture_a.clone();
            let dest = sim.texture_b.clone();

            set_camera(&Camera2D {
                zoom: vec2(2.0 / WIDTH as f32, 2.0 / HEIGHT as f32),
                target: vec2(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0),
                render_target: Some(dest),
                ..Default::default()
            });

            sim.material
                .set_texture("StateTexture", source.texture.clone());
            sim.material
                .set_texture("ParamTexture", sim.param_texture.clone());
            sim.material
                .set_uniform("ScreenSize", vec2(WIDTH as f32, HEIGHT as f32));

            gl_use_material(&sim.material);

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
            std::mem::swap(&mut sim.texture_a, &mut sim.texture_b);
            sim.step_count += 1;
        }

        // Render to Screen
        clear_background(BLACK);

        sim.render_material
            .set_texture("Texture", sim.texture_a.texture.clone());
        gl_use_material(&sim.render_material);

        // Draw centered
        let screen_size = vec2(screen_width(), screen_height());
        let scale = (screen_size.x / WIDTH as f32).min(screen_size.y / HEIGHT as f32);
        let dest_size = vec2(WIDTH as f32 * scale, HEIGHT as f32 * scale);
        let offset = (screen_size - dest_size) * 0.5;

        draw_texture_ex(
            &sim.texture_a.texture,
            offset.x,
            offset.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(dest_size),
                flip_y: true, // Macroquad render targets are flipped
                ..Default::default()
            },
        );

        gl_use_default_material();

        // UI / Interaction
        if is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();
            // Convert to texture coords
            let tx = (mx - offset.x) / scale;
            let ty = (my - offset.y) / scale; // Flip Y? Render target is flipped when drawn, but coordinates?
                                              // Actually, when we draw TO the render target, 0,0 is top-left usually.
                                              // But when we draw the render target texture to screen, we flipped it.
                                              // So if we click top-left on screen, it corresponds to top-left on texture (if flip is correct).
                                              // Let's assume standard mapping.
            let ty = HEIGHT as f32 - ty; // Flip mouse Y to match texture coordinate system if needed

            if tx >= 0.0 && tx < WIDTH as f32 && ty >= 0.0 && ty < HEIGHT as f32 {
                sim.add_catalyst(tx, ty);
            }
        }

        if is_key_pressed(KeyCode::R) {
            sim.seed();
        }

        draw_text(
            format!("Steps: {}", sim.step_count).as_str(),
            10.0,
            20.0,
            30.0,
            WHITE,
        );
        draw_text(
            "Left Click: Add Catalyst | R: Reset",
            10.0,
            50.0,
            20.0,
            LIGHTGRAY,
        );

        next_frame().await
    }
}
