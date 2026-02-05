// Compute Shader
@group(0) @binding(0) var current_tex: texture_storage_2d<r32float, read>;
@group(0) @binding(1) var prev_tex: texture_storage_2d<r32float, read>;
@group(0) @binding(2) var next_tex: texture_storage_2d<r32float, write>;

// Uniforms? Maybe hardcode constants for now or push constants.
// c^2 * dt^2 / dx^2 = 0.5 (stable if <= 0.5)
const C2: f32 = 0.5;
const DAMPING: f32 = 0.995;

@compute @workgroup_size(16, 16)
fn compute_main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let x = i32(global_id.x);
    let y = i32(global_id.y);
    let coords = vec2<i32>(x, y);

    let dim = textureDimensions(current_tex);
    if (global_id.x >= dim.x || global_id.y >= dim.y) {
        return;
    }

    // Boundary conditions: Fixed (0.0) or Reflective?
    // Fixed is easier: if on edge, u = 0.
    if (x == 0 || y == 0 || x == i32(dim.x) - 1 || y == i32(dim.y) - 1) {
        textureStore(next_tex, coords, vec4<f32>(0.0, 0.0, 0.0, 0.0));
        return;
    }

    let u_curr = textureLoad(current_tex, coords).r;
    let u_prev = textureLoad(prev_tex, coords).r;

    let u_up = textureLoad(current_tex, vec2<i32>(x, y - 1)).r;
    let u_down = textureLoad(current_tex, vec2<i32>(x, y + 1)).r;
    let u_left = textureLoad(current_tex, vec2<i32>(x - 1, y)).r;
    let u_right = textureLoad(current_tex, vec2<i32>(x + 1, y)).r;

    let laplacian = u_up + u_down + u_left + u_right - 4.0 * u_curr;

    // Verlet integration
    var u_next = 2.0 * u_curr - u_prev + C2 * laplacian;
    u_next = u_next * DAMPING;

    // Mouse interaction (handled via checking a uniform or just writing to texture from CPU?)
    // Better: CPU writes to "current_tex" or "prev_tex" before dispatch?
    // Or CPU updates a "Input" buffer/texture?
    // For now, let's assume CPU writes to `current_tex` directly via `queue.write_texture`?
    // But `current_tex` is bound as `read` here.
    // If I use `read_write`, I can't bind it as `read` in other pass?
    // Actually, `queue.write_texture` works on usage COPY_DST.

    textureStore(next_tex, coords, vec4<f32>(u_next, 0.0, 0.0, 0.0));
}

// Vertex Shader
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    // Full screen triangle
    let x = f32(i32(in_vertex_index) - 1);
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1);
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    out.tex_coords = vec2<f32>(x * 0.5 + 0.5, 0.5 - y * 0.5);
    return out;
}

// Fragment Shader
@group(0) @binding(0) var t_diffuse: texture_2d<f32>; // The "Current" texture visualized
@group(0) @binding(1) var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let val = textureSample(t_diffuse, s_diffuse, in.tex_coords).r;

    // Ocean palette
    // Deep blue: 0.0, 0.1, 0.3
    // High blue: 0.0, 0.5, 1.0
    // Highlight: 0.8, 0.9, 1.0

    let base_color = vec3<f32>(0.0, 0.1, 0.3);
    let high_color = vec3<f32>(0.0, 0.6, 1.0);
    let foam_color = vec3<f32>(0.8, 0.9, 1.0);

    var color = mix(base_color, high_color, val * 5.0 + 0.5);
    if (val > 0.1) {
        color = mix(color, foam_color, (val - 0.1) * 2.0);
    }

    return vec4<f32>(color, 1.0);
}
