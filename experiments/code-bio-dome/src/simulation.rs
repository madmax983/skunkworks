use crate::harvester::FunctionSignature;
use rand::Rng;
use tui_semantic::{Entity, Snapshot};

#[derive(Clone, Debug)]
pub struct Creature {
    pub id: usize,
    pub name: String,
    pub pos: (f64, f64),
    pub vel: (f64, f64),
    pub energy: f64,
    pub inputs: Vec<String>,
    pub satisfied_inputs: Vec<bool>,
    pub output: String,
}

#[derive(Clone, Debug)]
pub struct Particle {
    pub id: usize,
    pub type_name: String,
    pub pos: (f64, f64),
    pub vel: (f64, f64),
    pub lifetime: f64,
}

pub struct World {
    pub creatures: Vec<Creature>,
    pub particles: Vec<Particle>,
    pub width: f64,
    pub height: f64,
    pub frame: u64,
    next_id: usize,
}

impl World {
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            creatures: Vec::new(),
            particles: Vec::new(),
            width,
            height,
            frame: 0,
            next_id: 0,
        }
    }

    pub fn populate(&mut self, signatures: &[FunctionSignature]) {
        let mut rng = rand::thread_rng();
        for sig in signatures {
            self.creatures.push(Creature {
                id: self.next_id,
                name: sig.name.clone(),
                pos: (
                    rng.gen_range(0.0..self.width),
                    rng.gen_range(0.0..self.height),
                ),
                vel: (rng.gen_range(-0.5..0.5), rng.gen_range(-0.5..0.5)),
                energy: 100.0,
                inputs: sig.inputs.clone(),
                satisfied_inputs: vec![false; sig.inputs.len()],
                output: sig.output.clone(),
            });
            self.next_id += 1;
        }

        // Seed some initial particles based on inputs of creatures
        // to kickstart the ecosystem
        for _ in 0..50 {
            if let Some(creature) = self.creatures.get(rng.gen_range(0..self.creatures.len())) {
                if !creature.inputs.is_empty() {
                    let type_name = &creature.inputs[rng.gen_range(0..creature.inputs.len())];
                    self.spawn_particle(
                        type_name.clone(),
                        rng.gen_range(0.0..self.width),
                        rng.gen_range(0.0..self.height),
                    );
                }
            }
        }
    }

    pub fn spawn_particle(&mut self, type_name: String, x: f64, y: f64) {
        if type_name.is_empty() {
            return;
        }

        let mut rng = rand::thread_rng();
        self.particles.push(Particle {
            id: self.next_id,
            type_name,
            pos: (x, y),
            vel: (rng.gen_range(-0.2..0.2), rng.gen_range(-0.2..0.2)),
            lifetime: 100.0,
        });
        self.next_id += 1;
    }

    pub fn update(&mut self) {
        self.frame += 1;
        let dt = 1.0;

        // Move particles
        for p in &mut self.particles {
            p.pos.0 += p.vel.0 * dt;
            p.pos.1 += p.vel.1 * dt;

            // Bounce
            if p.pos.0 < 0.0 || p.pos.0 > self.width {
                p.vel.0 *= -1.0;
            }
            if p.pos.1 < 0.0 || p.pos.1 > self.height {
                p.vel.1 *= -1.0;
            }

            p.lifetime -= 0.1 * dt;
        }

        // Remove dead particles
        self.particles.retain(|p| p.lifetime > 0.0);

        // Creature Logic
        let mut new_particles = Vec::new();

        for c in &mut self.creatures {
            // Decay energy
            c.energy = (c.energy - 0.05 * dt).max(0.0);

            // Movement: Wander + Seek Food
            let mut seek_vec = (0.0, 0.0);

            // Find needed input
            let mut needed_type = None;
            for (i, input_type) in c.inputs.iter().enumerate() {
                if !c.satisfied_inputs[i] {
                    needed_type = Some(input_type);
                    break;
                }
            }

            if let Some(target_type) = needed_type {
                // Find nearest particle of this type
                let mut min_dist = 50.0; // Vision radius
                let mut target_pos = None;

                for p in &self.particles {
                    if &p.type_name == target_type {
                        let dx = p.pos.0 - c.pos.0;
                        let dy = p.pos.1 - c.pos.1;
                        let dist = (dx * dx + dy * dy).sqrt();
                        if dist < min_dist {
                            min_dist = dist;
                            target_pos = Some(p.pos);
                        }
                    }
                }

                if let Some(target) = target_pos {
                    seek_vec.0 = (target.0 - c.pos.0) * 0.05;
                    seek_vec.1 = (target.1 - c.pos.1) * 0.05;
                }
            }

            c.vel.0 += seek_vec.0;
            c.vel.1 += seek_vec.1;

            // Clamp velocity
            let speed = (c.vel.0.powi(2) + c.vel.1.powi(2)).sqrt();
            if speed > 1.0 {
                c.vel.0 = (c.vel.0 / speed) * 1.0;
                c.vel.1 = (c.vel.1 / speed) * 1.0;
            }

            c.pos.0 += c.vel.0 * dt;
            c.pos.1 += c.vel.1 * dt;

            // Bounce
            if c.pos.0 < 0.0 {
                c.pos.0 = 0.0;
                c.vel.0 *= -1.0;
            }
            if c.pos.0 > self.width {
                c.pos.0 = self.width;
                c.vel.0 *= -1.0;
            }
            if c.pos.1 < 0.0 {
                c.pos.1 = 0.0;
                c.vel.1 *= -1.0;
            }
            if c.pos.1 > self.height {
                c.pos.1 = self.height;
                c.vel.1 *= -1.0;
            }
        }

        // Collision: Eating
        // Doing this in a separate loop or carefully to avoid double-borrow issues
        // Simple O(N*M) check is fine for N,M < 500

        let mut eaten_particles = std::collections::HashSet::new();

        for c in &mut self.creatures {
            if c.energy <= 0.0 {
                continue;
            } // Dead things don't eat

            for (i, input_type) in c.inputs.iter().enumerate() {
                if c.satisfied_inputs[i] {
                    continue;
                }

                for (p_idx, p) in self.particles.iter().enumerate() {
                    if eaten_particles.contains(&p_idx) {
                        continue;
                    }

                    if &p.type_name == input_type {
                        let dx = p.pos.0 - c.pos.0;
                        let dy = p.pos.1 - c.pos.1;
                        let dist_sq = dx * dx + dy * dy;

                        if dist_sq < 4.0 {
                            // Eat radius sq (2.0 radius)
                            eaten_particles.insert(p_idx);
                            c.satisfied_inputs[i] = true;
                            c.energy = (c.energy + 20.0).min(100.0);
                            break; // Ate one thing this frame
                        }
                    }
                }
            }

            // Check if full
            if !c.inputs.is_empty() && c.satisfied_inputs.iter().all(|&b| b) {
                // Spawn Output
                new_particles.push((c.output.clone(), c.pos.0, c.pos.1));
                // Reset
                for b in &mut c.satisfied_inputs {
                    *b = false;
                }
                c.energy = (c.energy + 10.0).min(100.0); // Bonus for completing cycle
            }
        }

        // Apply removals and additions
        let mut i = 0;
        self.particles.retain(|_| {
            let retain = !eaten_particles.contains(&i);
            i += 1;
            retain
        });

        for (type_name, x, y) in new_particles {
            self.spawn_particle(type_name, x, y);
        }
    }

    pub fn snapshot(&self) -> Snapshot {
        let mut snapshot = Snapshot::new("code-bio-dome")
            .with_frame(self.frame)
            .with_viewport(self.width as u16, self.height as u16);

        for c in &self.creatures {
            snapshot = snapshot.with_entity(
                Entity::new("creature")
                    .with_id(c.id.to_string())
                    .at(c.pos.0, c.pos.1)
                    .moving(c.vel.0, c.vel.1)
                    .with_prop("name", c.name.clone())
                    .with_prop("energy", c.energy)
                    .with_prop("output", c.output.clone()),
            );
        }

        for p in &self.particles {
            snapshot = snapshot.with_entity(
                Entity::new("particle")
                    .with_id(p.id.to_string())
                    .at(p.pos.0, p.pos.1)
                    .moving(p.vel.0, p.vel.1)
                    .with_prop("type", p.type_name.clone()),
            );
        }

        snapshot
    }
}
