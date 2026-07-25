struct CameraUniform {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
};

struct InstanceInput {
    @location(2) model_pos: vec3<f32>,
    @location(3) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;
    let world_position = model.position + instance.model_pos;
    out.clip_position = camera.view_proj * vec4<f32>(world_position, 1.0);
    out.color = instance.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}

// Line Pipeline

@vertex
fn vs_line(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);

    // Generate color from UV
    // u, v are in [0, 2PI] approx
    let u = model.uv.x;
    let v = model.uv.y;

    // Moonshot aesthetic: Neon gradients
    // R based on U
    // G based on V
    // B oscillating

    let r = sin(u) * 0.5 + 0.5;
    let g = sin(v) * 0.5 + 0.5;
    let b = sin(u + v) * 0.5 + 0.5;

    out.color = vec4<f32>(r, g, b, 0.8);
    return out;
}

@fragment
fn fs_line(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
