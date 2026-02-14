use crate::boid::Boid;
use crate::fissure::Fissure;
use crate::git::{CommitData, GitScanner};
use crate::strata::Strata;
use flocking::{compute_force, FlockingParams, PhysicsState};
use locus::Vec2;
use rand::Rng;

pub struct World {
    pub boids: Vec<Boid>,
    pub strata: Vec<Strata>,
    pub fissures: Vec<Fissure>,
    pub width: f64,
    pub height: f64,
    pub scroll_y: f64,
}

impl World {
    pub fn new(width: f64, height: f64) -> Self {
        let mut rng = rand::thread_rng();

        // 1. Load Git History
        let commits = GitScanner::scan().unwrap_or_else(|_| {
            // Fallback for no-git env
            vec![
                CommitData {
                    hash: "init".into(),
                    timestamp: 0,
                    stress_level: 0.0,
                    details: "Initial".into(),
                },
                CommitData {
                    hash: "fix".into(),
                    timestamp: 100,
                    stress_level: 5.0,
                    details: "Fix panic".into(),
                },
                CommitData {
                    hash: "feat".into(),
                    timestamp: 200,
                    stress_level: 0.0,
                    details: "Add feature".into(),
                },
                CommitData {
                    hash: "refactor".into(),
                    timestamp: 300,
                    stress_level: 2.0,
                    details: "Refactor".into(),
                },
            ]
        });

        // 2. Create Strata
        let mut strata = Vec::new();
        let mut fissures = Vec::new();
        let strata_height = 5.0;

        let mut prev_offset = 0.0;

        for (i, commit) in commits.iter().enumerate() {
            let y_pos = i as f64 * strata_height;

            // Calculate shift
            let shift = if commit.stress_level > 2.0 {
                rng.gen_range(-2.0..2.0)
            } else {
                0.0
            };
            let offset_x = prev_offset + shift;
            prev_offset = offset_x;

            strata.push(Strata::new(commit.clone(), y_pos, offset_x));

            // Create Fissure if stressed
            if commit.stress_level > 1.0 {
                let start_x = rng.gen_range(width * 0.2..width * 0.8);
                // Fissure grows perpendicular to strata.
                // Since strata are horizontal (y is constant-ish), fissure grows up/down.
                fissures.push(Fissure::new(
                    Vec2::new(start_x, y_pos + strata_height / 2.0),
                    commit.stress_level,
                    if rng.gen_bool(0.5) { 1.0 } else { -1.0 },
                ));
            }
        }

        // 3. Create Boids
        let mut boids = Vec::new();
        for _ in 0..100 {
            boids.push(Boid::new(
                rng.gen_range(0.0..width),
                rng.gen_range(0.0..height),
            ));
        }

        Self {
            boids,
            strata,
            fissures,
            width,
            height,
            scroll_y: 0.0,
        }
    }

    pub fn update(&mut self) {
        // Scroll
        self.scroll_y += 0.2;
        let max_scroll = (self.strata.len() as f64 * 5.0) - self.height;
        if self.scroll_y > max_scroll {
            self.scroll_y = 0.0; // Loop? Or stop? Let's loop.
        }

        // Update Fissures
        for fissure in &mut self.fissures {
            if fissure.active {
                fissure.grow();
            }
        }

        // Update Boids
        let count = self.boids.len();
        let physics_states: Vec<PhysicsState> = self.boids.iter().map(|b| b.physics).collect();
        let mut forces = Vec::with_capacity(count);

        // Calculate forces
        for (i, boid) in self.boids.iter().enumerate() {
            let p1 = boid.position();
            let v1 = boid.physics.velocity;
            let dna = &boid.dna;

            let mut fissure_attract = Vec2::zero();

            // Flocking (using crate)
            let params = FlockingParams {
                view_radius: dna.view_radius,
                separation_radius: dna.view_radius / 2.0,
                max_speed: dna.max_speed,
                max_force: dna.max_force,
                separation_weight: dna.separation_weight,
                alignment_weight: dna.alignment_weight,
                cohesion_weight: dna.cohesion_weight,
            };

            let flocking_force = compute_force(
                &physics_states,
                i,
                &params
            );

            // Fissure Attraction
            // Find nearest ACTIVE fissure within view
            let mut closest_fissure_dist = f64::MAX;
            let mut closest_fissure_pt = Vec2::zero();
            let mut found_fissure = false;

            for fissure in &self.fissures {
                for pt in &fissure.points {
                    let screen_y = pt.y - self.scroll_y;
                    if screen_y >= 0.0 && screen_y <= self.height {
                        let screen_pt = Vec2::new(pt.x, screen_y);
                        let d_sq = p1.distance_squared(screen_pt);
                        if d_sq < (dna.view_radius * 3.0).powi(2) {
                            if d_sq < closest_fissure_dist {
                                closest_fissure_dist = d_sq;
                                closest_fissure_pt = screen_pt;
                                found_fissure = true;
                            }
                        }
                    }
                }
            }

            if found_fissure {
                let mut desired = closest_fissure_pt - p1;
                desired = desired.normalize() * dna.max_speed;
                desired -= v1;
                desired = desired.limit(dna.max_force);
                fissure_attract += desired * dna.fissure_attraction_weight;
            }

            // Combine Forces
            let total = flocking_force + fissure_attract;
            forces.push(total);
        }

        // Apply Forces
        for (i, boid) in self.boids.iter_mut().enumerate() {
            boid.apply_force(forces[i]);
            boid.update_physics(self.width, self.height);
            // Flash phase update (keep it simple, random drift for now, or copy sync logic if wanted)
            boid.phase += boid.dna.natural_freq;
            boid.update_flash();
        }
    }
}
