struct Particle {
    pos: vec2<f32>,
    vel: vec2<f32>,
    life: f32,
    char_idx: u32,
    pad: vec2<f32>,
}

@group(0) @binding(0) var<storage, read> particles: array<Particle>;
@group(0) @binding(1) var font_atlas: texture_2d<f32>;
@group(0) @binding(2) var font_sampler: sampler;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
}

@vertex
fn vs_main(
    @builtin(vertex_index) vertex_index: u32,
    @builtin(instance_index) instance_index: u32
) -> VertexOutput {
    let p = particles[instance_index];

    // Quad vertices (Triangle Strip)
    var pos = vec2<f32>(0.0, 0.0);
    var uv = vec2<f32>(0.0, 0.0);

    switch (vertex_index) {
        case 0u: { pos = vec2<f32>(-1.0, -1.0); uv = vec2<f32>(0.0, 1.0); }
        case 1u: { pos = vec2<f32>( 1.0, -1.0); uv = vec2<f32>(1.0, 1.0); }
        case 2u: { pos = vec2<f32>(-1.0,  1.0); uv = vec2<f32>(0.0, 0.0); }
        case 3u: { pos = vec2<f32>( 1.0,  1.0); uv = vec2<f32>(1.0, 0.0); }
        default: {}
    }

    // Instance size
    let size = 0.01;

    // Position in NDC (-1..1)
    let center = (p.pos * 2.0) - 1.0;

    // Flip Y because usually texture coords 0,0 is top-left, but NDC +Y is up.
    // If p.pos.y=0 is top, center.y = -1. That's bottom in NDC.
    // So we want p.pos.y=0 -> +1. p.pos.y=1 -> -1.
    // center.y = (1.0 - p.pos.y) * 2.0 - 1.0 = 1 - 2y.

    let ndc_y = 1.0 - p.pos.y * 2.0;
    let ndc_x = p.pos.x * 2.0 - 1.0;

    let world_pos = vec2<f32>(ndc_x, ndc_y) + pos * size;

    var out: VertexOutput;
    out.clip_position = vec4<f32>(world_pos.x, world_pos.y, 0.0, 1.0);

    // Atlas UV
    // Assume atlas is grid of 16x16 chars (256 chars)
    let char_idx = p.char_idx % 256u;
    let col = char_idx % 16u;
    let row = char_idx / 16u;

    let char_uv_size = 1.0 / 16.0;
    let atlas_uv = vec2<f32>(f32(col) * char_uv_size, f32(row) * char_uv_size) + uv * char_uv_size;

    out.uv = atlas_uv;

    // Color based on velocity
    let vel_mag = length(p.vel);
    // Visualize velocity as color (Red/Blue shift?)
    // Or just simple white/blue
    let r = 0.5 + p.vel.x * 20.0;
    let g = 0.5 + p.vel.y * 20.0;
    out.color = vec4<f32>(r, g, 1.0, 1.0);

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let alpha = textureSample(font_atlas, font_sampler, in.uv).r;
    // Alpha threshold
    if (alpha < 0.2) { discard; }
    return vec4<f32>(in.color.rgb, alpha);
}
