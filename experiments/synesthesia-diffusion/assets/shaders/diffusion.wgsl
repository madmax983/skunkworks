@group(0) @binding(0) var input_texture: texture_2d<f32>;
@group(0) @binding(1) var output_texture: texture_storage_2d<rgba32float, write>;

struct Params {
    feed: f32,
    kill: f32,
    du: f32,
    dv: f32,
    dt: f32,
    mouse_x: f32,
    mouse_y: f32,
    mouse_pressed: f32,
}

@group(0) @binding(2) var<uniform> params: Params;

@compute @workgroup_size(8, 8, 1)
fn update(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));
    let dims = textureDimensions(input_texture);

    if (invocation_id.x >= dims.x || invocation_id.y >= dims.y) {
        return;
    }

    // Read center cell
    let center = textureLoad(input_texture, location, 0).xy;
    let u = center.x;
    let v = center.y;

    // Laplacian convolution
    var laplacian = vec2<f32>(0.0, 0.0);

    let offsets = array<vec2<i32>, 9>(
        vec2<i32>(-1, -1), vec2<i32>(0, -1), vec2<i32>(1, -1),
        vec2<i32>(-1,  0), vec2<i32>(0,  0), vec2<i32>(1,  0),
        vec2<i32>(-1,  1), vec2<i32>(0,  1), vec2<i32>(1,  1)
    );

    let weights = array<f32, 9>(
        0.05, 0.2, 0.05,
        0.2, -1.0, 0.2,
        0.05, 0.2, 0.05
    );

    for (var i = 0; i < 9; i++) {
        let neighbor_loc = location + offsets[i];
        let wrapped_loc = vec2<i32>(
            (neighbor_loc.x + i32(dims.x)) % i32(dims.x),
            (neighbor_loc.y + i32(dims.y)) % i32(dims.y)
        );

        let neighbor = textureLoad(input_texture, wrapped_loc, 0).xy;
        laplacian += neighbor * weights[i];
    }

    // Reaction-Diffusion
    let reaction = u * v * v;
    let du_dt = params.du * laplacian.x - reaction + params.feed * (1.0 - u);
    let dv_dt = params.dv * laplacian.y + reaction - (params.feed + params.kill) * v;

    var next_u = clamp(u + du_dt * params.dt, 0.0, 1.0);
    var next_v = clamp(v + dv_dt * params.dt, 0.0, 1.0);

    // Mouse Interaction
    if (params.mouse_pressed > 0.5) {
        let mouse_pos = vec2<f32>(params.mouse_x, params.mouse_y);
        let dist = distance(vec2<f32>(f32(location.x), f32(location.y)), mouse_pos);
        if (dist < 20.0) {
            // Add V (activator)
            next_v = 0.9;
        }
    }

    textureStore(output_texture, location, vec4<f32>(next_u, next_v, 0.0, 1.0));
}
