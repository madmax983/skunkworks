#version 100
precision highp float;

varying vec2 uv;

uniform sampler2D Texture;
uniform vec2 Resolution;
uniform float Time;
uniform vec4 Params; // x=feed, y=kill, z=Du, w=Dv
uniform vec3 Mouse; // x,y normalized, z=clicked

// Laplacian convolution kernel
//  0.05  0.2  0.05
//  0.2  -1.0  0.2
//  0.05  0.2  0.05
vec2 laplacian(vec2 uv, vec2 pixel) {
    vec2 sum = vec2(0.0);

    // Center
    vec2 c = texture2D(Texture, uv).rg;

    // Neighbors
    vec2 n  = texture2D(Texture, uv + vec2(0.0, -pixel.y)).rg;
    vec2 s  = texture2D(Texture, uv + vec2(0.0,  pixel.y)).rg;
    vec2 e  = texture2D(Texture, uv + vec2( pixel.x, 0.0)).rg;
    vec2 w  = texture2D(Texture, uv + vec2(-pixel.x, 0.0)).rg;

    vec2 ne = texture2D(Texture, uv + vec2( pixel.x, -pixel.y)).rg;
    vec2 nw = texture2D(Texture, uv + vec2(-pixel.x, -pixel.y)).rg;
    vec2 se = texture2D(Texture, uv + vec2( pixel.x,  pixel.y)).rg;
    vec2 sw = texture2D(Texture, uv + vec2(-pixel.x,  pixel.y)).rg;

    // Convolution weights
    sum += c * -1.0;
    sum += (n + s + e + w) * 0.2;
    sum += (ne + nw + se + sw) * 0.05;

    return sum;
}

void main() {
    vec2 pixel = 1.0 / Resolution;
    vec2 st = uv;

    vec4 current = texture2D(Texture, st);
    float u = current.r;
    float v = current.g;

    vec2 lap = laplacian(st, pixel);

    // Gray-Scott parameters
    float F = Params.x;
    float k = Params.y;
    float Du = Params.z;
    float Dv = Params.w;

    // Reaction
    float uvv = u * v * v;

    // Diffusion + Reaction
    // du/dt = Du*Lap(u) - uv^2 + F(1-u)
    // dv/dt = Dv*Lap(v) + uv^2 - (F+k)v

    float du = Du * lap.x - uvv + F * (1.0 - u);
    float dv = Dv * lap.y + uvv - (F + k) * v;

    u += du; // Time step = 1.0 (implicit in coefficients)
    v += dv;

    // Mouse interaction: Add 'v' (catalyst)
    float dist = distance(st, Mouse.xy);
    if (Mouse.z > 0.5 && dist < 0.05) {
        v += 0.5;
    }

    // Clamp
    u = clamp(u, 0.0, 1.0);
    v = clamp(v, 0.0, 1.0);

    gl_FragColor = vec4(u, v, 0.0, 1.0);
}
