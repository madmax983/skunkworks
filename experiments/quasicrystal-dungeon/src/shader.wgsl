struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
};

struct InstanceInput {
    @location(5) model_pos: vec3<f32>,
    @location(6) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) normal: vec3<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = instance.color;
    // Simple translation
    let world_position = model.position + instance.model_pos;
    out.clip_position = camera.view_proj * vec4<f32>(world_position, 1.0);
    out.normal = model.normal;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Simple directional light
    let light_dir = normalize(vec3<f32>(1.0, 2.0, 3.0));
    let diffuse = max(dot(in.normal, light_dir), 0.2);
    let color = in.color * diffuse;
    return vec4<f32>(color, 1.0);
}
