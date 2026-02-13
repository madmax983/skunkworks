mod boid;
mod world;

use macroquad::prelude::*;
use noise::{NoiseFn, Perlin};
use world::World;

const W: usize = 256;
const H: usize = 256;

const VERTEX_SHADER_PASSTHROUGH: &str = r#"#version 100
attribute vec3 position;
attribute vec2 texcoord;
varying lowp vec2 uv;
void main() {
    gl_Position = vec4(position, 1.0);
    uv = texcoord;
}
"#;

const SIMULATION_FRAGMENT_SHADER: &str = r#"#version 100
precision highp float;

varying vec2 uv;

uniform sampler2D Texture;    // r=A, g=B
uniform sampler2D HeightMap;  // r=Height

uniform vec2 ScreenSize;
uniform float Time;
uniform float MouseDown;
uniform vec2 MousePos;

// Simulation Parameters
const float DA = 1.0;
const float DB = 0.5;
const float DT = 1.0;

// Advection
const float ADVECTION_STRENGTH = 0.002;

void main() {
    vec2 pixel = 1.0 / ScreenSize;

    // --- 1. Advection ---
    float hL = texture2D(HeightMap, uv - vec2(pixel.x, 0.0)).r;
    float hR = texture2D(HeightMap, uv + vec2(pixel.x, 0.0)).r;
    float hD = texture2D(HeightMap, uv - vec2(0.0, pixel.y)).r;
    float hU = texture2D(HeightMap, uv + vec2(0.0, pixel.y)).r;

    vec2 gradient = vec2(hR - hL, hU - hD);
    vec2 advection_offset = gradient * ADVECTION_STRENGTH;
    vec2 source_uv = uv + advection_offset;

    // --- 2. Laplacian ---
    vec4 val = texture2D(Texture, source_uv);
    float a = val.r;
    float b = val.g;

    vec4 sum_full = vec4(0.0);
    sum_full += texture2D(Texture, source_uv + vec2(-pixel.x, -pixel.y)) * 0.05;
    sum_full += texture2D(Texture, source_uv + vec2(0.0, -pixel.y))      * 0.20;
    sum_full += texture2D(Texture, source_uv + vec2( pixel.x, -pixel.y)) * 0.05;

    sum_full += texture2D(Texture, source_uv + vec2(-pixel.x, 0.0))      * 0.20;
    sum_full += texture2D(Texture, source_uv)                            * -1.0;
    sum_full += texture2D(Texture, source_uv + vec2( pixel.x, 0.0))      * 0.20;

    sum_full += texture2D(Texture, source_uv + vec2(-pixel.x,  pixel.y)) * 0.05;
    sum_full += texture2D(Texture, source_uv + vec2(0.0,  pixel.y))      * 0.20;
    sum_full += texture2D(Texture, source_uv + vec2( pixel.x,  pixel.y)) * 0.05;

    float lapA = sum_full.r;
    float lapB = sum_full.g;

    // --- 3. Parameters from Height ---
    float height = texture2D(HeightMap, uv).r;

    float f = mix(0.060, 0.020, height);
    float k = mix(0.060, 0.065, height);

    // --- 4. Reaction ---
    float ab2 = a * b * b;
    float newA = a + (DA * lapA - ab2 + f * (1.0 - a)) * DT;
    float newB = b + (DB * lapB + ab2 - (k + f) * b) * DT;

    // --- 5. Mouse Interaction ---
    float dist = distance(uv, MousePos);
    if (MouseDown > 0.5 && dist < 0.1) {
        newB = 0.9;
    }

    gl_FragColor = vec4(clamp(newA, 0.0, 1.0), clamp(newB, 0.0, 1.0), 0.0, 1.0);
}
"#;

const RENDER_VERTEX_SHADER: &str = r#"#version 100
attribute vec3 position;
attribute vec2 texcoord;

varying lowp vec2 uv;
varying lowp float vHeight;

uniform mat4 Model;
uniform mat4 View;
uniform mat4 Projection;

uniform sampler2D HeightMap;
const float HEIGHT_SCALE = 4.0;

void main() {
    uv = texcoord;

    float h = texture2D(HeightMap, uv).r;
    vHeight = h;

    vec3 pos = position;
    pos.y += h * HEIGHT_SCALE;

    gl_Position = Projection * View * Model * vec4(pos, 1.0);
}
"#;

const RENDER_FRAGMENT_SHADER: &str = r#"#version 100
precision highp float;

varying lowp vec2 uv;
varying lowp float vHeight;

uniform sampler2D Texture;

void main() {
    float b = texture2D(Texture, uv).g;

    vec3 sand = vec3(0.76, 0.70, 0.50);
    vec3 forest = vec3(0.13, 0.55, 0.13);
    vec3 rock = vec3(0.5, 0.5, 0.5);
    vec3 snow = vec3(1.0, 1.0, 1.0);

    vec3 terrainColor = sand;
    if (vHeight > 0.3) terrainColor = mix(sand, forest, clamp((vHeight - 0.3) * 5.0, 0.0, 1.0));
    if (vHeight > 0.6) terrainColor = mix(terrainColor, rock, clamp((vHeight - 0.6) * 5.0, 0.0, 1.0));
    if (vHeight > 0.85) terrainColor = mix(terrainColor, snow, clamp((vHeight - 0.85) * 10.0, 0.0, 1.0));

    vec3 lifeColor = vec3(0.0, 1.0, 0.8);
    vec3 finalColor = mix(terrainColor, lifeColor, b * 0.9);

    // Simple lighting
    finalColor *= 0.5 + 0.5 * vHeight;

    gl_FragColor = vec4(finalColor, 1.0);
}
"#;

fn generate_heightmap(seed: u32) -> Image {
    let perlin = Perlin::new(seed);
    let mut bytes = vec![0u8; W * H * 4];

    for y in 0..H {
        for x in 0..W {
            let nx = x as f64 / W as f64 - 0.5;
            let ny = y as f64 / H as f64 - 0.5;

            let mut val = 0.0;
            let mut freq = 3.0;
            let mut amp = 1.0;
            let mut max = 0.0;

            for _ in 0..6 {
                val += perlin.get([nx * freq, ny * freq]) * amp;
                max += amp;
                freq *= 2.0;
                amp *= 0.5;
            }

            val = val / max;
            val = (val + 1.0) * 0.5;
            val = val.clamp(0.0, 1.0);

            let i = (y * W + x) * 4;
            let v = (val * 255.0) as u8;
            bytes[i] = v;
            bytes[i + 1] = v;
            bytes[i + 2] = v;
            bytes[i + 3] = 255;
        }
    }

    Image {
        width: W as u16,
        height: H as u16,
        bytes,
    }
}

fn generate_plane_mesh(w: usize, h: usize) -> Mesh {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for y in 0..=h {
        for x in 0..=w {
            let u = x as f32 / w as f32;
            let v = y as f32 / h as f32;

            vertices.push(Vertex {
                position: Vec3::new(u * 20.0 - 10.0, 0.0, v * 20.0 - 10.0),
                uv: Vec2::new(u, v),
                color: WHITE.into(),
                normal: Vec4::ZERO,
            });
        }
    }

    for y in 0..h {
        for x in 0..w {
            let i = (y * (w + 1) + x) as u16;
            let i_right = i + 1;
            let i_down = ((y + 1) * (w + 1) + x) as u16;
            let i_down_right = i_down + 1;

            indices.push(i);
            indices.push(i_down);
            indices.push(i_right);

            indices.push(i_right);
            indices.push(i_down);
            indices.push(i_down_right);
        }
    }

    Mesh {
        vertices,
        indices,
        texture: None,
    }
}

#[macroquad::main("Luminous Valley")]
async fn main() {
    let heightmap_img = generate_heightmap(12345);
    let heightmap_tex = Texture2D::from_image(&heightmap_img);
    heightmap_tex.set_filter(FilterMode::Linear);

    let rt_a = render_target(W as u32, H as u32);
    let rt_b = render_target(W as u32, H as u32);
    rt_a.texture.set_filter(FilterMode::Linear);
    rt_b.texture.set_filter(FilterMode::Linear);

    let mut initial_bytes = vec![0u8; W * H * 4];
    for i in 0..W * H {
        initial_bytes[i * 4] = 255;
        initial_bytes[i * 4 + 1] = if macroquad::rand::gen_range(0, 100) > 98 {
            255
        } else {
            0
        };
        initial_bytes[i * 4 + 2] = 0;
        initial_bytes[i * 4 + 3] = 255;
    }
    rt_a.texture.update(&Image {
        width: W as u16,
        height: H as u16,
        bytes: initial_bytes,
    });

    let simulation_material = load_material(
        ShaderSource::Glsl {
            vertex: VERTEX_SHADER_PASSTHROUGH,
            fragment: SIMULATION_FRAGMENT_SHADER,
        },
        MaterialParams {
            uniforms: vec![
                UniformDesc::new("ScreenSize", UniformType::Float2),
                UniformDesc::new("Time", UniformType::Float1),
                UniformDesc::new("MouseDown", UniformType::Float1),
                UniformDesc::new("MousePos", UniformType::Float2),
            ],
            textures: vec!["HeightMap".to_string()],
            ..Default::default()
        },
    )
    .unwrap();

    let render_material = load_material(
        ShaderSource::Glsl {
            vertex: RENDER_VERTEX_SHADER,
            fragment: RENDER_FRAGMENT_SHADER,
        },
        MaterialParams {
            uniforms: vec![],
            textures: vec!["HeightMap".to_string(), "Texture".to_string()],
            ..Default::default()
        },
    )
    .unwrap();

    let plane_mesh = generate_plane_mesh(250, 250);

    let mut current_rt = rt_a;
    let mut next_rt = rt_b;

    // Initialize World (Boids)
    // Map is 20x20 (-10 to 10)
    let mut world = World::new(20.0, 20.0, 200);

    let mut cam_pos = Vec3::new(0.0, 15.0, 15.0);
    let cam_target = Vec3::new(0.0, 0.0, 0.0);

    loop {
        if is_key_down(KeyCode::Left) {
            cam_pos.x -= 0.1;
        }
        if is_key_down(KeyCode::Right) {
            cam_pos.x += 0.1;
        }
        if is_key_down(KeyCode::Up) {
            cam_pos.z -= 0.1;
        }
        if is_key_down(KeyCode::Down) {
            cam_pos.z += 0.1;
        }
        if is_key_down(KeyCode::W) {
            cam_pos.y += 0.1;
        }
        if is_key_down(KeyCode::S) {
            cam_pos.y -= 0.1;
        }

        // --- Simulation Pass ---
        set_camera(&Camera2D {
            render_target: Some(next_rt.clone()),
            ..Default::default()
        });

        gl_use_material(&simulation_material);

        simulation_material.set_uniform("ScreenSize", (W as f32, H as f32));
        simulation_material.set_uniform("Time", get_time() as f32);

        if is_key_down(KeyCode::Space) {
            simulation_material.set_uniform("MouseDown", 1.0f32);
            simulation_material.set_uniform("MousePos", (0.5f32, 0.5f32));
        } else {
            simulation_material.set_uniform("MouseDown", 0.0f32);
        }

        simulation_material.set_texture("HeightMap", heightmap_tex.clone());

        draw_texture_ex(
            &current_rt.texture,
            -1.0,
            -1.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(2.0, 2.0)),
                ..Default::default()
            },
        );

        gl_use_default_material();
        set_default_camera();

        // Swap
        let temp = current_rt;
        current_rt = next_rt;
        next_rt = temp;

        // --- Boid Logic ---
        // Read the chemical texture (current_rt) back to CPU
        let chemical_img = current_rt.texture.get_texture_data();

        // Update world
        world.update(&heightmap_img, Some(&chemical_img));

        // --- Render Pass ---
        clear_background(BLACK);

        set_camera(&Camera3D {
            position: cam_pos,
            up: Vec3::new(0.0, 1.0, 0.0),
            target: cam_target,
            ..Default::default()
        });

        // Draw Terrain
        gl_use_material(&render_material);
        render_material.set_texture("HeightMap", heightmap_tex.clone());
        render_material.set_texture("Texture", current_rt.texture.clone());

        draw_mesh(&plane_mesh);
        gl_use_default_material();

        // Draw Boids
        for boid in &world.boids {
            let chem = boid.chemical_exposure;
            let mut color = boid.dna.color;

            // Mix with Cyan based on chemical exposure
            let cyan = Color::new(0.0, 1.0, 0.8, 1.0);

            color.r = color.r * (1.0 - chem) + cyan.r * chem;
            color.g = color.g * (1.0 - chem) + cyan.g * chem;
            color.b = color.b * (1.0 - chem) + cyan.b * chem;

            // Flash overrides color
            if boid.flash_timer > 0 {
                color = WHITE;
            }

            draw_sphere(boid.position, 0.15, None, color);
        }

        set_default_camera();

        draw_text("Luminous Valley", 20.0, 30.0, 30.0, WHITE);
        draw_text(
            "Keys: Arrows/WASD to move camera. Space to Rain.",
            20.0,
            50.0,
            20.0,
            LIGHTGRAY,
        );
        draw_text(&format!("FPS: {}", get_fps()), 20.0, 70.0, 20.0, LIGHTGRAY);
        draw_text(
            &format!("Boids: {}", world.boids.len()),
            20.0,
            90.0,
            20.0,
            LIGHTGRAY,
        );

        next_frame().await
    }
}
