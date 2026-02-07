use cgmath::{Point3, Vector3, Zero};
use rand::prelude::*;
use rayon::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Particle {
    pub pos: Point3<f32>,
    pub vel: Vector3<f32>,
    pub lattice_pos: Point3<f32>,
}

pub struct Simulation {
    pub particles: Vec<Particle>,
    pub grid_size: u32,
}

impl Simulation {
    pub fn new(grid_size: u32) -> Self {
        let mut particles = Vec::with_capacity((grid_size * grid_size * grid_size) as usize);
        let spacing = 1.5;
        let offset = (grid_size as f32 * spacing) / 2.0;

        for x in 0..grid_size {
            for y in 0..grid_size {
                for z in 0..grid_size {
                    let lx = (x as f32) * spacing - offset;
                    let ly = (y as f32) * spacing - offset;
                    let lz = (z as f32) * spacing - offset;
                    let pos = Point3::new(lx, ly, lz);

                    particles.push(Particle {
                        pos,
                        vel: Vector3::zero(),
                        lattice_pos: pos,
                    });
                }
            }
        }

        Self {
            particles,
            grid_size,
        }
    }

    pub fn update(&mut self, dt: f32, temperature: f32) {
        let spring_k = 10.0; // Spring constant
        let damping = 0.90; // Damping factor

        self.particles.par_iter_mut().for_each(|p| {
            let mut rng = rand::thread_rng();

            // Force towards lattice position
            let displacement = p.pos - p.lattice_pos;
            let spring_force = -spring_k * displacement;

            // Thermal noise (Random walk)
            // Magnitude proportional to temperature
            let noise_mag = temperature * 10.0;
            let random_force = Vector3::new(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0)
            ) * noise_mag;

            let force = spring_force + random_force;

            // Euler integration
            p.vel += force * dt;
            p.vel *= damping;
            p.pos += p.vel * dt;
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_particle_update() {
        let mut sim = Simulation::new(2);
        // Manually add a particle for testing if new() is empty (it's not anymore, but just to be sure)
        // But new(2) creates 8 particles.

        let initial_pos = sim.particles[0].pos;

        // Update with high temperature
        sim.update(0.1, 1000.0);

        // Assert position changed due to thermal noise
        // Since temperature is high and random force is random, it *should* move.
        // There is a tiny chance it sums to zero but unlikely with floats.
        assert_ne!(sim.particles[0].pos, initial_pos, "Particle should move due to temperature");
    }
}
