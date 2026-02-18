@group(0) @binding(0) var density_in: texture_2d<f32>;
@group(0) @binding(1) var density_out: texture_storage_2d<rgba32float, write>;
@group(0) @binding(2) var velocity_in: texture_2d<f32>;
@group(0) @binding(3) var velocity_out: texture_storage_2d<rgba32float, write>;
@group(0) @binding(4) var force_tex: texture_2d<f32>;
@group(0) @binding(5) var<uniform> params: Params;

struct Params {
    dt: f32,
    grid_width: u32,
    grid_height: u32,
    decay: f32,
    viscosity: f32,
}

@compute @workgroup_size(8, 8, 1)
fn update(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = vec2<i32>(i32(params.grid_width), i32(params.grid_height));
    let pos = vec2<i32>(id.xy);

    if (pos.x >= size.x || pos.y >= size.y) { return; }

    // 1. Read current velocity at this cell
    let vel = textureLoad(velocity_in, pos, 0).xy;

    // 2. Advection Source Position
    // Backtrace: where did the fluid come from?
    let back_pos = vec2<f32>(pos) - vel * params.dt;
    let sample_pos = vec2<i32>(round(back_pos));
    let clamped_pos = clamp(sample_pos, vec2<i32>(0), size - vec2<i32>(1));

    // 3. Advect Density and Velocity
    // "What was the density/velocity back there?"
    var advected_density = textureLoad(density_in, clamped_pos, 0);
    var advected_vel = textureLoad(velocity_in, clamped_pos, 0).xy;

    // 4. Inject Forces
    // force_tex: R = Density Injection, G = Vel X, B = Vel Y
    let force = textureLoad(force_tex, pos, 0);

    // Add density
    advected_density = advected_density + vec4<f32>(force.r, 0.0, 0.0, 0.0);

    // Add velocity force (acceleration)
    advected_vel = advected_vel + force.gb;

    // 5. Apply Decay / Damping
    advected_density = advected_density * params.decay;
    advected_vel = advected_vel * 0.995; // Velocity damping

    // 6. Write Output
    textureStore(density_out, pos, advected_density);
    textureStore(velocity_out, pos, vec4<f32>(advected_vel, 0.0, 1.0));
}
