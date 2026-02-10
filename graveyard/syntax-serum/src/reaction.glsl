#version 100
precision highp float;

varying vec2 uv;

uniform sampler2D Texture;
uniform vec2 ScreenSize;
uniform float Feed;
uniform float Kill;
uniform float DA;
uniform float DB;
uniform float dt;

void main() {
    vec2 texel = 1.0 / ScreenSize;

    // Laplacian using 3x3 kernel with diagonal neighbors
    // Weights: Center -1.0, Orthogonal 0.2, Diagonal 0.05
    // Sum should be 0.

    vec4 center = texture2D(Texture, uv);

    vec4 up = texture2D(Texture, uv + vec2(0.0, texel.y));
    vec4 down = texture2D(Texture, uv + vec2(0.0, -texel.y));
    vec4 left = texture2D(Texture, uv + vec2(-texel.x, 0.0));
    vec4 right = texture2D(Texture, uv + vec2(texel.x, 0.0));

    vec4 ul = texture2D(Texture, uv + vec2(-texel.x, texel.y));
    vec4 ur = texture2D(Texture, uv + vec2(texel.x, texel.y));
    vec4 dl = texture2D(Texture, uv + vec2(-texel.x, -texel.y));
    vec4 dr = texture2D(Texture, uv + vec2(texel.x, -texel.y));

    vec4 laplacian =
        center * -1.0 +
        (up + down + left + right) * 0.2 +
        (ul + ur + dl + dr) * 0.05;

    float a = center.r;
    float b = center.g;

    // Reaction: A + 2B -> 3B
    // Rate of reaction = A * B^2
    float reaction = a * b * b;

    // Diffusion + Reaction + Feed/Kill
    // dA/dt = DA * lapA - reaction + f * (1 - A)
    float da = DA * laplacian.r - reaction + Feed * (1.0 - a);

    // dB/dt = DB * lapB + reaction - (k + f) * B
    float db = DB * laplacian.g + reaction - (Feed + Kill) * b;

    float next_a = a + da * dt;
    float next_b = b + db * dt;

    gl_FragColor = vec4(clamp(next_a, 0.0, 1.0), clamp(next_b, 0.0, 1.0), 0.0, 1.0);
}
