// 🧬 Lineage Notes:
// Inherits the `Platter` density map and Newtonian particle integration from `ferrous-fluid`.
// The magnetic poles are no longer manually placed objects, but are dynamically synced
// from the positions of active Bids and Asks inherited from `market-sim`.

use ferrous_core::Platter;
use locus::Vec2;
use market_sim::{Grid, Particle as MarketParticle};
use rand::Rng;

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
    pub polarity: bool, // true = North (Bid), false = South (Ask)
}

pub struct Universe {
    pub particles: Vec<Particle>,
    pub magnets: Vec<Magnet>,
    pub width: f64,
    pub height: f64,
    pub platter: Platter,
}

impl Universe {
    pub fn new(width: f64, height: f64) -> Self {
        let mut particles = Vec::new();
        let mut rng = rand::thread_rng();

        for _ in 0..800 {
            particles.push(Particle::new(
                rng.gen_range(width * 0.1..width * 0.9),
                rng.gen_range(height * 0.1..height * 0.9),
            ));
        }

        let grid_w = width as usize + 1;
        let grid_h = height as usize + 1;

        Self {
            particles,
            magnets: Vec::new(),
            width,
            height,
            platter: Platter::new(grid_w, grid_h),
        }
    }

    pub fn sync_magnets_with_market(&mut self, market: &Grid) {
        self.magnets.clear();
        let w = market.width as f64;
        let h = market.height as f64;

        let scale_x = self.width / w;
        let scale_y = self.height / h;

        for y in 0..market.height {
            for x in 0..market.width {
                let cell = market.get(x, y);
                match cell {
                    MarketParticle::Bid(_) => {
                        self.magnets.push(Magnet {
                            pos: Vec2::new(x as f64 * scale_x, y as f64 * scale_y),
                            strength: 500.0,
                            polarity: true,
                        });
                    }
                    MarketParticle::Ask(_) => {
                        self.magnets.push(Magnet {
                            pos: Vec2::new(x as f64 * scale_x, y as f64 * scale_y),
                            strength: 500.0,
                            polarity: false,
                        });
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn update(&mut self, dt: f64) {
        let damping = 0.94;

        // 1. Clear Grid (Platter)
        self.platter.clear();

        // 2. Populate Grid (Density)
        for p in &self.particles {
            let gx = p.pos.x.round() as usize;
            let gy = p.pos.y.round() as usize;
            self.platter.accumulate(gx, gy, 1.0);
        }

        // 3. Update Particles
        for i in 0..self.particles.len() {
            let mut force = Vec2::zero();

            // Magnetism
            for mag in &self.magnets {
                let delta = mag.pos - self.particles[i].pos;
                let dist_sq = delta.magnitude_squared();

                if dist_sq > 1.0 {
                    let dir = delta.normalize();
                    let mag_force = (mag.strength / dist_sq).min(150.0);

                    if mag.polarity {
                        force += dir * mag_force; // Attract to Bids
                    } else {
                        force += dir * -mag_force; // Repel from Asks
                    }
                }
            }

            // Fluid Pressure (from Platter)
            let p_pos = self.particles[i].pos;
            let gx = p_pos.x.round() as usize;
            let gy = p_pos.y.round() as usize;

            if gx > 0
                && gx < (self.platter.width() - 1)
                && gy > 0
                && gy < (self.platter.height() - 1)
            {
                let left = self.platter.get_magnetism(gx - 1, gy);
                let right = self.platter.get_magnetism(gx + 1, gy);
                let down = self.platter.get_magnetism(gx, gy - 1);
                let up = self.platter.get_magnetism(gx, gy + 1);

                let dx = right - left;
                let dy = up - down;

                let pressure_force = Vec2::new(-dx, -dy) * 40.0;
                force += pressure_force;
            }

            self.particles[i].acc = force;
        }

        // Integrate
        for p in &mut self.particles {
            p.vel += p.acc * dt;
            p.vel *= damping;
            p.pos += p.vel * dt;

            // Boundaries
            if p.pos.y < 0.0 {
                p.pos.y = 0.0;
                p.vel.y = -p.vel.y * 0.6;
            }
            if p.pos.x < 0.0 {
                p.pos.x = 0.0;
                p.vel.x = -p.vel.x * 0.6;
            }
            if p.pos.x > self.width {
                p.pos.x = self.width;
                p.vel.x = -p.vel.x * 0.6;
            }
            if p.pos.y > self.height {
                p.pos.y = self.height;
                p.vel.y = -p.vel.y * 0.6;
            }
        }
    }
}
