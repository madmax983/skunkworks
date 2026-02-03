pub struct Oscillator {
    pub phase: f64,
    pub frequency: f64,
    pub amplitude: f64,
    pub speed: f64,
}

impl Oscillator {
    pub fn new() -> Self {
        Self {
            phase: 0.0,
            frequency: 0.1, // Spatial frequency
            amplitude: 10.0,
            speed: 5.0, // Temporal speed
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.phase += self.speed * dt;
    }

    pub fn modulate(&self, points: &[(f64, f64)]) -> Vec<(f64, f64)> {
        points
            .iter()
            .map(|(x, y)| {
                // Modulate Y based on X
                let offset = (*x * self.frequency + self.phase).sin() * self.amplitude;
                (*x, *y + offset)
            })
            .collect()
    }
}
