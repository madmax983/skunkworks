use ::rand::Rng;
use macroquad::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BeeState {
    Scouting,
    Returning,
    Dancing,
    Observing,
    Foraging,
}

#[derive(Clone, Copy, Debug)]
pub struct Bee {
    pub position: Vec2,
    pub velocity: Vec2,
    pub state: BeeState,
    pub target_site_index: Option<usize>, // Index into world.sources
    pub memory_quality: f32,              // Quality of the site found/remembered
    pub dance_timer: f32,                 // Time left for current dance
    pub dance_angle: f32,                 // Angle of the dance (pointing to source)
    pub dance_duration: f32,              // Total duration of the dance (proportional to distance)
    pub waggle_intensity: f32,            // Intensity of waggle (proportional to quality)
}

#[derive(Clone, Copy, Debug)]
pub struct DanceInfo {
    pub target_index: usize,
    pub quality: f32,
    #[allow(dead_code)]
    pub angle: f32,
    #[allow(dead_code)]
    pub position: Vec2,
}

impl Bee {
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            velocity: Vec2::ZERO,
            state: BeeState::Scouting,
            target_site_index: None,
            memory_quality: 0.0,
            dance_timer: 0.0,
            dance_angle: 0.0,
            dance_duration: 0.0,
            waggle_intensity: 0.0,
        }
    }

    pub fn update(
        &mut self,
        sources: &[FoodSource],
        active_dances: &[DanceInfo],
        hive_pos: Vec2,
        hive_radius: f32,
    ) {
        let mut rng = ::rand::thread_rng();
        let max_speed = 2.0;
        let sight_range = 30.0;

        match self.state {
            BeeState::Scouting => {
                // Random walk / Levy flight
                self.velocity += vec2(rng.gen_range(-0.5..0.5), rng.gen_range(-0.5..0.5));
                if self.velocity.length() > max_speed {
                    self.velocity = self.velocity.normalize() * max_speed;
                }
                self.position += self.velocity;

                // Check for sources
                for (i, source) in sources.iter().enumerate() {
                    if self.position.distance(source.position) < source.radius + sight_range {
                        // Found a source!
                        self.target_site_index = Some(i);
                        self.memory_quality = source.quality;
                        self.state = BeeState::Returning;
                        break;
                    }
                }

                // Return to hive if too far (optional, keeps them from wandering off screen)
                if self.position.distance(hive_pos) > 500.0 {
                    self.velocity += (hive_pos - self.position).normalize() * 0.1;
                }
            }
            BeeState::Returning => {
                // Move towards hive
                let dir = (hive_pos - self.position).normalize_or_zero();
                self.velocity = dir * max_speed;
                self.position += self.velocity;

                if self.position.distance(hive_pos) < hive_radius {
                    // Arrived at hive. Prepare to dance!
                    // Duration based on distance (simplified: just constant * quality for now, or fetch source dist)
                    // Angle based on source pos
                    if let Some(idx) = self.target_site_index {
                        if let Some(source) = sources.get(idx) {
                            let vec_to_source = source.position - hive_pos;
                            self.dance_angle = vec_to_source.y.atan2(vec_to_source.x);
                            self.dance_duration = 100.0 * self.memory_quality; // Dance longer for better quality
                            self.waggle_intensity = self.memory_quality;
                            self.dance_timer = self.dance_duration;
                            self.state = BeeState::Dancing;
                        } else {
                            // Source disappeared?
                            self.state = BeeState::Observing;
                        }
                    } else {
                        self.state = BeeState::Observing;
                    }
                }
            }
            BeeState::Dancing => {
                // Stay at hive
                self.velocity = Vec2::ZERO;
                self.dance_timer -= 1.0;

                // Visualization: Jitter
                self.position = hive_pos + vec2(rng.gen_range(-2.0..2.0), rng.gen_range(-2.0..2.0));

                if self.dance_timer <= 0.0 {
                    // Done dancing.
                    // Decide whether to forage again or observe.
                    // High quality -> likely to forage again immediately (recruitment of self)
                    if rng.gen::<f32>() < self.memory_quality {
                        self.state = BeeState::Foraging;
                    } else {
                        self.state = BeeState::Observing;
                    }
                }
            }
            BeeState::Observing => {
                // Stay at hive
                self.velocity = Vec2::ZERO;
                // Slowly drift around hive
                self.position += vec2(rng.gen_range(-0.5..0.5), rng.gen_range(-0.5..0.5));
                if self.position.distance(hive_pos) > hive_radius {
                    self.position = hive_pos + (self.position - hive_pos).normalize() * hive_radius;
                }

                // Watch dances
                if !active_dances.is_empty() {
                    // Pick a random dance to observe
                    let dance = active_dances[rng.gen_range(0..active_dances.len())];

                    // Probability to be recruited proportional to quality
                    // Tunable parameter: Recruitment sensitivity
                    if rng.gen::<f32>() < dance.quality * 0.05 {
                        self.target_site_index = Some(dance.target_index);
                        self.memory_quality = dance.quality;
                        self.state = BeeState::Foraging;
                    }
                }

                // Chance to go scouting if bored
                if rng.gen::<f32>() < 0.01 {
                    self.state = BeeState::Scouting;
                    self.target_site_index = None;
                }
            }
            BeeState::Foraging => {
                // Move towards target
                if let Some(idx) = self.target_site_index {
                    if let Some(source) = sources.get(idx) {
                        let dir = (source.position - self.position).normalize_or_zero();
                        self.velocity = dir * max_speed;
                        self.position += self.velocity;

                        // Add some randomness/error
                        self.velocity += vec2(rng.gen_range(-0.2..0.2), rng.gen_range(-0.2..0.2));

                        if self.position.distance(source.position) < source.radius {
                            // Arrived at source.
                            // Re-evaluate quality (maybe it changed)
                            self.memory_quality = source.quality;
                            self.state = BeeState::Returning;
                        }
                    } else {
                        // Target invalid
                        self.state = BeeState::Returning;
                    }
                } else {
                    self.state = BeeState::Returning;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_update() {
        let mut world = World::new(vec2(0.0, 0.0), 10);
        world.add_source(vec2(100.0, 0.0), 1.0);

        // Run a few updates
        for _ in 0..100 {
            world.update();
        }

        // Check that bees are moving (not all at 0,0)
        let moving_bees = world
            .bees
            .iter()
            .filter(|b| b.position != vec2(0.0, 0.0))
            .count();
        assert!(moving_bees > 0);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct FoodSource {
    pub position: Vec2,
    pub quality: f32, // 0.0 to 1.0 (or higher)
    pub radius: f32,
}

pub struct World {
    pub hive_position: Vec2,
    pub hive_radius: f32,
    pub bees: Vec<Bee>,
    pub sources: Vec<FoodSource>,
}

impl World {
    pub fn new(hive_pos: Vec2, num_bees: usize) -> Self {
        let mut bees = Vec::with_capacity(num_bees);
        let mut rng = ::rand::thread_rng();

        for _ in 0..num_bees {
            let angle = rng.gen_range(0.0..std::f32::consts::TAU);
            let dist = rng.gen_range(0.0..20.0);
            let offset = vec2(angle.cos() * dist, angle.sin() * dist);
            bees.push(Bee::new(hive_pos + offset));
        }

        Self {
            hive_position: hive_pos,
            hive_radius: 30.0,
            bees,
            sources: Vec::new(),
        }
    }

    pub fn add_source(&mut self, pos: Vec2, quality: f32) {
        self.sources.push(FoodSource {
            position: pos,
            quality,
            radius: 20.0,
        });
    }

    pub fn update(&mut self) {
        // Collect active dances
        let active_dances: Vec<DanceInfo> = self
            .bees
            .iter()
            .filter(|b| b.state == BeeState::Dancing && b.target_site_index.is_some())
            .map(|b| DanceInfo {
                target_index: b.target_site_index.unwrap(),
                quality: b.memory_quality,
                angle: b.dance_angle,
                position: b.position,
            })
            .collect();

        // Update bees
        for bee in &mut self.bees {
            bee.update(
                &self.sources,
                &active_dances,
                self.hive_position,
                self.hive_radius,
            );
        }
    }
}
