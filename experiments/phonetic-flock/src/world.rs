use crate::boid::{distance_squared, Boid, Vec2};
use crate::phonology::SoundLaw;
use rand::Rng;
use ratatui::style::Color;

pub struct World {
    pub boids: Vec<Boid>,
    pub width: f64,
    pub height: f64,

    // Scratch buffers
    forces_buffer: Vec<Vec2>,
}

impl World {
    pub fn new(width: f64, height: f64) -> Self {
        let text = "One morning, when Gregor Samsa woke from troubled dreams, he found himself transformed in his bed into a horrible vermin. He lay on his armour-like back, and if he lifted his head a little he could see his brown belly, slightly domed and divided by arches into stiff sections. The bedding was hardly able to cover it and seemed ready to slide off any moment. His many legs, pitifully thin compared with the size of the rest of him, waved about helplessly as he looked. \"What's happened to me?\" he thought. It wasn't a dream. His room, a proper human room although a little too small, lay peacefully between its four familiar walls. A collection of textile samples lay spread out on the table - Samsa was a travelling salesman - and above it there hung a picture that he had recently cut out of an illustrated magazine and housed in a nice, gilded frame. It showed a lady fitted out with a fur hat and fur boa who sat upright, raising a heavy fur muff that covered the whole of her lower arm towards the viewer. Gregor then turned to look out the window at the dull weather. Drops of rain could be heard hitting the pane, which made him feel quite sad. \"How about if I sleep a little bit longer and forget all this nonsense\", he thought, but that was something he was unable to do because he was used to sleeping on his right, and in his present state couldn't get into that position. However hard he threw himself onto his right, he always rolled back to where he was. He must have tried it a hundred times, shut his eyes so that he wouldn't have to look at the floundering legs, and only stopped when he began to feel a mild, dull pain there that he had never felt before.";

        let words: Vec<String> = text
            .split_whitespace()
            .map(|s| s.trim_matches(|c: char| !c.is_alphabetic()).to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let mut boids = Vec::new();
        let mut rng = rand::thread_rng();

        // Initial population: one boid per word
        for word in words {
            let x = rng.gen_range(0.0..width);
            let y = rng.gen_range(0.0..height);
            boids.push(Boid::new(x, y, word));
        }

        Self {
            boids,
            width,
            height,
            forces_buffer: Vec::new(),
        }
    }

    pub fn update(&mut self) {
        let mut rng = rand::thread_rng();

        // 1. Calculate flocking forces
        self.forces_buffer.clear();
        self.forces_buffer.reserve(self.boids.len());

        for boid in &self.boids {
            let force = boid.calculate_flocking_force(&self.boids);
            self.forces_buffer.push(force);
        }

        // 2. Apply forces, update physics, and evolve
        // We need to query neighbors again for evolution, which is O(N^2) naive.
        // For N=200 it's fine.

        // We can't mutate boids while iterating immutable, so we do it in steps or use indices.
        // Let's iterate with indices.
        for i in 0..self.boids.len() {
            // Physics
            self.boids[i].apply_force(self.forces_buffer[i]);
            self.boids[i].update(self.width, self.height);

            // Evolution Logic
            let boid_pos = self.boids[i].position;
            let view_radius_sq = self.boids[i].dna.view_radius.powi(2);
            let mut neighbor_count = 0;

            for j in 0..self.boids.len() {
                if i == j {
                    continue;
                }
                if distance_squared(boid_pos, self.boids[j].position) < view_radius_sq {
                    neighbor_count += 1;
                }
            }

            // If crowded, chance to evolve
            if neighbor_count > 3 && rng.gen_bool(0.02) {
                // 2% chance per frame if crowded
                let law = random_law(&mut rng);
                self.boids[i].evolve(law);
            }

            // Update Color based on evolution
            let original_len = self.boids[i].dna.original_word.len();
            let current_len = self.boids[i].dna.word.len();

            if current_len < original_len {
                self.boids[i].dna.color = Color::Red; // Decay
            } else if current_len > original_len {
                self.boids[i].dna.color = Color::Green; // Growth
            } else if self.boids[i].dna.word != self.boids[i].dna.original_word {
                self.boids[i].dna.color = Color::Yellow; // Changed but same length
            } else {
                self.boids[i].dna.color = Color::White; // Unchanged
            }
        }
    }
}

fn random_law(rng: &mut rand::rngs::ThreadRng) -> SoundLaw {
    match rng.gen_range(0..9) {
        0 => SoundLaw::GrimmsLaw,
        1 => SoundLaw::GreatVowelShift,
        2 => SoundLaw::Lenition,
        3 => SoundLaw::LossOfEndings,
        4 => SoundLaw::Palatalization,
        5 => SoundLaw::Metathesis,
        6 => SoundLaw::Rhotacism,
        7 => SoundLaw::ClusterSimplification,
        _ => SoundLaw::HDropping,
    }
}
