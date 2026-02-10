use locus::Vec2;
use std::f64::consts::PI;

pub struct Rover {
    pub pos: Vec2,
    pub vel: Vec2,
    pub angle: f64, // Radians, 0 pointing Right (positive X)
    pub max_speed: f64,
    pub friction: f64,
}

impl Rover {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            pos: Vec2::new(x, y),
            vel: Vec2::zero(),
            angle: 0.0,
            max_speed: 2.0,
            friction: 0.90,
        }
    }

    pub fn update(&mut self) {
        self.pos += self.vel;
        self.vel *= self.friction;

        // Stop completely if very slow to avoid floating point drift
        if self.vel.magnitude_squared() < 0.001 {
            self.vel = Vec2::zero();
        }
    }

    pub fn thrust(&mut self, amount: f64) {
        let thrust_vec = Vec2::new(self.angle.cos(), self.angle.sin()) * amount;
        self.vel += thrust_vec;

        // Cap speed
        if self.vel.magnitude() > self.max_speed {
            self.vel = self.vel.normalize() * self.max_speed;
        }
    }

    pub fn rotate(&mut self, amount: f64) {
        self.angle += amount;
        // Normalize angle to 0..2PI just for sanity, though sin/cos don't care
        self.angle %= 2.0 * PI;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialization() {
        let rover = Rover::new(10.0, 10.0);
        assert_eq!(rover.pos.x, 10.0);
        assert_eq!(rover.pos.y, 10.0);
        assert_eq!(rover.vel.x, 0.0);
        assert_eq!(rover.angle, 0.0);
    }

    #[test]
    fn test_thrust() {
        let mut rover = Rover::new(0.0, 0.0);
        // Angle is 0 (Right). Thrusting should increase X velocity.
        rover.thrust(1.0);
        assert!(rover.vel.x > 0.0);
        assert_eq!(rover.vel.y, 0.0);
    }

    #[test]
    fn test_rotation() {
        let mut rover = Rover::new(0.0, 0.0);
        rover.rotate(PI / 2.0); // Rotate 90 deg (Down in screen coords usually, but Up in Math)
                                // Cos(PI/2) is ~0, Sin(PI/2) is 1.
        rover.thrust(1.0);
        assert!(rover.vel.x.abs() < 0.0001);
        assert!(rover.vel.y > 0.9);
    }

    #[test]
    fn test_update_moves_position() {
        let mut rover = Rover::new(0.0, 0.0);
        rover.vel = Vec2::new(1.0, 0.0);
        rover.update();
        assert_eq!(rover.pos.x, 1.0);
        // Friction applied
        assert!(rover.vel.x < 1.0);
    }
}
