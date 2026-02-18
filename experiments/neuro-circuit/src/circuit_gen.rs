use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use sha2::{Digest, Sha256};

#[derive(Clone, Debug)]
pub struct Trace {
    pub start_pad: usize,
    pub end_pad: usize,
    pub path: Vec<(u32, u32)>,
}

pub struct Circuit {
    pub width: u32,
    pub height: u32,
    pub pads: Vec<(u32, u32)>,
    pub traces: Vec<Trace>,
}

pub struct CircuitGenerator {
    width: u32,
    height: u32,
}

impl CircuitGenerator {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn generate(&self, hash_seed: &str) -> Circuit {
        let mut hasher = Sha256::new();
        hasher.update(hash_seed.as_bytes());
        let result = hasher.finalize();
        let seed: [u8; 32] = result.into();
        let mut rng = ChaCha20Rng::from_seed(seed);

        let mut pads = Vec::new();
        let mut traces = Vec::new();

        let num_pads = rng.gen_range(20..40);
        let min_dist_sq = (30 * 30) as i32; // Spread them out

        // Generate Pads
        for _ in 0..num_pads {
            for _ in 0..100 {
                // Max attempts per pad
                let x = rng.gen_range(20..self.width - 20);
                let y = rng.gen_range(20..self.height - 20);

                let mut collision = false;
                for &(px, py) in &pads {
                    let dx = x as i32 - px as i32;
                    let dy = y as i32 - py as i32;
                    if dx * dx + dy * dy < min_dist_sq {
                        collision = true;
                        break;
                    }
                }

                if !collision {
                    pads.push((x, y));
                    break;
                }
            }
        }

        // Generate Traces
        // Strategy: Ensure every pad has at least one outgoing connection to a neighbor
        // plus some random long-distance connections.

        for i in 0..pads.len() {
            // Connect to 1-3 other pads
            let num_connections = rng.gen_range(1..=3);

            // Sort potential targets by distance to find neighbors
            let mut targets: Vec<(usize, i32)> = pads
                .iter()
                .enumerate()
                .map(|(idx, &(px, py))| {
                    let dx = pads[i].0 as i32 - px as i32;
                    let dy = pads[i].1 as i32 - py as i32;
                    (idx, dx * dx + dy * dy)
                })
                .filter(|(idx, _)| *idx != i)
                .collect();

            targets.sort_by_key(|k| k.1);

            // Pick some nearest neighbors and maybe one random one
            let mut chosen_indices = Vec::new();

            // 50% chance to connect to nearest
            if !targets.is_empty() && rng.gen_bool(0.7) {
                chosen_indices.push(targets[0].0);
            }

            // Random others
            for _ in 0..num_connections {
                let target_idx = rng.gen_range(0..pads.len());
                if i != target_idx && !chosen_indices.contains(&target_idx) {
                    chosen_indices.push(target_idx);
                }
            }

            for target_idx in chosen_indices {
                let start = pads[i];
                let end = pads[target_idx];
                let path = self.generate_trace_path(start, end, &mut rng);

                traces.push(Trace {
                    start_pad: i,
                    end_pad: target_idx,
                    path,
                });
            }
        }

        Circuit {
            width: self.width,
            height: self.height,
            pads,
            traces,
        }
    }

    fn generate_trace_path(
        &self,
        start: (u32, u32),
        end: (u32, u32),
        rng: &mut ChaCha20Rng,
    ) -> Vec<(u32, u32)> {
        let mut path = Vec::new();
        let (x1, y1) = start;
        let (x2, y2) = end;

        // Coin flip for horizontal-first or vertical-first
        let h_first = rng.gen_bool(0.5);

        // Add some jitter/dogleg? No, stick to L-shapes for clean PCB look.

        if h_first {
            // (x1, y1) -> (x2, y1) -> (x2, y2)
            self.add_segment(&mut path, x1, y1, x2, y1);
            // Avoid duplicate point at corner
            if !path.is_empty() {
                path.pop();
            }
            self.add_segment(&mut path, x2, y1, x2, y2);
        } else {
            // (x1, y1) -> (x1, y2) -> (x2, y2)
            self.add_segment(&mut path, x1, y1, x1, y2);
            if !path.is_empty() {
                path.pop();
            }
            self.add_segment(&mut path, x1, y2, x2, y2);
        }

        path
    }

    fn add_segment(&self, path: &mut Vec<(u32, u32)>, x0: u32, y0: u32, x1: u32, y1: u32) {
        let dx = (x1 as i32 - x0 as i32).signum();
        let dy = (y1 as i32 - y0 as i32).signum();

        let mut x = x0 as i32;
        let mut y = y0 as i32;

        let dist = ((x1 as i32 - x0 as i32).abs() + (y1 as i32 - y0 as i32).abs()) as u32;

        for _ in 0..=dist {
            path.push((x as u32, y as u32));
            x += dx;
            y += dy;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit_generation() {
        let gen = CircuitGenerator::new(800, 600);
        let circuit = gen.generate("test_seed");

        assert!(circuit.width == 800);
        assert!(circuit.height == 600);
        assert!(!circuit.pads.is_empty(), "Should generate pads");
        assert!(!circuit.traces.is_empty(), "Should generate traces");

        // Verify traces start and end at valid pads
        for trace in &circuit.traces {
            assert!(trace.start_pad < circuit.pads.len());
            assert!(trace.end_pad < circuit.pads.len());
            assert!(!trace.path.is_empty());
        }
    }
}
