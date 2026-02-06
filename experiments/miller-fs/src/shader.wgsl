// Vertex shader

struct CameraUniform {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct InstanceInput {
    @location(1) model_pos: vec3<f32>,
    @location(2) color: vec4<f32>,
    @location(3) scale: vec3<f32>,
    @location(4) rotation: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

fn q_rotate(v: vec3<f32>, q: vec4<f32>) -> vec3<f32> {
    let t = 2.0 * cross(q.xyz, v);
    return v + q.w * t + cross(q.xyz, t);
}

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;
    // Scale -> Rotate -> Translate
    let scaled = model.position * instance.scale;
    let rotated = q_rotate(scaled, instance.rotation);
    let world_position = rotated + instance.model_pos;

    out.clip_position = camera.view_proj * vec4<f32>(world_position, 1.0);
    out.color = instance.color;
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
