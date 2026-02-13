pub const VERTEX: &str = r#"
#version 100
attribute vec3 position;
attribute vec2 texcoord;
varying vec2 uv;
void main() {
    gl_Position = vec4(position, 1.0);
    uv = texcoord;
}
"#;

pub const FRAGMENT: &str = r#"
#version 100
precision highp float;
varying vec2 uv;
uniform vec3 iResolution;
uniform float iTime;
uniform mat4 u_camera;

// Polyfill for acosh (missing in GLSL 100)
float acosh(float x) {
    return log(x + sqrt(x * x - 1.0));
}

// Hyperbolic Math
// We work in the Minkowski model (-+++ metric).
// But for rendering, we use the Klein model where rays are straight lines.

float h_dot(vec4 a, vec4 b) {
    return a.w * b.w - dot(a.xyz, b.xyz);
}

// Convert Klein point to Minkowski point (on hyperboloid)
vec4 klein_to_mink(vec3 k) {
    float k2 = dot(k, k);
    if (k2 >= 1.0) return vec4(0.0, 0.0, 0.0, 0.0); // Invalid
    float gamma = 1.0 / sqrt(1.0 - k2);
    return vec4(k * gamma, gamma);
}

// Distance estimator in Hyperbolic space
// Fold positions to fundamental domain of a tiling
// {4,3,5} cubic honeycomb?
// Or just a simple box fold.

// Hyperbolic translation of point p by vector -c (inverse translation)
// in Minkowski model is just Lorentz boost.
// But doing full tiling is complex.

// Let's do a simple "Bulb" or IFS.
// 3D Hyperbolic IFS from "Hyperbolic Fractal" on Shadertoy ideas.

float map(vec3 p_klein) {
    vec4 p = klein_to_mink(p_klein);
    if (p.w == 0.0) return 0.0; // Boundary

    // Apply folding in Minkowski space?
    // Or fold in Klein space?
    // Folding in Klein space is easier but distorts metric.

    // Let's place a sphere at the origin.
    // Hyperbolic distance from origin to p: acosh(p.w).
    // Radius R = 0.5.

    float d = acosh(p.w) - 0.5;

    // Create a grid by folding?
    // Reflections in planes passing through origin.
    // In Minkowski, these are linear reflections.
    // Refl across plane normal n: p' = p - 2*dot(p,n)*n?
    // Yes if n is spacelike unit vector.

    // Dodecahedral symmetry planes.
    // Normals to the 12 faces of dodecahedron.
    // Iterated folding.

    float scale = 1.0;

    // Simple cubic folds (gives {4,3,...})
    for (int i=0; i<4; i++) {
        p.xyz = abs(p.xyz);
        // Fold across x+y+z = k?
        // This generates the fractal structure.

        // This is pseudo-code for a visual.
        if (p.x < p.y) p.xy = p.yx;
        if (p.x < p.z) p.xz = p.zx;
        if (p.y < p.z) p.yz = p.zy;

        // Stretches?
        // In H3, translations are boosts.
    }

    // Let's just render a single "Hyperbolic Planet" for now,
    // or a "Hyperbolic Lattice" of spheres.

    // To make it infinite, we need to mod the position.
    // This is hard in H3 without proper group theory implementation.

    // Fallback: Render a cool 4D shape projected.
    // No, keep it H3.

    // Try: tiling of spheres.
    // Dist to origin.
    float d_origin = acosh(p.w) - 0.3;

    // Add some noise based on direction
    float noise = sin(p.x*10.0)*sin(p.y*10.0)*sin(p.z*10.0);

    return d_origin + noise * 0.05;
}

// Convert hyperbolic distance to Klein step
// Euclidean dist in Klein approx = Hyp dist * (1 - r^2) / 2 ?
// d_klein = tanh(d_hyp / 2) ?
// Formula: ds_hyp = 2 * ds_euc / (1 - r^2)  (Poincare)
// For Klein: ds_hyp = ds_euc / (1 - r^2)  (Radial)
//            ds_hyp = ds_euc / sqrt(1-r^2) (Tangential)
// Conservative step: ds_euc = ds_hyp * (1 - r^2).

void main() {
    vec2 p = (uv - 0.5) * 2.0;
    p.x *= iResolution.x / iResolution.y;

    // Ray origin in Klein model. Camera is at origin of Minkowski frame (0,0,0,1).
    // To move camera, we transform the RAY by the inverse camera matrix.
    // u_camera moves the "world" relative to camera?
    // Let's say u_camera transforms Camera Frame -> World Frame.
    // Ray in Camera Frame: Origin (0,0,0,1). Direction (p.x, p.y, -1.0, 0.0) -> Normalize to null vector.

    vec4 ro_mink = u_camera * vec4(0.0, 0.0, 0.0, 1.0);

    // Construct ray dir in camera frame (tangent space at origin).
    // For origin, tangent space is just x,y,z components.
    vec3 rd_cam_tangent = normalize(vec3(p, -1.0)); // Focal length 1.

    // Map tangent vector to world vector.
    // At origin, map is just the rotation part of u_camera?
    // Actually, u_camera acts on Minkowski vectors.
    // A vector (vx, vy, vz, 0) is in the tangent space of (0,0,0,1) ? No.
    // Tangent vectors at (0,0,0,1) are orthogonal to (0,0,0,1), so w=0. Correct.
    // So we can just treat rd as (rd.x, rd.y, rd.z, 0.0).

    vec4 rd_mink = u_camera * vec4(rd_cam_tangent, 0.0);

    // Now we have ro and rd in Minkowski World Frame.
    // Rays in Minkowski are lines: P(t) = ro + t * rd.
    // Wait, light rays are null geodesics.
    // Intersection with Hyperboloid? No.
    // We want to visualize the content OF the hyperboloid space.
    // The "content" exists at points Q where Q.Q = -1 (or +1).
    // The "ray" is the path of a photon.
    // In Minkowski space, the photon path is a line on the light cone.

    // To raymarch the "Hyperbolic Space":
    // We are looking AT objects inside the space.
    // We can just march along the line P(t) = ro + t * rd.
    // At each t, we project P(t) onto the hyperboloid?
    // NO. This is not how it works.

    // The "Klein Model" view IS the perspective view from the origin!
    // So if our camera is at origin, we just march in the unit ball.
    // If the camera moves, we transform the WORLD so the camera is at origin.
    // That's standard relativity.
    // So:
    // 1. Transform World Objects by Inverse(Camera).
    // 2. Camera is now at origin.
    // 3. Raymarch in Klein ball from origin.

    // So we don't need `u_camera` in the loop. We need to fold `p` by `Inverse(Camera)`.
    // Or just pass `InvCamera` as uniform.

    // Let's assume u_camera is the Inverse Camera Matrix (World -> Camera).
    // Wait, usually Camera Matrix is Camera -> World.
    // We want World -> Camera to put world in front of us.
    // Let's assume u_camera is passed as World -> Camera.

    vec3 ro = vec3(0.0);
    vec3 rd = normalize(vec3(p, -1.0));

    float t = 0.0;
    float tmax = 2.0; // Diameter of Klein disk is 2.0? No, radius 1.

    // Standard Euclidean Raymarch in Klein Ball
    // We march from 0 to boundary (dist 1).

    vec3 col = vec3(0.0);

    for(int i=0; i<64; i++) {
        vec3 pos = ro + rd * t;
        if (dot(pos, pos) >= 0.99) {
            // Hit boundary (Infinity)
            col = vec3(0.1, 0.0, 0.2); // Void color
            break;
        }

        // Transform pos to Minkowski
        vec4 P_mink = klein_to_mink(pos);

        // Transform P_mink by u_camera (World -> Camera? No, we are in Camera frame).
        // If u_camera is World->Camera, then P_cam = u_camera * P_world.
        // We want P_world = Inv(u_camera) * P_cam.
        // Let's pass "u_inv_camera" or just treat "u_camera" as the one that takes
        // local point and puts it in the fractal's frame.

        vec4 P_world = u_camera * P_mink;

        // Distance in world (hyperbolic)
        // Let's implement a simple shape in world frame.
        // A sphere at origin of world frame.

        float d_hyp = acosh(P_world.w) - 0.5;

        // Tiling Logic:
        // Fold P_world.xyz
        // Simple cubic reflections
        vec3 pw = P_world.xyz;
        for (int k=0; k<3; k++) {
             pw = abs(pw);
             pw -= 0.2; // Shift planes
             // Rotate
             if (pw.x < pw.y) pw.xy = pw.yx;
             if (pw.x < pw.z) pw.xz = pw.zx;
             if (pw.y < pw.z) pw.yz = pw.zy;
        }
        // Distance to "wall"
        // In H3, planes are linear in Minkowski?
        // Let's just use Euclidean approximation in the folded space
        float d_shape = length(pw) - 0.1;

        d_hyp = min(d_hyp, d_shape);

        // Convert d_hyp to d_klein approximation
        // d_klein approx d_hyp * (1 - r^2)
        float r2 = dot(pos, pos);
        float d_klein = d_hyp * (1.0 - r2) * 0.5;

        if (d_klein < 0.001) {
            // Hit
            // Lighting?
            // Normal in Klein space?
            // Simple coloring based on iteration or position
            col = vec3(1.0, 0.5, 0.2) * (1.0 - length(pos)); // Fade at distance

            // Add grid lines
            col += sin(P_world.x*20.0)*sin(P_world.y*20.0)*sin(P_world.z*20.0)*0.2;
            break;
        }

        t += d_klein;
    }

    gl_FragColor = vec4(col, 1.0);
}
"#;
