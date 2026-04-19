use crate::tree::Tree;
use macroquad::prelude::*;

pub struct Sun {
    pub position: Vec2,
    pub speed: f32,
    pub mode: ScheduleMode,
}

#[derive(PartialEq, Clone, Copy)]
pub enum ScheduleMode {
    RoundRobin,
    Priority,
}

impl Sun {
    pub fn new() -> Self {
        Self {
            position: vec2(0.0, 50.0),
            speed: 100.0,
            mode: ScheduleMode::RoundRobin,
        }
    }

    pub fn update(&mut self, dt: f32) {
        match self.mode {
            ScheduleMode::RoundRobin => {
                self.position.x += self.speed * dt;
                if self.position.x > screen_width() {
                    self.position.x = 0.0;
                }
            }
            ScheduleMode::Priority => {
                // Priority logic handled externally or here if we pass trees
            }
        }
    }

    pub fn draw(&self) {
        // Core
        draw_circle(self.position.x, self.position.y, 25.0, YELLOW);

        // Inner Glow
        draw_circle(
            self.position.x,
            self.position.y,
            40.0,
            Color::new(1.0, 0.9, 0.5, 0.4),
        );

        // Outer Glow
        draw_circle(
            self.position.x,
            self.position.y,
            60.0,
            Color::new(1.0, 0.8, 0.2, 0.2),
        );

        // Scheduling Ray (Searchlight)
        draw_triangle(
            vec2(self.position.x, self.position.y),
            vec2(self.position.x - 40.0, screen_height()),
            vec2(self.position.x + 40.0, screen_height()),
            Color::new(1.0, 1.0, 0.8, 0.1),
        );
    }

    // Check if sun is above a tree (Scheduling Window)
    pub fn is_shining_on(&self, tree: &Tree) -> bool {
        (self.position.x - tree.position.x).abs() < 40.0
    }
}
