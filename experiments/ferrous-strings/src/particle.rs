use macroquad::prelude::*;
use ferrous_core::Platter;

#[derive(Clone, Copy)]
pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub color: Color,
}

impl Particle {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            pos: vec2(x, y),
            vel: vec2(0.0, 0.0),
            color: Color::new(0.0, 0.8, 1.0, 0.5),
        }
    }

    pub fn update(&mut self, platter: &Platter, grid_scale: f32, dt: f32) {
        // Sample magnetic field gradient
        // Grid coordinates
        let gx = (self.pos.x / grid_scale) as i32;
        let gy = (self.pos.y / grid_scale) as i32;

        let mut force = vec2(0.0, 0.0);

        // Simple gradient: Look at neighbors
        // Center value
        let _c = get_mag(platter, gx, gy);
        let r = get_mag(platter, gx + 1, gy);
        let l = get_mag(platter, gx - 1, gy);
        let d = get_mag(platter, gx, gy + 1);
        let u = get_mag(platter, gx, gy - 1);

        // Gradient points towards HIGHER magnetism
        force.x = (r - l) * 100.0;
        force.y = (d - u) * 100.0;

        // Also add some random jitter (Brownian motion)
        force.x += rand::gen_range(-5.0, 5.0);
        force.y += rand::gen_range(-5.0, 5.0);

        // Apply force
        self.vel += force * dt;

        // Damping
        self.vel *= 0.95;

        // Move
        self.pos += self.vel * dt;

        // Wrap around screen
        let w = screen_width();
        let h = screen_height();

        if self.pos.x < 0.0 { self.pos.x += w; }
        if self.pos.x > w { self.pos.x -= w; }
        if self.pos.y < 0.0 { self.pos.y += h; }
        if self.pos.y > h { self.pos.y -= h; }
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, 2.0, self.color);
    }
}

fn get_mag(platter: &Platter, x: i32, y: i32) -> f32 {
    if x < 0 || y < 0 || x >= platter.width as i32 || y >= platter.height as i32 {
        return 0.0;
    }
    platter.get_magnetism(x as usize, y as usize) as f32
}
