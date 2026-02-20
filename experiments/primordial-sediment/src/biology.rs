use crate::physics::{FluidSolver, Species};
use rand::Rng;

impl FluidSolver {
    pub fn update_biology(&mut self, grid: &mut Vec<Vec<char>>) {
        let mut new_particles = Vec::new();
        let width = self.width;
        let height = self.height;
        let h2 = self.h * self.h; // Interaction radius

        // Track dead/eaten particles
        let mut dead_flags = vec![false; self.particles.len()];

        // 1. Metabolism & Feeding
        for (i, p) in self.particles.iter_mut().enumerate() {
            // Base metabolism
            p.energy -= 0.1;

            match p.species {
                Species::Algae => {
                    // Detritivore: Eat code
                    let col = p.x.round() as isize;
                    let row = p.y.round() as isize;

                    if row >= 0 && row < grid.len() as isize {
                        let r = row as usize;
                        if col >= 0 && col < grid[r].len() as isize {
                            let c = col as usize;
                            let char_at_pos = grid[r][c];

                            if !char_at_pos.is_whitespace() {
                                // Eat the char
                                p.energy += 5.0; // High energy from code
                                grid[r][c] = ' '; // Consumed
                            } else {
                                // Starvation if on empty space
                                p.energy -= 0.05;
                            }
                        }
                    }
                }
                Species::Grazer => {
                    p.energy -= 0.2; // Movement cost
                }
                Species::Predator => {
                    p.energy -= 0.4; // High metabolism
                }
            }

            // Boundary penalty (don't leave the screen)
            if p.x < 0.0 || p.x > width || p.y < 0.0 || p.y > height {
                p.energy -= 1.0;
            }

            if p.energy <= 0.0 {
                dead_flags[i] = true;
            }
        }

        // 2. Predation (Grazer eats Algae, Predator eats Grazer)
        for i in 0..self.particles.len() {
            if dead_flags[i] {
                continue;
            }
            if self.particles[i].species == Species::Algae {
                continue;
            } // Algae don't hunt

            for j in 0..self.particles.len() {
                if i == j {
                    continue;
                }
                if dead_flags[j] {
                    continue;
                }

                let dx = self.particles[j].x - self.particles[i].x;
                let dy = self.particles[j].y - self.particles[i].y;
                let r2 = dx * dx + dy * dy;

                if r2 < h2 {
                    let predator = self.particles[i].species;
                    let prey = self.particles[j].species;
                    let mut ate = false;

                    if predator == Species::Grazer && prey == Species::Algae {
                        self.particles[i].energy += 20.0;
                        ate = true;
                    } else if predator == Species::Predator && prey == Species::Grazer {
                        self.particles[i].energy += 50.0;
                        ate = true;
                    }

                    if ate {
                        dead_flags[j] = true;
                        // Limit to one meal per tick?
                        // break; // Uncomment to limit
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
                Species::Algae => 50.0, // Fast reproduction if code is abundant
                Species::Grazer => 150.0,
                Species::Predator => 300.0,
            };

            if p.energy > reproduce_threshold {
                p.energy *= 0.5;
                let mut child = *p;
                child.x += rng.gen_range(-2.0..2.0);
                child.y += rng.gen_range(-2.0..2.0);
                // Clamp child position
                child.x = child.x.max(0.0).min(width);
                child.y = child.y.max(0.0).min(height);

                child.energy = p.energy;
                child.vx = rng.gen_range(-1.0..1.0);
                child.vy = rng.gen_range(-1.0..1.0);

                new_particles.push(child);
            }
        }

        // Remove dead
        for i in (0..self.particles.len()).rev() {
            if dead_flags[i] {
                self.particles.swap_remove(i);
            }
        }

        // Add born
        self.particles.extend(new_particles);

        // Cap population to prevent lag
        if self.particles.len() > 1000 {
            let overflow = self.particles.len() - 1000;
            for _ in 0..overflow {
                if !self.particles.is_empty() {
                    self.particles
                        .swap_remove(rng.gen_range(0..self.particles.len()));
                }
            }
        }

        // Auto-seed Algae if extinct (Spontaneous Generation from "Rot")
        let algae_count = self
            .particles
            .iter()
            .filter(|p| p.species == Species::Algae)
            .count();
        if algae_count < 5 {
            // Spawn some algae at random locations
            for _ in 0..5 {
                self.add_particle(
                    rng.gen_range(0.0..width),
                    rng.gen_range(0.0..height),
                    Species::Algae,
                );
            }
        }
    }
}
