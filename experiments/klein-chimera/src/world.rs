use crate::agent::Agent;
use crate::topology::Topology;
use glam::Vec2;
use rand::Rng;

pub struct World {
    pub agents: Vec<Agent>,
    pub topology: Topology,
}

impl World {
    pub fn new(width: f32, height: f32, count: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut agents = Vec::new();
        for _ in 0..count {
            let pos = Vec2::new(
                rng.gen_range(-width / 2.0..width / 2.0),
                rng.gen_range(-height / 2.0..height / 2.0),
            );
            agents.push(Agent::new(pos));
        }

        Self {
            agents,
            topology: Topology::new(width, height),
        }
    }

    pub fn update(&mut self) {
        for agent in &mut self.agents {
            let force = agent.update();
            agent.vel += force;
            agent.vel = agent.vel.clamp_length_max(0.1); // Terminal velocity

            // Friction
            agent.vel *= 0.98;

            // Move
            let new_pos = agent.pos + agent.vel;

            // Wrap
            let (wrapped_pos, flipped) = self.topology.wrap(new_pos);
            agent.pos = wrapped_pos;

            if flipped {
                agent.flip_chirality();
                // On Klein twist (X -> -X), the X basis vector flips.
                // So a velocity of (+1, 0) becomes (-1, 0) in global coords
                // to maintain local "forward" momentum relative to the agent.
                agent.vel.x = -agent.vel.x;
            }
        }
    }
}
