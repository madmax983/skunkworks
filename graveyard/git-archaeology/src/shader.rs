pub const VERTEX_SHADER: &'static str = r#"
#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;

varying float v_light;
varying vec2 v_texcoord;
varying vec4 v_color;

uniform mat4 Model;
uniform mat4 View;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * View * Model * vec4(position, 1);
    v_texcoord = texcoord;
    v_color = color0;
}
"#;

pub const FRAGMENT_SHADER: &'static str = r#"
#version 100
precision mediump float;

varying vec2 v_texcoord;
varying vec4 v_color;

uniform sampler2D Texture;
uniform float time;
uniform float decay;

float rand(vec2 co) {
    return fract(sin(dot(co.xy, vec2(12.9898, 78.233))) * 43758.5453);
}

void main() {
    vec2 p = v_texcoord;

    // Glitch displacement (Horizontal bands)
    if (decay > 0.0) {
        float noise = rand(vec2(time * 10.0, floor(p.y * 20.0)));
        if (noise < decay * 0.3) {
            p.x += (rand(vec2(time, p.y)) - 0.5) * 0.1 * decay;
        }
    }

    vec4 tex = texture2D(Texture, p);

    // Color corruption (Rusting / Browning)
    if (decay > 0.1) {
        float gray = dot(tex.rgb, vec3(0.299, 0.587, 0.114));
        vec3 rust = vec3(0.8, 0.4, 0.1); // Rusty orange
        // Mix towards rust based on decay
        tex.rgb = mix(tex.rgb, rust * gray, decay * 0.6);
    }

    // Static Noise
    if (decay > 0.0) {
        float static_noise = rand(p + time);
        tex.rgb += (static_noise - 0.5) * decay * 0.2;
    }

    gl_FragColor = tex * v_color;
}
"#;
