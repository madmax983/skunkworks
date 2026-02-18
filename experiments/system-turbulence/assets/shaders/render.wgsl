#import bevy_sprite::mesh2d_vertex_output::Mesh2dVertexOutput

@group(1) @binding(0) var density_texture: texture_2d<f32>;
@group(1) @binding(1) var density_sampler: sampler;

@fragment
fn fragment(in: Mesh2dVertexOutput) -> @location(0) vec4<f32> {
    let density = textureSample(density_texture, density_sampler, in.uv).r;

    // "System Turbulence" Palette
    // 0.0 -> Deep Blue (Idle)
    // 0.5 -> Purple (Busy)
    // 1.0 -> Bright Red/Orange (Overload)

    let col_idle = vec3<f32>(0.05, 0.05, 0.2);
    let col_busy = vec3<f32>(0.5, 0.0, 0.5);
    let col_crit = vec3<f32>(1.0, 0.8, 0.2);

    var color = col_idle;
    if (density < 0.5) {
        color = mix(col_idle, col_busy, density * 2.0);
    } else {
        color = mix(col_busy, col_crit, (density - 0.5) * 2.0);
    }

    // Add some scanlines for "System" feel
    let scanline = sin(in.uv.y * 800.0) * 0.1;
    color += scanline;

    return vec4<f32>(color, 1.0);
}
