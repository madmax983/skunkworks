use crate::math::Vec3;

pub struct App {
    pub camera_pos: Vec3,
    pub camera_target: Vec3,
    pub time: f64,
    pub should_quit: bool,
    pub sphere1_pos: Vec3,
    pub sphere2_pos: Vec3,
    pub blend_factor: f64,
}

impl App {
    pub fn new() -> Self {
        Self {
            camera_pos: Vec3::new(0.0, 1.0, -4.0),
            camera_target: Vec3::ZERO,
            time: 0.0,
            should_quit: false,
            sphere1_pos: Vec3::new(-1.0, 0.0, 0.0),
            sphere2_pos: Vec3::new(1.0, 0.0, 0.0),
            blend_factor: 0.6,
        }
    }

    pub fn tick(&mut self) {
        self.time += 0.05;
        // Animate spheres
        self.sphere1_pos.y = (self.time * 2.0).sin() * 1.0;
        self.sphere2_pos.x = 1.0 + (self.time * 1.5).cos() * 0.5;
        self.sphere2_pos.z = (self.time * 1.0).sin() * 1.0;
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }
}
