use crate::audio::AudioCommand;
use crate::boid::{distance_squared, limit, Boid};
use crossbeam_channel::Sender;
use rand::Rng;

pub struct World {
    pub width: usize,
    pub height: usize,
    pub boids: Vec<Boid>,
    pub pressure_map: Vec<f32>,
    pub walls: Vec<bool>,
    pub audio_tx: Sender<AudioCommand>,
}

impl World {
    pub fn new(width: usize, height: usize, audio_tx: Sender<AudioCommand>) -> Self {
        let mut boids = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..40 {
            // Spawn in middle area
            let x = rng.gen_range((width / 4) as f64..(width * 3 / 4) as f64);
            let y = rng.gen_range((height / 4) as f64..(height * 3 / 4) as f64);
            boids.push(Boid::new(x, y));
        }

        Self {
            width,
            height,
            boids,
            pressure_map: vec![0.0; width * height],
            walls: vec![false; width * height],
            audio_tx,
        }
    }

    pub fn update(&mut self) {
        // We need a snapshot of boids to calculate N^2 forces
        let boids_snapshot = self.boids.clone();

        for i in 0..self.boids.len() {
            let mut force_sep = (0.0, 0.0);
            let mut force_ali = (0.0, 0.0);
            let mut force_coh = (0.0, 0.0);
            let mut force_sonar = (0.0, 0.0);

            let mut count_sep = 0;
            let mut count_ali = 0;
            let mut count_coh = 0;

            let mut center_of_mass = (0.0, 0.0);

            let boid = &mut self.boids[i];

            // Flocking Forces
            for other in &boids_snapshot {
                let d2 = distance_squared(boid.position, other.position);

                if d2 > 0.0 && d2 < boid.dna.view_radius.powi(2) {
                    // Separation
                    let diff = (
                        boid.position.0 - other.position.0,
                        boid.position.1 - other.position.1,
                    );
                    let d = d2.sqrt();
                    force_sep.0 += diff.0 / d; // Weight by distance? Usually 1/d or 1/d^2
                    force_sep.1 += diff.1 / d;
                    count_sep += 1;

                    // Alignment
                    force_ali.0 += other.velocity.0;
                    force_ali.1 += other.velocity.1;
                    count_ali += 1;

                    // Cohesion
                    center_of_mass.0 += other.position.0;
                    center_of_mass.1 += other.position.1;
                    count_coh += 1;
                }
            }

            if count_sep > 0 {
                force_sep.0 /= count_sep as f64;
                force_sep.1 /= count_sep as f64;
                force_sep = limit(force_sep, boid.dna.max_force);
            }

            if count_ali > 0 {
                force_ali.0 /= count_ali as f64;
                force_ali.1 /= count_ali as f64;
                force_ali = limit(force_ali, boid.dna.max_force);
            }

            if count_coh > 0 {
                center_of_mass.0 /= count_coh as f64;
                center_of_mass.1 /= count_coh as f64;
                let dir = (
                    center_of_mass.0 - boid.position.0,
                    center_of_mass.1 - boid.position.1,
                );
                force_coh = limit(dir, boid.dna.max_force);
            }

            // Sonar Force (Gradient Descent on Intensity)
            if !self.pressure_map.is_empty() {
                let x = boid.position.0 as usize;
                let y = boid.position.1 as usize;
                if x > 1 && x < self.width - 1 && y > 1 && y < self.height - 1 {
                    let idx = y * self.width + x;
                    let p_curr = self.pressure_map[idx].powi(2);
                    let p_right = self.pressure_map[idx + 1].powi(2);
                    let p_left = self.pressure_map[idx - 1].powi(2);
                    let p_down = self.pressure_map[idx + self.width].powi(2);
                    let p_up = self.pressure_map[idx - self.width].powi(2);

                    let grad_x = (p_right - p_left) * 0.5;
                    let grad_y = (p_down - p_up) * 0.5;

                    // Move away from high intensity -> Negative gradient
                    force_sonar.0 = -(grad_x as f64) * boid.dna.sonar_avoidance_weight;
                    force_sonar.1 = -(grad_y as f64) * boid.dna.sonar_avoidance_weight;
                }
            }

            // Apply Weighted Forces
            let w_sep = boid.dna.separation_weight;
            let w_ali = boid.dna.alignment_weight;
            let w_coh = boid.dna.cohesion_weight;

            boid.apply_force((force_sep.0 * w_sep, force_sep.1 * w_sep));
            boid.apply_force((force_ali.0 * w_ali, force_ali.1 * w_ali));
            boid.apply_force((force_coh.0 * w_coh, force_coh.1 * w_coh));
            boid.apply_force(force_sonar); // Already weighted by DNA

            // Random Pings
            let mut rng = rand::thread_rng();
            if rng.gen_bool(boid.dna.ping_probability) {
                let bx = boid.position.0 as usize;
                let by = boid.position.1 as usize;
                // Clamp to safe area
                if bx > 1 && bx < self.width - 1 && by > 1 && by < self.height - 1 {
                    let _ = self
                        .audio_tx
                        .send(AudioCommand::Pluck(bx, by, boid.dna.ping_strength));
                    boid.ping_timer = 5;
                }
            }

            boid.update_physics(self.width, self.height, &self.walls);
        }
    }
}
