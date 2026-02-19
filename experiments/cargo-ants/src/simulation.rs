use rayon::prelude::*;
use crate::graph::DependencyGraph;
use crate::pheromone::PheromoneMap;
use crate::ant::{Ant, AntState};

pub struct Simulation {
    pub graph: DependencyGraph,
    pub pheromones: PheromoneMap,
    pub ants: Vec<Ant>,
    pub step_count: u64,
}

impl Simulation {
    pub fn new(width: f32, height: f32) -> Self {
        let mut graph = DependencyGraph::new();
        graph.generate_random(width, height);
        let edge_count = graph.graph.edge_count();
        // Ensure pheromone map is large enough. Edge indices are sequential mostly.
        let pheromones = PheromoneMap::new(edge_count + 1000);

        Simulation {
            graph,
            pheromones,
            ants: Vec::new(),
            step_count: 0,
        }
    }

    pub fn step(&mut self) {
        self.step_count += 1;

        // Evaporate
        self.pheromones.evaporate(0.992); // Faster decay to highlight current paths

        // Spawn ants
        if self.step_count % 5 == 0 && self.ants.len() < 2000 {
            let root = self.graph.root_index;
            let pos = self.graph.graph[root].position;
            self.ants.push(Ant::new(root, pos));
        }

        // Update ants and collect deposits
        // We use par_iter_mut to update ants in parallel
        let graph = &self.graph;
        let pheromones = &self.pheromones;

        let deposits: Vec<_> = self.ants.par_iter_mut()
            .filter_map(|ant| ant.update(graph, pheromones))
            .collect();

        // Apply deposits
        for (edge, amount) in deposits {
            self.pheromones.deposit(edge, amount);
        }

        // Remove dead ants
        self.ants.retain(|ant| ant.state != AntState::Dead);
    }
}
