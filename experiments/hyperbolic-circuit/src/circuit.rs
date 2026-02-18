use num_complex::Complex;
use poincare_disk::{hyperbolic_dist, Point};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use sha2::{Digest, Sha256};

pub struct HyperCircuit {
    pub pads: Vec<Point>,
    pub traces: Vec<(usize, usize)>,
}

impl HyperCircuit {
    pub fn new() -> Self {
        Self {
            pads: Vec::new(),
            traces: Vec::new(),
        }
    }

    pub fn generate(seed_str: &str) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(seed_str.as_bytes());
        let result = hasher.finalize();
        let seed: [u8; 32] = result.into();
        let mut rng = ChaCha20Rng::from_seed(seed);

        let num_pads = rng.gen_range(20..40);
        let mut pads = Vec::new();

        // Generate pads
        for _ in 0..num_pads {
             // Uniform distribution in disk?
             // r = sqrt(random) to handle area density
             let r = rng.gen_range(0.0f64..0.95).sqrt();
             let theta = rng.gen_range(0.0..std::f64::consts::PI * 2.0);
             pads.push(Complex::from_polar(r, theta));
        }

        // Generate traces (Nearest Neighbors based on hyperbolic distance)
        let mut traces = Vec::new();
        // Use a simple strategy: Connect each pad to k nearest neighbors
        for i in 0..pads.len() {
            let mut neighbors: Vec<(usize, f64)> = pads.iter().enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(j, p)| (j, hyperbolic_dist(pads[i], *p)))
                .collect();

            // Sort by distance
            neighbors.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

            // Connect to 2 nearest
            for (neighbor_idx, _) in neighbors.iter().take(2) {
                let p1 = i.min(*neighbor_idx);
                let p2 = i.max(*neighbor_idx);
                if !traces.contains(&(p1, p2)) {
                    traces.push((p1, p2));
                }
            }
        }

        Self { pads, traces }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generation() {
        let circuit = HyperCircuit::generate("test");
        assert!(!circuit.pads.is_empty());
        // Traces might be empty if pads are too far? Unlikely with 20 pads in unit disk.
        // But let's check pads at least.
        assert!(circuit.pads.len() >= 20);
    }
}
