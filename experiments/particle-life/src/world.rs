use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Particle {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub species: usize,
}

pub struct Universe {
    pub width: f64,
    pub height: f64,
    pub particles: Vec<Particle>,
    pub rules: Vec<Vec<f64>>, // rules[i][j] is force factor of species j acting on species i
    pub r_max: f64,
    pub friction: f64,
    pub beta: f64, // repulsion radius ratio
}

impl Universe {
    pub fn new(width: f64, height: f64, n_particles: usize, n_species: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut particles = Vec::with_capacity(n_particles);
        for _ in 0..n_particles {
            particles.push(Particle {
                x: rng.gen_range(0.0..width),
                y: rng.gen_range(0.0..height),
                vx: 0.0,
                vy: 0.0,
                species: rng.gen_range(0..n_species),
            });
        }

        // Random attraction rules between -1.0 and 1.0
        let mut rules = vec![vec![0.0; n_species]; n_species];
        for row in rules.iter_mut() {
            for val in row.iter_mut() {
                *val = rng.gen_range(-1.0..=1.0);
            }
        }

        Self {
            width,
            height,
            particles,
            rules,
            r_max: 20.0, // Adjust based on terminal resolution density
            friction: 0.2, // Velocity decay
            beta: 0.3,
        }
    }

    pub fn update(&mut self, dt: f64) {
        let n = self.particles.len();
        // Clone positions for calculating forces to avoid borrowing conflicts
        // This is O(N) allocation but simplifies O(N^2) logic significantly
        let old_particles = self.particles.clone();

        let force_multiplier = 100.0; // Tune this for visual appeal

        for i in 0..n {
            let mut fx = 0.0;
            let mut fy = 0.0;
            let p1 = &old_particles[i];

            for (j, p2) in old_particles.iter().enumerate() {
                if i == j { continue; }

                let mut dx = p2.x - p1.x;
                let mut dy = p2.y - p1.y;

                // Toroidal wrap - Shortest path
                if dx > self.width * 0.5 { dx -= self.width; }
                if dx < -self.width * 0.5 { dx += self.width; }
                if dy > self.height * 0.5 { dy -= self.height; }
                if dy < -self.height * 0.5 { dy += self.height; }

                let dist_sq = dx * dx + dy * dy;

                // Optimization: Skip if too far
                if dist_sq >= self.r_max * self.r_max || dist_sq == 0.0 {
                    continue;
                }

                let dist = dist_sq.sqrt();
                let r = dist / self.r_max;

                let force = if r < self.beta {
                    // Repulsion: linearly increasing as we get closer
                    r / self.beta - 1.0
                } else if r < 1.0 {
                    // Attraction/Repulsion based on species rule
                    // Smooth tent function peaking between beta and 1.0
                     let numer = (2.0 * r - 1.0 - self.beta).abs();
                     let denom = 1.0 - self.beta;
                     self.rules[p1.species][p2.species] * (1.0 - numer / denom)
                } else {
                    0.0
                };

                // Direction vector (dx, dy) points from p1 to p2.
                // Positive force means attraction (move towards p2).
                // Negative force means repulsion (move away from p2).
                fx += (dx / dist) * force;
                fy += (dy / dist) * force;
            }

            // Update velocity
            let p = &mut self.particles[i];
            p.vx = (p.vx + fx * force_multiplier * dt) * (1.0 - self.friction);
            p.vy = (p.vy + fy * force_multiplier * dt) * (1.0 - self.friction);

            // Update position
            p.x += p.vx * dt;
            p.y += p.vy * dt;

            // Wrap position
            if p.x < 0.0 { p.x += self.width; }
            if p.x >= self.width { p.x -= self.width; }
            if p.y < 0.0 { p.y += self.height; }
            if p.y >= self.height { p.y -= self.height; }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_particle_update() {
        let mut u = Universe::new(100.0, 100.0, 2, 1);
        u.particles[0].x = 50.0;
        u.particles[0].y = 50.0;
        u.particles[1].x = 51.0; // Distance 1.0
        u.particles[1].y = 50.0;
        u.r_max = 10.0;
        u.beta = 0.3; // 1.0 distance is < 3.0 (0.3 * 10), so should repel.

        // 1.0 / 10.0 = 0.1 normalized distance (r).
        // r < beta (0.1 < 0.3).
        // Force = 0.1 / 0.3 - 1.0 = 0.333 - 1.0 = -0.666 (Repulsion).

        u.update(0.1);

        // p0 should be pushed LEFT (negative X velocity) away from p1
        assert!(u.particles[0].vx < 0.0, "Particle 0 should be pushed left, got vx={}", u.particles[0].vx);
        // p1 should be pushed RIGHT (positive X velocity) away from p0
        assert!(u.particles[1].vx > 0.0, "Particle 1 should be pushed right, got vx={}", u.particles[1].vx);
    }

    #[test]
    fn test_toroidal_wrapping() {
        let mut u = Universe::new(100.0, 100.0, 1, 1);
        u.particles[0].x = 101.0;
        u.update(0.0); // Should wrap
        assert!(u.particles[0].x < 100.0);

        u.particles[0].x = -1.0;
        u.update(0.0);
        assert!(u.particles[0].x > 0.0);
    }
}
