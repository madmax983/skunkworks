struct VertexInput {
    @location(0) position: vec4<f32>,
    @location(1) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

struct Uniforms {
    view_proj: mat4x4<f32>,
    rot_4d: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;

    // Apply 4D rotation
    let p4 = uniforms.rot_4d * model.position;

    // Project 4D to 3D (Stereographic projection)
    // Project from (0,0,0,-2) to w=0 plane
    let w_camera = 3.0;
    // Avoid division by zero
    let div = w_camera - p4.w;
    var scale = 1.0;
    if (abs(div) > 0.001) {
        scale = 1.0 / div;
    }

    let p3 = vec3<f32>(p4.x, p4.y, p4.z) * scale;

    out.clip_position = uniforms.view_proj * vec4<f32>(p3, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
