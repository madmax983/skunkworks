#version 100
precision mediump float;

varying vec2 uv;

uniform sampler2D Texture;
uniform vec2 Center;     // Center of black hole in normalized screen coords (0..1)
uniform float Aspect;    // Aspect ratio (width/height)
uniform float Mass;      // Strength of lensing

void main() {
    vec2 p = uv - Center;
    p.x *= Aspect; // Correct for aspect ratio so the hole is circular

    float r = length(p);
    float r2 = r * r;

    // Schwarzschild radius approximation for visual cut-off
    float rs = Mass * 0.1;

    if (r < rs) {
        gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
        return;
    }

    // Gravitational Lensing
    // Displace UV coordinate towards the center (light is bent around the mass)
    // Formula: u' = u - (theta_E^2 / |u|) * u
    // Here we just use a simple 1/r falloff for visual effect

    float distortion = (Mass * 0.05) / r;

    vec2 displacement = normalize(p) * distortion;

    // We look up the texture at the displaced position
    // If we look at pixel P, we see light coming from direction D bent by gravity.
    // The light effectively comes from a point "behind" the black hole's apparent position?
    // Let's just subtract displacement.
    vec2 tex_coord = uv - displacement;

    // Fix aspect ratio back for texture lookup? No, tex_coord is already normalized.
    // Wait, displacement is in "aspect-corrected" space. We need to un-correct it?
    // displacement.x /= Aspect;

    vec2 final_uv = uv - vec2(displacement.x / Aspect, displacement.y);

    vec4 color = texture2D(Texture, final_uv);

    // Accretion disk / Photon sphere glow
    float glow = exp(-(r - rs) * 20.0);
    color.rgb += vec3(1.0, 0.6, 0.2) * glow * 2.0;

    gl_FragColor = color;
}
