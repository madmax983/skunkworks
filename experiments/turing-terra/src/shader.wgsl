struct Uniforms {
    feed: f32,
    kill: f32,
    dt: f32,
    diff_u: f32,
    diff_v: f32,
    padding1: f32,
    padding2: f32,
    padding3: f32,
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

    // Laplacian convolution (3x3)
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

    // Gray-Scott Reaction Diffusion Equations
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
    // Using the 3-vertex huge triangle trick to cover the screen
    let u = f32((in_vertex_index << 1u) & 2u);
    let v = f32(in_vertex_index & 2u);
    out.tex_coords = vec2<f32>(u * 0.5, 1.0 - v * 0.5);
    out.clip_position = vec4<f32>(u - 1.0, v - 1.0, 0.0, 1.0);
    return out;
}

// Fragment Shader: Biome Mapping
@group(0) @binding(1) var display_tex: texture_2d<f32>;
@group(0) @binding(2) var display_sampler: sampler;

// Color Palette
// Deep Water: 0x1a2d57
// Water:      0x2b578c
// Sand:       0xe3d996
// Grass:      0x4a7c36
// Forest:     0x2d4c21
// Rock:       0x706c6c
// Snow:       0xfafafa

fn hex(r: u32, g: u32, b: u32) -> vec3<f32> {
    return vec3<f32>(f32(r)/255.0, f32(g)/255.0, f32(b)/255.0);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let state = textureSample(display_tex, display_sampler, in.tex_coords);
    let u = state.x;
    let v = state.y;

    // Use 'v' (activator) as elevation/density
    // Typically v ranges 0.0 to 0.4 or so in patterns.
    // We normalize it a bit for visualization.
    let val = clamp(v * 3.5, 0.0, 1.0);

    let c_deep_water = hex(26u, 45u, 87u);
    let c_water      = hex(43u, 87u, 140u);
    let c_sand       = hex(227u, 217u, 150u);
    let c_grass      = hex(74u, 124u, 54u);
    let c_forest     = hex(45u, 76u, 33u);
    let c_rock       = hex(112u, 108u, 108u);
    let c_snow       = hex(250u, 250u, 250u);

    // Thresholds
    // 0.0 - 0.2: Deep Water -> Water
    // 0.2 - 0.25: Water -> Sand
    // 0.25 - 0.4: Sand -> Grass
    // 0.4 - 0.6: Grass -> Forest
    // 0.6 - 0.8: Forest -> Rock
    // 0.8 - 1.0: Rock -> Snow

    var color = c_deep_water;

    if (val < 0.2) {
        color = mix(c_deep_water, c_water, smoothstep(0.0, 0.2, val));
    } else if (val < 0.25) {
        color = mix(c_water, c_sand, smoothstep(0.2, 0.25, val));
    } else if (val < 0.4) {
        color = mix(c_sand, c_grass, smoothstep(0.25, 0.4, val));
    } else if (val < 0.6) {
        color = mix(c_grass, c_forest, smoothstep(0.4, 0.6, val));
    } else if (val < 0.8) {
        color = mix(c_forest, c_rock, smoothstep(0.6, 0.8, val));
    } else {
        color = mix(c_rock, c_snow, smoothstep(0.8, 1.0, val));
    }

    return vec4<f32>(color, 1.0);
}
