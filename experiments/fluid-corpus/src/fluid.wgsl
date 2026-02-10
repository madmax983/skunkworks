struct FluidUniforms {
    dt: f32,
    grid_scale: f32,
    width: f32,
    height: f32,
    viscosity: f32,
    mouse_pos: vec2<f32>,
    mouse_active: f32,
    mouse_vel: vec2<f32>,
    color_shift: f32,
};

@group(0) @binding(0) var<uniform> params: FluidUniforms;
@group(0) @binding(1) var my_sampler: sampler;

@group(1) @binding(0) var input_tex_a: texture_2d<f32>;
@group(1) @binding(1) var input_tex_b: texture_2d<f32>;
@group(1) @binding(2) var output_tex: texture_storage_2d<rgba16float, write>;

fn sample_linear(tex: texture_2d<f32>, uv: vec2<f32>) -> vec4<f32> {
    return textureSampleLevel(tex, my_sampler, uv, 0.0);
}

@compute @workgroup_size(16, 16)
fn advect(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let coords = vec2<i32>(global_id.xy);
    let dims = vec2<f32>(params.width, params.height);

    if (f32(coords.x) >= dims.x || f32(coords.y) >= dims.y) {
        return;
    }

    // Input A: Velocity Field
    // Input B: Quantity Field (Density or Velocity)

    let pos = vec2<f32>(coords);
    let uv = (pos + 0.5) / dims;

    let velocity = textureLoad(input_tex_a, coords, 0).xy;

    // Backtrace
    // We scale velocity to be in UV space for sampling?
    // Velocity is in pixels/sec? Then * dt.
    let back_pos_px = pos - velocity * params.dt * 60.0; // Tuning factor
    let back_uv = (back_pos_px + 0.5) / dims;

    let quantity = sample_linear(input_tex_b, back_uv);

    // Dissipation (mostly for density, but velocity damping helps stability)
    let decay = 0.998;
    textureStore(output_tex, coords, quantity * decay);
}

@compute @workgroup_size(16, 16)
fn divergence(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let coords = vec2<i32>(global_id.xy);
    let dims = vec2<f32>(params.width, params.height);

    if (f32(coords.x) >= dims.x || f32(coords.y) >= dims.y) {
        return;
    }

    // Input A: Velocity

    let L = textureLoad(input_tex_a, coords - vec2<i32>(1, 0), 0).x;
    let R = textureLoad(input_tex_a, coords + vec2<i32>(1, 0), 0).x;
    let B = textureLoad(input_tex_a, coords - vec2<i32>(0, 1), 0).y;
    let T = textureLoad(input_tex_a, coords + vec2<i32>(0, 1), 0).y;

    let div = 0.5 * (R - L + T - B);

    textureStore(output_tex, coords, vec4<f32>(div, 0.0, 0.0, 1.0));
}

@compute @workgroup_size(16, 16)
fn jacobi(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let coords = vec2<i32>(global_id.xy);
    let dims = vec2<f32>(params.width, params.height);

    if (f32(coords.x) >= dims.x || f32(coords.y) >= dims.y) {
        return;
    }

    // Input A: Pressure
    // Input B: Divergence

    let L = textureLoad(input_tex_a, coords - vec2<i32>(1, 0), 0).x;
    let R = textureLoad(input_tex_a, coords + vec2<i32>(1, 0), 0).x;
    let B = textureLoad(input_tex_a, coords - vec2<i32>(0, 1), 0).x;
    let T = textureLoad(input_tex_a, coords + vec2<i32>(0, 1), 0).x;

    let bC = textureLoad(input_tex_b, coords, 0).x;

    let new_p = (L + R + B + T - bC) * 0.25;

    textureStore(output_tex, coords, vec4<f32>(new_p, 0.0, 0.0, 1.0));
}

@compute @workgroup_size(16, 16)
fn subtract_gradient(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let coords = vec2<i32>(global_id.xy);
    let dims = vec2<f32>(params.width, params.height);

    if (f32(coords.x) >= dims.x || f32(coords.y) >= dims.y) {
        return;
    }

    // Input A: Velocity
    // Input B: Pressure

    let L = textureLoad(input_tex_b, coords - vec2<i32>(1, 0), 0).x;
    let R = textureLoad(input_tex_b, coords + vec2<i32>(1, 0), 0).x;
    let B = textureLoad(input_tex_b, coords - vec2<i32>(0, 1), 0).x;
    let T = textureLoad(input_tex_b, coords + vec2<i32>(0, 1), 0).x;

    let old_v = textureLoad(input_tex_a, coords, 0).xy;

    let grad = vec2<f32>(R - L, T - B) * 0.5;
    let new_v = old_v - grad;

    textureStore(output_tex, coords, vec4<f32>(new_v, 0.0, 1.0));
}

@compute @workgroup_size(16, 16)
fn inject_velocity(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let coords = vec2<i32>(global_id.xy);
    let dims = vec2<f32>(params.width, params.height);

    if (f32(coords.x) >= dims.x || f32(coords.y) >= dims.y) {
        return;
    }

    // Input A: Current Velocity

    var vel = textureLoad(input_tex_a, coords, 0).xy;

    if (params.mouse_active > 0.5) {
        let pos = vec2<f32>(coords);
        let dist = distance(pos, params.mouse_pos);
        let radius = 20.0;

        if (dist < radius) {
            let strength = (1.0 - (dist / radius));
            vel = vel + params.mouse_vel * strength * 5.0;
        }
    }

    textureStore(output_tex, coords, vec4<f32>(vel, 0.0, 0.0));
}

@compute @workgroup_size(16, 16)
fn inject_density(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let coords = vec2<i32>(global_id.xy);
    let dims = vec2<f32>(params.width, params.height);

    if (f32(coords.x) >= dims.x || f32(coords.y) >= dims.y) {
        return;
    }

    // Input A: Current Density
    // Input B: Source Texture (Glyphs)

    var density = textureLoad(input_tex_a, coords, 0).r;
    let source = textureLoad(input_tex_b, coords, 0).r; // Assuming source is R8 or similar

    // Add source
    density = density + source * 2.0; // Boost visibility

    // Clamp
    density = min(density, 2.0);

    textureStore(output_tex, coords, vec4<f32>(density, 0.0, 0.0, 1.0));
}

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(i32(in_vertex_index) & 1);
    let y = f32(i32(in_vertex_index) >> 1);
    out.uv = vec2<f32>(x * 2.0, y * 2.0);
    out.position = vec4<f32>(x * 4.0 - 1.0, 1.0 - y * 4.0, 0.0, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let density = sample_linear(input_tex_a, in.uv).r;
    let velocity = sample_linear(input_tex_b, in.uv).xy;

    // Palette: Deep Ocean
    let color_bg = vec3<f32>(0.05, 0.05, 0.1);
    let color_mid = vec3<f32>(0.0, 0.4, 0.6);
    let color_high = vec3<f32>(0.6, 0.9, 1.0);

    var col = mix(color_bg, color_mid, density);
    if (density > 0.8) {
        col = mix(color_mid, color_high, (density - 0.8) * 5.0);
    }

    return vec4<f32>(col, 1.0);
}
