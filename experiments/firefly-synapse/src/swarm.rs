use rand::Rng;
use rayon::prelude::*;
use std::f32::consts::PI;

#[derive(Clone, Copy, Debug)]
pub struct Firefly {
    pub phase: f32,         // 0.0 to 1.0
    pub natural_freq: f32,  // Rate of phase increase
    pub flashed: bool,      // Did it flash in the last step?
}

pub struct Swarm {
    pub fireflies: Vec<Firefly>,
    pub width: usize,
    pub height: usize,
    pub coupling_strength: f32, // K: How much a neighbor's flash affects me
    pub dt: f32,
}

impl Swarm {
    pub fn new(width: usize, height: usize) -> Self {
        let mut rng = rand::thread_rng();
        let size = width * height;
        let fireflies = (0..size)
            .map(|_| Firefly {
                phase: rng.gen_range(0.0..1.0),
                natural_freq: rng.gen_range(0.8..1.2), // Slight variance
                flashed: false,
            })
            .collect();

        Self {
            fireflies,
            width,
            height,
            coupling_strength: 0.01, // Small nudge
            dt: 0.1,
        }
    }

    pub fn update(&mut self) {
        // 1. Identify who is flashing based on current state (from previous step's update)
        // Actually, we determine flashing based on if phase crossed 1.0 during the update.
        // But for parallel updates, we need a read-only snapshot of 'who flashed'.

        // Let's use a double-buffer approach logic implicitly.
        // We need to know who flashed *at the end of the last frame* or *during this frame*.
        // Let's say: At start of frame, we have phases.
        // We calculate new phases. If phase crosses 1.0, we mark 'flashed' for the NEXT frame to see?
        // No, standard CA: State T -> State T+1.
        // Neighbors at T determine input for T+1.
        // So, if I flashed at T, neighbors see it at T+1.

        let width = self.width;
        let height = self.height;
        let coupling = self.coupling_strength;
        let dt = self.dt;

        // Snapshot of current flash state to read from
        let current_flashes: Vec<bool> = self.fireflies.iter().map(|f| f.flashed).collect();

        // Update phases
        self.fireflies.par_iter_mut().enumerate().for_each(|(i, fly)| {
            let x = i % width;
            let y = i / width;

            // Count flashing neighbors (Moore neighborhood or Von Neumann?)
            // Let's use Moore (8 neighbors)
            let mut nudges = 0;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 { continue; }

                    let nx = x as isize + dx;
                    let ny = y as isize + dy;

                    if nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                        let ni = ny as usize * width + nx as usize;
                        if current_flashes[ni] {
                            nudges += 1;
                        }
                    }
                }
            }

            // Pulse Coupling:
            // phase += natural_freq * dt + (nudges * K)
            fly.phase += fly.natural_freq * dt + (nudges as f32 * coupling);

            // Check flash
            if fly.phase >= 1.0 {
                fly.phase = 0.0; // Reset
                fly.flashed = true;
            } else {
                fly.flashed = false;
            }
        });
    }

    /// Calculate global synchronization order parameter 'r'
    /// r = |(1/N) * sum(e^(i * theta))| where theta = phase * 2PI
    pub fn synchronization_index(&self) -> f32 {
        let (sum_sin, sum_cos) = self.fireflies.par_iter()
            .map(|f| {
                let theta = f.phase * 2.0 * PI;
                theta.sin_cos()
            })
            .reduce(|| (0.0, 0.0), |a, b| (a.0 + b.0, a.1 + b.1));

        let n = self.fireflies.len() as f32;
        ((sum_sin / n).powi(2) + (sum_cos / n).powi(2)).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_firefly_update() {
        let mut swarm = Swarm::new(10, 10);
        // Force a configuration
        swarm.fireflies[0].phase = 0.95;
        swarm.fireflies[0].natural_freq = 1.0;
        swarm.dt = 0.1;

        // 0.95 + 0.1 = 1.05 -> Should flash
        swarm.update();
        assert!(swarm.fireflies[0].flashed);
        assert_eq!(swarm.fireflies[0].phase, 0.0);
    }

    #[test]
    fn test_coupling() {
        let mut swarm = Swarm::new(3, 3);
        // Center fly (1,1) index 4
        // Top left fly (0,0) index 0

        // Set everyone to 0
        for f in &mut swarm.fireflies {
            f.phase = 0.0;
            f.natural_freq = 0.0; // Simplify: no natural growth
            f.flashed = false;
        }

        // Set neighbor (0,0) to HAVE flashed in previous theoretical step
        // But wait, our update logic reads `f.flashed` which was set in previous update.
        // So we need to manually set the state as if a frame just finished.
        swarm.fireflies[0].flashed = true;

        swarm.coupling_strength = 0.1;
        swarm.dt = 0.0; // No natural time passing

        swarm.update();

        // Center fly should have received a nudge
        // It is a neighbor of (0,0)? (0,0) is x=0, y=0. (1,1) is x=1, y=1.
        // dx=1, dy=1. Yes.

        assert_eq!(swarm.fireflies[4].phase, 0.1);
    }
}
