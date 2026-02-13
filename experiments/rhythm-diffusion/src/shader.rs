use macroquad::miniquad::{UniformDesc, UniformType};

pub const VERTEX_SHADER: &str = r#"
#version 100
attribute vec3 position;
attribute vec2 texcoord;

varying vec2 uv;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    uv = texcoord;
}
"#;

pub const FRAGMENT_SHADER: &str = r#"
#version 100
precision highp float;

varying vec2 uv;

uniform sampler2D Texture;
uniform vec2 Resolution;
uniform float Feed;
uniform float Kill;
uniform float Da;
uniform float Db;
uniform float dt;

void main() {
    vec2 pixel = 1.0 / Resolution;

    // Center cell
    vec4 val = texture2D(Texture, uv);
    float u = val.r;
    float v = val.g;

    // Laplacian (using 3x3 kernel with 5-point stencil)
    //   0  1  0
    //   1 -4  1
    //   0  1  0

    // Sample neighbors
    vec2 top = texture2D(Texture, uv + vec2(0.0, -pixel.y)).rg;
    vec2 bottom = texture2D(Texture, uv + vec2(0.0, pixel.y)).rg;
    vec2 left = texture2D(Texture, uv + vec2(-pixel.x, 0.0)).rg;
    vec2 right = texture2D(Texture, uv + vec2(pixel.x, 0.0)).rg;

    vec2 laplacian = top + bottom + left + right - 4.0 * vec2(u, v);

    // Gray-Scott Formula
    // du/dt = Da * lap(u) - u*v*v + f*(1-u)
    // dv/dt = Db * lap(v) + u*v*v - (f+k)*v

    float reaction = u * v * v;

    float du = Da * laplacian.x - reaction + Feed * (1.0 - u);
    float dv = Db * laplacian.y + reaction - (Feed + Kill) * v;

    float u_new = u + du * dt;
    float v_new = v + dv * dt;

    // Clamp to [0, 1]
    u_new = clamp(u_new, 0.0, 1.0);
    v_new = clamp(v_new, 0.0, 1.0);

    gl_FragColor = vec4(u_new, v_new, 0.0, 1.0);
}
"#;

pub fn get_uniforms() -> Vec<UniformDesc> {
    vec![
        UniformDesc::new("Resolution", UniformType::Float2),
        UniformDesc::new("Feed", UniformType::Float1),
        UniformDesc::new("Kill", UniformType::Float1),
        UniformDesc::new("Da", UniformType::Float1),
        UniformDesc::new("Db", UniformType::Float1),
        UniformDesc::new("dt", UniformType::Float1),
    ]
}
