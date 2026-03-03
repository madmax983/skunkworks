use crate::agent::Agent;
use crate::physics::Vec2;
use ferrous_core::Platter;

pub struct Universe {
    pub agents: Vec<Agent>,
    pub platter: Platter,
}

impl Universe {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            agents: Vec::new(),
            platter: Platter::new(width, height),
        }
    }

    pub fn add_agent(&mut self, agent: Agent) {
        self.agents.push(agent);
    }

    pub fn step(&mut self, dt: f64) {
        let mag_write = 5.0;
        let decay_rate = 0.99;

        for agent in &mut self.agents {
            // 1. Sense
            // Map physics pos to grid pos
            let gx = (agent.body.pos.x + 100.0) as i32;
            let gy = (agent.body.pos.y + 100.0) as i32;

            let mut mag_val = 0.0;
            if gx >= 0
                && gx < self.platter.width() as i32
                && gy >= 0
                && gy < self.platter.height() as i32
            {
                mag_val = self.platter.get_magnetism(gx as usize, gy as usize);
            }

            agent.sense(mag_val);

            // 2. Think
            // Run a few steps of VM per physics tick
            if !agent.vm.halted {
                for _ in 0..5 {
                    agent.vm.step();
                    if agent.vm.halted {
                        break;
                    }
                }
            } else {
                // Re-energize if halted? Or respawn logic?
                // For now, let them die (stop moving).
            }

            // 3. Act
            let (force, emit) = agent.act();

            // Apply force
            agent.body.acc += force * 50.0; // Scale up force to move mass

            // Physics
            agent.body.vel += agent.body.acc * dt;
            agent.body.vel *= 0.90; // Drag
            agent.body.pos += agent.body.vel * dt;
            agent.body.acc = Vec2::zero(); // Reset acc

            // Bounds check / Wrap around
            if agent.body.pos.x < -100.0 {
                agent.body.pos.x = 100.0;
            }
            if agent.body.pos.x > 100.0 {
                agent.body.pos.x = -100.0;
            }
            if agent.body.pos.y < -100.0 {
                agent.body.pos.y = 100.0;
            }
            if agent.body.pos.y > 100.0 {
                agent.body.pos.y = -100.0;
            }

            // Update trail
            if rand::random::<u8>() % 5 == 0 {
                agent.body.trail.push(agent.body.pos);
                if agent.body.trail.len() > 20 {
                    agent.body.trail.remove(0);
                }
            }

            // 4. Stigmergy (Emit)
            let gx = (agent.body.pos.x + 100.0) as i32;
            let gy = (agent.body.pos.y + 100.0) as i32;
            if gx >= 0
                && gx < self.platter.width() as i32
                && gy >= 0
                && gy < self.platter.height() as i32
            {
                self.platter
                    .magnetize(gx as usize, gy as usize, emit * mag_write * dt);
            }
        }

        // 5. Decay
        self.platter.decay(decay_rate);
    }
}
