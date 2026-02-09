pub struct Scheduler {
    pub sun_angle: f32, // Radians, 0 to PI (Day cycle)
    pub sun_speed: f32,
    pub beam_width: f32, // How wide the "core" execution window is
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            sun_angle: std::f32::consts::PI / 2.0, // Start at noon
            sun_speed: 0.5,
            beam_width: 0.2,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.sun_angle += self.sun_speed * dt;
        // Oscillate or Wrap?
        // Day/Night cycle.
        // Let's wrap 0 to PI.
        if self.sun_angle > std::f32::consts::PI {
            self.sun_angle = 0.0;
        }
    }

    /// Checks if a given angle (process position) is currently receiving CPU time.
    pub fn is_active(&self, angle: f32) -> bool {
        (angle - self.sun_angle).abs() < self.beam_width
    }
}
