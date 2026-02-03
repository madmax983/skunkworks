use rand::Rng;

pub struct Particle {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub char: char,
    pub settled: bool,
}

pub struct World {
    pub particles: Vec<Particle>,
    pub width: f64,
    pub height: f64,
    pub pile_heights: Vec<f64>, // Height of pile at each integer X
    pub gravity: f64,
}

impl World {
    pub fn new(width: f64, height: f64) -> Self {
        // Initialize pile heights based on width
        let pile_len = width.ceil() as usize + 1;
        Self {
            particles: Vec::new(),
            width,
            height,
            pile_heights: vec![0.0; pile_len],
            gravity: 50.0, // pixels per second squared
        }
    }

    pub fn spawn_particles(&mut self, chars: &[char]) {
        let mut rng = rand::thread_rng();
        for &ch in chars {
            self.particles.push(Particle {
                x: rng.gen_range(0.0..self.width),
                y: rng.gen_range(-50.0..0.0), // Start above screen
                vx: rng.gen_range(-2.0..2.0), // Slight drift
                vy: rng.gen_range(5.0..15.0), // Initial downward velocity
                char: ch,
                settled: false,
            });
        }
    }

    pub fn update(&mut self, dt: f64) {
        for p in &mut self.particles {
            if p.settled {
                continue;
            }

            // Apply gravity
            p.vy += self.gravity * dt;

            // Apply velocity
            p.x += p.vx * dt;
            p.y += p.vy * dt;

            // Wall collisions (bounce off sides)
            if p.x < 0.0 {
                p.x = 0.0;
                p.vx = -p.vx * 0.5;
            } else if p.x >= self.width {
                p.x = self.width - 0.1;
                p.vx = -p.vx * 0.5;
            }

            // Floor / Pile collision
            // Calculate floor height at this X
            let idx = (p.x as usize).min(self.pile_heights.len() - 1);
            let current_pile = self.pile_heights[idx];
            let floor_y = self.height - current_pile;

            if p.y >= floor_y {
                // Hit the pile
                p.y = floor_y;
                p.settled = true;

                // Add to pile
                // Assume each char adds 1.0 height? Or less?
                // Let's say 0.5 for compactness
                self.pile_heights[idx] += 0.8;
            }
        }
    }

    #[allow(dead_code)]
    pub fn reset_pile(&mut self) {
        self.pile_heights.fill(0.0);
        self.particles.retain(|p| !p.settled);
    }

    pub fn shake(&mut self) {
        let mut rng = rand::thread_rng();
        for p in &mut self.particles {
            if p.settled {
                p.settled = false;
                p.vy = -rng.gen_range(10.0..30.0); // Jump up
                p.vx = rng.gen_range(-5.0..5.0);
            }
        }
        // Reset pile heights as they are airborne now
        self.pile_heights.fill(0.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gravity() {
        let mut world = World::new(100.0, 100.0);
        world.particles.push(Particle {
            x: 50.0,
            y: 0.0,
            vx: 0.0,
            vy: 0.0,
            char: 'A',
            settled: false,
        });

        world.update(0.1);

        let p = &world.particles[0];
        assert!(p.vy > 0.0, "Gravity should increase velocity");
        assert!(p.y > 0.0, "Particle should move down");
    }

    #[test]
    fn test_pile_collision() {
        let mut world = World::new(100.0, 100.0);
        // Add a particle near the floor
        world.particles.push(Particle {
            x: 50.0,
            y: 99.0,
            vx: 0.0,
            vy: 100.0, // Moving fast
            char: 'A',
            settled: false,
        });

        // Floor is at 100.0 (height - 0.0)
        world.update(0.1);

        let p = &world.particles[0];
        assert!(p.settled, "Particle should settle");
        assert_eq!(p.y, 100.0, "Particle should be at floor");
        assert!(world.pile_heights[50] > 0.0, "Pile height should increase");
    }
}
