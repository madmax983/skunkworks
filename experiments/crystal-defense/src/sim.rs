use crate::math::Quasicrystal;
use rand::prelude::*;
use rayon::prelude::*;
use std::sync::Arc;

const HEAT_DECAY_RATE: f32 = 0.98;
const HEAT_DIFFUSION_RATE: f32 = 0.1;
const PHEROMONE_DECAY: f32 = 0.95;
const LOCUST_SPAWN_INTERVAL: usize = 20;
const TERMITE_SPAWN_INTERVAL: usize = 30;
const MAX_AGENTS: usize = 2000;
const LOCUST_SPEED: f32 = 0.05;
const PHEROMONE_THRESHOLD: f32 = 0.1;
const RANDOM_MOVE_CHANCE: f64 = 0.2;
const LOCUST_DAMAGE: f32 = 5.0;
const PHEROMONE_DROP: f32 = 5.0;
const HEAT_GENERATION: f32 = 2.0;
const COOLING_FACTOR: f32 = 0.8;
const SERVER_INITIAL_HEALTH: f32 = 1000.0;

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
            server_health: SERVER_INITIAL_HEALTH,
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

        self.update_heat_diffusion();
        self.update_pheromone_diffusion();
        self.spawn_agents();
        self.update_agents();
    }

    fn update_heat_diffusion(&mut self) {
        // Double buffering to avoid allocation
        let adj = &self.qc.adj;
        let (current_map, next_map) = (&self.heat_map, &mut self.next_heat_map);

        next_map.par_iter_mut().enumerate().for_each(|(i, out)| {
            let current = current_map[i];
            let neighbors = &adj[i];
            if neighbors.is_empty() {
                *out = current * HEAT_DECAY_RATE;
                return;
            }

            let mut inflow = 0.0;
            for &n in neighbors {
                inflow += current_map[n];
            }
            let avg_neighbor = inflow / neighbors.len() as f32;
            let next = current + HEAT_DIFFUSION_RATE * (avg_neighbor - current);
            *out = next * HEAT_DECAY_RATE;
        });
        std::mem::swap(&mut self.heat_map, &mut self.next_heat_map);
    }

    fn update_pheromone_diffusion(&mut self) {
        // Simplified: just decay for now, add diffusion later if needed
        self.pheromone_attack
            .par_iter_mut()
            .for_each(|p| *p *= PHEROMONE_DECAY);
        self.pheromone_defense
            .par_iter_mut()
            .for_each(|p| *p *= PHEROMONE_DECAY);
    }

    #[allow(clippy::manual_is_multiple_of)]
    fn spawn_agents(&mut self) {
        // Spawn Locusts (Attackers)
        if self.ticks % LOCUST_SPAWN_INTERVAL == 0 && self.agents.len() < MAX_AGENTS {
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
        if self.ticks % TERMITE_SPAWN_INTERVAL == 0 && self.agents.len() < MAX_AGENTS {
            self.agents.push(Agent {
                kind: AgentType::Termite,
                current_node: self.center_node,
                target_node: None,
                progress: 0.0,
                hp: 20.0,
            });
        }
    }

    fn update_agents(&mut self) {
        let adj = &self.qc.adj;
        let dists = &self.dist_to_center;
        let pheromone_attack = &mut self.pheromone_attack;
        let pheromone_defense = &mut self.pheromone_defense;
        let heat_map = &mut self.heat_map;
        let center = self.center_node;
        let mut damage = 0.0;

        for agent in self.agents.iter_mut() {
            if agent.hp <= 0.0 {
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
                            if rng.gen_bool(RANDOM_MOVE_CHANCE) {
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

                            if pheromone_attack[best] > PHEROMONE_THRESHOLD {
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
                agent.progress += LOCUST_SPEED;

                if agent.progress >= 1.0 {
                    // Arrived
                    agent.current_node = target;
                    agent.target_node = None;
                    agent.progress = 0.0;

                    // Actions on Arrival
                    match agent.kind {
                        AgentType::Locust => {
                            pheromone_attack[agent.current_node] += PHEROMONE_DROP;
                            heat_map[agent.current_node] += HEAT_GENERATION;
                            if agent.current_node == center {
                                damage += LOCUST_DAMAGE;
                                agent.hp = 0.0; // Suicide
                            }
                        }
                        AgentType::Termite => {
                            pheromone_defense[agent.current_node] += PHEROMONE_DROP;
                            // Cool down node
                            heat_map[agent.current_node] *= COOLING_FACTOR;
                        }
                    }
                }
            }
        }

        // Remove dead
        self.agents.retain(|a| a.hp > 0.0);

        self.server_health -= damage;
        if self.server_health < 0.0 {
            self.server_health = 0.0;
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

    #[test]
    fn test_agent_movement() {
        let qc = Arc::new(generate_icosahedral_lattice(2));
        let mut world = World::new(qc.clone());

        // Force spawn a termite
        world.agents.push(Agent {
            kind: AgentType::Termite,
            current_node: world.center_node,
            target_node: None,
            progress: 0.0,
            hp: 20.0,
        });

        // Ensure neighbor exists
        let neighbors = &qc.adj[world.center_node];
        assert!(!neighbors.is_empty(), "Center node should have neighbors");

        // Manually set some pheromone to guide it (so it doesn't just pick random)
        // Termites follow attack pheromone
        world.pheromone_attack[neighbors[0]] = 10.0;

        // Run update
        world.update();

        // Check if agent picked a target
        let agent = &world.agents[0];
        assert!(
            agent.target_node.is_some(),
            "Agent should have picked a target"
        );
        assert_eq!(
            agent.target_node,
            Some(neighbors[0]),
            "Agent should follow pheromone"
        );
        assert!(agent.progress > 0.0, "Agent should have moved");
    }
}
