struct FluidUniforms {
    dt: f32,
    width: u32,
    height: u32,
    dx: f32,
}

@group(0) @binding(0) var<uniform> params: FluidUniforms;

// Helper to clamp coordinates
fn clamp_coords(c: vec2<i32>) -> vec2<i32> {
    return clamp(c, vec2<i32>(0, 0), vec2<i32>(i32(params.width) - 1, i32(params.height) - 1));
}

// Bindings for Advect
@group(1) @binding(0) var vel_in: texture_2d<f32>;
@group(1) @binding(1) var vel_out: texture_storage_2d<rg32float, write>;
@group(1) @binding(2) var obstacles: texture_2d<f32>; // R channel: 1.0 = solid
@group(1) @binding(3) var linear_sampler: sampler;

@compute @workgroup_size(16, 16)
fn advect(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let x = global_id.x;
    let y = global_id.y;
    if (x >= params.width || y >= params.height) { return; }
    let coords = vec2<i32>(i32(x), i32(y));

    // Check obstacle
    let obs = textureLoad(obstacles, coords, 0).r;
    if (obs > 0.5) {
        textureStore(vel_out, coords, vec4<f32>(0.0));
        return;
    }

    // Follow velocity back in time
    let velocity = textureLoad(vel_in, coords, 0).xy;
    let back_pos = vec2<f32>(f32(x), f32(y)) - params.dt * velocity;

    // Sample interpolated velocity at back_pos
    let uv = (back_pos + 0.5) / vec2<f32>(f32(params.width), f32(params.height));
    let new_vel = textureSampleLevel(vel_in, linear_sampler, uv, 0.0).xy;

    textureStore(vel_out, coords, vec4<f32>(new_vel, 0.0, 0.0));
}

// Bindings for Divergence
@group(1) @binding(0) var div_vel_in: texture_2d<f32>;
@group(1) @binding(1) var divergence_out: texture_storage_2d<r32float, write>;
@group(1) @binding(2) var div_obstacles: texture_2d<f32>;

@compute @workgroup_size(16, 16)
fn divergence(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let x = global_id.x;
    let y = global_id.y;
    if (x >= params.width || y >= params.height) { return; }
    let coords = vec2<i32>(i32(x), i32(y));

    let w = textureLoad(div_vel_in, clamp_coords(coords + vec2<i32>(-1, 0)), 0).xy;
    let e = textureLoad(div_vel_in, clamp_coords(coords + vec2<i32>( 1, 0)), 0).xy;
    let s = textureLoad(div_vel_in, clamp_coords(coords + vec2<i32>( 0,-1)), 0).xy;
    let n = textureLoad(div_vel_in, clamp_coords(coords + vec2<i32>( 0, 1)), 0).xy;

    let div = 0.5 * (e.x - w.x + n.y - s.y);
    textureStore(divergence_out, coords, vec4<f32>(div, 0.0, 0.0, 0.0));
}

// Bindings for Pressure (Jacobi)
@group(1) @binding(0) var pressure_in: texture_2d<f32>;
@group(1) @binding(1) var pressure_out: texture_storage_2d<r32float, write>;
@group(1) @binding(2) var divergence_in: texture_2d<f32>;
@group(1) @binding(3) var press_obstacles: texture_2d<f32>;

@compute @workgroup_size(16, 16)
fn pressure(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let x = global_id.x;
    let y = global_id.y;
    if (x >= params.width || y >= params.height) { return; }
    let coords = vec2<i32>(i32(x), i32(y));

    let obs = textureLoad(press_obstacles, coords, 0).r;
    if (obs > 0.5) {
        textureStore(pressure_out, coords, vec4<f32>(0.0));
        return;
    }

    let pC = textureLoad(pressure_in, coords, 0).r;
    let pW = textureLoad(pressure_in, clamp_coords(coords + vec2<i32>(-1, 0)), 0).r;
    let pE = textureLoad(pressure_in, clamp_coords(coords + vec2<i32>( 1, 0)), 0).r;
    let pS = textureLoad(pressure_in, clamp_coords(coords + vec2<i32>( 0,-1)), 0).r;
    let pN = textureLoad(pressure_in, clamp_coords(coords + vec2<i32>( 0, 1)), 0).r;

    let div = textureLoad(divergence_in, coords, 0).r;
    let new_p = (pW + pE + pS + pN - div) * 0.25;

    textureStore(pressure_out, coords, vec4<f32>(new_p, 0.0, 0.0, 0.0));
}

// Bindings for Subtract Gradient
@group(1) @binding(0) var sub_vel_in: texture_2d<f32>;
@group(1) @binding(1) var sub_pressure: texture_2d<f32>;
@group(1) @binding(2) var sub_vel_out: texture_storage_2d<rg32float, write>;
@group(1) @binding(3) var sub_obstacles: texture_2d<f32>;

@compute @workgroup_size(16, 16)
fn subtract(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let x = global_id.x;
    let y = global_id.y;
    if (x >= params.width || y >= params.height) { return; }
    let coords = vec2<i32>(i32(x), i32(y));

    let obs = textureLoad(sub_obstacles, coords, 0).r;
    if (obs > 0.5) {
        textureStore(sub_vel_out, coords, vec4<f32>(0.0));
        return;
    }

    let pW = textureLoad(sub_pressure, clamp_coords(coords + vec2<i32>(-1, 0)), 0).r;
    let pE = textureLoad(sub_pressure, clamp_coords(coords + vec2<i32>( 1, 0)), 0).r;
    let pS = textureLoad(sub_pressure, clamp_coords(coords + vec2<i32>( 0,-1)), 0).r;
    let pN = textureLoad(sub_pressure, clamp_coords(coords + vec2<i32>( 0, 1)), 0).r;

    let grad = vec2<f32>(pE - pW, pN - pS) * 0.5;
    let old_vel = textureLoad(sub_vel_in, coords, 0).xy;
    let new_vel = old_vel - grad;

    textureStore(sub_vel_out, coords, vec4<f32>(new_vel, 0.0, 0.0));
}

struct Impulse {
    pos: vec2<f32>,
    radius: f32,
    val: vec2<f32>,
}

@group(2) @binding(0) var<uniform> impulse: Impulse;

@group(1) @binding(0) var imp_vel_in: texture_2d<f32>;
@group(1) @binding(1) var imp_vel_out: texture_storage_2d<rg32float, write>;

@compute @workgroup_size(16, 16)
fn add_impulse(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let x = global_id.x;
    let y = global_id.y;
    if (x >= params.width || y >= params.height) { return; }

    let coords = vec2<i32>(i32(x), i32(y));
    let uv = vec2<f32>(f32(x) / f32(params.width), f32(y) / f32(params.height));

    // Simple distance check
    let dist = distance(uv, impulse.pos);

    var vel = textureLoad(imp_vel_in, coords, 0).xy;

    if (dist < impulse.radius) {
        let falloff = 1.0 - smoothstep(0.0, impulse.radius, dist);
        vel += impulse.val * falloff;
    }

    textureStore(imp_vel_out, coords, vec4<f32>(vel, 0.0, 0.0));
}
