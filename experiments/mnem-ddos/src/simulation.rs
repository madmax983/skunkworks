use crate::graph::Graph;
use ::rand::prelude::*;
use macroquad::prelude::*;
use rayon::prelude::*;

pub const AGENT_COUNT: usize = 10_000;
pub const WORLD_SIZE: f32 = 1000.0;
pub const SPEED: f32 = 2.0;

#[derive(Clone, Copy)]
pub struct Locust {
    pub pos: Vec2,
    pub vel: Vec2,
    pub target_node: Option<usize>,
}

pub struct Simulation {
    pub agents: Vec<Locust>,
    pub graph: Graph,
}

impl Simulation {
    pub fn new(graph: Graph) -> Self {
        let mut rng = ::rand::thread_rng();
        let mut agents = Vec::with_capacity(AGENT_COUNT);

        // Spawn agents on the edges
        for _ in 0..AGENT_COUNT {
            agents.push(Locust {
                pos: vec2(
                    rng.gen_range(0.0..WORLD_SIZE),
                    rng.gen_range(0.0..WORLD_SIZE),
                ),
                vel: vec2(0.0, 0.0),
                target_node: None,
            });
        }

        Self { agents, graph }
    }

    pub fn update(&mut self) {
        let node_count = self.graph.nodes.len();
        if node_count == 0 {
            return;
        }

        // We need to pass read-only node data to agents to pick targets and update pos
        let mut node_healths: Vec<f32> = vec![0.0; node_count];
        let mut node_positions: Vec<Vec2> = vec![Vec2::ZERO; node_count];

        for (i, node) in self.graph.nodes.iter().enumerate() {
            node_healths[i] = node.health;
            node_positions[i] = node.pos;
        }

        // Parallel update of agents
        self.agents.par_iter_mut().for_each(|agent| {
            let mut rng = ::rand::thread_rng();

            // Assign target if none or randomly re-assign
            if agent.target_node.is_none() || rng.gen::<f32>() < 0.01 {
                agent.target_node = Some(rng.gen_range(0..node_count));
            }

            if let Some(target_idx) = agent.target_node {
                let target_pos = node_positions[target_idx];

                // Seek target
                let desired = (target_pos - agent.pos).normalize_or_zero() * SPEED;
                let steer = (desired - agent.vel).clamp_length_max(0.1);

                agent.vel += steer;
                agent.vel = agent.vel.clamp_length_max(SPEED);
                agent.pos += agent.vel;

                // Simple collision check (if close enough)
                if agent.pos.distance_squared(target_pos) < 25.0 {
                    // It hit the node, target a new one
                    agent.target_node = Some(rng.gen_range(0..node_count));
                    // Send agent back to edge
                    let side = rng.gen_range(0..4);
                    agent.pos = match side {
                        0 => vec2(rng.gen_range(-100.0..WORLD_SIZE + 100.0), -100.0), // Top
                        1 => vec2(
                            rng.gen_range(-100.0..WORLD_SIZE + 100.0),
                            WORLD_SIZE + 100.0,
                        ), // Bottom
                        2 => vec2(-100.0, rng.gen_range(-100.0..WORLD_SIZE + 100.0)), // Left
                        _ => vec2(
                            WORLD_SIZE + 100.0,
                            rng.gen_range(-100.0..WORLD_SIZE + 100.0),
                        ), // Right
                    };
                }
            }
        });

        // Now sequential pass to update node health based on agents nearby
        let mut hits = vec![0; node_count];
        for agent in &self.agents {
            if let Some(target_idx) = agent.target_node {
                let target_pos = node_positions[target_idx];
                if agent.pos.distance_squared(target_pos) < 100.0 {
                    hits[target_idx] += 1;
                }
            }
        }

        for (i, node) in self.graph.nodes.iter_mut().enumerate() {
            if hits[i] > 0 {
                node.health -= 0.005 * hits[i] as f32;
                if node.health < 0.0 {
                    node.health = 0.0;
                }
            } else {
                // slowly recover if not swarmed
                node.health += 0.001;
                if node.health > 1.0 {
                    node.health = 1.0;
                }
            }
        }
    }
}
