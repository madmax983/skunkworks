use crate::ik::IKChain;
use crate::world::World;
use glam::Vec2;

pub struct Leg {
    pub chain: IKChain,
    pub target: Vec2, // Current physical target in space
    pub state: LegState,
    pub offset: Vec2, // Offset from body center in "rest pose"
}

pub enum LegState {
    Stance {
        #[allow(dead_code)]
        node_index: Option<usize>,
    },
    Swing {
        start: Vec2,
        end: Vec2,
        progress: f32,
    },
}

pub struct Creature {
    pub body_pos: Vec2,
    pub velocity: Vec2,
    pub legs: Vec<Leg>,
    pub target_pos: Vec2, // Where the body wants to go
}

impl Creature {
    pub fn new(pos: Vec2) -> Self {
        // Create 8 legs like a spider
        let mut legs = Vec::new();
        let num_legs = 8;

        for i in 0..num_legs {
            let angle = (i as f32 / num_legs as f32) * std::f32::consts::TAU;
            let offset_dist = 6.0;
            let offset = Vec2::new(angle.cos(), angle.sin()) * offset_dist;

            // Initial target is further out
            let target = pos + offset * 3.0;

            legs.push(Leg {
                chain: IKChain::new(pos + offset, 3, 5.0), // 3 segments, 5 units each = 15 units reach
                target,
                state: LegState::Stance { node_index: None },
                offset,
            });
        }

        Self {
            body_pos: pos,
            velocity: Vec2::ZERO,
            legs,
            target_pos: pos,
        }
    }

    pub fn update(&mut self, world: &World, dt: f32) {
        // 1. Move Body towards target (Spring Physics)
        let diff = self.target_pos - self.body_pos;
        let dist = diff.length();

        // Simple easing
        if dist > 0.1 {
            let speed = 40.0; // Units per second
                              // Proportional control
            self.velocity = diff * 2.0;
            // Cap speed
            if self.velocity.length() > speed {
                self.velocity = self.velocity.normalize() * speed;
            }
            self.body_pos += self.velocity * dt;
        } else {
            self.body_pos = self.target_pos;
            self.velocity = Vec2::ZERO;
        }

        // 2. Update Legs
        for leg in &mut self.legs {
            // Update root position of the leg based on body
            let leg_root = self.body_pos + leg.offset;

            match &mut leg.state {
                LegState::Stance { node_index: _ } => {
                    // Check if we need to step
                    let current_dist = leg.target.distance(leg_root);
                    let max_reach = 15.0; // Total chain length is 15

                    // Trigger step if overextended OR if the body moved significantly and we are "behind"
                    // Or if we are just not in a good spot relative to the body

                    let ideal_pos = leg_root + leg.offset * 2.5; // Natural resting spot
                    let dist_from_ideal = leg.target.distance(ideal_pos);

                    if current_dist > max_reach * 0.9 || dist_from_ideal > 10.0 {
                        // Find new target
                        // Find closest node to ideal_pos
                        let mut best_pos = ideal_pos;
                        let mut min_dist = f32::MAX;

                        // Look for a node to step on
                        for node in &world.nodes {
                            let d = node.position.distance(ideal_pos);
                            // Prefer nodes within reach, but also close to ideal
                            if d < min_dist && d < 10.0 {
                                min_dist = d;
                                best_pos = node.position;
                            }
                        }

                        // Start Swing
                        leg.state = LegState::Swing {
                            start: leg.target,
                            end: best_pos,
                            progress: 0.0,
                        };
                    }
                }
                LegState::Swing {
                    start,
                    end,
                    progress,
                } => {
                    *progress += dt * 4.0; // Swing speed
                    if *progress >= 1.0 {
                        leg.target = *end;
                        leg.state = LegState::Stance { node_index: None };
                    } else {
                        // Lerp
                        leg.target = start.lerp(*end, *progress);
                    }
                }
            }

            // Solve IK
            // We need to re-initialize chain root because the body moved
            // But FABRIK usually works better if we just pass the root and target
            leg.chain.solve_fabrik(leg_root, leg.target);
        }
    }
}
