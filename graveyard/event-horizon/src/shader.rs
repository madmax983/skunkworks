pub const VERTEX_SHADER: &str = r#"
#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;

varying lowp vec2 uv;
varying lowp vec4 color;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1.0);
    uv = texcoord;
    color = color0;
}
"#;

pub const FRAGMENT_SHADER: &str = r#"
#version 100
precision highp float;

varying vec2 uv;
varying vec4 color;

uniform sampler2D Texture;
uniform vec3 bodies[32]; // x, y, mass (normalized coords 0..1, mass 0..1)
uniform int body_count;
uniform float aspect_ratio;

void main() {
    vec2 pos = uv;
    pos.x *= aspect_ratio;

    vec2 total_displacement = vec2(0.0);

    for (int i = 0; i < 32; i++) {
        if (i >= body_count) break;

        vec2 body_pos = bodies[i].xy;
        body_pos.x *= aspect_ratio;

        vec2 dir = pos - body_pos;
        float dist_sq = dot(dir, dir);
        float dist = sqrt(dist_sq);

        // Schwarzschild radius-ish logic
        // Deflection is proportional to Mass / Dist
        // But we want to avoid division by zero.

        float strength = 0.02 * bodies[i].z; // Scale factor

        // Einstein Ring approximation
        // shift = (pos - body_pos) * (strength / dist_sq)

        if (dist > 0.001) {
             total_displacement -= (dir / dist) * (strength / dist);
        }
    }

    vec2 final_uv = uv + total_displacement;

    // Event Horizon (black out if too close to center)
    // Actually, let's just let the texture distortion do the work.
    // But real black holes have a shadow.

    float shadow = 1.0;
    for (int i = 0; i < 32; i++) {
        if (i >= body_count) break;
        vec2 body_pos = bodies[i].xy;
        body_pos.x *= aspect_ratio;
        vec2 dir = pos - body_pos;
        float dist = length(dir);
        // If inside event horizon (approx), return black
        if (dist < bodies[i].z * 0.05) {
            shadow = 0.0;
        }
    }

    vec4 tex_color = texture2D(Texture, final_uv);
    gl_FragColor = tex_color * color * shadow;
}
"#;
