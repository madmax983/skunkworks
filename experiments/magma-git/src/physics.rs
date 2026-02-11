use ratatui::style::Color;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MagmaState {
    Liquid,
    Solid,
}

#[derive(Clone, Debug)]
pub struct MagmaParticle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub temperature: f32, // 1.0 = Hot, 0.0 = Cold
    pub state: MagmaState,
    pub color: Color,
    pub char: char,
    pub commit_hash: String, // Trace back to origin
}

impl MagmaParticle {
    pub fn new(x: f32, y: f32, commit_hash: String) -> Self {
        Self {
            x,
            y,
            vx: (rand::random::<f32>() - 0.5) * 2.0, // Random horizontal spread
            vy: rand::random::<f32>() * 0.5, // Initial downward velocity
            temperature: 1.0,
            state: MagmaState::Liquid,
            color: Color::Red,
            char: ':',
            commit_hash,
        }
    }

    pub fn update(&mut self, dt: f32, gravity: f32) {
        if self.state == MagmaState::Solid {
            return;
        }

        self.vy += gravity * dt;
        self.x += self.vx * dt;
        self.y += self.vy * dt;

        // Cooling
        self.temperature -= 0.005 * dt;
        if self.temperature < 0.0 {
            self.temperature = 0.0;
        }

        // Color based on temp
        self.update_color();
    }

    fn update_color(&mut self) {
        if self.temperature > 0.8 {
            self.color = Color::Red;
            self.char = '▓';
        } else if self.temperature > 0.5 {
            self.color = Color::LightRed; // Orange-ish
            self.char = '▒';
        } else if self.temperature > 0.2 {
            self.color = Color::Yellow;
            self.char = '░';
        } else {
            self.color = Color::DarkGray;
            self.char = '.';
        }
    }
}
