use glam::Vec2;

#[derive(Clone, Debug)]
pub struct Particle {
    pub pos: Vec2,
    pub prev_pos: Vec2,
    pub acc: Vec2,
    pub char: char,
    pub mass: f32,
    pub locked: bool,
    pub color: ratatui::style::Color,
}

impl Particle {
    pub fn new(x: f32, y: f32, char: char) -> Self {
        let pos = Vec2::new(x, y);
        Self {
            pos,
            prev_pos: pos,
            acc: Vec2::ZERO,
            char,
            mass: 1.0,
            locked: false,
            color: ratatui::style::Color::White,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.locked {
            self.prev_pos = self.pos;
            self.acc = Vec2::ZERO;
            return;
        }

        let vel = self.pos - self.prev_pos;
        // Damping
        let vel = vel * 0.99;

        self.prev_pos = self.pos;
        self.pos += vel + self.acc * dt * dt;
        self.acc = Vec2::ZERO;
    }

    pub fn apply_force(&mut self, force: Vec2) {
        if !self.locked {
            self.acc += force / self.mass;
        }
    }
}

#[derive(Clone, Debug)]
pub struct Constraint {
    pub p1: usize,
    pub p2: usize,
    pub rest_length: f32,
    pub stiffness: f32,
}

pub struct World {
    pub particles: Vec<Particle>,
    pub constraints: Vec<Constraint>,
    pub width: f32,
    pub height: f32,
    pub gravity: Vec2,
}

impl World {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            particles: Vec::new(),
            constraints: Vec::new(),
            width,
            height,
            gravity: Vec2::new(0.0, 50.0), // Positive Y is down
        }
    }

    pub fn update(&mut self, dt: f32) {
        let sub_steps = 4;
        let sub_dt = dt / sub_steps as f32;

        for _ in 0..sub_steps {
            self.step(sub_dt);
        }
    }

    fn step(&mut self, dt: f32) {
        // Apply Gravity
        let gravity = self.gravity;
        for p in &mut self.particles {
            p.apply_force(gravity);
            p.update(dt);
        }

        // Solve Constraints
        for _ in 0..2 {
            self.solve_constraints();
            self.solve_collisions();
        }
    }

    fn solve_constraints(&mut self) {
        for c in &self.constraints {
            let p1 = self.particles[c.p1].clone();
            let p2 = self.particles[c.p2].clone();

            let delta = p2.pos - p1.pos;
            let dist = delta.length();

            if dist < 0.001 {
                continue;
            }

            let diff = (dist - c.rest_length) / dist;
            let offset = delta * diff * 0.5 * c.stiffness;

            if !self.particles[c.p1].locked {
                self.particles[c.p1].pos += offset;
            }
            if !self.particles[c.p2].locked {
                self.particles[c.p2].pos -= offset;
            }
        }
    }

    fn solve_collisions(&mut self) {
        for p in &mut self.particles {
            if p.pos.x < 0.0 {
                p.pos.x = 0.0;
                p.prev_pos.x = p.pos.x; // Stop velocity? Or bounce?
            }
            if p.pos.x > self.width {
                p.pos.x = self.width;
                p.prev_pos.x = p.pos.x;
            }
            if p.pos.y < 0.0 {
                p.pos.y = 0.0;
                p.prev_pos.y = p.pos.y;
            }
            if p.pos.y > self.height {
                p.pos.y = self.height;
                // Friction on ground
                let vel_x = (p.pos.x - p.prev_pos.x) * 0.9;
                p.prev_pos.x = p.pos.x - vel_x;
            }
        }

        // Particle-Particle Collisions (Simple radius check)
        // This is O(N^2), but N is small (< 100 particles usually)
        let count = self.particles.len();
        for i in 0..count {
            for j in (i + 1)..count {
                let pos_i = self.particles[i].pos;
                let pos_j = self.particles[j].pos;

                let delta = pos_j - pos_i;
                let dist_sq = delta.length_squared();
                let min_dist = 2.0; // Radius 1.0 each

                if dist_sq < min_dist * min_dist && dist_sq > 0.0001 {
                    let dist = dist_sq.sqrt();
                    let overlap = min_dist - dist;
                    let correction = (delta / dist) * overlap * 0.5;

                    if !self.particles[i].locked {
                        self.particles[i].pos -= correction;
                    }
                    if !self.particles[j].locked {
                        self.particles[j].pos += correction;
                    }
                }
            }
        }
    }

    pub fn add_word(&mut self, text: &str, pos: Vec2, vel: Vec2) {
        let start_idx = self.particles.len();
        let char_spacing = 2.0; // Width of a character

        // Create particles
        for (i, c) in text.chars().enumerate() {
            let mut p = Particle::new(pos.x + i as f32 * char_spacing, pos.y, c);
            p.prev_pos = p.pos - vel * 0.016; // Set initial velocity
            // Color based on index
            p.color = match i % 6 {
                0 => ratatui::style::Color::Red,
                1 => ratatui::style::Color::Yellow,
                2 => ratatui::style::Color::Green,
                3 => ratatui::style::Color::Cyan,
                4 => ratatui::style::Color::Blue,
                _ => ratatui::style::Color::Magenta,
            };
            self.particles.push(p);
        }

        // Connect them with constraints
        for i in 0..text.len() - 1 {
            self.constraints.push(Constraint {
                p1: start_idx + i,
                p2: start_idx + i + 1,
                rest_length: char_spacing,
                stiffness: 1.0,
            });

            // Cross-links for rigidity?
            if i + 2 < text.len() {
                self.constraints.push(Constraint {
                    p1: start_idx + i,
                    p2: start_idx + i + 2,
                    rest_length: char_spacing * 2.0,
                    stiffness: 0.5,
                });
            }
        }
    }

    pub fn clear(&mut self) {
        self.particles.clear();
        self.constraints.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_update() {
        let mut world = World::new(100.0, 100.0);
        world.add_word("TEST", Vec2::new(50.0, 50.0), Vec2::ZERO);
        assert_eq!(world.particles.len(), 4);

        // Run update
        world.update(0.1);

        // Particles should have moved due to gravity
        assert!(world.particles[0].pos.y > 50.0);
    }

    #[test]
    fn test_clear() {
        let mut world = World::new(100.0, 100.0);
        world.add_word("TEST", Vec2::ZERO, Vec2::ZERO);
        world.clear();
        assert_eq!(world.particles.len(), 0);
        assert_eq!(world.constraints.len(), 0);
    }
}
