use crate::blame::LineInfo;
use anyhow::Result;
use locus::flocking::{compute_force, FlockingParams};
use locus::Vec2;
use rand::Rng;
use std::fs;

pub struct App {
    pub path: String,
    pub content: Vec<String>,
    pub blame_info: Vec<LineInfo>,
    pub scroll: usize,
    pub selected_line: usize,

    // Flock simulation
    pub positions: Vec<Vec2>,
    pub velocities: Vec<Vec2>,
    pub flock_params: FlockingParams,
    pub bounds: (f64, f64),
}

impl App {
    pub fn new(path: String, blame_info: Vec<LineInfo>) -> Result<Self> {
        let content_str = fs::read_to_string(&path)?;
        let content: Vec<String> = content_str.lines().map(|s| s.to_string()).collect();

        // Let's create some boids
        let mut rng = rand::thread_rng();
        let num_boids = 200;
        let bounds = (100.0, 100.0); // Will be updated by UI to canvas size

        let mut positions = Vec::with_capacity(num_boids);
        let mut velocities = Vec::with_capacity(num_boids);

        for _ in 0..num_boids {
            positions.push(Vec2::new(
                rng.gen_range(0.0..bounds.0),
                rng.gen_range(0.0..bounds.1),
            ));
            let angle = rng.gen_range(0.0..std::f64::consts::TAU);
            velocities.push(Vec2::new(angle.cos(), angle.sin()));
        }

        let flock_params = FlockingParams {
            view_radius: 10.0,
            separation_radius: 3.0,
            max_speed: 1.5,
            max_force: 0.1,
            separation_weight: 1.5,
            alignment_weight: 1.0,
            cohesion_weight: 1.0,
        };

        Ok(Self {
            path,
            content,
            blame_info,
            scroll: 0,
            selected_line: 0,
            positions,
            velocities,
            flock_params,
            bounds,
        })
    }

    pub fn next_line(&mut self) {
        if self.selected_line < self.content.len().saturating_sub(1) {
            self.selected_line += 1;
        }
    }

    pub fn previous_line(&mut self) {
        if self.selected_line > 0 {
            self.selected_line -= 1;
        }
    }

    pub fn update_scroll(&mut self, height: usize) {
        if height == 0 {
            return;
        }
        if self.selected_line >= self.scroll + height {
            self.scroll = self.selected_line + 1 - height;
        } else if self.selected_line < self.scroll {
            self.scroll = self.selected_line;
        }
    }

    pub fn update(&mut self) {
        let width = self.bounds.0;
        let height = self.bounds.1;

        if width <= 0.0 || height <= 0.0 {
            return;
        }

        let mut new_velocities = self.velocities.clone();

        // Compute flocking forces + code attraction
        for (i, vel_ref) in new_velocities
            .iter_mut()
            .enumerate()
            .take(self.positions.len())
        {
            let mut force = compute_force(&self.positions, &self.velocities, i, &self.flock_params);

            // Codebase Attraction
            // Each line of code in view has an age_score. New code (age_score near 1) attracts,
            // Old code (age_score near 0) repels.
            let boid_pos = self.positions[i];

            // Map boid_pos.y to a line index
            // Boid Y is 0 (bottom) to height (top) in traditional coordinates,
            // but in canvas it's up to us. Let's assume height corresponds to visible lines.
            // If we have `height` as terminal lines, let's roughly scale:
            let visible_lines = self.content.len().min(height as usize * 2); // approximate
            let mut text_force = Vec2::zero();

            // To prevent N^2 between boids and all lines, just sample the lines nearest to the boid
            let approx_line_idx =
                ((height - boid_pos.y) / height * visible_lines as f64) as usize + self.scroll;

            let start_idx = approx_line_idx.saturating_sub(5);
            let end_idx = (approx_line_idx + 5).min(self.blame_info.len());

            for line_idx in start_idx..end_idx {
                let info = &self.blame_info[line_idx];

                // mapped y coord of this line
                let line_canvas_y =
                    height - ((line_idx - self.scroll) as f64 / visible_lines as f64 * height);
                // mapped x coord - maybe just left aligned or attract to x=10?
                let line_canvas_x = 20.0;

                let line_pos = Vec2::new(line_canvas_x, line_canvas_y);
                let dist_sq = boid_pos.distance_squared(line_pos);

                if dist_sq > 0.0 && dist_sq < 100.0 {
                    let dir = (line_pos - boid_pos).normalize();
                    // New code (score ~ 1.0) attracts, old code (score ~ 0.0) repels slightly
                    let weight = (info.age_score - 0.5) * 0.2;
                    text_force += dir * weight;
                }
            }

            force += text_force;

            // Apply force
            let mut vel = self.velocities[i] + force;

            if vel.magnitude_squared() > self.flock_params.max_speed * self.flock_params.max_speed {
                vel = vel.normalize() * self.flock_params.max_speed;
            }
            *vel_ref = vel;
        }

        self.velocities = new_velocities;

        // Apply velocities & topology
        for (pos, vel) in self.positions.iter_mut().zip(self.velocities.iter()) {
            *pos += *vel;

            // Torus wrap
            if pos.x < 0.0 {
                pos.x += width;
            } else if pos.x >= width {
                pos.x -= width;
            }

            if pos.y < 0.0 {
                pos.y += height;
            } else if pos.y >= height {
                pos.y -= height;
            }
        }
    }
}
