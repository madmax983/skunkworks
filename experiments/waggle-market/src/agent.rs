use macroquad::prelude::*;
use market_sim::{Grid, Particle};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SourceType {
    Supply, // Cheap goods -> Sell (Ask)
    Demand, // Expensive buyer -> Buy (Bid)
}

#[derive(Clone, Copy, Debug)]
pub struct Source {
    pub position: Vec2,
    pub radius: f32,
    pub source_type: SourceType,
    pub value: f32, // "Profitability" or "Intensity"
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BeeState {
    Scouting,
    Returning { target_pos: Vec2, source_type: SourceType, value: f32 },
    Dancing { target_pos: Vec2, source_type: SourceType, value: f32, timer: f32 },
    Recruited { target_pos: Vec2, source_type: SourceType },
    Foraging { target_pos: Vec2, source_type: SourceType },
}

pub struct Bee {
    pub position: Vec2,
    pub velocity: Vec2,
    pub state: BeeState,
    pub id: usize,
}

impl Bee {
    pub fn new(pos: Vec2, id: usize) -> Self {
        Self {
            position: pos,
            velocity: vec2(macroquad::rand::gen_range(-1.0, 1.0), macroquad::rand::gen_range(-1.0, 1.0)),
            state: BeeState::Scouting,
            id,
        }
    }

    pub fn update(&mut self, sources: &[Source], market: &mut Grid, hive_pos: Vec2, field_rect: Rect, dt: f32) {
        let speed = 200.0 * dt;

        match self.state {
            BeeState::Scouting => {
                // Random walk
                self.velocity += vec2(macroquad::rand::gen_range(-0.5, 0.5), macroquad::rand::gen_range(-0.5, 0.5));
                self.velocity = self.velocity.normalize_or_zero() * speed;
                self.position += self.velocity;

                // Check collisions with sources
                for source in sources {
                    if self.position.distance(source.position) < source.radius {
                        // Found a source!
                        self.state = BeeState::Returning {
                            target_pos: source.position,
                            source_type: source.source_type,
                            value: source.value,
                        };
                        break;
                    }
                }

                // Bounce off field boundaries
                if self.position.x < field_rect.x {
                    self.velocity.x = self.velocity.x.abs();
                    self.position.x = field_rect.x;
                }
                if self.position.x > field_rect.x + field_rect.w {
                    self.velocity.x = -self.velocity.x.abs();
                    self.position.x = field_rect.x + field_rect.w;
                }
                if self.position.y < field_rect.y {
                    self.velocity.y = self.velocity.y.abs();
                    self.position.y = field_rect.y;
                }
                if self.position.y > field_rect.y + field_rect.h {
                    self.velocity.y = -self.velocity.y.abs();
                    self.position.y = field_rect.y + field_rect.h;
                }
            }
            BeeState::Returning { target_pos, source_type, value } => {
                // Fly to hive
                let dir = (hive_pos - self.position).normalize_or_zero();
                self.position += dir * speed;

                if self.position.distance(hive_pos) < 20.0 {
                    // Start dancing
                    self.state = BeeState::Dancing {
                        target_pos,
                        source_type,
                        value,
                        timer: value * 2.0 + 1.0, // Duration based on value
                    };
                }
            }
            BeeState::Dancing { target_pos, source_type, value, ref mut timer } => {
                // Waggle
                *timer -= dt;

                // Spawn orders in market!
                // Probability proportional to value
                // Higher value = More intense trading
                if macroquad::rand::gen_range(0.0, 1.0) < (value * 0.2) {
                     use ::rand::Rng;
                     let mut rng = ::rand::thread_rng();
                     let x = rng.gen_range(0..market.width);

                     match source_type {
                         SourceType::Supply => {
                             // Selling cheap goods -> Ask (High -> Low)
                             // Start at top (y=0)
                             if matches!(market.get(x, 0), Particle::Empty) {
                                 market.set(x, 0, Particle::Ask(self.id));
                             }
                         }
                         SourceType::Demand => {
                             // Buying -> Bid (Low -> High)
                             // Start at bottom (y=height-1)
                             if matches!(market.get(x, market.height - 1), Particle::Empty) {
                                 market.set(x, market.height - 1, Particle::Bid(self.id));
                             }
                         }
                     }
                }

                if *timer <= 0.0 {
                    // Go back to forage
                    self.state = BeeState::Foraging { target_pos, source_type };
                }
            }
            BeeState::Foraging { target_pos, .. } | BeeState::Recruited { target_pos, .. } => {
                 // Fly to source
                let dir = (target_pos - self.position).normalize_or_zero();
                self.position += dir * speed;

                if self.position.distance(target_pos) < 10.0 {
                     // Arrived at source.
                     // In a real sim, we would check if source is depleted.
                     // For now, assume infinite source, go back to scouting or return immediately?
                     // Let's scout a bit around the source to simulate "collecting"
                     self.state = BeeState::Scouting;
                }
            }
        }
    }
}
