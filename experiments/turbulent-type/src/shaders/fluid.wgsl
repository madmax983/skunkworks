struct Params {
    width: u32,
    height: u32,
    dt: f32,
    dx: f32,
    mouse_x: f32,
    mouse_y: f32,
    mouse_active: u32,
    time: f32,
}

@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var linear_sampler: sampler;

// Texture Bindings
// Note: We will bind different resources to these bindings depending on the pass.
// But all resources must be present in the bind group layout.
// Bind Group 1 Layout:
// 0: Velocity Input (Texture2D)
// 1: Velocity Output (Storage RG16Float)
// 2: Density Input (Texture2D)
// 3: Density Output (Storage R16Float)
// 4: Divergence Output (Storage R16Float)
// 5: Pressure Input (Texture2D)
// 6: Pressure Output (Storage R16Float)
// 7: Divergence Input (Texture2D) - reuse binding?
// 8: Text Input (Texture2D)

@group(1) @binding(0) var velocity_in: texture_2d<f32>;
@group(1) @binding(1) var velocity_out: texture_storage_2d<rg16float, write>;
@group(1) @binding(2) var density_in: texture_2d<f32>;
@group(1) @binding(3) var density_out: texture_storage_2d<r16float, write>;
@group(1) @binding(4) var divergence_out: texture_storage_2d<r16float, write>;
@group(1) @binding(5) var pressure_in: texture_2d<f32>;
@group(1) @binding(6) var pressure_out: texture_storage_2d<r16float, write>;
@group(1) @binding(7) var divergence_in: texture_2d<f32>;
@group(1) @binding(8) var text_in: texture_2d<f32>;

@compute @workgroup_size(16, 16)
fn advect_velocity(@builtin(global_invocation_id) id: vec3<u32>) {
    if (id.x >= params.width || id.y >= params.height) { return; }
    let size = vec2<f32>(f32(params.width), f32(params.height));
    let coords = vec2<i32>(id.xy);
    let pos = vec2<f32>(id.xy) + 0.5;

    let vel = textureLoad(velocity_in, coords, 0).xy;
    let back_pos = pos - params.dt * vel;
    let uv = back_pos / size;

    let new_vel = textureSampleLevel(velocity_in, linear_sampler, uv, 0.0).xy;

    textureStore(velocity_out, coords, vec4<f32>(new_vel * 0.999, 0.0, 1.0));
}

@compute @workgroup_size(16, 16)
fn advect_density(@builtin(global_invocation_id) id: vec3<u32>) {
    if (id.x >= params.width || id.y >= params.height) { return; }
    let size = vec2<f32>(f32(params.width), f32(params.height));
    let coords = vec2<i32>(id.xy);
    let pos = vec2<f32>(id.xy) + 0.5;

    let vel = textureLoad(velocity_in, coords, 0).xy;
    let back_pos = pos - params.dt * vel;
    let uv = back_pos / size;

    let new_rho = textureSampleLevel(density_in, linear_sampler, uv, 0.0).r;

    textureStore(density_out, coords, vec4<f32>(new_rho * 0.995, 0.0, 0.0, 1.0));
}

@compute @workgroup_size(16, 16)
fn divergence(@builtin(global_invocation_id) id: vec3<u32>) {
    if (id.x >= params.width || id.y >= params.height) { return; }
    let coords = vec2<i32>(id.xy);

    let w = textureLoad(velocity_in, coords + vec2<i32>(1, 0), 0).x;
    let e = textureLoad(velocity_in, coords - vec2<i32>(1, 0), 0).x;
    let n = textureLoad(velocity_in, coords + vec2<i32>(0, 1), 0).y;
    let s = textureLoad(velocity_in, coords - vec2<i32>(0, 1), 0).y;

    let div = 0.5 * (w - e + n - s);
    textureStore(divergence_out, coords, vec4<f32>(div, 0.0, 0.0, 1.0));
}

@compute @workgroup_size(16, 16)
fn jacobi(@builtin(global_invocation_id) id: vec3<u32>) {
    if (id.x >= params.width || id.y >= params.height) { return; }
    let coords = vec2<i32>(id.xy);

    let pR = textureLoad(pressure_in, coords + vec2<i32>(1, 0), 0).r;
    let pL = textureLoad(pressure_in, coords - vec2<i32>(1, 0), 0).r;
    let pT = textureLoad(pressure_in, coords + vec2<i32>(0, 1), 0).r;
    let pB = textureLoad(pressure_in, coords - vec2<i32>(0, 1), 0).r;

    let bC = textureLoad(divergence_in, coords, 0).r;

    let pNew = (pL + pR + pB + pT - bC) * 0.25;
    textureStore(pressure_out, coords, vec4<f32>(pNew, 0.0, 0.0, 1.0));
}

@compute @workgroup_size(16, 16)
fn subtract_gradient(@builtin(global_invocation_id) id: vec3<u32>) {
    if (id.x >= params.width || id.y >= params.height) { return; }
    let coords = vec2<i32>(id.xy);

    let pR = textureLoad(pressure_in, coords + vec2<i32>(1, 0), 0).r;
    let pL = textureLoad(pressure_in, coords - vec2<i32>(1, 0), 0).r;
    let pT = textureLoad(pressure_in, coords + vec2<i32>(0, 1), 0).r;
    let pB = textureLoad(pressure_in, coords - vec2<i32>(0, 1), 0).r;

    let vel = textureLoad(velocity_in, coords, 0).xy;
    let new_vel = vel - 0.5 * vec2<f32>(pR - pL, pT - pB);

    textureStore(velocity_out, coords, vec4<f32>(new_vel, 0.0, 1.0));
}

@compute @workgroup_size(16, 16)
fn inject(@builtin(global_invocation_id) id: vec3<u32>) {
    if (id.x >= params.width || id.y >= params.height) { return; }
    let coords = vec2<i32>(id.xy);
    let size = vec2<f32>(f32(params.width), f32(params.height));

    let mx = params.mouse_x * size.x;
    let my = params.mouse_y * size.y;

    var rho = textureLoad(density_in, coords, 0).r;
    var vel = textureLoad(velocity_in, coords, 0).xy;

    let text_val = textureLoad(text_in, coords, 0).r;
    if (text_val > 0.01) {
        rho += text_val * 0.2;
        // Turbulence from text "surface"
        vel.x += 1.0;
        vel.y += (text_val - 0.5) * 2.0;
    }

    let dist_sq = (f32(id.x) - mx)*(f32(id.x) - mx) + (f32(id.y) - my)*(f32(id.y) - my);
    if (params.mouse_active > 0u && dist_sq < 400.0) {
        if (params.mouse_active == 1u) {
            rho += 0.5;
        } else {
             // Right click: push
             vel += vec2<f32>(10.0, 0.0);
        }
    }

    textureStore(density_out, coords, vec4<f32>(rho, 0.0, 0.0, 1.0));
    textureStore(velocity_out, coords, vec4<f32>(vel, 0.0, 1.0));
}
