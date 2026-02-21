use crate::physics::{Body, Universe};
use macroquad::prelude::*;
use std::collections::HashMap;

pub struct Sequencer {
    last_positions: HashMap<usize, Vec2>,
}

impl Sequencer {
    pub fn new() -> Self {
        Self {
            last_positions: HashMap::new(),
        }
    }

    /// Checks if any body has crossed the positive X-axis (y=0, x>0) in the counter-clockwise direction.
    /// Returns a list of body IDs that triggered.
    pub fn check_triggers(&mut self, universe: &Universe) -> Vec<usize> {
        let mut triggers = Vec::new();

        for body in &universe.bodies {
            if let Some(&prev_pos) = self.last_positions.get(&body.id) {
                // Check for crossing of Positive X Axis
                // Condition: prev_y < 0 and curr_y >= 0, and x > 0 (approx)
                // We should check if the segment (prev_pos, curr_pos) intersects the ray (0,0) -> (inf, 0)

                let curr_pos = body.position;

                // Simple check:
                // Crossed from below (-y) to above (+y) while x is positive.
                if prev_pos.y < 0.0 && curr_pos.y >= 0.0 {
                    // Check if x-coordinate at crossing is positive.
                    // Interpolate x at y=0.
                    // x = x1 + (x2 - x1) * (0 - y1) / (y2 - y1)
                    // y1 = prev_pos.y, y2 = curr_pos.y
                    // t = -y1 / (y2 - y1)

                    let dy = curr_pos.y - prev_pos.y;
                    if dy > 0.0 {
                        let t = -prev_pos.y / dy;
                        let crossing_x = prev_pos.x + (curr_pos.x - prev_pos.x) * t;

                        if crossing_x > 0.0 {
                            triggers.push(body.id);
                        }
                    }
                }
            }

            // Update last position
            self.last_positions.insert(body.id, body.position);
        }

        triggers
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::physics::Body;
    use macroquad::prelude::{Color, RED};

    #[test]
    fn test_trigger_crossing() {
        let mut sequencer = Sequencer::new();
        let mut universe = Universe::new();

        // Add a body
        let body = Body::new(0, Vec2::new(10.0, -1.0), Vec2::ZERO, 1.0, 1.0, RED);
        universe.add_body(body);

        // First check: Just updates last_pos, no trigger (since no history)
        let triggers = sequencer.check_triggers(&universe);
        assert!(triggers.is_empty());

        // Move body to cross X axis
        universe.bodies[0].position = Vec2::new(10.0, 1.0);
        let triggers = sequencer.check_triggers(&universe);
        assert_eq!(triggers.len(), 1);
        assert_eq!(triggers[0], 0);

        // Move body further up (no crossing)
        universe.bodies[0].position = Vec2::new(10.0, 5.0);
        let triggers = sequencer.check_triggers(&universe);
        assert!(triggers.is_empty());

        // Move body down (crossing back, clockwise)
        // My logic only checks -y to +y (counter-clockwise).
        // Standard orbital direction is CCW.
        universe.bodies[0].position = Vec2::new(10.0, -5.0);
        let triggers = sequencer.check_triggers(&universe);
        assert!(triggers.is_empty());
    }
}
