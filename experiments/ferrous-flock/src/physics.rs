use locus::{
    flocking::{compute_force, FlockingParams},
    Vec2,
};

#[derive(Clone)]
pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
    pub polarity: f64, // +1.0 for North, -1.0 for South
}

impl Particle {
    pub fn new(x: f64, y: f64, polarity: f64) -> Self {
        Self {
            pos: Vec2::new(x, y),
            vel: Vec2::zero(),
            acc: Vec2::zero(),
            polarity,
        }
    }
}

pub struct Flock {
    pub particles: Vec<Particle>,
    pub width: f64,
    pub height: f64,
    pub flocking_params: FlockingParams,
    pub magnetic_strength: f64,
}

impl Flock {
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            particles: Vec::new(),
            width,
            height,
            flocking_params: FlockingParams {
                view_radius: 50.0,
                separation_radius: 15.0,
                max_speed: 3.0,
                max_force: 0.1,
                separation_weight: 1.5,
                alignment_weight: 1.0,
                cohesion_weight: 1.0,
            },
            magnetic_strength: 50.0,
        }
    }

    pub fn add_particle(&mut self, x: f64, y: f64, polarity: f64) {
        self.particles.push(Particle::new(x, y, polarity));
    }

    pub fn update(&mut self, dt: f64) {
        let positions: Vec<Vec2> = self.particles.iter().map(|p| p.pos).collect();
        let velocities: Vec<Vec2> = self.particles.iter().map(|p| p.vel).collect();

        // Compute forces first
        let mut forces = Vec::with_capacity(self.particles.len());

        for i in 0..self.particles.len() {
            // Flocking Force
            let mut force = compute_force(&positions, &velocities, i, &self.flocking_params);

            // Magnetic Force
            let p1 = &self.particles[i];
            let mut mag_force = Vec2::zero();

            for j in 0..self.particles.len() {
                if i == j {
                    continue;
                }

                let p2 = &self.particles[j];
                let diff = p1.pos - p2.pos;
                let dist_sq = diff.magnitude_squared();

                if dist_sq > 0.1 && dist_sq < 10000.0 {
                    // Cap max distance
                    let dist = dist_sq.sqrt();
                    let dir = diff / dist;
                    // Opposite polarities attract (force is negative relative to diff),
                    // same polarities repel (force is positive).
                    // p1.polarity * p2.polarity => +1 if same, -1 if opposite.
                    let strength = (p1.polarity * p2.polarity * self.magnetic_strength) / dist_sq;
                    mag_force += dir * strength;
                }
            }

            force += mag_force;
            forces.push(force);
        }

        // Apply Force and Integrate
        for (i, p) in self.particles.iter_mut().enumerate() {
            p.acc += forces[i];

            p.vel += p.acc * dt;

            // Limit speed
            let speed_sq = p.vel.magnitude_squared();
            if speed_sq > self.flocking_params.max_speed * self.flocking_params.max_speed {
                p.vel = (p.vel / speed_sq.sqrt()) * self.flocking_params.max_speed;
            }

            p.pos += p.vel * dt;
            p.acc = Vec2::zero();

            // Wrap around edges
            if p.pos.x < 0.0 {
                p.pos.x += self.width;
            } else if p.pos.x >= self.width {
                p.pos.x -= self.width;
            }

            if p.pos.y < 0.0 {
                p.pos.y += self.height;
            } else if p.pos.y >= self.height {
                p.pos.y -= self.height;
            }
        }
    }
}
