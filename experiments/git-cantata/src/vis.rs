use crate::git::Commit;
use rand::Rng;
use ratatui::{
    style::Color,
    widgets::canvas::{Context, Line, Points},
};

pub enum StrokeKind {
    Spray,
    Line,
}

pub struct BrushStroke {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub color: Color,
    pub age: f64,
    pub max_age: f64,
    pub kind: StrokeKind,
}

pub struct VisualState {
    pub strokes: Vec<BrushStroke>,
    pub width: f64,
    pub height: f64,
}

impl VisualState {
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            strokes: Vec::new(),
            width,
            height,
        }
    }

    pub fn spawn_commit(&mut self, commit: &Commit) {
        let mut rng = rand::thread_rng();

        // Start position based on hash (roughly)
        let hash_val = commit
            .hash
            .as_bytes()
            .iter()
            .map(|&b| b as usize)
            .sum::<usize>();
        let center_x = (hash_val % (self.width as usize)) as f64;
        let center_y = ((hash_val / 100) % (self.height as usize)) as f64;

        for change in &commit.files {
            let color = match change.extension.as_str() {
                "rs" => Color::Red,
                "toml" => Color::Magenta,
                "json" => Color::Yellow,
                "md" => Color::Blue,
                "yml" | "yaml" => Color::Cyan,
                _ => Color::White,
            };

            // Scale particles by change size
            let count = ((change.insertions + change.deletions) / 10).min(50).max(5);

            for _ in 0..count {
                let angle = rng.gen_range(0.0..std::f64::consts::TAU);
                let speed = rng.gen_range(0.5..2.0);

                self.strokes.push(BrushStroke {
                    x: center_x + rng.gen_range(-10.0..10.0),
                    y: center_y + rng.gen_range(-10.0..10.0),
                    vx: angle.cos() * speed,
                    vy: angle.sin() * speed,
                    color,
                    age: 0.0,
                    max_age: rng.gen_range(20.0..100.0),
                    kind: if rng.gen_bool(0.7) {
                        StrokeKind::Spray
                    } else {
                        StrokeKind::Line
                    },
                });
            }
        }
    }

    pub fn update(&mut self, dt: f64) {
        for stroke in &mut self.strokes {
            stroke.x += stroke.vx * dt;
            stroke.y += stroke.vy * dt;
            stroke.age += dt;

            // Gravity or Flow? Let's make them flow up like smoke
            // Canvas (0,0) is Bottom-Left in Ratatui Canvas. So Up is Positive Y.
            stroke.vy += 10.0 * dt; // Stronger upward flow

            // Wrap
            if stroke.x < 0.0 {
                stroke.x += self.width;
            }
            if stroke.x > self.width {
                stroke.x -= self.width;
            }
            if stroke.y < 0.0 {
                stroke.y += self.height;
            }
            if stroke.y > self.height {
                stroke.y -= self.height;
            }
        }

        self.strokes.retain(|s| s.age < s.max_age);
    }

    pub fn draw(&self, ctx: &mut Context) {
        for stroke in &self.strokes {
            match stroke.kind {
                StrokeKind::Spray => {
                    ctx.draw(&Points {
                        coords: &[(stroke.x, stroke.y)],
                        color: stroke.color,
                    });
                }
                StrokeKind::Line => {
                    ctx.draw(&Line {
                        x1: stroke.x,
                        y1: stroke.y,
                        x2: stroke.x + stroke.vx * 2.0,
                        y2: stroke.y + stroke.vy * 2.0,
                        color: stroke.color,
                    });
                }
            }
        }
    }
}
