use num_complex::Complex;
use poincare_disk::{hyperbolic_dist, Point};
use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Pad {
    pub id: usize,
    pub pos: Point,
}

#[derive(Clone, Copy, Debug)]
pub struct Trace {
    pub start: usize, // Pad ID
    pub end: usize,   // Pad ID
}

pub struct HyperbolicCircuit {
    pub pads: Vec<Pad>,
    pub traces: Vec<Trace>,
    pub adjacency: Vec<Vec<usize>>, // Map Pad ID -> List of Connected Pad IDs
}

impl HyperbolicCircuit {
    pub fn new() -> Self {
        Self {
            pads: Vec::new(),
            traces: Vec::new(),
            adjacency: Vec::new(),
        }
    }

    pub fn generate(&mut self, num_pads: usize, connection_threshold: f64) {
        let mut rng = rand::thread_rng();

        // 1. Generate Pads
        self.pads.clear();
        for i in 0..num_pads {
            // Random point in disk
            // Uniform distribution in disk: r = sqrt(random), theta = random
            // Scale r by 0.95 to keep away from strict boundary where distance -> infinity
            let r = rng.gen::<f64>().sqrt() * 0.95;
            let theta = rng.gen::<f64>() * 2.0 * std::f64::consts::PI;
            let pos = Complex::from_polar(r, theta);
            self.pads.push(Pad { id: i, pos });
        }

        // 2. Generate Traces (Edges)
        self.traces.clear();
        self.adjacency = vec![Vec::new(); num_pads];

        for i in 0..num_pads {
            for j in (i + 1)..num_pads {
                let dist = hyperbolic_dist(self.pads[i].pos, self.pads[j].pos);
                if dist < connection_threshold {
                    // Connect
                    self.traces.push(Trace { start: i, end: j });
                    self.adjacency[i].push(j);
                    self.adjacency[j].push(i);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit_generation() {
        let mut circuit = HyperbolicCircuit::new();
        // Threshold 2.0 covers a fair bit of distance in the center
        circuit.generate(50, 2.0);

        assert_eq!(circuit.pads.len(), 50);
        // Traces should exist (unless extremely unlucky or threshold too small)
        assert!(circuit.traces.len() > 0);

        // Verify adjacency symmetry
        for i in 0..50 {
            for &neighbor in &circuit.adjacency[i] {
                assert!(circuit.adjacency[neighbor].contains(&i));
            }
        }
    }
}
