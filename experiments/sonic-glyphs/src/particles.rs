use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
    pub target: Vec2,
    pub mass: f32,
}

impl Particle {
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos,
            vel: Vec2::ZERO,
            acc: Vec2::ZERO,
            target: pos,
            mass: 1.0,
        }
    }
}

pub struct ParticleSystem {
    pub particles: Vec<Particle>,
    pub springs: Vec<(usize, usize)>, // Indices of connected particles
    pub stiffness: f32,
    pub damping: f32,
    pub rest_lengths: Vec<f32>,
}

impl ParticleSystem {
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
            springs: Vec::new(),
            stiffness: 150.0,
            damping: 0.9,
            rest_lengths: Vec::new(),
        }
    }

    pub fn add_contour(&mut self, contour: &[Vec2]) {
        if contour.is_empty() {
            return;
        }

        let start_idx = self.particles.len();
        for &p in contour {
            self.particles.push(Particle::new(p));
        }
        let end_idx = self.particles.len();

        // Connect springs in a loop
        for i in start_idx..end_idx {
            let next = if i == end_idx - 1 { start_idx } else { i + 1 };
            self.springs.push((i, next));
            let dist = self.particles[i].pos.distance(self.particles[next].pos);
            self.rest_lengths.push(dist);
        }
    }

    pub fn update(&mut self, dt: f32, audio_energy: f32, mouse_pos: Vec2) {
        // 1. Accumulate forces
        let mut forces = vec![Vec2::ZERO; self.particles.len()];

        // Spring to target (Home force)
        for (i, p) in self.particles.iter().enumerate() {
            let displacement = p.pos - p.target;
            // Strong pull back to home
            let home_force = -displacement * 5.0;
            forces[i] += home_force;
        }

        // Structural springs (Contour integrity)
        for (k, &(i, j)) in self.springs.iter().enumerate() {
            let p1 = self.particles[i].pos;
            let p2 = self.particles[j].pos;
            let dir = p2 - p1;
            let dist = dir.length();

            if dist > 0.001 {
                let stretch = dist - self.rest_lengths[k];
                let force = (dir / dist) * stretch * self.stiffness;
                forces[i] += force;
                forces[j] -= force;
            }
        }

        // Mouse repulsion / Interaction
        // Only if mouse is valid (not 0,0 usually means uninitialized in some frameworks, but here let's assume valid)
        let mouse_dist_sq = 10000.0; // 100 pixels radius squared
        for (i, p) in self.particles.iter().enumerate() {
            let dir = p.pos - mouse_pos;
            let dist_sq = dir.length_squared();
            if dist_sq < mouse_dist_sq && dist_sq > 0.1 {
                let dist = dist_sq.sqrt();
                let force_mag = (100.0 - dist) * 10.0;
                forces[i] += (dir / dist) * force_mag;
            }
        }

        // Audio "Shockwave" / Jitter
        // If energy is high, push particles away from their center or add noise
        if audio_energy > 0.1 {
            // Add random jitter
            for (i, _p) in self.particles.iter().enumerate() {
                let noise = vec2(rand::gen_range(-1.0, 1.0), rand::gen_range(-1.0, 1.0));
                forces[i] += noise * audio_energy * 200.0;
            }
        }

        // 2. Integration (Euler for simplicity)
        for (i, p) in self.particles.iter_mut().enumerate() {
            p.acc = forces[i] / p.mass;
            p.vel += p.acc * dt;
            p.vel *= self.damping; // Damping
            p.pos += p.vel * dt;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_particle_update() {
        let mut sys = ParticleSystem::new();
        let contour = vec![
            vec2(0.0, 0.0),
            vec2(10.0, 0.0),
            vec2(10.0, 10.0),
        ];
        sys.add_contour(&contour);

        assert_eq!(sys.particles.len(), 3);
        assert_eq!(sys.springs.len(), 3);

        // Run update
        let initial_pos = sys.particles[0].pos;
        // Apply some "audio energy" to force movement
        sys.update(0.016, 1.0, vec2(1000.0, 1000.0));

        // Check if position changed (random noise should move it)
        let new_pos = sys.particles[0].pos;
        assert!(new_pos != initial_pos);
    }
}
