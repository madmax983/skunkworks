use crate::boid::Boid;
use crate::strata::StrataManager;
use locus::Vec2;
use rand::Rng;

pub struct World {
    pub strata: StrataManager,
    pub boids: Vec<Boid>,
    pub width: f64,
    pub height: f64,
}

impl World {
    pub fn new(width: f64, height: f64) -> Self {
        let mut boids = Vec::new();
        // Create a nice swarm
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            boids.push(Boid::new(
                rng.gen_range(0.0..width),
                rng.gen_range(0.0..height),
            ));
        }

        Self {
            strata: StrataManager::new(width),
            boids,
            width,
            height,
        }
    }

    pub fn update(&mut self) {
        // Update Strata (grow fissures)
        self.strata.update();

        // Update Boids
        let boid_count = self.boids.len();
        let mut forces = Vec::with_capacity(boid_count);
        let mut nudges = vec![0.0; boid_count]; // Firefly phase nudges

        for i in 0..boid_count {
            let mut separation = Vec2::zero();
            let mut alignment = Vec2::zero();
            let mut cohesion = Vec2::zero();
            let mut fissure_attraction = Vec2::zero();

            let mut sep_count = 0;
            let mut ali_count = 0;
            let mut coh_count = 0;
            let mut fis_count = 0;

            let mut nudge = 0.0;

            let p1 = self.boids[i].position;
            let v1 = self.boids[i].velocity;
            let dna = self.boids[i].dna.clone();

            let view_radius_sq = dna.view_radius.powi(2);
            let coupling_radius_sq = dna.coupling_radius.powi(2);
            let separation_radius_sq = (dna.view_radius / 2.0).powi(2);

            // --- Flocking with Neighbors ---
            for j in 0..boid_count {
                if i == j {
                    continue;
                }

                let b2 = &self.boids[j];
                let dist_sq = p1.distance_squared(b2.position);

                if dist_sq > view_radius_sq && dist_sq > coupling_radius_sq {
                    continue;
                }

                if dist_sq < view_radius_sq {
                    // Separation
                    if dist_sq < separation_radius_sq {
                        let diff = p1 - b2.position;
                        separation += diff / dist_sq;
                        sep_count += 1;
                    }

                    // Alignment
                    alignment += b2.velocity;
                    ali_count += 1;

                    // Cohesion
                    cohesion += b2.position;
                    coh_count += 1;
                }

                // Firefly Coupling
                if dist_sq < coupling_radius_sq && b2.flash_timer > 3 {
                    nudge += dna.coupling_strength;
                }
            }

            // --- Fissure Attraction ---
            // Boids are attracted to active fissures
            let mut nearest_fissure_dist = f64::MAX;
            for fissure in &self.strata.fissures {
                if !fissure.active { continue; } // Only attracted to growing fissures? Or all? Let's say all.

                for point in &fissure.points {
                    // Adjust point for scroll
                    // Wait, fissure points are in absolute world coords?
                    // Strata scroll logic in tectonic-git was "scroll_y += ...".
                    // Here we need to account for visual position relative to boids.
                    // Let's assume boids live in the same coordinate space as fissures.
                    // If fissures scroll, boids should too? Or boids fly over static window?

                    // In tectonic-git, scroll_y moves the "camera". Strata.y_pos is absolute.
                    // Rendered Y = strata.y_pos - scroll_y + screen_height/2

                    // Here, let's say boids live in "Screen Space" or "World Space"?
                    // luminous-flock is screen space (0..width, 0..height).
                    // tectonic-git is world space (infinite Y).

                    // Hybrid solution:
                    // Boids live in Screen Space.
                    // We project Fissures into Screen Space to calculate attraction.

                    // Screen Y = Point.y - self.strata.scroll_y + self.height / 2.0; (roughly)
                    // Let's align coordinate systems.
                    // tectonic-git:
                    //   canvas y_min = center_y - view_height/2
                    //   draw at y

                    // Let's just use raw coordinates and assume the flock follows the scroll?
                    // No, simpler: The boids fly in the *viewport*.
                    // We check fissures that are currently visible.

                    // Visible range:
                    let visible_y_min = self.strata.scroll_y - self.height / 2.0;
                    let visible_y_max = self.strata.scroll_y + self.height / 2.0;

                    if point.y < visible_y_min || point.y > visible_y_max {
                        continue;
                    }

                    // Map point to screen space (0..height)
                    // point.y is absolute.
                    // screen_y = point.y - visible_y_min;
                    let screen_point = Vec2::new(
                        point.x,
                        point.y - visible_y_min
                    );

                    let dist_sq = p1.distance_squared(screen_point);
                    if dist_sq < view_radius_sq * 4.0 { // Can see fissures further away
                        fissure_attraction += screen_point;
                        fis_count += 1;
                        if dist_sq < nearest_fissure_dist {
                            nearest_fissure_dist = dist_sq;
                        }
                    }
                }
            }

            // Set nearby intensity for rendering
            // We'll store this in a temporary vector to avoid mutating self.boids during read
            // Actually, we can just apply it later

            // --- Apply Forces ---
            let mut total_force = Vec2::zero();

            if sep_count > 0 {
                separation = separation.normalize() * dna.max_speed;
                separation -= v1;
                separation = separation.limit(dna.max_force);
                total_force += separation * dna.separation_weight * 1.5; // Strong separation
            }

            if ali_count > 0 {
                alignment = alignment.normalize() * dna.max_speed;
                alignment -= v1;
                alignment = alignment.limit(dna.max_force);
                total_force += alignment * dna.alignment_weight;
            }

            if coh_count > 0 {
                cohesion /= coh_count as f64;
                cohesion -= p1;
                cohesion = cohesion.normalize() * dna.max_speed;
                cohesion -= v1;
                cohesion = cohesion.limit(dna.max_force);
                total_force += cohesion * dna.cohesion_weight;
            }

            if fis_count > 0 {
                fissure_attraction /= fis_count as f64;
                fissure_attraction -= p1;
                fissure_attraction = fissure_attraction.normalize() * dna.max_speed;
                fissure_attraction -= v1;
                fissure_attraction = fissure_attraction.limit(dna.max_force);
                total_force += fissure_attraction * 2.0; // Strong attraction to bugs!
            }

            forces.push(total_force);
            nudges[i] = nudge;
        }

        // Apply
        for (i, boid) in self.boids.iter_mut().enumerate() {
            boid.apply_force(forces[i]);
            boid.update_physics(self.width, self.height);

            boid.phase += boid.dna.natural_freq + nudges[i];
            boid.update_flash();

            // Recalculate nearby_fissure_intensity post-movement or just use the pre-movement value?
            // Let's re-check quickly or just random decay?
            // Actually, we didn't store the intensity calculation.
            // Let's just set it to 0 for now to avoid complexity or re-calculate.
            // Or better, let's store it in a `intensities` vec.
            boid.nearby_fissure_intensity = if nudges[i] > 0.0 { 1.0 } else { 0.0 }; // Hack: if nudged by firefly, glow?
            // No, that's firefly logic.
            // Let's just say if they are moving fast (attracted), they glow.
            if boid.velocity.magnitude_squared() > boid.dna.max_speed.powi(2) * 0.9 {
                boid.nearby_fissure_intensity = 1.0;
            } else {
                boid.nearby_fissure_intensity = 0.0;
            }
        }
    }
}
