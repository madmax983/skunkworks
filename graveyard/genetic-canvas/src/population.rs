use crate::genome::Genome;
use rand::Rng;

pub struct Population {
    pub genomes: Vec<Genome>,
    pub generation: usize,
}

impl Population {
    pub fn new(size: usize) -> Self {
        // Start with simple genomes
        let genomes = (0..size).map(|_| Genome::random(5)).collect();
        Population {
            genomes,
            generation: 1,
        }
    }

    pub fn evolve(&mut self, selected_indices: &[usize]) {
        if selected_indices.is_empty() {
            return;
        }

        let mut next_generation = Vec::new();
        let mut rng = rand::thread_rng();

        // 1. Elitism: Keep the best ones as is (or mutated slightly?)
        // Let's keep them as is to ensure we don't regress.
        for &idx in selected_indices {
            if idx < self.genomes.len() {
                next_generation.push(self.genomes[idx].clone());
            }
        }

        // 2. Fill the rest
        while next_generation.len() < self.genomes.len() {
            // Pick two parents from selection
            let idx1 = selected_indices[rng.gen_range(0..selected_indices.len())];
            let idx2 = selected_indices[rng.gen_range(0..selected_indices.len())];

            let p1 = &self.genomes[idx1];
            let p2 = &self.genomes[idx2];

            // Crossover
            let mut child = Genome::crossover(p1, p2);

            // Mutate
            // Mutation rate can be dynamic or fixed.
            // Let's stick to fixed for now.
            child.mutate(0.1);

            next_generation.push(child);
        }

        self.genomes = next_generation;
        self.generation += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evolution_preserves_size() {
        let mut pop = Population::new(10);
        assert_eq!(pop.genomes.len(), 10);

        let selected = vec![0, 1];
        pop.evolve(&selected);

        assert_eq!(pop.genomes.len(), 10);
        assert_eq!(pop.generation, 2);
    }

    #[test]
    fn test_evolution_with_no_selection_does_nothing() {
        let mut pop = Population::new(10);
        let original_gen = pop.generation;

        pop.evolve(&[]);

        assert_eq!(pop.generation, original_gen);
    }
}
