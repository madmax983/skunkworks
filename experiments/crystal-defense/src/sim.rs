use crate::math::Quasicrystal;
use rand::prelude::*;
use rayon::prelude::*;
use std::sync::Arc;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AgentType {
    Termite, // Defender
    Locust,  // Attacker
}

#[derive(Clone, Debug)]
pub struct Agent {
    pub kind: AgentType,
    pub current_node: usize,
    pub target_node: Option<usize>,
    pub progress: f32, // 0.0 to 1.0 along the edge
    pub hp: f32,
}

pub struct World {
    pub agents: Vec<Agent>,
    pub heat_map: Vec<f32>,
    pub next_heat_map: Vec<f32>,
    pub pheromone_attack: Vec<f32>,
    pub pheromone_defense: Vec<f32>,
    pub server_health: f32,
    pub ticks: usize,

    // Graph reference
    pub qc: Arc<Quasicrystal>,

    // Cached center node index
    pub center_node: usize,
    // Cached leaf nodes (for spawning)
    pub leaf_nodes: Vec<usize>,

    // Distances to center (for navigation)
    pub dist_to_center: Vec<f32>,
}

impl World {
    pub fn new(qc: Arc<Quasicrystal>) -> Self {
        let n = qc.atoms.len();

        // Find center node (closest to 0,0,0)
        let mut center_node = 0;
        let mut min_dist = f32::MAX;
        let mut dist_to_center = vec![0.0; n];

        for (i, p) in qc.atoms.iter().enumerate() {
            let dist = (p.x * p.x + p.y * p.y + p.z * p.z).sqrt();
            dist_to_center[i] = dist;
            if dist < min_dist {
                min_dist = dist;
                center_node = i;
            }
        }

        // Find leaf nodes (furthest 10%)
        let mut dists: Vec<(usize, f32)> = dist_to_center
            .iter()
            .enumerate()
            .map(|(i, &d)| (i, d))
            .collect();
        dists.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        let leaf_nodes: Vec<usize> = dists.iter().take(n / 10).map(|(i, _)| *i).collect();

        Self {
            agents: Vec::new(),
            heat_map: vec![0.0; n],
            next_heat_map: vec![0.0; n],
            pheromone_attack: vec![0.0; n],
            pheromone_defense: vec![0.0; n],
            server_health: 1000.0,
            ticks: 0,
            qc,
            center_node,
            leaf_nodes,
            dist_to_center,
        }
    }

    #[allow(clippy::manual_is_multiple_of)]
    pub fn update(&mut self) {
        self.ticks += 1;

        // --- 1. Heat Diffusion (Parallel) ---
        let decay_rate = 0.98;
        let diffusion_rate = 0.1;

        // Double buffering to avoid allocation
        let adj = &self.qc.adj;
        let (current_map, next_map) = (&self.heat_map, &mut self.next_heat_map);

        next_map.par_iter_mut().enumerate().for_each(|(i, out)| {
            let current = current_map[i];
            let neighbors = &adj[i];
            if neighbors.is_empty() {
                *out = current * decay_rate;
                return;
            }

            let mut inflow = 0.0;
            for &n in neighbors {
                inflow += current_map[n];
            }
            let avg_neighbor = inflow / neighbors.len() as f32;
            let next = current + diffusion_rate * (avg_neighbor - current);
            *out = next * decay_rate;
        });
        std::mem::swap(&mut self.heat_map, &mut self.next_heat_map);

        // --- 2. Pheromone Diffusion (Parallel) ---
        // Simplified: just decay for now, add diffusion later if needed
        let pheromone_decay = 0.95;
        self.pheromone_attack
            .par_iter_mut()
            .for_each(|p| *p *= pheromone_decay);
        self.pheromone_defense
            .par_iter_mut()
            .for_each(|p| *p *= pheromone_decay);

        // --- 3. Spawn Agents ---
        // Spawn Locusts (Attackers)
        if self.ticks % 20 == 0 && self.agents.len() < 2000 {
            let mut rng = rand::thread_rng();
            if let Some(&start_node) = self.leaf_nodes.choose(&mut rng) {
                self.agents.push(Agent {
                    kind: AgentType::Locust,
                    current_node: start_node,
                    target_node: None,
                    progress: 0.0,
                    hp: 10.0,
                });
            }
        }

        // Spawn Termites (Defenders)
        if self.ticks % 30 == 0 && self.agents.len() < 2000 {
            self.agents.push(Agent {
                kind: AgentType::Termite,
                current_node: self.center_node,
                target_node: None,
                progress: 0.0,
                hp: 20.0,
            });
        }

        // --- 4. Update Agents ---
        let adj = &self.qc.adj;
        let dists = &self.dist_to_center;
        let pheromone_attack = &mut self.pheromone_attack;
        let pheromone_defense = &mut self.pheromone_defense;
        let heat_map = &mut self.heat_map;
        let center = self.center_node;
        let mut damage = 0.0;

        let mut dead_agents = Vec::new();

        // Single threaded update for agents (logic is complex with interactions)
        for (idx, agent) in self.agents.iter_mut().enumerate() {
            if agent.hp <= 0.0 {
                dead_agents.push(idx);
                continue;
            }

            // Movement Selection
            if agent.target_node.is_none() {
                let neighbors = &adj[agent.current_node];
                if !neighbors.is_empty() {
                    let mut rng = rand::thread_rng();
                    let target = match agent.kind {
                        AgentType::Locust => {
                            // Move towards center (smaller dist) + randomness
                            // Find neighbor with min dist
                            let best = neighbors
                                .iter()
                                .min_by(|&&a, &&b| {
                                    dists[a]
                                        .partial_cmp(&dists[b])
                                        .unwrap_or(std::cmp::Ordering::Equal)
                                })
                                .cloned()
                                .unwrap_or(neighbors[0]);

                            // 20% chance to move randomly (avoid local minima / traffic)
                            if rng.gen_bool(0.2) {
                                *neighbors.choose(&mut rng).unwrap()
                            } else {
                                best
                            }
                        }
                        AgentType::Termite => {
                            // Move towards Attack Pheromone
                            // If no pheromone, move randomly or patrol
                            let best = neighbors
                                .iter()
                                .max_by(|&&a, &&b| {
                                    pheromone_attack[a]
                                        .partial_cmp(&pheromone_attack[b])
                                        .unwrap_or(std::cmp::Ordering::Equal)
                                })
                                .cloned()
                                .unwrap_or(neighbors[0]);

                            if pheromone_attack[best] > 0.1 {
                                best
                            } else {
                                *neighbors.choose(&mut rng).unwrap()
                            }
                        }
                    };
                    agent.target_node = Some(target);
                    agent.progress = 0.0;
                }
            }

            // Move along edge
            if let Some(target) = agent.target_node {
                agent.progress += 0.05; // Speed

                if agent.progress >= 1.0 {
                    // Arrived
                    agent.current_node = target;
                    agent.target_node = None;
                    agent.progress = 0.0;

                    // Actions on Arrival
                    match agent.kind {
                        AgentType::Locust => {
                            pheromone_attack[agent.current_node] += 5.0; // Drop trail
                            heat_map[agent.current_node] += 2.0; // Generate heat
                            if agent.current_node == center {
                                damage += 5.0;
                                agent.hp = 0.0; // Suicide
                            }
                        }
                        AgentType::Termite => {
                            pheromone_defense[agent.current_node] += 5.0;
                            // Cool down node
                            heat_map[agent.current_node] *= 0.8;
                        }
                    }
                }
            }
        }

        // Remove dead
        // Simple swap remove from back to avoid n^2 if possible, but indices change.
        // We can just retain.
        self.agents.retain(|a| a.hp > 0.0);

        self.server_health -= damage;
        if self.server_health < 0.0 {
            self.server_health = 0.0;
            // Game Over state?
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::generate_icosahedral_lattice;

    #[test]
    fn test_heat_diffusion_allocation() {
        let qc = Arc::new(generate_icosahedral_lattice(2));
        let mut world = World::new(qc.clone());

        // Inject some heat
        world.heat_map[0] = 100.0;

        let initial_heat_sum: f32 = world.heat_map.iter().sum();

        // Run update
        world.update();

        let next_heat_sum: f32 = world.heat_map.iter().sum();

        // Heat should decay
        assert!(next_heat_sum < initial_heat_sum);
        assert!(next_heat_sum > 0.0);

        // Heat should diffuse
        let neighbors = &qc.adj[0];
        for &n in neighbors {
            assert!(world.heat_map[n] > 0.0, "Heat should diffuse to neighbors");
        }
    }
}
