// Vertex shader

struct CameraUniform {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct MillerUniform {
    normal: vec3<f32>,
    distance: f32,
};

@group(1) @binding(0)
var<uniform> miller: MillerUniform;

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
    @location(1) dist: f32,
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

    // Calculate signed distance to the Miller plane
    // Plane: dot(p, n) - d = 0
    // We normalize 'n' in the CPU app, so this is true distance.
    out.dist = dot(world_position, miller.normal) - miller.distance;

    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Slicing logic: discard if outside a certain thickness
    // If the plane normal is zero (no plane selected), we shouldn't discard.
    // We can encode "disabled" as normal length 0.

    let normal_len_sq = dot(miller.normal, miller.normal);
    if (normal_len_sq > 0.001) {
        if (abs(in.dist) > 0.5) {
             discard;
             // Optional: visual flair instead of hard discard
             // return vec4<f32>(in.color.rgb * 0.2, 0.5);
        }
    }

    return in.color;
}
