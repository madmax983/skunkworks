#version 100
precision highp float;

varying vec2 uv;

uniform sampler2D Texture;
uniform vec2 ScreenSize;

uniform float feed;
uniform float kill;
uniform float diff_a;
uniform float diff_b;
uniform float dt;

void main() {
    vec2 pixel = 1.0 / ScreenSize;

    // Laplacian stencil (convolution)
    //  0.05  0.20  0.05
    //  0.20 -1.00  0.20
    //  0.05  0.20  0.05

    vec4 center = texture2D(Texture, uv);
    float a = center.r;
    float b = center.g;

    // Convolution for Laplacian
    vec4 sum = vec4(0.0);

    // Cardinal neighbors
    sum += texture2D(Texture, uv + vec2(-pixel.x, 0.0)) * 0.20;
    sum += texture2D(Texture, uv + vec2(pixel.x, 0.0)) * 0.20;
    sum += texture2D(Texture, uv + vec2(0.0, -pixel.y)) * 0.20;
    sum += texture2D(Texture, uv + vec2(0.0, pixel.y)) * 0.20;

    // Diagonal neighbors
    sum += texture2D(Texture, uv + vec2(-pixel.x, -pixel.y)) * 0.05;
    sum += texture2D(Texture, uv + vec2(pixel.x, -pixel.y)) * 0.05;
    sum += texture2D(Texture, uv + vec2(-pixel.x, pixel.y)) * 0.05;
    sum += texture2D(Texture, uv + vec2(pixel.x, pixel.y)) * 0.05;

    // Center weight
    sum += center * -1.0;

    float lap_a = sum.r;
    float lap_b = sum.g;

    // Reaction-Diffusion
    // dA/dt = Da * lapA - AB^2 + f(1-A)
    // dB/dt = Db * lapB + AB^2 - (k+f)B

    float reaction = a * b * b;

    float new_a = a + (diff_a * lap_a - reaction + feed * (1.0 - a)) * dt;
    float new_b = b + (diff_b * lap_b + reaction - (kill + feed) * b) * dt;

    gl_FragColor = vec4(clamp(new_a, 0.0, 1.0), clamp(new_b, 0.0, 1.0), 0.0, 1.0);
}
