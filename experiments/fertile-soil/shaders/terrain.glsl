#version 100
precision highp float;

varying vec2 uv;
uniform sampler2D Texture;
uniform vec2 ScreenSize;
uniform float Time;

void main() {
    vec2 pixel = 1.0 / ScreenSize;

    // Sample height (using Green/B channel)
    // Scale it to make features more pronounced
    float h = texture2D(Texture, uv).g;

    // Compute Normal via derivatives
    // Use finite difference
    float h_r = texture2D(Texture, uv + vec2(pixel.x, 0.0)).g;
    float h_l = texture2D(Texture, uv + vec2(-pixel.x, 0.0)).g;
    float h_t = texture2D(Texture, uv + vec2(0.0, pixel.y)).g;
    float h_b = texture2D(Texture, uv + vec2(0.0, -pixel.y)).g;

    float dx = (h_r - h_l) * 5.0; // Enhance slope
    float dy = (h_t - h_b) * 5.0;

    vec3 normal = normalize(vec3(-dx, -dy, 1.0));
    vec3 lightDir = normalize(vec3(0.5, 0.5, 1.0)); // Top-Right sun

    float diffuse = max(dot(normal, lightDir), 0.0);
    float ambient = 0.3;
    float lighting = ambient + diffuse * 0.7;

    // Biomes based on Height (h)
    // Deep Ocean < 0.1
    // Water < 0.2
    // Sand < 0.22
    // Grass < 0.4
    // Forest < 0.6
    // Rock < 0.8
    // Snow > 0.8

    vec3 color = vec3(0.0, 0.0, 0.2); // Deep Blue

    if (h > 0.1) color = mix(vec3(0.0, 0.0, 0.4), vec3(0.0, 0.5, 0.8), (h - 0.1) * 10.0);
    if (h > 0.2) color = vec3(0.0, 0.5, 0.8); // Water
    if (h > 0.22) color = mix(vec3(0.8, 0.7, 0.4), vec3(0.2, 0.6, 0.1), (h - 0.22) * 5.0); // Sand to Grass
    if (h > 0.3) color = vec3(0.2, 0.6, 0.1); // Grass
    if (h > 0.5) color = mix(vec3(0.1, 0.5, 0.0), vec3(0.4, 0.3, 0.2), (h - 0.5) * 5.0); // Forest to Rock
    if (h > 0.7) color = vec3(0.4, 0.3, 0.2); // Rock
    if (h > 0.8) color = mix(vec3(0.4, 0.3, 0.2), vec3(1.0, 1.0, 1.0), (h - 0.8) * 5.0); // Rock to Snow

    gl_FragColor = vec4(color * lighting, 1.0);
}
