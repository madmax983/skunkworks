struct SimulationParams {
    temperature: f32,
    width: u32,
    height: u32,
    mouse_x: f32,
    mouse_y: f32,
    mouse_active: u32,
    time: f32,
    _padding: u32,
}

@group(0) @binding(0) var input_texture: texture_2d<f32>;
@group(0) @binding(1) var output_texture: texture_storage_2d<r32float, write>;
@group(0) @binding(2) var<uniform> params: SimulationParams;

// Pseudo-random number generator
fn hash(p: vec2<u32>) -> f32 {
    var p2 = vec2<f32>(p);
    p2 = fract(p2 * vec2<f32>(0.1031, 0.1030));
    p2 += dot(p2, p2.yx + 33.33);
    return fract((p2.x + p2.y) * p2.x);
}

fn rand(co: vec2<f32>) -> f32 {
    return fract(sin(dot(co, vec2<f32>(12.9898, 78.233))) * 43758.5453);
}

const PI: f32 = 3.14159265359;

@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let x = global_id.x;
    let y = global_id.y;

    if (x >= params.width || y >= params.height) {
        return;
    }

    let coord = vec2<i32>(i32(x), i32(y));
    let center_angle = textureLoad(input_texture, coord, 0).r;

    // Neighbors (Periodic Boundary Conditions)
    let w = i32(params.width);
    let h = i32(params.height);

    let left = textureLoad(input_texture, vec2<i32>((coord.x - 1 + w) % w, coord.y), 0).r;
    let right = textureLoad(input_texture, vec2<i32>((coord.x + 1) % w, coord.y), 0).r;
    let up = textureLoad(input_texture, vec2<i32>(coord.x, (coord.y - 1 + h) % h), 0).r;
    let down = textureLoad(input_texture, vec2<i32>(coord.x, (coord.y + 1) % h), 0).r;

    // Calculate Torque (Sum of sin differences)
    // J = 1.0 (Ferromagnetic)
    var torque: f32 = 0.0;
    torque += sin(left - center_angle);
    torque += sin(right - center_angle);
    torque += sin(up - center_angle);
    torque += sin(down - center_angle);

    // Langevin Dynamics
    let dt = 0.1;
    let noise_scale = sqrt(2.0 * params.temperature * dt);

    // Random noise
    let rng = rand(vec2<f32>(f32(x) + params.time, f32(y) + params.time));
    let noise = (rng - 0.5) * 2.0 * noise_scale * 5.0; // Boost noise for visibility

    var new_angle = center_angle + torque * dt + noise;

    // Mouse Interaction
    // If mouse is close, align or disorder
    let mx = params.mouse_x * f32(params.width);
    let my = params.mouse_y * f32(params.height);
    let dx = f32(x) - mx;
    let dy = f32(y) - my;
    let dist_sq = dx*dx + dy*dy;
    let radius = 50.0; // Influence radius

    if (params.mouse_active == 1u && dist_sq < radius * radius) {
        // Align to mouse direction (or just a fixed angle)
        // Let's align to the direction from center to mouse?
        // Or just align to 0.
        // Let's align to the vector (dx, dy)
        let target_angle = atan2(dy, dx);
        // Mix heavily towards target
        new_angle = mix(new_angle, target_angle, 0.5);
    } else if (params.mouse_active == 2u && dist_sq < radius * radius) {
        // Inject Heat (Randomize)
        new_angle += (rand(vec2<f32>(f32(x)*params.time, f32(y))) - 0.5) * 10.0;
    }

    // Normalize angle to [-PI, PI] or [0, 2PI]
    // Actually, storing unbounded angle works fine for sin/cos,
    // but keeping it bounded helps precision.
    new_angle = new_angle % (2.0 * PI);

    textureStore(output_texture, coord, vec4<f32>(new_angle, 0.0, 0.0, 1.0));
}
