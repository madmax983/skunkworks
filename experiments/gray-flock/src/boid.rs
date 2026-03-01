use locus::Vec2;
use rand::Rng;

pub struct Boid {
    pub position: Vec2,
    pub velocity: Vec2,
    pub view_radius: f64,
    pub max_speed: f64,
    pub max_force: f64,
}

impl Boid {
    pub fn new(x: f64, y: f64) -> Self {
        let mut rng = rand::thread_rng();
        // Give a random initial velocity
        let angle = rng.gen_range(0.0..std::f64::consts::TAU);
        let speed = rng.gen_range(0.5..1.5);
        Self {
            position: Vec2::new(x, y),
            velocity: Vec2::new(angle.cos() * speed, angle.sin() * speed),
            view_radius: 10.0,
            max_speed: 1.5,
            max_force: 0.2,
        }
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.velocity += force;

        // Limit speed
        let speed_sq = self.velocity.magnitude_squared();
        if speed_sq > self.max_speed * self.max_speed {
            self.velocity = self.velocity.normalize() * self.max_speed;
        }
    }

    pub fn update_physics(&mut self, width: f64, height: f64) {
        self.position += self.velocity;

        // Wrap around torus topology
        if self.position.x < 0.0 {
            self.position.x += width;
        } else if self.position.x >= width {
            self.position.x -= width;
        }

        if self.position.y < 0.0 {
            self.position.y += height;
        } else if self.position.y >= height {
            self.position.y -= height;
        }
    }
}
