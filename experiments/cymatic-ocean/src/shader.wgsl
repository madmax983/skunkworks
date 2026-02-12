// Compute Shader

struct Params {
    damping: f32,
    freq: f32,
    amp: f32,
    time: f32,
}

@group(0) @binding(0) var u_prev: texture_2d<f32>;
@group(0) @binding(1) var u_curr: texture_2d<f32>;
@group(0) @binding(2) var u_next: texture_storage_2d<r32float, write>;
@group(0) @binding(3) var<uniform> params: Params;

@compute @workgroup_size(16, 16)
fn compute_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(u_curr);
    if (id.x >= size.x || id.y >= size.y) {
        return;
    }
    let pos = vec2<i32>(id.xy);

    let curr = textureLoad(u_curr, pos, 0).r;
    let prev = textureLoad(u_prev, pos, 0).r;

    // Laplacian (Finite Difference)
    let u = textureLoad(u_curr, vec2<i32>(pos.x, pos.y - 1), 0).r;
    let d = textureLoad(u_curr, vec2<i32>(pos.x, pos.y + 1), 0).r;
    let l = textureLoad(u_curr, vec2<i32>(pos.x - 1, pos.y), 0).r;
    let r = textureLoad(u_curr, vec2<i32>(pos.x + 1, pos.y), 0).r;

    // Handle boundary (clamp)
    // Actually textureLoad handles out of bounds by clamping if we configured it?
    // No, textureLoad with integer coords is unsafe or clamps depending on backend/texture mode?
    // Usually manual check is safer or just ignore boundaries (fixed 0).
    // Let's assume 0 at boundaries for "fixed rim".
    // Or just let it reflect naturally if we clamp coords.

    // Wave Equation Discretization
    // u(t+1) = 2u(t) - u(t-1) + c^2 * dt^2 * laplacian
    // With damping: u(t+1) = u(t) + (u(t) - u(t-1)) * damping + laplacian * speed

    let laplacian = (u + d + l + r - 4.0 * curr);
    let damping = params.damping;
    let speed = 0.25; // Speed parameter

    var next = curr + (curr - prev) * damping + laplacian * speed;

    // Audio Driver (Source)
    let center = vec2<f32>(f32(size.x) / 2.0, f32(size.y) / 2.0);
    let dist = distance(vec2<f32>(pos), center);

    // Smooth source area
    if (dist < 10.0) {
        let factor = (10.0 - dist) / 10.0;
        next += sin(params.time * params.freq) * params.amp * 0.5 * factor;
    }

    textureStore(u_next, pos, vec4<f32>(next, 0.0, 0.0, 1.0));
}

// Render Shader

struct Uniforms {
    view_proj: mat4x4<f32>,
}
@group(1) @binding(0) var<uniform> uniforms: Uniforms;
@group(1) @binding(1) var t_height: texture_2d<f32>;
@group(1) @binding(2) var s_height: sampler;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) height: f32,
}

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    let size = textureDimensions(t_height);
    let width = size.x;

    // Map vertex index to grid coordinates
    // We render Points, so 1 vertex = 1 grid point
    let x = f32(in_vertex_index % width);
    let y = f32(in_vertex_index / width);

    let uv = vec2<f32>(x / f32(size.x), y / f32(size.y));

    // Sample height
    let h = textureSampleLevel(t_height, s_height, uv, 0.0).r;

    // Map to world space: -1 to 1 on XZ plane, Y is height
    let world_pos = vec4<f32>(
        (uv.x - 0.5) * 20.0, // Scale X
        h * 2.0,             // Scale Height
        (uv.y - 0.5) * 20.0, // Scale Z
        1.0
    );

    var out: VertexOutput;
    out.clip_position = uniforms.view_proj * world_pos;
    out.uv = uv;
    out.height = h;

    // Make points larger if close?
    // WGPU doesn't support gl_PointSize easily in WGSL without extensions or Points primitive mode
    // We'll rely on pipeline configuration for point size if possible, or just render small points.

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let h = in.height;

    // Color palette: Deep Blue (-1) -> Cyan (0) -> White (+1)
    let deep_blue = vec3<f32>(0.0, 0.0, 0.2);
    let cyan = vec3<f32>(0.0, 0.8, 0.8);
    let white = vec3<f32>(1.0, 1.0, 1.0);

    var color = cyan;
    if (h < 0.0) {
        color = mix(cyan, deep_blue, -h);
    } else {
        color = mix(cyan, white, h);
    }

    return vec4<f32>(color, 1.0);
}
