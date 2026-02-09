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
    @location(5) model_0: vec4<f32>,
    @location(6) model_1: vec4<f32>,
    @location(7) model_2: vec4<f32>,
    @location(8) model_3: vec4<f32>,
    @location(9) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) world_pos: vec3<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    let model_matrix = mat4x4<f32>(
        instance.model_0,
        instance.model_1,
        instance.model_2,
        instance.model_3,
    );

    var out: VertexOutput;
    out.color = instance.color;

    let world_position = model_matrix * vec4<f32>(model.position, 1.0);
    out.world_pos = world_position.xyz;
    out.clip_position = camera.view_proj * world_position;

    // Transform normal
    // Ideally use inverse transpose for non-uniform scaling, but for simple visualization this is ok.
    // Or just use the rotation part of the matrix.
    let normal_matrix = mat3x3<f32>(
        instance.model_0.xyz,
        instance.model_1.xyz,
        instance.model_2.xyz,
    );
    out.normal = normalize(normal_matrix * model.normal);

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Simple directional light
    let light_dir = normalize(vec3<f32>(1.0, 2.0, 3.0));
    let diffuse = max(dot(in.normal, light_dir), 0.2);
    let lighting = in.color.rgb * diffuse;

    // Fog
    // We don't have camera pos, but we can assume camera is at origin relative to view,
    // or just use z-depth.
    // Actually, we can just use length(in.world_pos - vec3(5.0, 5.0, 5.0)) if hardcoded,
    // but camera moves.
    // Let's use w component of clip position which is linear depth in view space (z distance).
    let dist = in.clip_position.w;

    let fog_start = 10.0;
    let fog_end = 40.0;
    let fog_color = vec3<f32>(0.05, 0.05, 0.1); // Same as clear color

    let fog_factor = clamp((dist - fog_start) / (fog_end - fog_start), 0.0, 1.0);

    let final_color = mix(lighting, fog_color, fog_factor);

    return vec4<f32>(final_color, in.color.a);
}
