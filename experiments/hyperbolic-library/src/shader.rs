pub const VERTEX: &str = r#"
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

pub const FRAGMENT: &str = r#"
#version 100
precision highp float;

varying vec2 uv;
uniform vec3 iResolution;
uniform float iTime;
uniform vec2 u_player_pos; // Real, Imag
uniform float u_neighbor_offset;
uniform float u_root_seed;

uniform sampler2D u_font_texture;
uniform float u_grid_cols;
uniform float u_grid_rows;

// --- Complex Math ---
vec2 complex_mul(vec2 a, vec2 b) {
    return vec2(a.x * b.x - a.y * b.y, a.x * b.y + a.y * b.x);
}

vec2 complex_div(vec2 a, vec2 b) {
    float den = dot(b, b);
    return vec2(dot(a, b), a.y * b.x - a.x * b.y) / den;
}

vec2 complex_conj(vec2 a) {
    return vec2(a.x, -a.y);
}

// (z + a) / (1 + conj(a)z)
vec2 mobius_add(vec2 z, vec2 a) {
    vec2 num = z + a;
    vec2 den = vec2(1.0, 0.0) + complex_mul(complex_conj(a), z);
    return complex_div(num, den);
}

// (z - a) / (1 - conj(a)z)
vec2 mobius_sub(vec2 z, vec2 a) {
    vec2 num = z - a;
    vec2 den = vec2(1.0, 0.0) - complex_mul(complex_conj(a), z);
    return complex_div(num, den);
}

// --- Hashing ---
float hash12(vec2 p) {
	vec3 p3  = fract(vec3(p.xyx) * .1031);
    p3 += dot(p3, p3.yzx + 33.33);
    return fract((p3.x + p3.y) * p3.z);
}

float hash_tile(float seed, int neighbor_idx) {
    // Combine seed and neighbor index
    return hash12(vec2(seed, float(neighbor_idx) * 12.345));
}

// --- Tiling ---

vec2 get_neighbor_pos(int k) {
    float angle = float(k) * 1.57079632679; // PI/2
    return vec2(cos(angle), sin(angle)) * u_neighbor_offset;
}

void main() {
    // Map UV to [-1, 1], correcting aspect ratio
    vec2 z = (uv - 0.5) * 2.0;
    z.x *= iResolution.x / iResolution.y;

    float dist_sq = dot(z, z);
    if (dist_sq >= 1.0) {
        // Outside disk
        gl_FragColor = vec4(0.05, 0.0, 0.1, 1.0);
        return;
    }

    vec2 z_curr = mobius_add(z, u_player_pos);

    float current_seed = u_root_seed;
    int depth = 0;

    // Tiling Traversal
    for (int i=0; i<32; i++) {
        int best_k = -1;
        float best_dist = dot(z_curr, z_curr);
        vec2 best_z = z_curr;

        // Check 4 neighbors
        for (int k=0; k<4; k++) {
            vec2 nk = get_neighbor_pos(k);
            vec2 z_prime = mobius_sub(z_curr, nk);
            float d = dot(z_prime, z_prime);
            if (d < best_dist - 0.001) {
                best_dist = d;
                best_k = k;
                best_z = z_prime;
            }
        }

        if (best_k != -1) {
            z_curr = best_z;
            current_seed = hash_tile(current_seed, best_k);
            depth++;
        } else {
            // Closest to current center
            break;
        }
    }

    // Determine Character
    float char_seed = hash12(vec2(current_seed, 42.0));
    float num_chars = u_grid_cols * u_grid_rows;
    float char_idx = floor(char_seed * num_chars);

    // Map z_curr to UV [0, 1]
    // z_curr is in [-0.6, 0.6] roughly for {4,5} tiling center
    vec2 local_uv = z_curr * 0.8 + 0.5;

    vec3 color = vec3(0.0);

    if (local_uv.x >= 0.0 && local_uv.x <= 1.0 && local_uv.y >= 0.0 && local_uv.y <= 1.0) {
        // Calculate UV in Atlas
        float col_idx = mod(char_idx, u_grid_cols);
        float row_idx = floor(char_idx / u_grid_cols);

        vec2 atlas_uv = (vec2(col_idx, row_idx) + local_uv) / vec2(u_grid_cols, u_grid_rows);

        // Sample Font
        float alpha = texture2D(u_font_texture, atlas_uv).r;

        // Color
        vec3 text_color = vec3(0.8, 0.9, 1.0); // Cyan/White text
        vec3 bg_color = vec3(0.05, 0.05, 0.1); // Dark Blue background

        // Add some noise/texture to background
        bg_color += vec3(sin(z_curr.x*10.0)*0.02);

        color = mix(bg_color, text_color, alpha);
    } else {
        color = vec3(0.05, 0.05, 0.1);
    }

    // Fog / Depth darkening
    color *= 1.0 - float(depth) * 0.05;

    // Boundary of Poincaré disk
    float disk_dist = length(z);
    color += vec3(0.2, 0.0, 0.4) * smoothstep(0.95, 1.0, disk_dist);

    gl_FragColor = vec4(color, 1.0);
}
"#;
