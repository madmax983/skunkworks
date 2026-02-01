use rand::Rng;
use std::f64::consts::PI;

#[derive(Clone, Debug)]
pub struct Firefly {
    pub x: f64,
    pub y: f64,
    pub phase: f64,
    pub natural_freq: f64,
    pub flash_timer: usize,
}

impl Firefly {
    pub fn new(x: f64, y: f64) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x,
            y,
            phase: rng.r#gen::<f64>(),
            // Frequency between 0.005 and 0.025
            natural_freq: 0.005 + rng.r#gen::<f64>() * 0.02,
            flash_timer: 0,
        }
    }
}

pub struct Swarm {
    pub fireflies: Vec<Firefly>,
    pub coupling: f64,
    pub radius: f64,
    pub width: f64,
    pub height: f64,
}

impl Swarm {
    pub fn new(count: usize, width: f64, height: f64) -> Self {
        let mut fireflies = Vec::with_capacity(count);
        let mut rng = rand::thread_rng();
        for _ in 0..count {
            fireflies.push(Firefly::new(
                rng.r#gen::<f64>() * width,
                rng.r#gen::<f64>() * height,
            ));
        }

        Self {
            fireflies,
            coupling: 0.01, // Small nudge
            radius: 15.0,   // Interaction radius
            width,
            height,
        }
    }

    pub fn update(&mut self) {
        let count = self.fireflies.len();
        let mut nudges = vec![0.0; count];

        // 1. Integrate natural motion
        for fly in self.fireflies.iter_mut() {
            fly.phase += fly.natural_freq;
            if fly.flash_timer > 0 {
                fly.flash_timer -= 1;
            }

            if fly.phase >= 1.0 {
                fly.phase -= 1.0;
                fly.flash_timer = 5; // Flash duration
            }
        }

        // 2. Calculate coupling (O(N^2))
        // We check who IS flashing (timer == 5 means they just started)
        for i in 0..count {
            if self.fireflies[i].flash_timer == 5 {
                for j in 0..count {
                    if i == j {
                        continue;
                    }

                    let dx = self.fireflies[i].x - self.fireflies[j].x;
                    let dy = self.fireflies[i].y - self.fireflies[j].y;
                    let dist_sq = dx * dx + dy * dy;

                    if dist_sq < self.radius * self.radius {
                        nudges[j] += self.coupling;
                    }
                }
            }
        }

        // 3. Apply nudges
        for (i, nudge) in nudges.iter().enumerate() {
            if *nudge > 0.0 {
                self.fireflies[i].phase += *nudge;
            }
        }
    }

    pub fn randomize_phases(&mut self) {
        let mut rng = rand::thread_rng();
        for fly in &mut self.fireflies {
            fly.phase = rng.r#gen::<f64>();
        }
    }

    pub fn synchronization_index(&self) -> f64 {
        let mut sum_sin = 0.0;
        let mut sum_cos = 0.0;

        for fly in &self.fireflies {
            let theta = fly.phase * 2.0 * PI;
            sum_sin += theta.sin();
            sum_cos += theta.cos();
        }

        let n = self.fireflies.len() as f64;
        if n == 0.0 {
            return 0.0;
        }
        ((sum_sin / n).powi(2) + (sum_cos / n).powi(2)).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase_wrapping() {
        let mut fly = Firefly::new(0.0, 0.0);
        fly.phase = 0.99;
        fly.natural_freq = 0.02;

        // Manual update logic simulation
        fly.phase += fly.natural_freq;
        if fly.phase >= 1.0 {
            fly.phase -= 1.0;
            fly.flash_timer = 5;
        }

        assert!(fly.phase < 1.0);
        assert_eq!(fly.flash_timer, 5);
    }

    #[test]
    fn test_swarm_sync_calc() {
        let mut swarm = Swarm::new(2, 100.0, 100.0);
        // Force sync
        swarm.fireflies[0].phase = 0.5;
        swarm.fireflies[1].phase = 0.5;

        let r = swarm.synchronization_index();
        assert!((r - 1.0).abs() < 1e-6);

        // Force anti-sync
        swarm.fireflies[0].phase = 0.0;
        swarm.fireflies[1].phase = 0.5; // PI difference
        let r2 = swarm.synchronization_index();
        assert!((r2 - 0.0).abs() < 1e-6);
    }
}
