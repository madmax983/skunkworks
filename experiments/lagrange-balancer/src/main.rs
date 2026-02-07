use ::rand::Rng;
use macroquad::prelude::*;

const G: f32 = 1000.0;
const SOFTENING: f32 = 5.0; // Prevent singularities
const DT: f32 = 1.0 / 60.0;

struct Particle {
    pos: Vec2,
    vel: Vec2,
    color: Color,
}

struct Body {
    pos: Vec2,
    mass: f32,
    radius: f32,
    load: f32,
    capacity: f32,
}

struct Simulation {
    bodies: Vec<Body>,
    particles: Vec<Particle>,
    omega: f32,
    rotation: f32,
}

impl Simulation {
    fn new() -> Self {
        let mut bodies = Vec::new();
        // Initial setup: 3 servers in a triangle
        for i in 0..3 {
            let angle = i as f32 * 2.0 * std::f32::consts::PI / 3.0;
            let dist = 200.0;
            bodies.push(Body {
                pos: vec2(angle.cos() * dist, angle.sin() * dist),
                mass: 500.0,
                radius: 20.0,
                load: 0.0,
                capacity: 100.0,
            });
        }

        Self {
            bodies,
            particles: Vec::new(),
            omega: 0.5, // Initial rotation speed
            rotation: 0.0,
        }
    }

    fn update(&mut self) {
        let mut rng = ::rand::thread_rng();

        // Spawn particles at the center (Gateway)
        if self.particles.len() < 2000 {
            for _ in 0..5 {
                let angle = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
                let speed = rng.gen_range(50.0..150.0);
                self.particles.push(Particle {
                    pos: vec2(0.0, 0.0), // Start at center
                    vel: vec2(angle.cos() * speed, angle.sin() * speed),
                    color: Color::from_rgba(100, 200, 255, 200),
                });
            }
        }

        // Update Bodies (Load decay)
        for body in &mut self.bodies {
            body.load = (body.load - 0.5).max(0.0);
            // Effective mass decreases as load increases (Load Balancer Logic: loaded servers repel?)
            // Or maybe they just stop attracting?
            // Let's say Mass = BaseMass / (1 + Load/Capacity)
            // But for "Lagrange" effects, fixed mass is better. Let's keep mass fixed for now,
            // and maybe visualize load as color.
        }

        // Update Particles
        let mut dead_indices = Vec::new();
        for (i, p) in self.particles.iter_mut().enumerate() {
            // Forces
            let mut acc = Vec2::ZERO;

            // 1. Gravity from Bodies
            for body in &self.bodies {
                let delta = body.pos - p.pos;
                let dist_sq = delta.length_squared();
                let dist = dist_sq.sqrt();
                let dir = delta / dist;

                // F = G * M / r^2
                // Softened gravity
                let force_mag = G * body.mass / (dist_sq + SOFTENING);
                acc += dir * force_mag;
            }

            // 2. Centrifugal Force: Omega^2 * r
            acc += p.pos * self.omega * self.omega;

            // 3. Coriolis Force: 2 * Omega * v_perp
            // v_perp = (-vy, vx)
            // Check sign: if omega > 0 (CCW), force is to the right of velocity?
            // F_cor = -2m * Omega x v.
            // Omega = (0,0,w). v = (vx, vy, 0).
            // w k x (vx i + vy j) = w vx j - w vy i = (-w vy, w vx).
            // Force = -2m (-w vy, w vx) = (2m w vy, -2m w vx).
            // So v_perp should be (vy, -vx).
            let coriolis_acc = vec2(p.vel.y, -p.vel.x) * 2.0 * self.omega;
            acc += coriolis_acc;

            // Integration (Symplectic Euler-ish)
            p.vel += acc * DT;
            p.pos += p.vel * DT;

            // Bounds / Life
            if p.pos.length_squared() > 1000.0 * 1000.0 {
                dead_indices.push(i);
                continue;
            }

            // Collision with bodies
            for body in &mut self.bodies {
                if p.pos.distance(body.pos) < body.radius {
                    body.load += 1.0;
                    dead_indices.push(i);
                    break;
                }
            }
        }

        // Remove dead particles (iterate backwards)
        // Sort indices and dedup just in case
        dead_indices.sort_unstable();
        dead_indices.dedup();
        for &i in dead_indices.iter().rev() {
            if i < self.particles.len() {
                self.particles.swap_remove(i);
            }
        }

        self.rotation += self.omega * DT;
    }

    fn draw(&self) {
        clear_background(BLACK);

        // Draw effective potential field (Grid)
        // This is expensive, so maybe draw contour lines or just a few sample points?
        // Let's skip for now and focus on particles.

        // Draw Center (Gateway)
        draw_circle(screen_width() / 2.0, screen_height() / 2.0, 5.0, WHITE);

        // Camera transform (Center is 0,0)
        // We render relative to screen center
        let center_screen = vec2(screen_width() / 2.0, screen_height() / 2.0);

        // Draw Bodies
        for body in &self.bodies {
            // Transform pos to screen space (rotate by -rotation if we want to see the rotating frame stationary?)
            // No, let's view the rotating frame as stationary.
            // So body positions are fixed on screen (relative to center).
            let screen_pos = center_screen + body.pos;

            // Color based on load
            let load_ratio = (body.load / body.capacity).clamp(0.0, 1.0);
            let color = Color::from_rgba(
                ((1.0 - load_ratio) * 0.0 + load_ratio * 255.0) as u8,
                ((1.0 - load_ratio) * 255.0 + load_ratio * 0.0) as u8,
                100,
                255,
            );

            draw_circle(screen_pos.x, screen_pos.y, body.radius, color);
            draw_circle_lines(screen_pos.x, screen_pos.y, body.radius, 2.0, WHITE);
        }

        // Draw Particles
        for p in &self.particles {
            let screen_pos = center_screen + p.pos;
            draw_circle(screen_pos.x, screen_pos.y, 1.5, p.color);
        }

        // UI
        draw_text("Lagrange Balancer", 10.0, 20.0, 30.0, WHITE);
        draw_text(
            &format!("Requests: {}", self.particles.len()),
            10.0,
            50.0,
            20.0,
            GRAY,
        );
        draw_text(
            &format!("Omega: {:.2} (Left/Right to change)", self.omega),
            10.0,
            70.0,
            20.0,
            GRAY,
        );
        draw_text("Click to add server", 10.0, 90.0, 20.0, GRAY);
    }
}

#[macroquad::main("Lagrange Balancer")]
async fn main() {
    let mut sim = Simulation::new();

    loop {
        // Input
        if is_key_down(KeyCode::Right) {
            sim.omega += 0.01;
        }
        if is_key_down(KeyCode::Left) {
            sim.omega -= 0.01;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let mpos = mouse_position();
            let center_screen = vec2(screen_width() / 2.0, screen_height() / 2.0);
            let world_pos = vec2(mpos.0, mpos.1) - center_screen;
            sim.bodies.push(Body {
                pos: world_pos,
                mass: 500.0,
                radius: 20.0,
                load: 0.0,
                capacity: 100.0,
            });
        }

        sim.update();
        sim.draw();

        next_frame().await
    }
}
