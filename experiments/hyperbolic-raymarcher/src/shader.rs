pub const VERTEX: &str = r#"
#version 100
attribute vec3 position;
attribute vec2 texcoord;
varying vec2 uv;
void main() {
    gl_Position = vec4(position, 1.0);
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
// MurmurHash3 style mixing
float hash12(vec2 p) {
	vec3 p3  = fract(vec3(p.xyx) * .1031);
    p3 += dot(p3, p3.yzx + 33.33);
    return fract((p3.x + p3.y) * p3.z);
}

float hash_tile(float seed, int neighbor_idx) {
    // Combine seed and neighbor index
    // seed is a float acting as integer ID
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

    // Apply Camera (Inverse View): world = mobius_add(screen, player)
    // Wait, u_player_pos is the "Shift".
    // If player moves "Right", u_player_pos increases.
    // The world should move "Left".
    // So z_world = mobius_sub(z, u_player_pos)?
    // Or mobius_add(z, u_player_pos)?
    // Usually ViewMatrix * WorldPos = ScreenPos.
    // So WorldPos = InvViewMatrix * ScreenPos.
    // If u_player_pos represents "Camera Position in World", then
    // To bring Camera to Origin, we do mobius_sub(World, Camera).
    // So Screen = mobius_sub(World, Camera).
    // Therefore World = mobius_add(Screen, Camera).
    // Correct.

    vec2 z_curr = mobius_add(z, u_player_pos);

    float current_seed = u_root_seed;
    int depth = 0;
    int max_depth = 20;

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
            if (d < best_dist - 0.001) { // Epsilon to avoid noise
                best_dist = d;
                best_k = k;
                best_z = z_prime;
            }
        }

        if (best_k != -1) {
            z_curr = best_z;
            // Update seed for the next tile
            // We need a deterministic way to update seed based on path.
            // S_new = hash(S_old, k).
            // But we must be careful: traversing 0 then 2 (Right then Left) should return to S_old.
            // Our traversal is "Greedy Descent" towards the tile containing the point.
            // The path is unique in a tree.
            // So simpler hash update: S_new = S_old + 1.0 + k? No.

            // Actually, we don't need to "backtrack" here because we are rendering a static point.
            // We are finding "Where is this point in the tree?".
            // The point IS in some tile. We just descend until we find it.
            // So we just accumulate the hash.
            current_seed = hash_tile(current_seed, best_k);
            depth++;
        } else {
            // Closest to current center -> We are in this tile.
            break;
        }
    }

    // Determine Tile Type
    float h = hash12(vec2(current_seed, 42.0));
    bool is_wall = h < 0.3; // 30% walls

    // Determine Color
    vec3 color = vec3(0.0);

    // Local coordinates z_curr are now relative to the tile center.
    // Tile is roughly a square (in {4,5}).
    // Visuals:

    float r = length(z_curr);

    if (is_wall) {
        // Wall color
        color = vec3(0.2, 0.1, 0.1);
        // Add texture
        color += vec3(sin(z_curr.x*20.0)*sin(z_curr.y*20.0)*0.05);

        // Inner glow
        color += vec3(0.5, 0.0, 0.0) * smoothstep(0.4, 0.0, r);
    } else {
        // Floor color
        color = vec3(0.1, 0.1, 0.15);
        // Grid pattern
        float grid = step(0.9, sin(z_curr.x * 10.0)) + step(0.9, sin(z_curr.y * 10.0));
        color += vec3(0.1) * grid;

        // Center marker
        float d_center = length(z_curr);
        color += vec3(0.0, 0.5, 1.0) * smoothstep(0.05, 0.0, d_center);
    }

    // Fog / Depth darkening
    color *= 1.0 - float(depth) * 0.05;

    // Boundary of Poincaré disk
    float disk_dist = length(z);
    color += vec3(0.2, 0.0, 0.4) * smoothstep(0.95, 1.0, disk_dist);

    gl_FragColor = vec4(color, 1.0);
}
"#;
