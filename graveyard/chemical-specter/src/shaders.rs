pub const VERTEX_SHADER: &str = r#"
#version 100
attribute vec3 position;
attribute vec2 texcoord;
varying vec2 uv;
uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1.0);
    uv = texcoord;
}
"#;

pub const FRAGMENT_SHADER_SIM: &str = r#"
#version 100
precision highp float;
varying vec2 uv;
uniform sampler2D Texture;
uniform float feed;
uniform float kill;
uniform float beat;
uniform vec2 resolution;

void main() {
    vec2 pixel = 1.0 / resolution;

    vec4 center = texture2D(Texture, uv);

    // Laplacian
    // Kernel:
    // 0.05 0.2 0.05
    // 0.2 -1.0 0.2
    // 0.05 0.2 0.05

    vec4 sum = vec4(0.0);
    sum += texture2D(Texture, uv + vec2(-pixel.x, -pixel.y)) * 0.05;
    sum += texture2D(Texture, uv + vec2(0.0, -pixel.y)) * 0.2;
    sum += texture2D(Texture, uv + vec2(pixel.x, -pixel.y)) * 0.05;

    sum += texture2D(Texture, uv + vec2(-pixel.x, 0.0)) * 0.2;
    sum += center * -1.0;
    sum += texture2D(Texture, uv + vec2(pixel.x, 0.0)) * 0.2;

    sum += texture2D(Texture, uv + vec2(-pixel.x, pixel.y)) * 0.05;
    sum += texture2D(Texture, uv + vec2(0.0, pixel.y)) * 0.2;
    sum += texture2D(Texture, uv + vec2(pixel.x, pixel.y)) * 0.05;

    float u = center.r;
    float v = center.g;

    // Modulate params with beat
    // Beat increases feed slightly (more chaotic growth)
    // Beat decreases kill slightly (explosions)
    float f = feed + beat * 0.01;
    float k = kill - beat * 0.005;

    // Gray-Scott Equations
    // du/dt = Du * lap(u) - uv^2 + f(1-u)
    // dv/dt = Dv * lap(v) + uv^2 - (k+f)v
    // Using Du=1.0, Dv=0.5

    float du = 1.0 * sum.r - u * v * v + f * (1.0 - u);
    float dv = 0.5 * sum.g + u * v * v - (k + f) * v;

    // Integration step (dt = 1.0)
    float next_u = u + du;
    float next_v = v + dv;

    gl_FragColor = vec4(clamp(next_u, 0.0, 1.0), clamp(next_v, 0.0, 1.0), 0.0, 1.0);
}
"#;

pub const FRAGMENT_SHADER_RENDER: &str = r#"
#version 100
precision highp float;
varying vec2 uv;
uniform sampler2D Texture;

// Palette function
vec3 palette(float t) {
    // Rainbow palette based on t
    vec3 a = vec3(0.5, 0.5, 0.5);
    vec3 b = vec3(0.5, 0.5, 0.5);
    vec3 c = vec3(1.0, 1.0, 1.0);
    vec3 d = vec3(0.0, 0.33, 0.67);
    return a + b * cos(6.28318 * (c * t + d));
}

void main() {
    vec4 data = texture2D(Texture, uv);
    float u = data.r;
    float v = data.g;

    // Color mapping
    // Usually we visualize V (the "activator" or "pattern")
    // Or u - v

    float val = v * 3.0; // Boost contrast
    // Or mix
    float t = smoothstep(0.2, 0.6, v);

    // Use palette
    vec3 col = palette(t + 0.5); // Offset for better colors

    // If empty (u=1, v=0), make it dark
    col *= smoothstep(0.05, 0.1, v);

    gl_FragColor = vec4(col, 1.0);
}
"#;
