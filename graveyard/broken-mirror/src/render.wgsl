struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    var positions = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(3.0, -1.0),
        vec2<f32>(-1.0, 3.0)
    );
    let pos = positions[in_vertex_index];
    out.clip_position = vec4<f32>(pos, 0.0, 1.0);
    // UV: Map [-1, 3] to [0, 2] -> [0, 1] range needs care.
    // Actually we just map Clip Space [-1, 1] to UV [0, 1].
    // Since the triangle covers [-1, 3], the UVs will go beyond [0, 1] but usually clamped or wrapped by sampler if used.
    // But we are calculating coords manually based on tex_coords in fragment shader.
    // Wait, if we use tex_coords to sample textureLoad, we need to be careful.
    // The visible screen is [-1, 1].
    // At pos (-1, -1), UV is (0, 1).
    // At pos (1, -1), UV is (1, 1).
    // At pos (-1, 1), UV is (0, 0).
    // The large triangle extends beyond.
    // Let's just use the position directly.
    out.tex_coords = vec2<f32>(pos.x * 0.5 + 0.5, 1.0 - (pos.y * 0.5 + 0.5));
    return out;
}

@group(0) @binding(0) var input_texture: texture_2d<f32>;

fn hsv2rgb(c: vec3<f32>) -> vec3<f32> {
    let K = vec4<f32>(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    let p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let dims = textureDimensions(input_texture);
    let x = i32(in.tex_coords.x * f32(dims.x));
    let y = i32(in.tex_coords.y * f32(dims.y));

    // Clamp coordinates just in case
    let coord = vec2<i32>(
        clamp(x, 0, i32(dims.x) - 1),
        clamp(y, 0, i32(dims.y) - 1)
    );

    let angle = textureLoad(input_texture, coord, 0).r;

    // Map angle to hue
    // Angle is roughly [-PI, PI] but can drift.
    // Modulo 2PI
    let normalized = angle / (2.0 * 3.14159265);
    let hue = fract(normalized);

    // Saturation and Value constant for now
    let color = hsv2rgb(vec3<f32>(hue, 1.0, 1.0));

    return vec4<f32>(color, 1.0);
}
