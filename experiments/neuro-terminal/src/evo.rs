use crate::nn::Network;
use rand::Rng;

pub struct Population {
    pub networks: Vec<Network>,
    pub generation: usize,
    pub best_fitness: f64,
}

impl Population {
    pub fn new(size: usize, layers: Vec<usize>) -> Self {
        let mut networks = vec![];
        for _ in 0..size {
            // Initial learning rate doesn't matter for GA
            networks.push(Network::new(layers.clone(), 0.0));
        }
        Self {
            networks,
            generation: 0,
            best_fitness: 0.0,
        }
    }

    pub fn best(&self) -> &Network {
        // Assumes networks are sorted by fitness after evolve()
        &self.networks[0]
    }

    pub fn evolve(&mut self, inputs: &[Vec<f64>], targets: &[Vec<f64>]) {
        // 1. Evaluate Fitness
        let mut fitnesses: Vec<(usize, f64)> = self
            .networks
            .iter()
            .enumerate()
            .map(|(i, net)| {
                let score = evaluate(net, inputs, targets);
                (i, score)
            })
            .collect();

        // 2. Sort by Fitness (Descending)
        // Handle NaN by using unwrap_or(Ordering::Equal) or similar.
        fitnesses.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        self.best_fitness = fitnesses[0].1;

        // 3. Selection (Elitism + Top 50%)
        let keep_count = (self.networks.len() / 2).max(1);
        let mut next_gen = Vec::with_capacity(self.networks.len());

        // Keep the best ones (Elitism)
        for (idx, _) in fitnesses.iter().take(keep_count) {
            next_gen.push(self.networks[*idx].clone());
        }

        // 4. Fill the rest with offspring
        let mut rng = rand::thread_rng();
        while next_gen.len() < self.networks.len() {
            // Select 2 random parents from the survivors (simple tournament or random from top)
            // Here: Random from top 50%
            let p1_idx = rng.gen_range(0..keep_count);
            let p2_idx = rng.gen_range(0..keep_count);

            let p1 = &next_gen[p1_idx];
            let p2 = &next_gen[p2_idx];

            let mut child = crossover(p1, p2);
            // Dynamic mutation rate could be better, but fixed is fine for MVP
            mutate(&mut child, 0.1, 0.5); // Rate 10%, Strength 0.5
            next_gen.push(child);
        }

        self.networks = next_gen;
        self.generation += 1;
    }
}

fn evaluate(net: &Network, inputs: &[Vec<f64>], targets: &[Vec<f64>]) -> f64 {
    let mut error_sum = 0.0;
    for (input, target) in inputs.iter().zip(targets.iter()) {
        let output = net.predict(input);
        // Mean Squared Error per sample
        for (o, t) in output.iter().zip(target.iter()) {
            error_sum += (o - t).powi(2);
        }
    }
    // Fitness is inverse of error.
    // Max error per sample is roughly 1.0 (0 vs 1).
    // Total error can be large.
    // 1.0 / (1.0 + error) ensures range (0, 1]
    1.0 / (1.0 + error_sum)
}

fn crossover(p1: &Network, p2: &Network) -> Network {
    let mut child = p1.clone(); // Start as clone of p1
    let mut rng = rand::thread_rng();

    // Crossover weights
    for (l, w_matrix) in child.weights.iter_mut().enumerate() {
        let p2_w = &p2.weights[l];
        for i in 0..w_matrix.data.len() {
            // Uniform crossover
            if rng.gen_bool(0.5) {
                w_matrix.data[i] = p2_w.data[i];
            }
        }
    }

    // Crossover biases
    for (l, b_matrix) in child.biases.iter_mut().enumerate() {
        let p2_b = &p2.biases[l];
        for i in 0..b_matrix.data.len() {
            if rng.gen_bool(0.5) {
                b_matrix.data[i] = p2_b.data[i];
            }
        }
    }

    child
}

fn mutate(net: &mut Network, rate: f64, strength: f64) {
    let mut rng = rand::thread_rng();

    for w_matrix in net.weights.iter_mut() {
        for val in w_matrix.data.iter_mut() {
            if rng.gen_bool(rate) {
                *val += rng.gen_range(-strength..strength);
            }
        }
    }

    for b_matrix in net.biases.iter_mut() {
        for val in b_matrix.data.iter_mut() {
            if rng.gen_bool(rate) {
                *val += rng.gen_range(-strength..strength);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_population_creation() {
        let pop = Population::new(10, vec![2, 3, 1]);
        assert_eq!(pop.networks.len(), 10);
        assert_eq!(pop.generation, 0);
    }

    #[test]
    fn test_evolution_step() {
        let mut pop = Population::new(10, vec![2, 3, 1]);
        let inputs = vec![vec![0.0, 0.0], vec![1.0, 1.0]];
        let targets = vec![vec![0.0], vec![1.0]];

        // Evolve
        pop.evolve(&inputs, &targets);

        assert_eq!(pop.generation, 1);
        assert_eq!(pop.networks.len(), 10);
        // Best fitness should be > 0.0
        assert!(pop.best_fitness > 0.0);
    }
}
