use macroquad::prelude::*;
use poincare_disk::{hyperbolic_dist, mobius_add, Point};
use std::f64::consts::PI;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BeeState {
    Scouting,
    Returning,
    Dancing,
    Observing,
    Foraging,
}

#[derive(Clone, Debug)]
pub struct Bee {
    pub pos: Point,
    pub state: BeeState,
    pub target_source: Option<usize>,
    pub dance_angle: f64,
    pub dance_quality: f64,
    pub dance_timer: f32,
    pub forage_timer: f32,
}

#[derive(Clone, Debug)]
pub struct Source {
    pub pos: Point,
    pub quality: f64,
}

pub struct World {
    pub bees: Vec<Bee>,
    pub sources: Vec<Source>,
    pub hive_radius: f64,
}

impl World {
    pub fn new(num_bees: usize) -> Self {
        let mut bees = Vec::new();
        for _ in 0..num_bees {
            bees.push(Bee {
                pos: Point::new(0.0, 0.0),
                state: BeeState::Scouting,
                target_source: None,
                dance_angle: 0.0,
                dance_quality: 0.0,
                dance_timer: 0.0,
                forage_timer: 0.0,
            });
        }

        Self {
            bees,
            sources: Vec::new(),
            hive_radius: 0.2, // Hyperbolic radius
        }
    }

    pub fn add_source(&mut self, pos: Point, quality: f64) {
        self.sources.push(Source { pos, quality });
    }

    pub fn update(&mut self) {
        let mut rng = ::rand::thread_rng();
        use ::rand::Rng;

        // Separate dancing bees for observation
        let dancers: Vec<(f64, f64)> = self.bees.iter()
            .filter(|b| b.state == BeeState::Dancing)
            .map(|b| (b.dance_angle, b.dance_quality))
            .collect();

        for bee in &mut self.bees {
            match bee.state {
                BeeState::Scouting => {
                    // Random walk
                    let angle = rng.gen_range(0.0..2.0 * PI);
                    let step_size = 0.05; // Hyperbolic step
                    let step = Point::from_polar(step_size, angle);
                    bee.pos = mobius_add(bee.pos, step);

                    // Check for food
                    for (i, source) in self.sources.iter().enumerate() {
                        if hyperbolic_dist(bee.pos, source.pos) < 0.2 {
                            bee.state = BeeState::Returning;
                            bee.target_source = Some(i);
                            bee.dance_quality = source.quality;
                            // Calculate dance angle: direction from Origin to Source
                            // Since we return to origin, the angle is just source.arg()
                            // But strictly it should be the angle of the geodesic at the origin.
                            // For a point P on the disk, the geodesic from 0 to P is a straight line.
                            // So the angle is indeed P.arg().
                            bee.dance_angle = source.pos.arg();
                            break;
                        }
                    }

                    // Too far? Turn back
                    if bee.pos.norm() > 0.95 {
                        bee.state = BeeState::Returning;
                        bee.target_source = None; // Just going home
                    }
                }
                BeeState::Returning => {
                    // Move towards origin
                    // Scaling down moves towards origin
                    let r = bee.pos.norm();
                    if r < 0.05 {
                        bee.pos = Point::new(0.0, 0.0);
                        if let Some(_) = bee.target_source {
                            bee.state = BeeState::Dancing;
                            bee.dance_timer = 5.0 * bee.dance_quality as f32; // Dance duration based on quality
                        } else {
                            bee.state = BeeState::Observing; // Failed scout becomes observer
                        }
                    } else {
                        // Move towards origin along radial line
                        // In Poincare disk, radial lines are geodesics.
                        // So we just scale the magnitude down.
                        // We want a constant hyperbolic step.
                        // dist(0, r) = 2 artanh(r)
                        // new_dist = dist - step
                        // new_r = tanh(new_dist / 2)

                        let dist = 2.0 * r.atanh();
                        let step = 0.1;
                        if dist > step {
                            let new_dist = dist - step;
                            let new_r = (new_dist / 2.0).tanh();
                            bee.pos = Point::from_polar(new_r, bee.pos.arg());
                        } else {
                            bee.pos = Point::new(0.0, 0.0);
                        }
                    }
                }
                BeeState::Dancing => {
                    bee.dance_timer -= 0.016; // Approx 60fps
                    if bee.dance_timer <= 0.0 {
                        bee.state = BeeState::Observing;
                    }

                    // Wiggle effect (visual only, handled in render usually, but position noise here?)
                    // Let's keep position at origin for simplicity logic-wise.
                }
                BeeState::Observing => {
                    // Chance to be recruited
                    if !dancers.is_empty() {
                         let (angle, quality) = dancers[rng.gen_range(0..dancers.len())];
                         if rng.gen_bool(0.1 * quality) {
                             bee.state = BeeState::Foraging;
                             bee.dance_angle = angle;
                             bee.forage_timer = 10.0; // Give up after some time
                         }
                    }

                    // If not recruited, maybe go scout
                    if rng.gen_bool(0.01) {
                        bee.state = BeeState::Scouting;
                    }
                }
                BeeState::Foraging => {
                    // Move in dance_angle direction
                    // If at origin, just move out.
                    // If already out, continue in that direction?
                    // "Direction" in hyperbolic space means following the geodesic.
                    // If we started at origin with angle theta, we stay on the radial line theta.

                    let r = bee.pos.norm();
                    let dist = 2.0 * r.atanh();
                    let step = 0.1; // Move fast
                    let new_dist = dist + step;
                    let new_r = (new_dist / 2.0).tanh();

                    // If we are at origin, we set angle to dance_angle
                    // If we are already moving, we maintain current angle?
                    // Ideally we should drift a bit or have noise.
                    // But for now, perfect transmission.

                    if r < 0.01 {
                         bee.pos = Point::from_polar(new_r, bee.dance_angle);
                    } else {
                         // Maintain radial direction (since dance gives radial direction from origin)
                         bee.pos = Point::from_polar(new_r, bee.pos.arg());
                    }

                    bee.forage_timer -= 0.016;

                    // Check for food
                    for (i, source) in self.sources.iter().enumerate() {
                        if hyperbolic_dist(bee.pos, source.pos) < 0.2 {
                            bee.state = BeeState::Returning;
                            bee.target_source = Some(i);
                            bee.dance_quality = source.quality;
                            bee.dance_angle = source.pos.arg();
                            break;
                        }
                    }

                    if bee.forage_timer <= 0.0 {
                        bee.state = BeeState::Returning; // Give up
                        bee.target_source = None;
                    }

                     // Too far?
                    if bee.pos.norm() > 0.95 {
                        bee.state = BeeState::Returning;
                        bee.target_source = None;
                    }
                }
            }
        }
    }
}
