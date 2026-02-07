use cgmath::{Point3, Vector3, Zero, InnerSpace, MetricSpace};
use rand::prelude::*;
use rayon::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
    Solid,
    Liquid,
}

#[derive(Debug, Clone, Copy)]
pub struct Particle {
    pub pos: Point3<f32>,
    pub vel: Vector3<f32>,
    pub lattice_pos: Point3<f32>,
    pub temperature: f32,
    pub state: State,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AgentType {
    Cooler, // Freezes things (Blue)
    Heater, // Melts things (Red)
}

#[derive(Debug, Clone, Copy)]
pub struct Agent {
    pub pos: Point3<f32>,
    pub vel: Vector3<f32>,
    pub kind: AgentType,
}

pub struct Simulation {
    pub particles: Vec<Particle>,
    pub agents: Vec<Agent>,
    pub grid_size: u32,
    pub spacing: f32,
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
                        temperature: 0.05, // Start cold
                        state: State::Solid,
                    });
                }
            }
        }

        let mut agents = Vec::new();
        let mut rng = rand::thread_rng();
        let num_agents = 500;

        for _ in 0..num_agents {
             let x = rng.gen_range(-offset..offset);
             let y = rng.gen_range(-offset..offset);
             let z = rng.gen_range(-offset..offset);

             let kind = if rng.gen_bool(0.5) { AgentType::Cooler } else { AgentType::Heater };

             agents.push(Agent {
                 pos: Point3::new(x, y, z),
                 vel: Vector3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)).normalize() * 5.0,
                 kind,
             });
        }

        Self {
            particles,
            agents,
            grid_size,
            spacing,
        }
    }

    pub fn update(&mut self, dt: f32, global_temp_mod: f32) {
        let spring_k = 10.0; // Spring constant for solids
        let liquid_k = 0.5;  // Weak spring for liquids (viscosity/tension)
        let damping = 0.90;

        // Update Agents first
        for agent in &mut self.agents {
             // Random walk / change direction occasionally
             let mut rng = rand::thread_rng();
             if rng.gen_bool(0.05) {
                 agent.vel = Vector3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)).normalize() * 5.0;
             }

             // Bounce off bounds (rough approximation)
             let limit = (self.grid_size as f32 * self.spacing) / 2.0;
             if agent.pos.x.abs() > limit { agent.vel.x *= -1.0; }
             if agent.pos.y.abs() > limit { agent.vel.y *= -1.0; }
             if agent.pos.z.abs() > limit { agent.vel.z *= -1.0; }

             agent.pos += agent.vel * dt;
        }

        // Parallel update for particles
        let agents_copy = self.agents.clone();
        let spacing = self.spacing;

        self.particles.par_iter_mut().for_each(|p| {
            let mut rng = rand::thread_rng();

            // Check for nearby agents
            // Optimization: This is O(N*M), naive but might pass for M=500.
            for agent in &agents_copy {
                 if p.pos.distance2(agent.pos) < (spacing * spacing * 4.0) {
                      match agent.kind {
                          AgentType::Heater => p.temperature += 5.0 * dt,
                          AgentType::Cooler => p.temperature -= 5.0 * dt,
                      }
                 }
            }

            // Natural cooling/warming towards global ambient
            let ambient = global_temp_mod;
            p.temperature += (ambient - p.temperature) * dt * 0.5;
            p.temperature = p.temperature.max(0.0);

            // Phase transition
            if p.temperature > 1.0 {
                p.state = State::Liquid;
            } else {
                p.state = State::Solid;
            }

            // Dynamics based on state
            let k = match p.state {
                State::Solid => spring_k,
                State::Liquid => liquid_k,
            };

            // Force towards lattice position
            let displacement = p.pos - p.lattice_pos;
            let spring_force = -k * displacement;

            // Thermal noise (Random walk)
            let noise_mag = p.temperature * 5.0;
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
        let initial_pos = sim.particles[0].pos;

        // Update with high temperature
        sim.update(0.1, 1000.0);

        // Assert position changed due to thermal noise
        assert_ne!(sim.particles[0].pos, initial_pos, "Particle should move due to temperature");
    }
}
