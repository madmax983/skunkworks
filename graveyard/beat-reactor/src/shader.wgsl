struct Uniforms {
    feed: f32,
    kill: f32,
    dt: f32,
    diff_u: f32,
    diff_v: f32,
    time: f32,       // Added for pulsing colors
    padding1: f32,
    padding2: f32,
};

@group(0) @binding(0) var<uniform> params: Uniforms;

// We use two textures: one for reading the current state, one for writing the next state.
// Format: R = u, G = v.
@group(1) @binding(0) var current_tex: texture_2d<f32>;
@group(1) @binding(1) var next_tex: texture_storage_2d<rgba32float, write>;

fn get_chem(x: i32, y: i32) -> vec2<f32> {
    let dims = textureDimensions(current_tex);
    // Wrap around coordinates (Toroidal)
    let w = i32(dims.x);
    let h = i32(dims.y);
    let wx = (x + w) % w;
    let wy = (y + h) % h;

    let c = textureLoad(current_tex, vec2<i32>(wx, wy), 0);
    return c.xy;
}

@compute @workgroup_size(16, 16)
fn update(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let x = i32(global_id.x);
    let y = i32(global_id.y);
    let dims = textureDimensions(current_tex);

    if (x >= i32(dims.x) || y >= i32(dims.y)) {
        return;
    }

    let center = get_chem(x, y);
    let u = center.x;
    let v = center.y;

    // Laplacian convolution
    // Weights:
    // 0.05  0.20  0.05
    // 0.20 -1.00  0.20
    // 0.05  0.20  0.05

    var laplacian = vec2<f32>(0.0, 0.0);

    laplacian += get_chem(x - 1, y - 1) * 0.05;
    laplacian += get_chem(x    , y - 1) * 0.20;
    laplacian += get_chem(x + 1, y - 1) * 0.05;

    laplacian += get_chem(x - 1, y    ) * 0.20;
    laplacian += get_chem(x    , y    ) * -1.0;
    laplacian += get_chem(x + 1, y    ) * 0.20;

    laplacian += get_chem(x - 1, y + 1) * 0.05;
    laplacian += get_chem(x    , y + 1) * 0.20;
    laplacian += get_chem(x + 1, y + 1) * 0.05;

    // Gray-Scott Reaction Diffusion
    // du/dt = diff_u * lapL_u - uv^2 + f(1 - u)
    // dv/dt = diff_v * lapL_v + uv^2 - (k + f)v

    let reaction = u * v * v;
    let du = params.diff_u * laplacian.x - reaction + params.feed * (1.0 - u);
    let dv = params.diff_v * laplacian.y + reaction - (params.kill + params.feed) * v;

    let next_u = saturate(u + du * params.dt);
    let next_v = saturate(v + dv * params.dt);

    textureStore(next_tex, vec2<i32>(x, y), vec4<f32>(next_u, next_v, 0.0, 1.0));
}

// Vertex Shader for Visualization
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    // Using the 3-vertex huge triangle trick
    // Vertices: (-1, -1), (3, -1), (-1, 3)
    let u = f32((in_vertex_index << 1u) & 2u);
    let v = f32(in_vertex_index & 2u);
    out.tex_coords = vec2<f32>(u * 0.5, 1.0 - v * 0.5); // Flip Y for texture
    out.clip_position = vec4<f32>(u - 1.0, v - 1.0, 0.0, 1.0);

    return out;
}

// Fragment Shader
@group(0) @binding(1) var display_tex: texture_2d<f32>;
@group(0) @binding(2) var display_sampler: sampler;

fn hsv2rgb(c: vec3<f32>) -> vec3<f32> {
    let K = vec4<f32>(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    let p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, vec3<f32>(0.0), vec3<f32>(1.0)), c.y);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let state = textureSample(display_tex, display_sampler, in.tex_coords);
    let u = state.x;
    let v = state.y;

    // Psychedelic mapping
    // Value = concentration of V
    let val = smoothstep(0.1, 0.7, v);

    // Shift Hue based on Time and Position
    let hue_shift = params.time * 0.1 + (in.tex_coords.x + in.tex_coords.y) * 0.2;

    // Base Color from V
    // If V is high -> Bright
    // If V is low -> Dark

    // Palette:
    // Low V: Deep Purple / Black
    // Mid V: Blue / Cyan
    // High V: White / Yellow

    let base_hue = 0.6 + val * 0.4; // Blue to Red
    let sat = 1.0 - val * 0.5;
    let bri = val * 2.0;

    // let rgb = hsv2rgb(vec3<f32>(base_hue + hue_shift, sat, bri));

    // Simpler mapping for punchy contrast
    let col1 = vec3<f32>(0.1, 0.0, 0.2); // Background
    let col2 = vec3<f32>(0.0, 0.5, 1.0); // Mid
    let col3 = vec3<f32>(1.0, 0.9, 0.2); // High

    var color = mix(col1, col2, smoothstep(0.0, 0.3, v));
    color = mix(color, col3, smoothstep(0.3, 0.6, v));

    return vec4<f32>(color, 1.0);
}
