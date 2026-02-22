use crate::agent::{Agent, mutate_dna};
use crate::tablet::Tablet;
use rand::thread_rng;

pub struct Simulation {
    pub tablet: Tablet,
    pub agents: Vec<Agent>,
    pub ticks: u64,
    pub flood_cycle: u64,
}

impl Simulation {
    pub fn new(width: usize, height: usize, num_agents: usize) -> Self {
        let mut agents = Vec::with_capacity(num_agents);
        for i in 0..num_agents {
            agents.push(Agent::new(i as u64, width, height));
        }
        Self {
            tablet: Tablet::new(width, height),
            agents,
            ticks: 0,
            flood_cycle: 223, // Saros cycle length
        }
    }

    pub fn step(&mut self) {
        self.ticks += 1;

        // Step Agents
        for agent in &mut self.agents {
            agent.step(&mut self.tablet);
        }

        // Update Tablet (Hardening)
        self.tablet.update();

        // Handle Flood
        if self.ticks % self.flood_cycle == 0 {
            let _survivors = self.tablet.flood();

            // Reward authors of surviving cells
            for cell in &self.tablet.cells {
                if cell.value != 0 && cell.is_immutable() {
                    if let Some(author_id) = cell.author {
                        if let Some(agent) = self.agents.iter_mut().find(|a| a.id == author_id) {
                            agent.energy += 50.0;
                        }
                    }
                }
            }

            // Repopulate if necessary
            self.repopulate();
        }
    }

    fn repopulate(&mut self) {
        // Simple elitism
        self.agents.sort_by(|a, b| b.energy.partial_cmp(&a.energy).unwrap());
        let survivors_count = self.agents.len() / 2;
        if survivors_count == 0 { return; } // Extinction event

        let mut new_agents = Vec::new();
        let mut rng = thread_rng();

        // Top 50% reproduce
        for i in 0..survivors_count {
            let parent = &self.agents[i];
            let mut child = Agent::new(self.agents.len() as u64 + i as u64, self.tablet.width, self.tablet.height);

            child.dna = mutate_dna(&parent.dna, &mut rng);
            child.vm = chimera_lang::prelude::ChimeraVM::new(child.dna.clone());
            new_agents.push(child);
        }

        // Replace bottom 50%
        self.agents.truncate(survivors_count);
        self.agents.append(&mut new_agents);

        // Reset energy for next cycle (mostly)
        for agent in &mut self.agents {
            agent.energy = 100.0;
        }
    }
}
