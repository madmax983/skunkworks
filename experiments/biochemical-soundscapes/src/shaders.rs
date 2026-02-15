pub const VERTEX_SHADER: &str = r#"#version 100
attribute vec3 position;
attribute vec2 texcoord;
varying vec2 uv;
void main() {
    gl_Position = vec4(position, 1.0);
    uv = texcoord;
}
"#;

pub const FRAGMENT_SHADER_SIMULATE: &str = r#"#version 100
precision highp float;
varying vec2 uv;
uniform sampler2D tex;
uniform vec2 resolution;
uniform float dt;
uniform float feed;
uniform float kill;
uniform float diff_u;
uniform float diff_v;
uniform vec3 mouse; // x, y, click

void main() {
    vec2 pixel = 1.0 / resolution;

    vec4 current = texture2D(tex, uv);
    float u = current.r;
    float v = current.g;

    // Laplacian (5-point stencil)
    // 0 1 0
    // 1 -4 1
    // 0 1 0

    float sum_u = 0.0;
    float sum_v = 0.0;

    sum_u += texture2D(tex, uv + vec2(pixel.x, 0.0)).r;
    sum_u += texture2D(tex, uv + vec2(-pixel.x, 0.0)).r;
    sum_u += texture2D(tex, uv + vec2(0.0, pixel.y)).r;
    sum_u += texture2D(tex, uv + vec2(0.0, -pixel.y)).r;

    sum_v += texture2D(tex, uv + vec2(pixel.x, 0.0)).g;
    sum_v += texture2D(tex, uv + vec2(-pixel.x, 0.0)).g;
    sum_v += texture2D(tex, uv + vec2(0.0, pixel.y)).g;
    sum_v += texture2D(tex, uv + vec2(0.0, -pixel.y)).g;

    float lap_u = sum_u - 4.0 * u;
    float lap_v = sum_v - 4.0 * v;

    float uvv = u * v * v;

    // Gray-Scott formulas
    float du = diff_u * lap_u - uvv + feed * (1.0 - u);
    float dv = diff_v * lap_v + uvv - (kill + feed) * v;

    float next_u = clamp(u + du * dt, 0.0, 1.0);
    float next_v = clamp(v + dv * dt, 0.0, 1.0);

    // Mouse interaction: add V
    if (mouse.z > 0.0) {
        float dist = distance(uv, mouse.xy);
        // Correct aspect ratio for distance?
        // Assuming square texture or handling elsewhere.
        // For simplicity, just use UV distance.
        if (dist < 0.02) {
            next_v = 0.9;
        }
    }

    gl_FragColor = vec4(next_u, next_v, 0.0, 1.0);
}
"#;

pub const FRAGMENT_SHADER_RENDER: &str = r#"#version 100
precision highp float;
varying vec2 uv;
uniform sampler2D tex;

vec3 hsv2rgb(vec3 c) {
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

void main() {
    vec4 c = texture2D(tex, uv);
    float u = c.r;
    float v = c.g;

    // Visualize V (the pattern forming chemical)
    // Map V to color.
    // V typically ranges 0.0 to 0.4-0.6 in patterns.

    // Simple gradient: Dark Blue -> Cyan -> White
    vec3 col = vec3(0.0);

    // if v is small, it's background (u=1, v=0)

    float t = v * 3.0;
    col = mix(vec3(0.05, 0.05, 0.1), vec3(0.0, 0.8, 0.9), t);
    col = mix(col, vec3(1.0), smoothstep(0.4, 0.8, v));

    // Add some U modulation for depth
    col *= u * 0.5 + 0.5;

    gl_FragColor = vec4(col, 1.0);
}
"#;
