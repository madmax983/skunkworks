struct Uniforms {
    view_matrix: mat4x4<f32>,
    proj_matrix: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) world_pos: vec3<f32>,
};

fn unproject_poincare_to_hyperboloid(p: vec3<f32>) -> vec4<f32> {
    let r2 = dot(p, p);
    let scale = 1.0 / (1.0 - r2);
    // (2x, 2y, 2z, 1+r^2) / (1-r^2)
    return vec4<f32>(
        2.0 * p.x * scale,
        2.0 * p.y * scale,
        2.0 * p.z * scale,
        (1.0 + r2) * scale
    );
}

fn project_hyperboloid_to_poincare(p: vec4<f32>) -> vec3<f32> {
    // (x, y, z) / (1 + w)
    return p.xyz / (1.0 + p.w);
}

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    // 1. Local (Poincaré Ball coords of cube) -> Hyperboloid
    let p_local_hyp = unproject_poincare_to_hyperboloid(model.position);

    // 2. Apply View Transform (Hyperbolic Isometry in R^4)
    // view_matrix represents the transform that moves the world relative to the player.
    // Usually View = Inverse(PlayerPos).
    let p_view_hyp = uniforms.view_matrix * p_local_hyp;

    // 3. Project back to Poincaré Ball for rendering
    let p_ball = project_hyperboloid_to_poincare(p_view_hyp);

    // 4. Apply Standard Euclidean Projection (Perspective)
    // This maps the 3D ball to 2D screen.
    out.clip_position = uniforms.proj_matrix * vec4<f32>(p_ball, 1.0);

    out.color = model.color;
    out.world_pos = p_ball;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Distance fog (black at edge of Poincaré disk)
    let dist = length(in.world_pos);
    // Poincaré disk radius is 1.0.
    // Let's fade out near 1.0.
    let fog = 1.0 - smoothstep(0.85, 0.99, dist);

    // Simple shading based on fake normal?
    // No, just color + fog is enough for "Neon Void" aesthetic.

    return vec4<f32>(in.color * fog, 1.0);
}
