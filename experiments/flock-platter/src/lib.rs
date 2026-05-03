use flocking::{compute_force, FlockingParams};
use locus::Vec2;
use platter::Platter;

/// A hybrid swarm entity that leaves a trail on a Platter.
pub struct SwarmPlatter {
    pub positions: Vec<Vec2>,
    pub velocities: Vec<Vec2>,
    pub platter: Platter,
    pub params: FlockingParams,
    pub width: f64,
    pub height: f64,
    pub trail_strength: f64,
    pub decay_rate: f64,
}

impl SwarmPlatter {
    pub fn new(
        num_boids: usize,
        width: f64,
        height: f64,
        params: FlockingParams,
    ) -> Self {
        let mut positions = Vec::with_capacity(num_boids);
        let mut velocities = Vec::with_capacity(num_boids);

        // Simple initialization
        for i in 0..num_boids {
            positions.push(Vec2::new(
                (i as f64 * 10.0) % width,
                (i as f64 * 5.0) % height,
            ));
            velocities.push(Vec2::new(1.0, 0.0)); // Initial push
        }

        Self {
            positions,
            velocities,
            platter: Platter::new(width as usize, height as usize),
            params,
            width,
            height,
            trail_strength: 0.1,
            decay_rate: 0.95,
        }
    }

    pub fn tick(&mut self) {
        // 1. Compute forces and update velocities
        let mut new_velocities = self.velocities.clone();
        for i in 0..self.positions.len() {
            let force = compute_force(&self.positions, &self.velocities, i, &self.params);
            new_velocities[i] += force;
            new_velocities[i] = new_velocities[i].limit(self.params.max_speed);
        }
        self.velocities = new_velocities;

        // 2. Update positions and wrap around
        for i in 0..self.positions.len() {
            self.positions[i] += self.velocities[i];

            // Wrap around edges
            if self.positions[i].x < 0.0 {
                self.positions[i].x += self.width;
            } else if self.positions[i].x >= self.width {
                self.positions[i].x -= self.width;
            }
            if self.positions[i].y < 0.0 {
                self.positions[i].y += self.height;
            } else if self.positions[i].y >= self.height {
                self.positions[i].y -= self.height;
            }

            // 3. Saturate platter at new position
            let px = self.positions[i].x as usize;
            let py = self.positions[i].y as usize;
            if px < self.platter.width() && py < self.platter.height() {
                self.platter.saturate(px, py, self.trail_strength);
            }
        }

        // 4. Decay the platter
        self.platter.decay(self.decay_rate);
    }
}
