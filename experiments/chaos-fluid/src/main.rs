// Lineage:
// - From chaos-pendulum: Double pendulum chaotic motion logic (`step_pendulum`)
// - From ferrous-fluid: Magnetic `Platter` field simulation and `MagneticParticle` fluid mechanics
// - Novel Trait: Chaotic Swarm Manipulation where the pendulum acts as a strange attractor for the fluid

use ferrous_core::Platter;
use locus::Vec2;
use macroquad::prelude::*;
use ::rand::Rng;

const WIDTH: usize = 120;
const HEIGHT: usize = 120;
const PARTICLE_COUNT: usize = 2000;

#[derive(Clone, Copy)]
struct MagneticParticle {
    pos: Vec2,
    vel: Vec2,
}

struct ChaosFluid {
    platter: Platter,
    particles: Vec<MagneticParticle>,
    theta1: f32,
    theta2: f32,
    omega1: f32,
    omega2: f32,
    mass1: f32,
    mass2: f32,
    length1: f32,
    length2: f32,
    gravity: f32,
}

impl ChaosFluid {
    fn new() -> Self {
        let mut rng = ::rand::thread_rng();
        let mut particles = Vec::with_capacity(PARTICLE_COUNT);
        for _ in 0..PARTICLE_COUNT {
            particles.push(MagneticParticle {
                pos: Vec2::new(
                    rng.gen_range(0.0..WIDTH as f64),
                    rng.gen_range(0.0..HEIGHT as f64),
                ),
                vel: Vec2::new(0.0, 0.0),
            });
        }

        Self {
            platter: Platter::new(WIDTH, HEIGHT),
            particles,
            theta1: std::f32::consts::PI / 2.0,
            theta2: std::f32::consts::PI / 2.0,
            omega1: 0.0,
            omega2: 0.0,
            mass1: 10.0,
            mass2: 10.0,
            length1: (WIDTH as f32) / 4.0,
            length2: (WIDTH as f32) / 4.0,
            gravity: 1.0,
        }
    }

    fn step_pendulum(&mut self, dt: f32) {
        let m1 = self.mass1;
        let m2 = self.mass2;
        let l1 = self.length1;
        let l2 = self.length2;
        let t1 = self.theta1;
        let t2 = self.theta2;
        let w1 = self.omega1;
        let w2 = self.omega2;
        let g = self.gravity;

        let num1 = -g * (2.0 * m1 + m2) * t1.sin();
        let num2 = -m2 * g * (t1 - 2.0 * t2).sin();
        let num3 = -2.0 * (t1 - t2).sin() * m2;
        let num4 = w2 * w2 * l2 + w1 * w1 * l1 * (t1 - t2).cos();
        let den = l1 * (2.0 * m1 + m2 - m2 * (2.0 * t1 - 2.0 * t2).cos());
        let a1 = (num1 + num2 + num3 * num4) / den;

        let num1 = 2.0 * (t1 - t2).sin();
        let num2 = w1 * w1 * l1 * (m1 + m2);
        let num3 = g * (m1 + m2) * t1.cos();
        let num4 = w2 * w2 * l2 * m2 * (t1 - t2).cos();
        let den = l2 * (2.0 * m1 + m2 - m2 * (2.0 * t1 - 2.0 * t2).cos());
        let a2 = (num1 * (num2 + num3 + num4)) / den;

        self.omega1 += a1 * dt;
        self.omega2 += a2 * dt;
        self.theta1 += self.omega1 * dt;
        self.theta2 += self.omega2 * dt;

        // Damping
        self.omega1 *= 0.999;
        self.omega2 *= 0.999;
    }

    fn update(&mut self, dt: f32) {
        self.step_pendulum(dt);

        let cx = WIDTH as f32 / 2.0;
        let cy = HEIGHT as f32 / 2.0;

        let x1 = cx + self.length1 * self.theta1.sin();
        let y1 = cy + self.length1 * self.theta1.cos();

        let x2 = x1 + self.length2 * self.theta2.sin();
        let y2 = y1 + self.length2 * self.theta2.cos();

        self.platter.decay(0.95);

        // Pendulum tip deposits magnetic force
        if x2 >= 0.0 && x2 < WIDTH as f32 && y2 >= 0.0 && y2 < HEIGHT as f32 {
            let cx2 = x2 as usize;
            let cy2 = y2 as usize;
            self.platter.accumulate(cx2, cy2, 10.0);

            // Smear a bit
            for dx in -2..=2 {
                for dy in -2..=2 {
                    let nx = x2 as i32 + dx;
                    let ny = y2 as i32 + dy;
                    if nx >= 0 && nx < WIDTH as i32 && ny >= 0 && ny < HEIGHT as i32 {
                        let nxu = nx as usize;
                        let nyu = ny as usize;
                        self.platter.accumulate(nxu, nyu, 2.0);
                    }
                }
            }
        }

        // Particles update
        for particle in &mut self.particles {
            let px = particle.pos.x as i32;
            let py = particle.pos.y as i32;

            if px >= 0 && px < WIDTH as i32 && py >= 0 && py < HEIGHT as i32 {
                // Approximate gradient
                let pxu = px as usize;
                let pyu = py as usize;
                let mut gx = 0.0;
                let mut gy = 0.0;
                if pxu > 0 && pxu < WIDTH - 1 {
                    gx = self.platter.get(pxu + 1, pyu) - self.platter.get(pxu - 1, pyu);
                }
                if pyu > 0 && pyu < HEIGHT - 1 {
                    gy = self.platter.get(pxu, pyu + 1) - self.platter.get(pxu, pyu - 1);
                }

                particle.vel.x += gx * 100.0 * (dt as f64);
                particle.vel.y += gy * 100.0 * (dt as f64);
            }

            particle.pos.x += particle.vel.x * (dt as f64);
            particle.pos.y += particle.vel.y * (dt as f64);
            particle.vel.x *= 0.95; // Fluid friction
            particle.vel.y *= 0.95;

            // Bounds
            if particle.pos.x < 0.0 {
                particle.pos.x = 0.0;
                particle.vel.x *= -1.0;
            }
            if particle.pos.x >= (WIDTH as f64) {
                particle.pos.x = (WIDTH as f64) - 1.0;
                particle.vel.x *= -1.0;
            }
            if particle.pos.y < 0.0 {
                particle.pos.y = 0.0;
                particle.vel.y *= -1.0;
            }
            if particle.pos.y >= (HEIGHT as f64) {
                particle.pos.y = (HEIGHT as f64) - 1.0;
                particle.vel.y *= -1.0;
            }
        }
    }
}

fn draw(state: &ChaosFluid) {
    clear_background(BLACK);

    let scale_x = screen_width() / WIDTH as f32;
    let scale_y = screen_height() / HEIGHT as f32;

    // Draw field
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let val = state.platter.get(x, y);
            if val > 0.01 {
                let intensity = (val / 10.0).min(1.0) as f32;
                draw_rectangle(
                    x as f32 * scale_x,
                    y as f32 * scale_y,
                    scale_x,
                    scale_y,
                    Color::new(intensity * 0.2, intensity * 0.2, intensity * 0.8, 1.0),
                );
            }
        }
    }

    // Draw particles
    for p in &state.particles {
        draw_circle(
            p.pos.x as f32 * scale_x,
            p.pos.y as f32 * scale_y,
            scale_x.min(scale_y) * 0.5,
            Color::new(0.8, 0.8, 1.0, 0.8),
        );
    }

    // Draw pendulum
    let cx = WIDTH as f32 / 2.0;
    let cy = HEIGHT as f32 / 2.0;

    let x1 = cx + state.length1 * state.theta1.sin();
    let y1 = cy + state.length1 * state.theta1.cos();

    let x2 = x1 + state.length2 * state.theta2.sin();
    let y2 = y1 + state.length2 * state.theta2.cos();

    draw_line(
        cx * scale_x,
        cy * scale_y,
        x1 * scale_x,
        y1 * scale_y,
        2.0,
        Color::new(1.0, 0.0, 0.0, 0.5),
    );
    draw_line(
        x1 * scale_x,
        y1 * scale_y,
        x2 * scale_x,
        y2 * scale_y,
        2.0,
        Color::new(1.0, 0.0, 0.0, 0.5),
    );
    draw_circle(
        x2 * scale_x,
        y2 * scale_y,
        scale_x.min(scale_y) * 2.0,
        RED,
    );
}

#[macroquad::main("Chaos Fluid")]
async fn main() {
    let mut state = ChaosFluid::new();

    // Warm up the pendulum to get chaos going
    for _ in 0..100 {
        state.step_pendulum(0.016);
    }

    loop {
        let dt = get_frame_time().min(0.05); // cap dt to avoid explosions

        // multi-stepping the physics
        for _ in 0..4 {
            state.update(dt / 4.0);
        }

        draw(&state);

        next_frame().await
    }
}
