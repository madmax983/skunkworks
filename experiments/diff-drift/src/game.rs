use crate::track::Track;

#[derive(Debug, Clone)]
pub struct Car {
    pub x: f64,        // Horizontal position (-1.0 to 1.0 is track width)
    pub distance: f64, // Distance traveled along the track
    pub speed: f64,    // Current speed (units per second)
    pub max_speed: f64,
    pub steer_angle: f64, // -1.0 to 1.0
}

impl Default for Car {
    fn default() -> Self {
        Self {
            x: 0.0,
            distance: 0.0,
            speed: 0.0,
            max_speed: 100.0,
            steer_angle: 0.0,
        }
    }
}

pub struct GameState {
    pub track: Track,
    pub car: Car,
    pub score: u64,
    pub game_over: bool,
    // pub track_width: f64,
}

impl GameState {
    pub fn new(track: Track) -> Self {
        Self {
            track,
            car: Car::default(),
            score: 0,
            game_over: false,
            // track_width: 1.0, // Normalized
        }
    }

    pub fn update(&mut self, dt: f64) {
        if self.game_over {
            return;
        }

        // Apply steering
        // Steering moves X
        // Sensitivity
        let steer_speed = 2.0;
        self.car.x += self.car.steer_angle * steer_speed * dt;

        // Apply curvature force (Centrifugal)
        if let Some(segment) = self.track.get_segment_at(self.car.distance) {
            // Curvature: + is Right turn.
            // If turning Right, force pushes Left (-).
            // Force depends on speed squared usually, but let's keep it linear for fun
            let force = segment.curvature * (self.car.speed / self.car.max_speed) * 1.5;
            self.car.x -= force * dt;
        } else {
            // End of track - Loop or Stop?
            // Let's loop for endless gameplay
            if self.track.total_length() > 0.0 {
                self.car.distance %= self.track.total_length();
            }
        }

        // Move forward
        self.car.distance += self.car.speed * dt;

        // Collision Check
        // Safe zone is -1.0 to 1.0
        // We allow a bit of buffer
        if self.car.x.abs() > 1.2 {
            // Crash!
            self.car.speed *= 0.9; // Slow down punishment
                                   // Visual feedback?
                                   // For now just clamp or bounce?
                                   // Let's bounce
            self.car.x = 1.2 * self.car.x.signum();
            // self.game_over = true; // Maybe too harsh
        }

        // Increase score
        self.score += (self.car.speed * dt) as u64;
    }

    pub fn steer(&mut self, amount: f64) {
        // Amount is -1.0 (Left) to 1.0 (Right)
        self.car.steer_angle = amount;
    }

    pub fn accelerate(&mut self, amount: f64) {
        // Amount is acceleration factor
        self.car.speed += amount;
        self.car.speed = self.car.speed.clamp(0.0, self.car.max_speed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::track::Segment;

    #[test]
    fn test_car_physics() {
        let segments = vec![Segment {
            curvature: 0.0,
            length: 100.0,
            description: "Straight".into(),
        }];
        let mut track = Track {
            segments,
            points: vec![],
        };
        track.compute_geometry(); // Helper needed? Or just dummy points
                                  // Or just let compute_geometry run if I exposed it?
                                  // I didn't expose compute_geometry as pub.
                                  // But I can make points empty for this test as physics uses get_segment_at which uses segments.
                                  // BUT wait, physics might use get_x_at later? No, currently physics uses segments.

        let mut game = GameState::new(track);

        game.car.speed = 50.0;
        game.update(0.1);

        assert!(game.car.distance > 0.0);
        assert_eq!(game.car.x, 0.0); // No steering, no curve
    }

    #[test]
    fn test_curve_physics() {
        let segments = vec![
            // Sharp Right Turn (+1.0)
            Segment {
                curvature: 1.0,
                length: 100.0,
                description: "Right".into(),
            },
        ];
        let mut track = Track {
            segments,
            points: vec![],
        };
        // We don't need points for physics calculation yet, as it uses segments directly.

        let mut game = GameState::new(track);

        game.car.speed = 50.0;
        game.update(0.1);

        // Should be pushed Left (negative X) by centrifugal force
        assert!(game.car.x < 0.0);
    }
}
