struct Particle {
    pos: vec2<f32>,
    vel: vec2<f32>,
    life: f32,
    char_idx: u32,
    pad: vec2<f32>,
}

struct SimParams {
    dt: f32,
    width: u32,
    height: u32,
}

@group(0) @binding(0) var<uniform> params: SimParams;

@group(1) @binding(0) var<storage, read_write> particles: array<Particle>;
@group(1) @binding(1) var vel_tex: texture_2d<f32>;
@group(1) @binding(2) var linear_sampler: sampler;

@compute @workgroup_size(64)
fn update_particles(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;
    if (index >= arrayLength(&particles)) { return; }

    var p = particles[index];

    // Sample velocity at current position
    // Pos is normalized 0..1
    let vel = textureSampleLevel(vel_tex, linear_sampler, p.pos, 0.0).xy;

    // Update pos
    // Scale velocity? Fluid velocity is usually in grid units/sec?
    // If fluid vel is 0..1 per second, then direct add is fine.
    // If fluid vel is pixels/sec, we need to scale by 1/width.
    // Let's assume fluid vel is normalized (0..1 per second).

    p.pos += vel * params.dt * 0.5; // Slow down a bit if needed

    // Wrap around
    if (p.pos.x > 1.0) { p.pos.x -= 1.0; }
    if (p.pos.x < 0.0) { p.pos.x += 1.0; }
    if (p.pos.y > 1.0) { p.pos.y -= 1.0; }
    if (p.pos.y < 0.0) { p.pos.y += 1.0; }

    // Decay life
    p.life -= params.dt * 0.1;
    if (p.life < 0.0) {
        p.life = 1.0;
        // Respawn random?
        // Randomness in shader is hard. Just wrap.
    }

    particles[index] = p;
}
