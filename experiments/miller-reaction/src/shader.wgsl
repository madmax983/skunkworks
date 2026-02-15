// Vertex shader

struct CameraUniform {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
};

struct InstanceInput {
    @location(2) model_pos: vec3<f32>,
    @location(3) color: vec4<f32>,
    @location(4) scale: vec3<f32>,
    @location(5) rotation: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) tex_coords: vec2<f32>,
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
    out.tex_coords = model.tex_coords;
    return out;
}

// Fragment shader

@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(1) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let chemicals = textureSample(t_diffuse, s_diffuse, in.tex_coords);
    let v = chemicals.y; // Green channel is 'v' (activator/pattern)

    // Visualization:
    // Pattern color (Cyan-ish)
    let val = v * 5.0; // Boost
    let pattern = vec3<f32>(0.2, 0.9, 1.0) * smoothstep(0.1, 0.4, val);

    // Mix instance color (base crystal color) with pattern
    // If pattern is strong, it glows.
    let mixed = in.color.rgb + pattern;

    return vec4<f32>(mixed, in.color.a);
}
