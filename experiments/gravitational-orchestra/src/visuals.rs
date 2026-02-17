use macroquad::prelude::*;

pub const LENS_SHADER_VERTEX: &str = "#version 100
attribute vec3 position;
attribute vec2 texcoord;
varying lowp vec2 uv;
uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    uv = texcoord;
}
";

// 8 Unrolled Uniforms.
pub const LENS_SHADER_FRAGMENT_8: &str = "#version 100
precision highp float;

varying vec2 uv;
uniform sampler2D u_texture;
uniform vec2 u_resolution;
uniform int u_body_count;

// Individual uniforms
uniform vec3 u_b0;
uniform vec3 u_b1;
uniform vec3 u_b2;
uniform vec3 u_b3;
uniform vec3 u_b4;
uniform vec3 u_b5;
uniform vec3 u_b6;
uniform vec3 u_b7;

vec3 get_body(int i) {
    if (i == 0) return u_b0;
    if (i == 1) return u_b1;
    if (i == 2) return u_b2;
    if (i == 3) return u_b3;
    if (i == 4) return u_b4;
    if (i == 5) return u_b5;
    if (i == 6) return u_b6;
    return u_b7;
}

void main() {
    vec2 screen_pos = uv * u_resolution;
    vec2 deflection = vec2(0.0);

    for (int i = 0; i < 8; i++) {
        if (i >= u_body_count) break;

        vec3 body = get_body(i);
        vec2 body_pos = body.xy;
        float mass = body.z;

        vec2 r_vec = screen_pos - body_pos;
        float r_sq = dot(r_vec, r_vec);

        if (r_sq > 1.0) {
            // Simplified deflection
            float strength = (mass * 50.0) / (r_sq + 100.0);
            deflection -= normalize(r_vec) * strength;
        }
    }

    vec2 uv_offset = deflection / u_resolution;
    vec2 new_uv = uv + uv_offset;
    new_uv = fract(new_uv);

    vec4 color = texture2D(u_texture, new_uv);
    gl_FragColor = color;
}
";

pub struct Visuals {
    pub material: Material,
    pub background_texture: Texture2D,
}

impl Visuals {
    pub fn new() -> Self {
        let material = load_material(
            ShaderSource::Glsl {
                vertex: LENS_SHADER_VERTEX,
                fragment: LENS_SHADER_FRAGMENT_8,
            },
            MaterialParams {
                uniforms: vec![
                    UniformDesc::new("u_resolution", UniformType::Float2),
                    UniformDesc::new("u_body_count", UniformType::Int1),
                    UniformDesc::new("u_b0", UniformType::Float3),
                    UniformDesc::new("u_b1", UniformType::Float3),
                    UniformDesc::new("u_b2", UniformType::Float3),
                    UniformDesc::new("u_b3", UniformType::Float3),
                    UniformDesc::new("u_b4", UniformType::Float3),
                    UniformDesc::new("u_b5", UniformType::Float3),
                    UniformDesc::new("u_b6", UniformType::Float3),
                    UniformDesc::new("u_b7", UniformType::Float3),
                ],
                ..Default::default()
            },
        )
        .expect("Failed to load material");

        let render_target = render_target(1024, 1024);
        render_target.texture.set_filter(FilterMode::Linear);

        let cam = Camera2D {
            render_target: Some(render_target.clone()),
            ..Default::default()
        };
        set_camera(&cam);
        clear_background(BLACK);

        let text = [
            "void main() {",
            "    vec2 pos = uv * resolution;",
            "    float mass = singularity.mass;",
            "    vec2 event_horizon = pos - singularity.pos;",
            "    if (length(event_horizon) < Schwarzschild) discard;",
            "    // The gravity of the situation",
            "    // pulls the light of knowledge",
            "    // into the black hole of ignorance.",
            "    return unit_vector(hope);",
            "}",
            "GENESIS_OS_KERNEL_PANIC",
            "0xDEADBEEF 0xCAFEBABE",
            "SYSTEM FAILURE: REALITY NOT FOUND",
            "REBOOTING UNIVERSE...",
            "LOADING...",
        ];

        for _ in 0..50 {
            let x = rand::gen_range(0.0, 1024.0);
            let y = rand::gen_range(0.0, 1024.0);
            let size = rand::gen_range(20.0, 60.0);
            let line = text[rand::gen_range(0, text.len())];
            let color = Color::new(
                rand::gen_range(0.2, 0.8),
                rand::gen_range(0.2, 0.8),
                rand::gen_range(0.5, 1.0),
                1.0,
            );
            draw_text(line, x, y, size, color);
        }

        for _ in 0..1000 {
            let x = rand::gen_range(0.0, 1024.0);
            let y = rand::gen_range(0.0, 1024.0);
            draw_circle(x, y, rand::gen_range(0.5, 2.0), WHITE);
        }

        set_default_camera();

        Self {
            material,
            background_texture: render_target.texture,
        }
    }

    pub fn draw(&self, bodies: &[crate::physics::Body]) {
        self.material.set_uniform("u_resolution", (screen_width(), screen_height()));
        self.material.set_uniform("u_body_count", bodies.len().min(8) as i32);

        let default_body = vec3(0.0, 0.0, 0.0);

        // Helper to get body as vec3
        let get_b = |i: usize| -> Vec3 {
            if i < bodies.len() {
                let b = bodies[i];
                vec3(b.pos.x, b.pos.y, b.mass)
            } else {
                default_body
            }
        };

        self.material.set_uniform("u_b0", get_b(0));
        self.material.set_uniform("u_b1", get_b(1));
        self.material.set_uniform("u_b2", get_b(2));
        self.material.set_uniform("u_b3", get_b(3));
        self.material.set_uniform("u_b4", get_b(4));
        self.material.set_uniform("u_b5", get_b(5));
        self.material.set_uniform("u_b6", get_b(6));
        self.material.set_uniform("u_b7", get_b(7));

        gl_use_material(&self.material);
            draw_texture_ex(
                &self.background_texture,
                0.0,
                0.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(screen_width(), screen_height())),
                    ..Default::default()
                },
            );
        gl_use_default_material();

        for (i, body) in bodies.iter().enumerate() {
            if i >= 8 { break; } // Only show 8 if shader limits
            draw_circle(body.pos.x, body.pos.y, body.radius, body.color);
        }
    }
}
