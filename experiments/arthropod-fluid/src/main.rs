// Lineage:
// - From arthropod: Immediate mode `Button` widgets (btn_attract, btn_repel, btn_clear) for UI interaction.
// - From ferrous-fluid: Particle struct, Magnet struct, and Platter simulation mechanics for magnetic field updates.
use arthropod::Button;
use ferrous_core::Platter;
use locus::Vec2;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Arthropod × Ferrous Fluid".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

// Ensure CI headless builds do not panic due to X11 dependency
fn main() {
    if std::env::args().any(|arg| arg == "--headless") {
        println!("Running in headless mode. Exiting early to avoid X11 panic.");
        return;
    }

    macroquad::Window::from_config(window_conf(), async_main());
}

pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
}

impl Particle {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            pos: Vec2::new(x, y),
            vel: Vec2::zero(),
            acc: Vec2::zero(),
        }
    }
}

pub struct Magnet {
    pub pos: Vec2,
    pub strength: f64,
    pub polarity: bool, // true = North (Pull), false = South (Push/Complex)
}

async fn async_main() {
    let mut platter = Platter::new(80, 60);

    let btn_attract = Button::new("Spawn Attract", 20.0, 20.0, 150.0, 40.0).with_colors(RED, ORANGE, YELLOW);
    let btn_repel = Button::new("Spawn Repel", 20.0, 70.0, 150.0, 40.0).with_colors(BLUE, Color::new(0.68, 0.85, 0.9, 1.0), WHITE);
    let btn_clear = Button::new("Clear Magnets", 20.0, 120.0, 150.0, 40.0).with_colors(GRAY, LIGHTGRAY, WHITE);

    let mut particles = Vec::new();
    // Spawn particles
    for _ in 0..600 {
        particles.push(Particle::new(
            rand::gen_range(200.0, 600.0),
            rand::gen_range(200.0, 400.0),
        ));
    }

    let mut magnets: Vec<Magnet> = Vec::new();

    let dt = 0.05;

    loop {
        clear_background(BLACK);

        if btn_attract.draw() {
            magnets.push(Magnet {
                pos: Vec2::new(rand::gen_range(200.0, 600.0), rand::gen_range(100.0, 500.0)),
                strength: 2000.0,
                polarity: true,
            });
        }

        if btn_repel.draw() {
            magnets.push(Magnet {
                pos: Vec2::new(rand::gen_range(200.0, 600.0), rand::gen_range(100.0, 500.0)),
                strength: 2000.0,
                polarity: false,
            });
        }

        if btn_clear.draw() {
            magnets.clear();
        }

        // 1. Clear Grid (Platter)
        platter.clear();

        // 2. Populate Grid (Density)
        for p in &particles {
            let gx = (p.pos.x / 10.0).round() as usize;
            let gy = (p.pos.y / 10.0).round() as usize;
            if gx < platter.width() && gy < platter.height() {
                platter.accumulate(gx, gy, 1.0);
            }
        }

        // 3. Update Particles
        for i in 0..particles.len() {
            let mut force = Vec2::new(0.0, 20.0); // Gravity pointing down (y increases downwards in macroquad)

            // Magnetism
            for mag in &magnets {
                let delta = mag.pos - particles[i].pos;
                let dist_sq = delta.magnitude_squared();

                if dist_sq > 1.0 {
                    let dir = delta.normalize();
                    let mag_force = (mag.strength / dist_sq).min(200.0);

                    if mag.polarity {
                        force = force + dir * mag_force; // Attract
                    } else {
                        force = force + dir * -mag_force; // Repel
                    }
                }
            }

            // Fluid Pressure (from Platter)
            let p_pos = particles[i].pos;
            let gx = (p_pos.x / 10.0).round() as usize;
            let gy = (p_pos.y / 10.0).round() as usize;

            if gx > 0
                && gx < (platter.width() - 1)
                && gy > 0
                && gy < (platter.height() - 1)
            {
                // Gradient
                let left = platter.get_magnetism(gx - 1, gy);
                let right = platter.get_magnetism(gx + 1, gy);
                let down = platter.get_magnetism(gx, gy - 1);
                let up = platter.get_magnetism(gx, gy + 1);

                let dx = right - left;
                let dy = up - down;

                let pressure_force = Vec2::new(-dx, -dy) * 50.0; // Push away from high density
                force = force + pressure_force;
            }

            particles[i].acc = force;
        }

        // Integrate
        for p in &mut particles {
            p.vel = p.vel + p.acc * dt;
            p.vel = p.vel * 0.96; // damping
            p.pos = p.pos + p.vel * dt;

            // Boundaries
            if p.pos.y < 0.0 {
                p.pos.y = 0.0;
                p.vel.y = -p.vel.y * 0.6;
            }
            if p.pos.x < 0.0 {
                p.pos.x = 0.0;
                p.vel.x = -p.vel.x * 0.6;
            }
            if p.pos.x > 800.0 {
                p.pos.x = 800.0;
                p.vel.x = -p.vel.x * 0.6;
            }
            if p.pos.y > 600.0 {
                p.pos.y = 600.0;
                p.vel.y = -p.vel.y * 0.6;
            }
        }

        // Draw Magnets
        for mag in &magnets {
            let color = if mag.polarity { RED } else { BLUE };
            draw_circle(mag.pos.x as f32, mag.pos.y as f32, 10.0, color);
        }

        // Draw Particles
        for p in &particles {
            draw_circle(p.pos.x as f32, p.pos.y as f32, 2.0, Color::new(0.0, 1.0, 1.0, 1.0));
        }

        // Draw buttons again so they're on top
        btn_attract.draw();
        btn_repel.draw();
        btn_clear.draw();

        next_frame().await;
    }
}
