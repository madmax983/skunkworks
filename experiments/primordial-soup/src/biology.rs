use crate::physics::{FluidSolver, Species};
use rand::Rng;

impl FluidSolver {
    pub fn update_biology(&mut self) {
        let mut new_particles = Vec::new();
        let height = self.height;
        let h2 = self.h * self.h; // Interaction radius

        // Track dead and eaten particles
        // We use a set or boolean flag to avoid O(N) search in "contains" if possible,
        // but for now Vec is fine for small N.
        let mut dead_flags = vec![false; self.particles.len()];

        // 1. Metabolism & Photosynthesis
        for (i, p) in self.particles.iter_mut().enumerate() {
            // Base metabolism
            p.energy -= 0.1;

            match p.species {
                Species::Algae => {
                    // Photosynthesis: Gain energy if near surface
                    if p.y < height * 0.4 {
                        p.energy += 0.8;
                    }
                }
                Species::Grazer => {
                    p.energy -= 0.15; // Movement cost
                }
                Species::Predator => {
                    p.energy -= 0.3; // High metabolism
                }
            }

            if p.energy <= 0.0 {
                dead_flags[i] = true;
            }
        }

        // 2. Interactions (Eating)
        // We iterate and modify energies. If eaten, mark dead.
        // To avoid conflicts (A eats B, C eats B), we check dead_flags.

        for i in 0..self.particles.len() {
            if dead_flags[i] {
                continue;
            }

            // Only Grazers and Predators eat
            if self.particles[i].species == Species::Algae {
                continue;
            }

            for j in 0..self.particles.len() {
                if i == j {
                    continue;
                }
                if dead_flags[j] {
                    continue;
                } // Already dead/eaten

                let dx = self.particles[j].x - self.particles[i].x;
                let dy = self.particles[j].y - self.particles[i].y;
                let r2 = dx * dx + dy * dy;

                if r2 < h2 {
                    let prey_species = self.particles[j].species;
                    let predator_species = self.particles[i].species;

                    let mut ate = false;
                    if predator_species == Species::Grazer && prey_species == Species::Algae {
                        self.particles[i].energy += 30.0;
                        ate = true;
                    } else if predator_species == Species::Predator
                        && prey_species == Species::Grazer
                    {
                        self.particles[i].energy += 60.0;
                        ate = true;
                    }

                    if ate {
                        dead_flags[j] = true;
                        // Limit eating per tick? One bite per tick per predator.
                        break;
                    }
                }
            }
        }

        // 3. Reproduction
        let mut rng = rand::thread_rng();
        for (i, p) in self.particles.iter_mut().enumerate() {
            if dead_flags[i] {
                continue;
            }

            let reproduce_threshold = match p.species {
                Species::Algae => 120.0,
                Species::Grazer => 250.0,
                Species::Predator => 400.0,
            };

            if p.energy > reproduce_threshold {
                p.energy *= 0.5; // Split energy
                let mut child = *p;
                child.x += rng.gen_range(-1.0..1.0);
                child.y += rng.gen_range(-1.0..1.0);
                child.energy = p.energy; // Child gets half
                new_particles.push(child);
            }
        }

        // Apply deaths
        // We traverse in reverse to swap_remove safely
        for i in (0..self.particles.len()).rev() {
            if dead_flags[i] {
                self.particles.swap_remove(i);
            }
        }

        // Apply births
        self.particles.extend(new_particles);

        // Cap population
        if self.particles.len() > 800 {
            let to_remove = self.particles.len() - 800;
            for _ in 0..to_remove {
                if !self.particles.is_empty() {
                    self.particles
                        .swap_remove(rng.gen_range(0..self.particles.len()));
                }
            }
        }

        // Auto-reseed if extinct
        let algae_count = self
            .particles
            .iter()
            .filter(|p| p.species == Species::Algae)
            .count();
        if algae_count < 10 {
            for _ in 0..10 {
                self.add_particle(
                    rng.gen_range(0.0..self.width),
                    rng.gen_range(0.0..self.height * 0.3),
                    Species::Algae,
                );
            }
        }
    }
}
