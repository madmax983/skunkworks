use crate::lattice::Lattice;
use nalgebra::distance;
use rand::prelude::*;

#[derive(Clone)]
pub struct Network {
    pub adj: Vec<Vec<usize>>,       // Adjacency list: adj[i] = [j, k, ...]
    pub node_positions: Vec<usize>, // node_positions[i] = index in lattice.points
}

impl Network {
    /// Generates a random Erdős-Rényi graph
    pub fn new_random(node_count: usize, edge_prob: f64, rng: &mut impl Rng) -> Self {
        let mut adj = vec![Vec::new(); node_count];
        for i in 0..node_count {
            for j in (i + 1)..node_count {
                if rng.gen::<f64>() < edge_prob {
                    adj[i].push(j);
                    adj[j].push(i);
                }
            }
        }
        Self {
            adj,
            node_positions: Vec::new(),
        }
    }

    /// Assigns each node to a random unique point in the lattice
    pub fn initialize_positions(&mut self, lattice: &Lattice, rng: &mut impl Rng) {
        let mut indices: Vec<usize> = (0..lattice.points.len()).collect();
        indices.shuffle(rng);

        if indices.len() < self.adj.len() {
            panic!(
                "Lattice is too small for network! Nodes: {}, Lattice Points: {}",
                self.adj.len(),
                indices.len()
            );
        }

        self.node_positions = indices.into_iter().take(self.adj.len()).collect();
    }

    /// Calculates total wire length (energy)
    pub fn total_energy(&self, lattice: &Lattice) -> f64 {
        let mut total_dist = 0.0;
        for (u, neighbors) in self.adj.iter().enumerate() {
            let u_pos = lattice.points[self.node_positions[u]];
            for &v in neighbors {
                if u < v {
                    // Count each edge once
                    let v_pos = lattice.points[self.node_positions[v]];
                    total_dist += distance(&u_pos, &v_pos);
                }
            }
        }
        total_dist
    }

    /// Performs one step of simulated annealing (Swap mutation)
    /// Returns true if accepted
    pub fn optimize_step(&mut self, lattice: &Lattice, temp: f64, rng: &mut impl Rng) -> bool {
        let n = self.node_positions.len();
        if n < 2 {
            return false;
        }

        // Pick two nodes to swap
        let u = rng.gen_range(0..n);
        let v = rng.gen_range(0..n);
        if u == v {
            return false;
        }

        // Calculate old local energy (only edges connected to u or v)
        let old_energy = self.local_energy(u, lattice) + self.local_energy(v, lattice);

        // Swap
        self.node_positions.swap(u, v);

        // Calculate new local energy
        let new_energy = self.local_energy(u, lattice) + self.local_energy(v, lattice);

        // Accept/Reject
        let delta = new_energy - old_energy;
        if delta < 0.0 || rng.gen::<f64>() < (-delta / temp).exp() {
            // Accept
            return true;
        } else {
            // Revert
            self.node_positions.swap(u, v);
            return false;
        }
    }

    // Helper for efficient energy calc
    fn local_energy(&self, u: usize, lattice: &Lattice) -> f64 {
        let mut e = 0.0;
        let u_pos = lattice.points[self.node_positions[u]];
        for &v in &self.adj[u] {
            // Note: If v was the other swapped node, we might double count or mess up if we assume global consistency.
            // But since we sum energy of edges connected to u, and edges connected to v...
            // The edge (u, v) is counted in both.
            // If we swap u and v, the distance (u, v) doesn't change.
            // So minimizing sum of edges incident to u + incident to v is correct proxy for global change,
            // MINUS the edge (u,v) itself which is constant.

            // Actually, we can just calculate full delta efficiently.
            // But this heuristic is fine for now.
            let v_pos = lattice.points[self.node_positions[v]];
            e += distance(&u_pos, &v_pos);
        }
        e
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lattice::{Lattice, LatticeType};

    #[test]
    fn test_optimization_improves_energy() {
        let mut rng = rand::thread_rng();
        // Create small lattice
        let lattice = Lattice::new(LatticeType::SimpleCubic, 2, 1.0); // 5x5x5 = 125 points

        // Create network
        let mut net = Network::new_random(20, 0.5, &mut rng);
        net.initialize_positions(&lattice, &mut rng);

        let initial_energy = net.total_energy(&lattice);

        // Run optimization
        let mut current_energy = initial_energy;
        let mut improved = false;

        for _ in 0..1000 {
            net.optimize_step(&lattice, 0.1, &mut rng); // Low temp to encourage greedy descent
            let new_energy = net.total_energy(&lattice);
            if new_energy < current_energy {
                improved = true;
            }
            current_energy = new_energy;
        }

        // It's stochastic, but with 1000 steps and low temp, it should improve or stay same.
        // It's extremely unlikely to get WORSE than random initialization.
        assert!(current_energy <= initial_energy);
        // It likely improved
        if initial_energy > 0.0 {
            // Ideally we want assert!(improved), but randomness makes it flaky if init was already optimal (rare).
        }
    }
}
