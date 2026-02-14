use crate::math::Quasicrystal;
use glam::Vec3;
use rand::Rng;
use rayon::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum AgentState {
    CommutingToWork,
    CommutingHome,
}

#[derive(Clone, Copy)]
pub struct Agent {
    pub pos: Vec3,
    pub current_node: usize,
    pub target_node: usize,
    pub home_node: usize,
    pub work_node: usize,
    pub state: AgentState,
    pub color: [f32; 3], // RGB
}

impl Agent {
    pub fn new(start_node: usize, home_node: usize, work_node: usize, world: &World) -> Self {
        let pos = world.lattice.atoms[start_node];
        // Pick initial random neighbor as target
        let neighbors = &world.lattice.adj[start_node];
        let target_node = if !neighbors.is_empty() {
            neighbors[rand::thread_rng().gen_range(0..neighbors.len())]
        } else {
            start_node
        };

        // Random color
        let mut rng = rand::thread_rng();
        let color = [rng.gen(), rng.gen(), rng.gen()];

        Self {
            pos,
            current_node: start_node,
            target_node,
            home_node,
            work_node,
            state: AgentState::CommutingToWork,
            color,
        }
    }

    pub fn update(&mut self, world: &World, settings: &Settings) {
        let target_pos = world.lattice.atoms[self.target_node];
        let direction = target_pos - self.pos;
        let distance = direction.length();

        if distance < settings.move_speed {
            // Arrived
            self.pos = target_pos;
            self.current_node = self.target_node;

            // Check if reached destination
            let destination_node = match self.state {
                AgentState::CommutingToWork => self.work_node,
                AgentState::CommutingHome => self.home_node,
            };

            if self.current_node == destination_node {
                self.state = match self.state {
                    AgentState::CommutingToWork => AgentState::CommutingHome,
                    AgentState::CommutingHome => AgentState::CommutingToWork,
                };
            }

            // Pick next node
            self.pick_next_node(world, settings);
        } else {
            // Move
            self.pos += direction.normalize() * settings.move_speed;
        }
    }

    fn pick_next_node(&mut self, world: &World, settings: &Settings) {
        let neighbors = &world.lattice.adj[self.current_node];
        if neighbors.is_empty() {
            return;
        }

        let destination_node = match self.state {
            AgentState::CommutingToWork => self.work_node,
            AgentState::CommutingHome => self.home_node,
        };
        let destination_pos = world.lattice.atoms[destination_node];

        // Calculate weights
        let mut weights = Vec::with_capacity(neighbors.len());
        let mut total_weight = 0.0;

        for &neighbor in neighbors {
            let neighbor_pos = world.lattice.atoms[neighbor];

            // Pheromone Weight
            let pheromone = world.pheromone_levels[neighbor];
            let w_pheromone = 1.0 + pheromone * settings.sensor_strength;

            // Direction Weight (Dot product)
            let to_dest = (destination_pos - self.pos).normalize_or_zero();
            let to_neighbor = (neighbor_pos - self.pos).normalize_or_zero();
            let alignment = to_dest.dot(to_neighbor); // -1 to 1
            // Map -1..1 to 0.1..2.0 roughly
            let w_direction = (alignment + 1.2).powf(2.0); // Bias towards forward

            // Avoid going back immediately if possible (unless dead end)
            // But we don't store previous node.

            let weight = w_pheromone * w_direction;
            weights.push(weight);
            total_weight += weight;
        }

        // Weighted Random Choice
        let mut rng = rand::thread_rng();
        let mut choice = rng.gen::<f32>() * total_weight;

        for (i, &weight) in weights.iter().enumerate() {
            choice -= weight;
            if choice <= 0.0 {
                self.target_node = neighbors[i];
                return;
            }
        }
        // Fallback
        self.target_node = neighbors[neighbors.len() - 1];
    }
}

pub struct World {
    pub lattice: Quasicrystal,
    pub pheromone_levels: Vec<f32>,
    pub cities: Vec<usize>, // Node indices
}

impl World {
    pub fn new(lattice: Quasicrystal) -> Self {
        let len = lattice.atoms.len();
        Self {
            lattice,
            pheromone_levels: vec![0.0; len],
            cities: Vec::new(),
        }
    }

    pub fn deposit(&mut self, node_idx: usize, amount: f32) {
        if node_idx < self.pheromone_levels.len() {
            self.pheromone_levels[node_idx] += amount;
            // Cap?
            if self.pheromone_levels[node_idx] > 10.0 {
                self.pheromone_levels[node_idx] = 10.0;
            }
        }
    }

    pub fn diffuse(&mut self, decay_rate: f32) {
        // Parallel diffusion
        // New buffer
        let mut next_levels = self.pheromone_levels.clone();

        // This is graph diffusion: node value = (self + sum(neighbors)) / (1 + degree) * decay
        // Or simpler: average of neighbors.

        // Let's do: value = (self * 0.5 + average(neighbors) * 0.5) * decay

        next_levels.par_iter_mut().enumerate().for_each(|(i, level)| {
            let neighbors = &self.lattice.adj[i];
            if neighbors.is_empty() {
                *level *= decay_rate;
                return;
            }

            let mut sum = self.pheromone_levels[i]; // Include self
            for &n in neighbors {
                sum += self.pheromone_levels[n];
            }

            let avg = sum / (neighbors.len() + 1) as f32;
            *level = avg * decay_rate;
        });

        self.pheromone_levels = next_levels;
    }
}

pub struct Settings {
    pub move_speed: f32,
    pub sensor_strength: f32, // How much pheromones affect pathfinding
    pub decay_rate: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            move_speed: 0.1,
            sensor_strength: 5.0,
            decay_rate: 0.98,
        }
    }
}

pub struct Simulation {
    pub world: World,
    pub agents: Vec<Agent>,
    pub settings: Settings,
}

impl Simulation {
    pub fn new(lattice: Quasicrystal, num_agents: usize) -> Self {
        let mut world = World::new(lattice);
        let mut rng = rand::thread_rng();

        // Spawn Cities
        let num_cities = 6;
        let num_atoms = world.lattice.atoms.len();
        for _ in 0..num_cities {
            world.cities.push(rng.gen_range(0..num_atoms));
        }

        // Spawn Agents
        let mut agents = Vec::with_capacity(num_agents);
        for _ in 0..num_agents {
            let home = world.cities[rng.gen_range(0..num_cities)];
            let mut work = world.cities[rng.gen_range(0..num_cities)];
            while work == home {
                work = world.cities[rng.gen_range(0..num_cities)];
            }

            // Start at home
            agents.push(Agent::new(home, home, work, &world));
        }

        Self {
            world,
            agents,
            settings: Settings::default(),
        }
    }

    pub fn step(&mut self) {
        // Update Agents
        // We need to pass world to agents, but agents modify self.
        // We also need to deposit pheromones.

        // To allow parallelism, we can compute updates first, then apply.
        // But for simplicity in this prototype, we run serial updates or handle borrow checker carefully.
        // Let's do serial agent update first. Rayon is used for diffusion.

        let world = &self.world;
        let settings = &self.settings;

        for agent in &mut self.agents {
            agent.update(world, settings);
        }

        // Deposit
        for agent in &self.agents {
            // Deposit at current node (or interpolation along edge?)
            // Just deposit at current_node for now.
            self.world.deposit(agent.current_node, 0.5);
        }

        // Diffuse
        self.world.diffuse(self.settings.decay_rate);
    }
}
