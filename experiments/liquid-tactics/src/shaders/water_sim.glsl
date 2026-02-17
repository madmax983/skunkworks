#version 100
precision highp float;

varying vec2 uv;
uniform sampler2D _Texture; // Previous state (R=Height, G=VelX, B=VelY)
uniform sampler2D _Terrain; // Terrain (R=Height)
uniform vec2 _ScreenSize;
uniform vec4 _Mouse; // x, y, click_strength, brush_radius

void main() {
    vec2 size = _ScreenSize;
    vec2 texel = 1.0 / size;

    // Read current state
    vec4 state = texture2D(_Texture, uv);
    float h = state.r;
    vec2 v = (state.gb - 0.5) * 2.0;

    // Read terrain at center
    float b = texture2D(_Terrain, uv).r;

    // Neighbors
    float h_r = texture2D(_Texture, uv + vec2(texel.x, 0.0)).r;
    float b_r = texture2D(_Terrain, uv + vec2(texel.x, 0.0)).r;
    float H_r = h_r + b_r;

    float h_l = texture2D(_Texture, uv - vec2(texel.x, 0.0)).r;
    float b_l = texture2D(_Terrain, uv - vec2(texel.x, 0.0)).r;
    float H_l = h_l + b_l;

    float h_u = texture2D(_Texture, uv + vec2(0.0, texel.y)).r;
    float b_u = texture2D(_Terrain, uv + vec2(0.0, texel.y)).r;
    float H_u = h_u + b_u;

    float h_d = texture2D(_Texture, uv - vec2(0.0, texel.y)).r;
    float b_d = texture2D(_Terrain, uv - vec2(0.0, texel.y)).r;
    float H_d = h_d + b_d;

    // Gradient of Total Surface Height (H = h + b)
    float dHdx = (H_r - H_l) * 0.5;
    float dHdy = (H_u - H_d) * 0.5;

    // Physics Parameters
    float g = 0.4;
    float damping = 0.985;

    // Update Velocity
    v.x -= g * dHdx;
    v.y -= g * dHdy;
    v *= damping;

    // Calculate Velocity Neighbors for Divergence
    vec2 v_r = (texture2D(_Texture, uv + vec2(texel.x, 0.0)).gb - 0.5) * 2.0;
    vec2 v_l = (texture2D(_Texture, uv - vec2(texel.x, 0.0)).gb - 0.5) * 2.0;
    vec2 v_u = (texture2D(_Texture, uv + vec2(0.0, texel.y)).gb - 0.5) * 2.0;
    vec2 v_d = (texture2D(_Texture, uv - vec2(0.0, texel.y)).gb - 0.5) * 2.0;

    // Update Height (Conservation of Mass)
    float div = (v_r.x - v_l.x + v_u.y - v_d.y) * 0.5;
    h -= div * 0.8; // 0.8 is timestep-like factor

    // Mouse Interaction (Rain/Source)
    vec2 pixel_pos = uv * size;
    float dist = distance(pixel_pos, _Mouse.xy);
    if (_Mouse.z > 0.0 && dist < _Mouse.w) {
         h += 0.5; // Add water
    }

    // Bounds check
    h = max(h, 0.0);
    h = min(h, 5.0);

    gl_FragColor = vec4(h, v * 0.5 + 0.5, 1.0);
}
