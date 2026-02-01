use crate::boid::{Boid, DNA};
use rand::Rng;
use ratatui::style::Color;

#[derive(Clone, Debug)]
pub struct Food {
    pub position: (f64, f64),
    pub content: char,
}

pub struct World {
    pub boids: Vec<Boid>,
    pub food: Vec<Food>,
    pub width: f64,
    pub height: f64,
    pub text_source: Vec<char>,
    pub text_index: usize,
}

impl World {
    pub fn new(width: f64, height: f64, text: String) -> Self {
        let mut boids = Vec::new();
        // Initial population
        for _ in 0..50 {
            boids.push(Boid::new(width / 2.0, height / 2.0));
        }

        let text_source: Vec<char> = text.chars().filter(|c| !c.is_whitespace()).collect();

        let mut world = Self {
            boids,
            food: Vec::new(),
            width,
            height,
            text_source,
            text_index: 0,
        };

        // Initial food spawn
        for _ in 0..50 {
            world.spawn_food();
        }

        world
    }

    pub fn spawn_food(&mut self) {
        if self.text_index >= self.text_source.len() {
            self.text_index = 0; // Loop text
        }

        let char_to_spawn = self.text_source[self.text_index];
        self.text_index += 1;

        let mut rng = rand::rng();
        let x = rng.random_range(0.0..self.width);
        let y = rng.random_range(0.0..self.height);

        self.food.push(Food {
            position: (x, y),
            content: char_to_spawn,
        });
    }

    pub fn update(&mut self) {
        // 1. Calculate flocking forces
        // We need to clone boids or use a way to access them immutably while modifying others.
        // Since Boid struct is small enough, cloning the vector for read-access is acceptable for performance in this context (~100s of boids).
        // For millions, we'd use a spatial grid or separate arrays.
        let boids_snapshot = self.boids.clone();
        let mut forces = Vec::with_capacity(self.boids.len());

        for boid in &self.boids {
            forces.push(boid.calculate_flocking_force(&boids_snapshot));
        }

        // 2. Apply forces and update physics
        for (i, boid) in self.boids.iter_mut().enumerate() {
            boid.apply_force(forces[i]);
            boid.update(self.width, self.height);
        }

        // 3. Interactions (Eating)
        let mut eaten_indices = Vec::new();

        for (_boid_idx, boid) in self.boids.iter_mut().enumerate() {
            for (food_idx, food) in self.food.iter().enumerate() {
                if distance(boid.position, food.position) < 2.0 {
                    // Eating radius
                    // Eat
                    boid.energy += 20.0;
                    boid.dna.char_representation = food.content; // Transform into what you eat
                    eaten_indices.push(food_idx);
                    break; // One food per frame per boid? Or greedy? Let's say one.
                }
            }
        }

        // Remove eaten food (in reverse order to keep indices valid)
        eaten_indices.sort_unstable();
        eaten_indices.dedup();
        for &index in eaten_indices.iter().rev() {
            if index < self.food.len() {
                self.food.swap_remove(index);
                // Respawn food to keep the ecosystem going
                self.spawn_food();
            }
        }

        // 4. Reproduction
        let mut new_boids = Vec::new();
        for boid in &mut self.boids {
            if boid.energy > 150.0 {
                boid.energy -= 80.0; // Cost of reproduction
                let mut child = boid.clone();
                child.energy = 80.0;
                // Mutate DNA
                mutate_dna(&mut child.dna);
                new_boids.push(child);
            }
        }
        self.boids.append(&mut new_boids);

        // 5. Death
        self.boids.retain(|b| b.energy > 0.0);

        // Extinction prevention
        if self.boids.is_empty() {
            for _ in 0..10 {
                self.boids
                    .push(Boid::new(self.width / 2.0, self.height / 2.0));
            }
        }
    }
}

fn distance(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    ((p1.0 - p2.0).powi(2) + (p1.1 - p2.1).powi(2)).sqrt()
}

fn mutate_dna(dna: &mut DNA) {
    let mut rng = rand::rng();
    if rng.random_bool(0.1) {
        dna.max_speed += rng.random_range(-0.1..0.1);
        dna.max_speed = dna.max_speed.clamp(0.2, 2.0);
    }
    if rng.random_bool(0.1) {
        dna.view_radius += rng.random_range(-1.0..1.0);
        dna.view_radius = dna.view_radius.clamp(2.0, 20.0);
    }
    // Color mutation based on traits
    if dna.max_speed > 1.0 {
        dna.color = Color::Red;
    } else if dna.view_radius > 10.0 {
        dna.color = Color::Blue;
    } else {
        dna.color = Color::Green;
    }
}
