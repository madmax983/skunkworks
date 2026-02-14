use macroquad::miniquad::{UniformDesc, UniformType};

pub const MAX_BODIES: usize = 16;

pub fn get_vertex_shader() -> &'static str {
    r#"
#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;

varying lowp vec2 uv;
varying lowp vec4 color;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1.0);
    uv = texcoord;
    color = color0;
}
"#
}

pub fn get_fragment_shader() -> String {
    let mut s = String::from(
        r#"
#version 100
precision highp float;

varying vec2 uv;
varying vec4 color;

uniform sampler2D Texture;
uniform vec2 ViewSize;
uniform vec2 ViewCenter;
uniform int Count;
uniform float LensingStrength;

"#,
    );

    for i in 0..MAX_BODIES {
        s.push_str(&format!("uniform vec2 Body{}_Pos;\n", i));
        s.push_str(&format!("uniform float Body{}_Mass;\n", i));
    }

    s.push_str(
        r#"
void main() {
    vec2 world_pos = (uv - 0.5) * ViewSize + ViewCenter;
    vec2 offset = vec2(0.0);
"#,
    );

    for i in 0..MAX_BODIES {
        s.push_str(&format!(
            r#"
    if ({} < Count) {{
        vec2 r = Body{}_Pos - world_pos;
        float d2 = dot(r, r);
        float soft_d2 = d2 + 100.0;
        float strength = Body{}_Mass * LensingStrength;
        offset += r * (strength / soft_d2);
    }}
"#,
            i, i, i
        ));
    }

    s.push_str(
        r#"
    vec2 source_world_pos = world_pos + offset;
    vec2 source_uv = (source_world_pos - ViewCenter) / ViewSize + 0.5;

    gl_FragColor = texture2D(Texture, source_uv) * color;
}
"#,
    );

    s
}

pub fn get_uniforms() -> Vec<UniformDesc> {
    let mut u = vec![
        UniformDesc::new("ViewSize", UniformType::Float2),
        UniformDesc::new("ViewCenter", UniformType::Float2),
        UniformDesc::new("Count", UniformType::Int1),
        UniformDesc::new("LensingStrength", UniformType::Float1),
    ];
    for i in 0..MAX_BODIES {
        u.push(UniformDesc::new(
            &format!("Body{}_Pos", i),
            UniformType::Float2,
        ));
        u.push(UniformDesc::new(
            &format!("Body{}_Mass", i),
            UniformType::Float1,
        ));
    }
    u
}
