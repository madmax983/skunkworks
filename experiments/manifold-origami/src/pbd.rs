use macroquad::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Particle {
    pub pos: Vec3,
    pub prev_pos: Vec3,
    pub inv_mass: f32,
    pub vel: Vec3,
}

#[derive(Debug, Clone, Copy)]
pub enum Constraint {
    Distance {
        p1: usize,
        p2: usize,
        rest_length: f32,
        stiffness: f32,
    },
    // Isometric bending: controls the distance between opposite vertices of two triangles sharing an edge.
    Bending {
        p1: usize,
        p2: usize,
        flat_length: f32,
        target_length: f32,
        stiffness: f32,
    },
    Pin {
        p: usize,
        pos: Vec3,
    },
}

#[derive(Clone)]
pub struct PbdSystem {
    pub particles: Vec<Particle>,
    pub constraints: Vec<Constraint>,
}

impl PbdSystem {
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
            constraints: Vec::new(),
        }
    }

    pub fn add_particle(&mut self, pos: Vec3, mass: f32) -> usize {
        let idx = self.particles.len();
        self.particles.push(Particle {
            pos,
            prev_pos: pos,
            inv_mass: if mass == 0.0 { 0.0 } else { 1.0 / mass },
            vel: Vec3::ZERO,
        });
        idx
    }

    pub fn add_distance_constraint(&mut self, p1: usize, p2: usize, stiff: f32) {
        let dist = self.particles[p1].pos.distance(self.particles[p2].pos);
        self.constraints.push(Constraint::Distance {
            p1,
            p2,
            rest_length: dist,
            stiffness: stiff,
        });
    }

    pub fn add_bending_constraint(&mut self, p1: usize, p2: usize, stiff: f32) -> usize {
        let dist = self.particles[p1].pos.distance(self.particles[p2].pos);
        let idx = self.constraints.len();
        self.constraints.push(Constraint::Bending {
            p1,
            p2,
            flat_length: dist,
            target_length: dist,
            stiffness: stiff,
        });
        idx
    }

    pub fn add_pin_constraint(&mut self, p: usize, pos: Vec3) {
        self.constraints.push(Constraint::Pin { p, pos });
    }

    pub fn step(&mut self, dt: f32, iterations: usize) {
        // Integrate
        for p in &mut self.particles {
            if p.inv_mass == 0.0 {
                continue;
            }
            p.vel += vec3(0.0, 0.0, 0.0) * dt;
            p.prev_pos = p.pos;
            p.pos += p.vel * dt;
        }

        // Constraints
        for _ in 0..iterations {
            for i in 0..self.constraints.len() {
                self.solve_constraint(i);
            }
        }

        // Update Velocity
        for p in &mut self.particles {
            if p.inv_mass == 0.0 {
                continue;
            }
            p.vel = (p.pos - p.prev_pos) / dt;
            p.vel *= 0.95; // Damping
        }
    }

    fn solve_constraint(&mut self, idx: usize) {
        let constraint = self.constraints[idx];
        match constraint {
            Constraint::Distance {
                p1,
                p2,
                rest_length,
                stiffness,
            } => {
                self.solve_distance(p1, p2, rest_length, stiffness);
            }
            Constraint::Bending {
                p1,
                p2,
                target_length,
                stiffness,
                ..
            } => {
                self.solve_distance(p1, p2, target_length, stiffness);
            }
            Constraint::Pin { p, pos } => {
                self.particles[p].pos = pos;
            }
        }
    }

    fn solve_distance(&mut self, p1: usize, p2: usize, target_len: f32, stiffness: f32) {
        let pos1 = self.particles[p1].pos;
        let pos2 = self.particles[p2].pos;
        let w1 = self.particles[p1].inv_mass;
        let w2 = self.particles[p2].inv_mass;
        if w1 + w2 == 0.0 {
            return;
        }

        let delta = pos1 - pos2;
        let len = delta.length();
        if len == 0.0 {
            return;
        }

        let diff = (len - target_len) / len;
        let correction = delta * diff * stiffness / (w1 + w2);

        if w1 > 0.0 {
            self.particles[p1].pos -= correction * w1;
        }
        if w2 > 0.0 {
            self.particles[p2].pos += correction * w2;
        }
    }
}
