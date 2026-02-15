// Camera Uniform
struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct InstanceInput {
    @location(5) model_pos: vec3<f32>,
    @location(6) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

// --- Node Shader (Instanced) ---

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;
    // Simple translation for instance
    let world_position = model.position + instance.model_pos;
    out.clip_position = camera.view_proj * vec4<f32>(world_position, 1.0);
    out.color = instance.color;
    out.normal = model.normal;
    out.uv = model.uv;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Simple lighting
    let light_dir = normalize(vec3<f32>(1.0, 1.0, 1.0));
    let ambient = 0.2;
    let diffuse = max(dot(in.normal, light_dir), 0.0);

    // Add a glow based on UV or something? No, let's keep it simple.
    let lighting = ambient + diffuse;

    return vec4<f32>(in.color.rgb * lighting, in.color.a);
}

// --- Line Shader (No Instances) ---

struct LineVertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_line(
    model: VertexInput,
) -> LineVertexOutput {
    var out: LineVertexOutput;
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    // Gradient along line based on UV? Or just white/cyan.
    // Let's use UV.x to fade out?
    // Or just a fixed color for lines.
    // Maybe pass color as uniform?
    // Let's use UV to color gradients.
    // If UV.x is 0 -> Start (Source), UV.x is 1 -> End (Target).

    // Default color for edges
    out.color = vec4<f32>(0.5, 0.8, 1.0, 0.5); // Light Blue, semi-transparent

    return out;
}

@fragment
fn fs_line(in: LineVertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}

// --- Wireframe Shader (Using VertexInput but different color) ---
@vertex
fn vs_wireframe(
    model: VertexInput,
) -> LineVertexOutput {
    var out: LineVertexOutput;
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    out.color = vec4<f32>(0.1, 0.1, 0.1, 0.1); // Very faint gray for the tube
    return out;
}
