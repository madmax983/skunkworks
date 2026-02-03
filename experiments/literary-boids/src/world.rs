use crate::boid::{Boid, DNA, Vec2, distance};
#[cfg(feature = "nova")]
use crate::critic::Critic;
#[cfg(feature = "nova")]
use crate::syntax_physics;
use rand::Rng;
use ratatui::style::Color;

#[derive(Clone, Debug)]
pub struct Food {
    pub position: Vec2,
    pub content: char,
}

pub struct World {
    pub boids: Vec<Boid>,
    pub food: Vec<Food>,
    #[cfg(feature = "nova")]
    pub critics: Vec<Critic>,
    pub width: f64,
    pub height: f64,
    pub text_source: Vec<char>,
    pub text_index: usize,

    // Scratch buffers to avoid allocations
    forces_buffer: Vec<Vec2>,
    eaten_indices_buffer: Vec<usize>,
    new_boids_buffer: Vec<Boid>,
    #[cfg(feature = "nova")]
    eaten_boid_indices_buffer: Vec<usize>,
    #[cfg(feature = "nova")]
    critic_forces_buffer: Vec<Vec2>,
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
            #[cfg(feature = "nova")]
            critics: Vec::new(),
            width,
            height,
            text_source,
            text_index: 0,
            forces_buffer: Vec::new(),
            eaten_indices_buffer: Vec::new(),
            new_boids_buffer: Vec::new(),
            #[cfg(feature = "nova")]
            eaten_boid_indices_buffer: Vec::new(),
            #[cfg(feature = "nova")]
            critic_forces_buffer: Vec::new(),
        };

        #[cfg(feature = "nova")]
        {
            // Spawn one critic
            world.critics.push(Critic::new(width / 2.0, height / 2.0));
        }

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

        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0.0..self.width);
        let y = rng.gen_range(0.0..self.height);

        self.food.push(Food {
            position: Vec2::new(x, y),
            content: char_to_spawn,
        });
    }

    pub fn update(&mut self) {
        // 1. Calculate flocking forces
        // We use a persistent buffer to avoid allocation
        self.forces_buffer.clear();
        self.forces_buffer.reserve(self.boids.len());

        for boid in &self.boids {
            #[allow(unused_mut)]
            let mut force = boid.calculate_flocking_force(&self.boids);

            #[cfg(feature = "nova")]
            {
                // Flee from critics
                for critic in &self.critics {
                    let flee_force = crate::critic::flee(
                        boid.position,
                        boid.velocity,
                        critic.position,
                        boid.dna.max_speed,
                        boid.dna.max_force,
                    );
                    force += flee_force;
                }
            }
            self.forces_buffer.push(force);
        }

        // 2. Apply forces and update physics
        for (i, boid) in self.boids.iter_mut().enumerate() {
            boid.apply_force(self.forces_buffer[i]);
            boid.update(self.width, self.height);
        }

        #[cfg(feature = "nova")]
        {
            // Update critics
            self.eaten_boid_indices_buffer.clear();

            // Calculate forces for critics
            self.critic_forces_buffer.clear();
            for critic in &self.critics {
                self.critic_forces_buffer.push(critic.hunt(&self.boids));
            }

            for (i, critic) in self.critics.iter_mut().enumerate() {
                let f = self.critic_forces_buffer[i];
                critic.apply_force(f);
                critic.update(self.width, self.height);

                // Eat boids
                for (b_idx, boid) in self.boids.iter().enumerate() {
                    if distance(critic.position, boid.position) < critic.kill_radius {
                        self.eaten_boid_indices_buffer.push(b_idx);
                    }
                }
            }

            // Remove eaten boids
            self.eaten_boid_indices_buffer.sort_unstable();
            self.eaten_boid_indices_buffer.dedup();
            for &index in self.eaten_boid_indices_buffer.iter().rev() {
                if index < self.boids.len() {
                    self.boids.swap_remove(index);
                }
            }
        }

        // 3. Interactions (Eating)
        self.eaten_indices_buffer.clear();

        for boid in self.boids.iter_mut() {
            for (food_idx, food) in self.food.iter().enumerate() {
                if distance(boid.position, food.position) < 2.0 {
                    // Eating radius
                    // Eat
                    boid.energy += 20.0;
                    boid.dna.char_representation = food.content; // Transform into what you eat

                    #[cfg(feature = "nova")]
                    syntax_physics::apply_syntax_mutation(&mut boid.dna, food.content);

                    self.eaten_indices_buffer.push(food_idx);
                    break; // One food per frame per boid? Or greedy? Let's say one.
                }
            }
        }

        // Remove eaten food (in reverse order to keep indices valid)
        self.eaten_indices_buffer.sort_unstable();
        self.eaten_indices_buffer.dedup();
        for i in (0..self.eaten_indices_buffer.len()).rev() {
            let index = self.eaten_indices_buffer[i];
            if index < self.food.len() {
                self.food.swap_remove(index);
                // Respawn food to keep the ecosystem going
                self.spawn_food();
            }
        }

        // 4. Reproduction
        self.new_boids_buffer.clear();
        for boid in &mut self.boids {
            if boid.energy > 150.0 {
                boid.energy -= 80.0; // Cost of reproduction
                let mut child = boid.clone();
                child.energy = 80.0;
                // Mutate DNA
                mutate_dna(&mut child.dna);
                self.new_boids_buffer.push(child);
            }
        }
        self.boids.append(&mut self.new_boids_buffer);

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

fn mutate_dna(dna: &mut DNA) {
    let mut rng = rand::thread_rng();
    if rng.gen_bool(0.1) {
        dna.max_speed += rng.gen_range(-0.1..0.1);
        dna.max_speed = dna.max_speed.clamp(0.2, 2.0);
    }
    if rng.gen_bool(0.1) {
        dna.view_radius += rng.gen_range(-1.0..1.0);
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
