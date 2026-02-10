// Vertex shader

struct CameraUniform {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct PlayerUniform {
    position: vec2<f32>,
    radius: f32,
    padding: f32,
};

@group(1) @binding(0)
var<uniform> player: PlayerUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) world_pos: vec2<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    out.world_pos = model.position.xy;
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let dist = distance(in.world_pos, player.position);

    // Hard cutoff fog
    // if (dist > player.radius) {
    //     discard;
    // }

    // Soft fog
    let fog_start = player.radius * 0.8;
    let fog_end = player.radius;
    let fog_factor = smoothstep(fog_end, fog_start, dist);

    // Also add a grid line effect?
    // Use modulo on world_pos?
    // No, simple color is fine.

    return vec4<f32>(in.color * fog_factor, 1.0);
}
