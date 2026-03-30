use crate::graph::Graph;
use chimera_lang::prelude::*;
use locus::Vec2;
use rand::Rng;

pub struct Agent {
    pub vm: ChimeraVM,
    pub current_node: usize,
    pub energy: f64,
}

pub struct GameState {
    pub graph: Graph,
    pub agents: Vec<Agent>,
    pub ticks: u64,
}

impl GameState {
    pub fn new() -> Self {
        let mut graph = Graph::new();
        graph.scan_directory("experiments/mnem-rot/src");
        if graph.nodes.is_empty() {
            graph.scan_directory(".");
        }

        let mut agents = Vec::new();
        if !graph.nodes.is_empty() {
            let mut rng = rand::thread_rng();
            for _ in 0..10 {
                let current_node = rng.gen_range(0..graph.nodes.len());
                // Safe Assumption: Use None for evolution_config, as seen in previous hybrid bugfixes
                // to avoid instantiating the complex EvolutionConfig struct manually.
                let dna = Dna::from_genes(vec![Gene::new(OpCode::Push, vec![Nucleotide::from(1)]), Gene::new(OpCode::Nop, vec![])]);
                let mut vm = ChimeraVM::new(dna);
                vm.dna.evolution_config = None;
                agents.push(Agent {
                    vm,
                    current_node,
                    energy: 100.0,
                });
            }
        }

        Self {
            graph,
            agents,
            ticks: 0,
        }
    }

    pub fn tick(&mut self) {
        self.ticks += 1;

        let mut rng = rand::thread_rng();
        for i in 0..self.agents.len() {
            let agent = &mut self.agents[i];
            let current_node_entropy = self.graph.nodes[agent.current_node].entropy;

            // Push entropy to VM stack
            agent.vm.stack.push(Value::Int((current_node_entropy * 1000.0) as i64));
            let _ = agent.vm.step();

            // Consume action
            if let Some(val) = agent.vm.stack.pop() {
                if let Value::Int(action) = val {
                    // action > 0: move to adjacent
                    // action < 0: forage (reduce entropy)
                    if action > 0 {
                        if let Some(neighbors) = self.graph.adjacency.get(&agent.current_node) {
                            if !neighbors.is_empty() {
                                let next_node = neighbors[rng.gen_range(0..neighbors.len())];
                                agent.current_node = next_node;
                                agent.energy -= 1.0;
                            }
                        }
                    } else if action < 0 {
                        if self.graph.nodes[agent.current_node].entropy > 0.05 {
                            self.graph.nodes[agent.current_node].entropy -= 0.05;
                            agent.energy += 10.0;
                        }
                    }
                }
            }

            // Mutation / death
            agent.energy -= 0.1;
            if agent.energy <= 0.0 {
                agent.energy = 100.0;
                agent.current_node = rng.gen_range(0..self.graph.nodes.len());
                // Safe Assumption: Instead of mutate() which is missing, reset dna on death
                agent.vm.dna = Dna::from_genes(vec![Gene::new(OpCode::Push, vec![Nucleotide::from(1)]), Gene::new(OpCode::Nop, vec![])]);
                agent.vm.dna.evolution_config = None;
            }
        }

        // Physics step for nodes (spring graph)
        for i in 0..self.graph.nodes.len() {
            let mut force = Vec2::new(0.0, 0.0);

            // Push apart
            for j in 0..self.graph.nodes.len() {
                if i == j { continue; }
                let diff = self.graph.nodes[i].pos - self.graph.nodes[j].pos;
                // Safe Assumption: Use magnitude() over length() as locus::Vec2 uses it.
                let dist = diff.magnitude();
                if dist > 0.1 && dist < 100.0 {
                    force += diff.normalize() * (500.0 / dist);
                }
            }

            self.graph.nodes[i].vel += force * 0.01;
            self.graph.nodes[i].vel *= 0.9;
        }

        // Spring edges
        let edges = self.graph.edges.clone();
        for edge in edges {
            let diff = self.graph.nodes[edge.to].pos - self.graph.nodes[edge.from].pos;
            let dist = diff.magnitude();
            if dist > 0.0 {
                let spring_force = (dist - 50.0) * 0.01;
                let dir = diff.normalize();

                self.graph.nodes[edge.from].vel += dir * spring_force;
                self.graph.nodes[edge.to].vel -= dir * spring_force;
            }
        }

        for node in &mut self.graph.nodes {
            node.pos += node.vel;
        }
    }
}
