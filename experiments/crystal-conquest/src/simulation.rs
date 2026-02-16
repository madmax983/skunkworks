use cgmath::{InnerSpace, Vector3};
use bytemuck::{Pod, Zeroable};
use rand::Rng;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Particle {
    pub position: [f32; 3],
    pub _pad1: f32, // Padding for alignment (vec3 is 12 bytes, need 16 for GPU)
    pub velocity: [f32; 3],
    pub _pad2: f32,
    pub color: [f32; 3],
    pub faction: u32,
}

impl Particle {
    pub fn new(position: Vector3<f32>, velocity: Vector3<f32>, faction: u32) -> Self {
        Self {
            position: position.into(),
            _pad1: 0.0,
            velocity: velocity.into(),
            _pad2: 0.0,
            color: [1.0, 1.0, 1.0], // Default white
            faction,
        }
    }
}

pub struct Simulation {
    pub particles: Vec<Particle>,
}

impl Simulation {
    pub fn new(count: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut particles = Vec::with_capacity(count);

        for i in 0..count {
            // Random point on sphere
            let z: f32 = rng.gen_range(-1.0..1.0);
            let theta: f32 = rng.gen_range(0.0..std::f32::consts::TAU);
            let r = (1.0 - z * z).sqrt();
            let x = r * theta.cos();
            let y = r * theta.sin();

            let position = Vector3::new(x, y, z).normalize();

            // Tangent velocity
            let random_vec = Vector3::new(rng.gen(), rng.gen(), rng.gen());
            let mut velocity = random_vec.cross(position);
            if velocity.magnitude2() < 1e-6 {
                velocity = Vector3::unit_x().cross(position);
            }
            velocity = velocity.normalize() * 0.1; // Initial speed

            particles.push(Particle::new(position, velocity, (i % 3) as u32));
        }

        Self { particles }
    }

    pub fn update(&mut self, dt: f32) {
        for p in &mut self.particles {
            let mut pos: Vector3<f32> = p.position.into();
            let mut vel: Vector3<f32> = p.velocity.into();

            // Simple movement
            pos += vel * dt;
            pos = pos.normalize();

            // Project velocity to be tangent
            let normal = pos;
            vel = vel - normal * vel.dot(normal);

            p.position = pos.into();
            p.velocity = vel.into();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cgmath::InnerSpace;

    #[test]
    fn test_particle_initialization() {
        let p = Particle::new([1.0, 0.0, 0.0].into(), [0.0, 1.0, 0.0].into(), 0);
        let pos: Vector3<f32> = p.position.into();
        let vel: Vector3<f32> = p.velocity.into();

        assert!((pos.magnitude() - 1.0).abs() < 1e-6);
        assert!((vel.magnitude() - 1.0).abs() < 1e-6);
        assert_eq!(p.faction, 0);
    }

    #[test]
    fn test_simulation_step() {
        let mut sim = Simulation::new(10);
        assert_eq!(sim.particles.len(), 10);

        // Initial check: all particles on sphere
        for p in &sim.particles {
            let pos: Vector3<f32> = p.position.into();
            assert!((pos.magnitude() - 1.0).abs() < 1e-6);
        }

        sim.update(0.016); // One frame

        // Check after update: all particles still on sphere
        for p in &sim.particles {
            let pos: Vector3<f32> = p.position.into();
            assert!((pos.magnitude() - 1.0).abs() < 1e-6);
        }
    }
}
