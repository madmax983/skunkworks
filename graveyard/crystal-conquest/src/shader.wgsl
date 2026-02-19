struct GlobalUniform {
    view_proj: mat4x4<f32>,
    inv_view_proj: mat4x4<f32>,
    view_pos: vec4<f32>,
    time: f32,
    _pad: f32,
    resolution: vec2<f32>,
};

struct Particle {
    position: vec3<f32>,
    _pad1: f32,
    velocity: vec3<f32>,
    _pad2: f32,
    color: vec3<f32>,
    faction: u32,
};

@group(0) @binding(0) var<uniform> global: GlobalUniform;
@group(1) @binding(0) var<storage, read> particles: array<Particle>;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    // Full screen triangle
    let x = f32((in_vertex_index & 1u) << 2u);
    let y = f32((in_vertex_index & 2u) << 1u);
    out.uv = vec2<f32>(x * 0.5, y * 0.5);
    out.clip_position = vec4<f32>(x - 1.0, y - 1.0, 0.0, 1.0);
    return out;
}

// Ray-Sphere Intersection
// Returns distance to intersection, or -1.0 if miss
fn intersect_sphere(ro: vec3<f32>, rd: vec3<f32>, radius: f32) -> f32 {
    let b = dot(ro, rd);
    let c = dot(ro, ro) - radius * radius;
    let h = b * b - c;
    if (h < 0.0) { return -1.0; }
    return -b - sqrt(h);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Generate Ray
    // NDC coord: (in.uv * 2.0 - 1.0)
    // But in.uv from full screen triangle might be > 1.0?
    // Wait, the vertex shader above generates UV 0..2
    // Let's use clip_position.xy / clip_position.w which is screen space -1..1

    let ndc_x = in.clip_position.x / global.resolution.x * 2.0 - 1.0;
    let ndc_y = (1.0 - in.clip_position.y / global.resolution.y) * 2.0 - 1.0; // Flip Y?

    // Better: use the UV passed from vertex shader if correctly mapped to 0..1 for the screen area.
    // The vertex shader outputs UV in 0..2 range for the big triangle.
    // But clip_position is in -1..1 (mostly).

    // Standard way to unproject:
    // 1. Get clip space coordinate (x, y, 0, 1)
    // 2. Multiply by inv_view_proj
    // 3. Divide by w

    // We need the UV of the *screen*.
    let screen_uv = in.clip_position.xy / global.resolution.xy;
    let ndc = vec4<f32>(screen_uv.x * 2.0 - 1.0, (1.0 - screen_uv.y) * 2.0 - 1.0, 1.0, 1.0);

    let world_pos_h = global.inv_view_proj * ndc;
    let world_pos = world_pos_h.xyz / world_pos_h.w;

    let ro = global.view_pos.xyz;
    let rd = normalize(world_pos - ro);

    let t = intersect_sphere(ro, rd, 1.0);

    if (t < 0.0) {
        // Background
        return vec4<f32>(0.05, 0.05, 0.1, 1.0);
    }

    let p = ro + rd * t;
    let normal = normalize(p);

    // Voronoi Logic
    var min_dist = 1000.0;
    var second_min_dist = 1000.0;
    var closest_color = vec3<f32>(0.0);
    var closest_idx = 0u;

    let num_particles = arrayLength(&particles);

    for (var i = 0u; i < num_particles; i++) {
        let particle = particles[i];
        // Chord distance on unit sphere = length(p - particle.pos)
        let d = distance(p, particle.position);

        if (d < min_dist) {
            second_min_dist = min_dist;
            min_dist = d;
            closest_color = particle.color;
            closest_idx = i;
        } else if (d < second_min_dist) {
            second_min_dist = d;
        }
    }

    // Lighting
    let light_dir = normalize(vec3<f32>(1.0, 1.0, 1.0));
    let diff = max(dot(normal, light_dir), 0.0);
    let ambient = 0.2;

    // Cell visual
    var color = closest_color * (diff + ambient);

    // Borders
    let border_width = 0.02;
    if (second_min_dist - min_dist < border_width) {
        color = vec3<f32>(1.0, 0.8, 0.2); // Glowing border
    }

    // Center glow
    if (min_dist < 0.05) {
         color += vec3<f32>(0.5, 0.5, 0.5);
    }

    return vec4<f32>(color, 1.0);
}
