use macroquad::prelude::*;

pub struct StringVisual {
    pub pos: Vec2,
    pub length: f32,
    pub vibration: f32,
    pub velocity: f32,
    pub color: Color,
    pub label: String,
    pub is_hovered: bool,
}

impl StringVisual {
    pub fn new(pos: Vec2, length: f32, color: Color, label: String) -> Self {
        Self {
            pos,
            length,
            vibration: 0.0,
            velocity: 0.0,
            color,
            label,
            is_hovered: false,
        }
    }

    pub fn pluck(&mut self, strength: f32) {
        self.velocity += strength;
    }

    pub fn update(&mut self, dt: f32) {
        // Damped spring simulation for visual vibration
        // Adjust these constants for "feel"
        let k = 300.0; // Spring constant
        let damping = 4.0; // Damping factor

        let acceleration = -k * self.vibration - damping * self.velocity;
        self.velocity += acceleration * dt;
        self.vibration += self.velocity * dt;

        // Clamp vibration to avoid blowing up if unstable
        self.vibration = self.vibration.clamp(-50.0, 50.0);
    }

    pub fn draw(&self) {
        let start = self.pos;
        let end = self.pos + vec2(0.0, self.length);

        // Visual vibration is horizontal displacement
        let mid = (start + end) * 0.5;
        let offset = vec2(self.vibration, 0.0);
        let control = mid + offset;

        // Draw the string as two segments
        let thickness = if self.is_hovered { 2.0 } else { 1.0 };
        let color = if self.is_hovered { WHITE } else { self.color };

        draw_line(start.x, start.y, control.x, control.y, thickness, color);
        draw_line(control.x, control.y, end.x, end.y, thickness, color);

        // Add a glow effect if vibrating significantly
        if self.vibration.abs() > 1.0 {
            draw_line(start.x, start.y, control.x, control.y, thickness * 3.0, Color::new(color.r, color.g, color.b, 0.2));
            draw_line(control.x, control.y, end.x, end.y, thickness * 3.0, Color::new(color.r, color.g, color.b, 0.2));
        }
    }

    pub fn check_hover(&mut self, mouse_pos: Vec2) -> bool {
        // Simple AABB check around the string
        let start = self.pos;
        let end = self.pos + vec2(0.0, self.length);

        // Add some margin for easier selection
        let margin = 5.0;

        let x_min = start.x.min(end.x) - margin + self.vibration.min(0.0);
        let x_max = start.x.max(end.x) + margin + self.vibration.max(0.0);
        let y_min = start.y.min(end.y);
        let y_max = start.y.max(end.y);

        self.is_hovered = mouse_pos.x >= x_min && mouse_pos.x <= x_max && mouse_pos.y >= y_min && mouse_pos.y <= y_max;
        self.is_hovered
    }
}
