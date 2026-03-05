use crate::lsystem::LSystem;
use crate::turtle::{Line, Turtle};
use rand::Rng;
use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Particle {
    pub x: f64,
    pub y: f64,
    pub vy: f64,
    pub alive: bool,
}

#[derive(Debug, Clone)]
pub struct Plant {
    pub x: f64,
    pub y: f64,
    pub lsystem: LSystem,
    pub iterations: u32,
    pub lines: Vec<Line>,
    pub step_size: f64,
    pub angle_incr: f64,
    pub color: (u8, u8, u8),
}

impl Plant {
    pub fn new(x: f64, y: f64, axiom: &str, rules: Vec<(char, &str)>) -> Self {
        let mut rng = rand::thread_rng();
        let color = (
            rng.gen_range(0..50),
            rng.gen_range(150..255),
            rng.gen_range(0..100),
        );
        let mut p = Self {
            x,
            y,
            lsystem: LSystem::new(axiom, rules),
            iterations: 1,
            lines: Vec::new(),
            step_size: 5.0,
            angle_incr: PI / 6.0, // 30 degrees
            color,
        };
        p.regenerate();
        p
    }

    pub fn grow(&mut self) {
        if self.iterations < 6 {
            self.iterations += 1;
            self.step_size *= 0.8; // Shrink as we grow to fit screen
            self.regenerate();
        }
    }

    pub fn regenerate(&mut self) {
        let instructions = self.lsystem.expand(self.iterations);
        // Start turtle at (x, y) facing DOWN (PI/2)
        // Note: Turtle coords are relative or absolute? Let's assume Turtle handles absolute coords.
        let mut turtle = Turtle::new(self.x, self.y, PI / 2.0, self.step_size, self.angle_incr);
        self.lines = turtle.interpret(&instructions);
    }
}

pub struct Garden {
    pub plants: Vec<Plant>,
    pub particles: Vec<Particle>,
    pub width: f64,
    pub height: f64,
    pub gravity: f64,
}

impl Garden {
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            plants: Vec::new(),
            particles: Vec::new(),
            width,
            height,
            gravity: 40.0,
        }
    }

    pub fn spawn_plant(&mut self, x: f64, axiom: &str, rules: Vec<(char, &str)>) {
        // If plant exists nearby, grow it instead of spawning new
        if let Some(p) = self.plants.iter_mut().find(|p| (p.x - x).abs() < 10.0) {
            p.grow();
        } else {
            self.plants.push(Plant::new(x, 0.0, axiom, rules));
        }
    }

    pub fn spawn_rain(&mut self, x: f64, count: usize) {
        let mut rng = rand::thread_rng();
        for _ in 0..count {
            let spread = rng.gen_range(-5.0..5.0);
            let px = (x + spread).clamp(0.0, self.width);
            self.particles.push(Particle {
                x: px,
                y: 0.0,
                vy: rng.gen_range(10.0..30.0), // Fast rain
                alive: true,
            });
        }
    }

    pub fn update(&mut self, dt: f64) {
        // Update particles physics
        for p in &mut self.particles {
            if !p.alive {
                continue;
            }
            p.vy += self.gravity * dt;
            p.y += p.vy * dt;

            if p.y > self.height {
                p.alive = false;
            }
        }

        // Collision: Particles vs Lines
        // For each particle, check if it hits any line of any plant.
        // O(P * N * L) - might be heavy. Optimization: spatial partitioning or limit checks.
        // Simple optimization: bounding box per plant.

        // We need to modify lines, so we can't iterate immutably.
        // We'll collect indices of lines to remove? Or modify in place using retain?
        // Since a particle dies upon hitting a line, and a line dies upon being hit,
        // we can process particles one by one against the mutable garden.

        let particles_len = self.particles.len();
        if particles_len == 0 {
            return;
        }

        for p_idx in 0..particles_len {
            let p = self.particles[p_idx];
            if !p.alive {
                continue;
            }

            let mut particle_killed = false;

            for plant in &mut self.plants {
                // Bounds check
                if (p.x - plant.x).abs() > 200.0 {
                    continue;
                } // Rough bound

                // Check lines
                // We want to remove the line if hit.
                // retain allows us to remove elements based on a predicate.
                plant.lines.retain(|line| {
                    if particle_killed {
                        return true;
                    } // Already hit something this frame?
                      // Actually, one particle kills one line.

                    // Simple AABB check for line
                    let min_x = line.x1.min(line.x2) - 1.0;
                    let max_x = line.x1.max(line.x2) + 1.0;
                    let min_y = line.y1.min(line.y2) - 1.0;
                    let max_y = line.y1.max(line.y2) + 1.0;

                    if p.x < min_x || p.x > max_x || p.y < min_y || p.y > max_y {
                        return true;
                    }

                    // Precise distance check
                    let dist = point_segment_distance(p.x, p.y, line.x1, line.y1, line.x2, line.y2);
                    if dist < 2.0 {
                        particle_killed = true;
                        return false; // Remove line
                    }
                    true
                });

                if particle_killed {
                    break;
                }
            }

            if particle_killed {
                self.particles[p_idx].alive = false;
            }
        }

        // Cleanup dead particles
        self.particles.retain(|p| p.alive);
        // Remove empty plants? Maybe keep them as stumps.
        // self.plants.retain(|p| !p.lines.is_empty());
    }
}

fn point_segment_distance(px: f64, py: f64, x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let l2 = (x2 - x1).powi(2) + (y2 - y1).powi(2);
    if l2 == 0.0 {
        return ((px - x1).powi(2) + (py - y1).powi(2)).sqrt();
    }

    let t = ((px - x1) * (x2 - x1) + (py - y1) * (y2 - y1)) / l2;
    let t = t.clamp(0.0, 1.0);

    let proj_x = x1 + t * (x2 - x1);
    let proj_y = y1 + t * (y2 - y1);

    ((px - proj_x).powi(2) + (py - proj_y).powi(2)).sqrt()
}
