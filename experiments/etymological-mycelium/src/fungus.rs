use macroquad::prelude::*;
use std::f32::consts::PI;
use ::rand::Rng; // Use the external rand crate explicitly

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct Spore {
    pub pos: Vec2,
    pub target: Vec2,
    pub velocity: Vec2,
    pub age: f32,
    pub max_age: f32,
    pub color: Color,
    pub active: bool,
    pub semantic_similarity: f32, // 0.0 to 1.0 (1.0 = identical)
    pub branching_factor: f32,
}

pub struct Mycelium {
    pub spores: Vec<Spore>,
    pub trail: Vec<(Vec2, Vec2, Color)>, // P1, P2, Color
}

impl Mycelium {
    pub fn new() -> Self {
        Self {
            spores: Vec::new(),
            trail: Vec::new(),
        }
    }

    pub fn spawn(&mut self, start: Vec2, target: Vec2, similarity: f32) {
        let distance = start.distance(target);

        // Similar = Stable Green
        // Dissimilar = Mutation Red
        let color = if similarity > 0.95 {
            Color::new(0.2, 0.8, 0.2, 0.6)
        } else if similarity > 0.6 {
            Color::new(0.8, 0.8, 0.2, 0.6)
        } else {
            Color::new(0.8, 0.2, 0.2, 0.6)
        };

        self.spores.push(Spore {
            pos: start,
            target,
            velocity: (target - start).normalize_or_zero(),
            age: 0.0,
            max_age: distance * 2.0, // Give enough time to reach
            color,
            active: true,
            semantic_similarity: similarity,
            branching_factor: (1.0 - similarity).max(0.1),
        });
    }

    pub fn update(&mut self) {
        let mut new_spores = Vec::new();
        let mut rng = ::rand::thread_rng();

        for spore in &mut self.spores {
            if !spore.active { continue; }

            let old_pos = spore.pos;

            // Vector to target
            let to_target = spore.target - spore.pos;
            let dist = to_target.length();

            if dist < 5.0 {
                // Reached target
                spore.pos = spore.target;
                spore.active = false;
                self.trail.push((old_pos, spore.pos, spore.color));
                continue;
            }

            let direction = to_target.normalize_or_zero();

            // Random noise vector
            let angle = rng.gen_range(0.0..2.0 * PI);
            let noise = vec2(angle.cos(), angle.sin());

            // Weights
            // If similar, high target weight, low noise
            // If dissimilar, low target weight, high noise
            let w_target = 0.05 + spore.semantic_similarity * 0.2;
            let w_noise = (1.0 - spore.semantic_similarity) * 0.1;

            let steer = direction * w_target + noise * w_noise;
            spore.velocity = (spore.velocity + steer).normalize_or_zero();

            // Speed varies
            let speed = 2.0 + spore.semantic_similarity * 2.0;
            spore.pos += spore.velocity * speed;
            spore.age += 1.0;

            if spore.age > spore.max_age {
                spore.active = false; // Died of old age
            }

            self.trail.push((old_pos, spore.pos, spore.color));

            // Branching for mutations
            if spore.semantic_similarity < 0.8 && rng.gen_bool(0.01) {
                // Create a decorative branch
                let branch_angle = rng.gen_range(0.0..2.0*PI);
                let branch_dir = vec2(branch_angle.cos(), branch_angle.sin());

                new_spores.push(Spore {
                    pos: spore.pos,
                    target: spore.pos + branch_dir * 30.0, // Short decorative branch
                    velocity: branch_dir,
                    age: 0.0,
                    max_age: 20.0,
                    color: Color::new(spore.color.r, spore.color.g, spore.color.b, 0.2),
                    active: true,
                    semantic_similarity: spore.semantic_similarity,
                    branching_factor: 0.0,
                });
            }
        }

        // Add new spores
        self.spores.extend(new_spores);
        // Remove dead spores
        self.spores.retain(|s| s.active);
    }

    pub fn draw(&self) {
        for (p1, p2, color) in &self.trail {
            draw_line(p1.x, p1.y, p2.x, p2.y, 1.5, *color);
        }

        // Draw active tips
        for spore in &self.spores {
            draw_circle(spore.pos.x, spore.pos.y, 1.0, WHITE);
        }
    }
}
