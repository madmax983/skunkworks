struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    // Fullscreen triangle
    let uv = vec2<f32>(f32((in_vertex_index << 1u) & 2u), f32(in_vertex_index & 2u));
    out.clip_position = vec4<f32>(uv * 2.0 - 1.0, 0.0, 1.0);
    out.tex_coords = vec2<f32>(uv.x, 1.0 - uv.y); // Flip Y because WGPU texture coords are top-left?
    // Actually, simulation coords: (0,0) is usually top-left.
    // Standard UV: (0,0) top-left (DirectX/Metal/WGPU).
    // OpenGL is bottom-left.
    return out;
}

@group(0) @binding(0) var t_diffuse: texture_2d<f32>;
@group(0) @binding(1) var s_diffuse: sampler;

fn palette(t: f32) -> vec3<f32> {
    // "Moonshot" palette
    // Let's use a cosine palette
    // a + b*cos( 6.28318*(c*t+d) )
    let a = vec3<f32>(0.5, 0.5, 0.5);
    let b = vec3<f32>(0.5, 0.5, 0.5);
    let c = vec3<f32>(1.0, 1.0, 1.0);
    let d = vec3<f32>(0.00, 0.33, 0.67);
    return a + b * cos(6.28318 * (c * t + d));
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let density = textureSample(t_diffuse, s_diffuse, in.tex_coords).r;

    // Visualize
    let color = palette(density * 2.0 + 0.5); // Offset to make 0 density look interesting? No.
    // Let's map density 0 -> Black, >0 -> Color.

    let c = mix(vec3<f32>(0.0), color, smoothstep(0.0, 0.1, density));
    // And if density is very high, maybe white?

    // Simple:
    // density -> brightness
    // color based on something else?
    // Just use density for now.

    // Moonshot: "Turbulent Type" - maybe look like ink.
    // Ink is black on white?
    // Or glowing neon on black.

    let neon = vec3<f32>(0.1, 0.8, 1.0) * density * 2.0;

    return vec4<f32>(neon, 1.0);
}
