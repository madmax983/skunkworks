use macroquad::prelude::*;

mod scanner;
use scanner::scan_codebase;
use std::path::Path;

const VERTEX_SHADER: &str = "#version 100
attribute vec3 position;
attribute vec2 texcoord;

varying lowp vec2 uv;

void main() {
    gl_Position = vec4(position, 1.0);
    uv = texcoord;
}
";

const REACTION_FRAGMENT_SHADER: &str = "#version 100
precision highp float;

varying vec2 uv;

uniform sampler2D Texture; // Previous State (r=A, g=B)
uniform sampler2D Params;  // Parameters (r=Feed, g=Kill)
uniform vec2 ScreenSize;
uniform vec2 Mouse;
uniform float MouseDown;

// Gray-Scott Parameters
// Default if params texture is empty
const float DA = 1.0;
const float DB = 0.5;
const float DT = 1.0;

void main() {
    vec2 pixel = 1.0 / ScreenSize;

    // 3x3 Laplacian Convolution
    // Center
    vec4 val = texture2D(Texture, uv);
    float a = val.r;
    float b = val.g;

    // Neighbors
    vec2 u = vec2(0.0, pixel.y);
    vec2 r = vec2(pixel.x, 0.0);

    // Laplacian weights:
    // 0.05 0.2 0.05
    // 0.2 -1.0 0.2
    // 0.05 0.2 0.05

    vec4 sum = vec4(0.0);
    sum += texture2D(Texture, uv - u - r) * 0.05;
    sum += texture2D(Texture, uv - u)     * 0.20;
    sum += texture2D(Texture, uv - u + r) * 0.05;

    sum += texture2D(Texture, uv - r)     * 0.20;
    sum += texture2D(Texture, uv)         * -1.0;
    sum += texture2D(Texture, uv + r)     * 0.20;

    sum += texture2D(Texture, uv + u - r) * 0.05;
    sum += texture2D(Texture, uv + u)     * 0.20;
    sum += texture2D(Texture, uv + u + r) * 0.05;

    float lapA = sum.r;
    float lapB = sum.g;

    // Read local parameters
    vec4 params = texture2D(Params, uv);
    float f = params.r; // Feed
    float k = params.g; // Kill

    // Add some noise to F/K if params are 0 (unmapped area)
    if (f == 0.0) f = 0.055;
    if (k == 0.0) k = 0.062;

    // Reaction-Diffusion
    float ab2 = a * b * b;

    float newA = a + (DA * lapA - ab2 + f * (1.0 - a)) * DT;
    float newB = b + (DB * lapB + ab2 - (k + f) * b) * DT;

    // Mouse Interaction (Adding chemical B)
    vec2 dist = (uv - Mouse);
    dist.x *= ScreenSize.x / ScreenSize.y; // Aspect correction for distance
    if (MouseDown > 0.5 && length(dist) < 0.01) {
        newB = 0.9;
    }

    gl_FragColor = vec4(clamp(newA, 0.0, 1.0), clamp(newB, 0.0, 1.0), 0.0, 1.0);
}
";

const RENDER_FRAGMENT_SHADER: &str = "#version 100
precision highp float;

varying vec2 uv;

uniform sampler2D Texture;

void main() {
    vec4 val = texture2D(Texture, uv);
    float a = val.r;
    float b = val.g;

    // Visualization
    // A = Substrate (White/Clear)
    // B = Pattern (Color)

    vec3 colorA = vec3(0.1, 0.1, 0.2); // Dark Blue background
    vec3 colorB = vec3(0.0, 1.0, 0.8); // Cyan patterns
    vec3 colorC = vec3(1.0, 1.0, 1.0); // White highlights

    // Mix based on B concentration
    vec3 color = mix(colorA, colorB, b * 2.0);
    color = mix(color, colorC, pow(b, 3.0)); // Highlight peaks

    gl_FragColor = vec4(color, 1.0);
}
";

fn window_conf() -> Conf {
    Conf {
        window_title: "Code Phage".to_owned(),
        window_width: 1024,
        window_height: 1024,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() -> anyhow::Result<()> {
    let w = 512;
    let h = 512;

    // Initialize Render Targets (Ping-Pong)
    let rt_a = render_target(w, h);
    let rt_b = render_target(w, h);
    rt_a.texture.set_filter(FilterMode::Nearest);
    rt_b.texture.set_filter(FilterMode::Nearest);

    // Params Texture
    // Fill Params Image from Scanner
    println!("Scanning codebase...");
    let genes = scan_codebase(Path::new("."))?;
    println!("Found {} files/genes.", genes.len());

    let mut param_bytes = vec![0u8; (w * h * 4) as usize];

    // Seed the map
    for gene in &genes {
        let px = (gene.x * w as f32) as usize;
        let py = (gene.y * h as f32) as usize;

        // Draw a small circle of influence for each file
        let radius = 2;
        for dy in -(radius as isize)..=radius as isize {
            for dx in -(radius as isize)..=radius as isize {
                let nx = px as isize + dx;
                let ny = py as isize + dy;

                if nx >= 0 && nx < w as isize && ny >= 0 && ny < h as isize {
                    let idx = (ny as usize * w as usize + nx as usize) * 4;
                    // R = Feed, G = Kill
                    param_bytes[idx] = (gene.feed * 255.0) as u8;
                    param_bytes[idx + 1] = (gene.kill * 255.0) as u8;
                    param_bytes[idx + 2] = 0;
                    param_bytes[idx + 3] = 255;
                }
            }
        }
    }

    let texture_params = Texture2D::from_rgba8(w as u16, h as u16, &param_bytes);
    texture_params.set_filter(FilterMode::Nearest);

    // Initialize state with some noise to kickstart reaction
    let mut initial_bytes = vec![0u8; (w * h * 4) as usize];
    for i in 0..initial_bytes.len() / 4 {
        // A = 1.0 (Full substrate)
        initial_bytes[i * 4] = 255;
        // B = 0.0 (No reagent), except small noise
        if rand::gen_range(0, 100) > 99 {
            initial_bytes[i * 4 + 1] = 255;
        }
        initial_bytes[i * 4 + 3] = 255;
    }
    rt_a.texture.update(&Image {
        width: w as u16,
        height: h as u16,
        bytes: initial_bytes,
    });

    // Materials
    let reaction_material = load_material(
        ShaderSource::Glsl {
            vertex: VERTEX_SHADER,
            fragment: REACTION_FRAGMENT_SHADER,
        },
        MaterialParams {
            uniforms: vec![
                UniformDesc::new("ScreenSize", UniformType::Float2),
                UniformDesc::new("Mouse", UniformType::Float2),
                UniformDesc::new("MouseDown", UniformType::Float1),
                // Samplers are handled via `textures`
            ],
            textures: vec![
                "Params".to_string(), // Name in shader
            ],
            ..Default::default()
        },
    )?;

    let render_material = load_material(
        ShaderSource::Glsl {
            vertex: VERTEX_SHADER,
            fragment: RENDER_FRAGMENT_SHADER,
        },
        MaterialParams {
            ..Default::default()
        },
    )?;

    // Ping Pong State
    let mut current_rt = rt_a;
    let mut next_rt = rt_b;

    println!("Simulation started.");

    loop {
        // --- Physics Step ---

        // We render TO next_rt
        set_camera(&Camera2D {
            render_target: Some(next_rt.clone()),
            ..Default::default()
        });

        gl_use_material(&reaction_material);

        // Pass Uniforms
        reaction_material.set_uniform("ScreenSize", (w as f32, h as f32));

        let (mx, my) = mouse_position();
        // Normalize mouse to 0..1 UV space
        let sw = screen_width();
        let sh = screen_height();

        reaction_material.set_uniform("Mouse", (mx / sw, 1.0 - (my / sh)));
        reaction_material.set_uniform(
            "MouseDown",
            if is_mouse_button_down(MouseButton::Left) {
                1.0f32
            } else {
                0.0f32
            },
        );
        reaction_material.set_texture("Params", texture_params.clone());

        // Draw the full screen quad (which is the previous state texture)
        draw_texture_ex(
            &current_rt.texture,
            -1.0,
            -1.0, // Position (NDC)
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(2.0, 2.0)), // Full NDC coverage
                ..Default::default()
            },
        );

        gl_use_default_material();
        set_default_camera();

        // --- Render Step ---
        clear_background(BLACK);

        gl_use_material(&render_material);

        // Draw the NEW state to screen
        // We use the same pass-through vertex shader, so we must provide NDC coordinates.
        draw_texture_ex(
            &next_rt.texture,
            -1.0,
            -1.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(2.0, 2.0)),
                ..Default::default()
            },
        );

        gl_use_default_material();

        // Swap buffers
        let temp = current_rt;
        current_rt = next_rt;
        next_rt = temp;

        // UI Overlay
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 20.0, 30.0, WHITE);
        draw_text(
            &format!("Files: {}", genes.len()),
            10.0,
            50.0,
            20.0,
            LIGHTGRAY,
        );

        next_frame().await
    }
}
