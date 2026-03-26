pub struct StringPhysics {
    pub vibration: f32,
    pub velocity: f32,
    pub tension: f32,
}

impl StringPhysics {
    pub fn new(tension: f32) -> Self {
        Self {
            vibration: 0.0,
            velocity: 0.0,
            tension,
        }
    }

    pub fn update(&mut self, dt: f32) {
        let acceleration = -self.tension * self.vibration - 2.0 * self.velocity;
        self.velocity += acceleration * dt;
        self.vibration += self.velocity * dt;
    }

    pub fn pluck(&mut self, strength: f32) {
        self.velocity += strength;
    }
}
