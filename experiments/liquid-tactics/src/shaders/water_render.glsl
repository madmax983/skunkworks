#version 100
precision highp float;

varying vec2 uv;
uniform sampler2D _Texture; // Water State
uniform sampler2D _Terrain; // Terrain
uniform vec2 _ScreenSize;
uniform float _Time;

void main() {
    vec2 size = _ScreenSize;
    vec2 texel = 1.0 / size;

    float h = texture2D(_Texture, uv).r;
    float b = texture2D(_Terrain, uv).r;

    // Terrain Color
    vec3 colGrass = vec3(0.1, 0.6, 0.2);
    vec3 colDirt = vec3(0.5, 0.4, 0.3);
    vec3 colSnow = vec3(0.9, 0.9, 1.0);
    vec3 terrainColor = mix(colGrass, colDirt, b * 1.5);
    if (b > 0.8) terrainColor = mix(colDirt, colSnow, (b - 0.8) * 5.0);

    // Water Surface Normal
    // Calculate neighbor heights including terrain for waves against shore
    // Actually, just water surface normal is fine for specular.
    float h_r = texture2D(_Texture, uv + vec2(texel.x, 0.0)).r;
    float h_u = texture2D(_Texture, uv + vec2(0.0, texel.y)).r;

    float dhdx = (h_r - h) * 15.0;
    float dhdy = (h_u - h) * 15.0;
    vec3 normal = normalize(vec3(-dhdx, -dhdy, 1.0));

    // Lighting
    vec3 lightDir = normalize(vec3(0.3, 0.5, 0.8));
    float diffuse = max(dot(normal, lightDir), 0.2);

    // Specular
    vec3 viewDir = vec3(0.0, 0.0, 1.0);
    vec3 reflectDir = reflect(-lightDir, normal);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), 64.0);

    // Water Color
    vec3 waterDeep = vec3(0.0, 0.1, 0.3);
    vec3 waterShallow = vec3(0.0, 0.5, 0.8);
    vec3 waterColor = mix(waterShallow, waterDeep, h * 0.5) * diffuse + vec3(1.0) * spec;

    // Refraction (Fake)
    vec2 distort = normal.xy * 0.01 * min(h, 1.0);
    float b_refract = texture2D(_Terrain, uv + distort).r;
    vec3 terrainRefract = mix(colGrass, colDirt, b_refract * 1.5);
    if (b_refract > 0.8) terrainRefract = mix(colDirt, colSnow, (b_refract - 0.8) * 5.0);

    // Alpha based on depth
    float alpha = smoothstep(0.0, 0.5, h);

    // Foam at edges (where depth is small but turbulence is high?)
    // Simple foam: where h is small but gradient is high?
    // Or just shoreline.
    float foam = smoothstep(0.1, 0.0, h) * smoothstep(0.0, 0.1, h);
    // Actually simple alpha blending is enough for now.

    vec3 finalColor = mix(terrainRefract, waterColor, alpha);

    // Add Grid/Strategy overlay?
    // Maybe just scanlines for "Digital" feel
    // finalColor *= 0.9 + 0.1 * sin(uv.y * size.y * 3.14);

    gl_FragColor = vec4(finalColor, 1.0);
}
