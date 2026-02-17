use macroquad::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Resource {
    pub pos: Vec2,
    pub value: f32,
    pub radius: f32,
}

pub struct Scheduler {
    pub resources: Vec<Resource>,
    pub spawn_timer: f32,
    pub spawn_interval: f32,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            resources: Vec::new(),
            spawn_timer: 0.0,
            spawn_interval: 0.5, // Faster spawn
        }
    }

    pub fn update(&mut self, dt: f32, bounds: Rect) {
        self.spawn_timer += dt;
        if self.spawn_timer > self.spawn_interval {
            self.spawn_timer = 0.0;
            // Spawn resource
            // Use macroquad's rand
            let pos = Vec2::new(
                rand::gen_range(bounds.x + 20.0, bounds.x + bounds.w - 20.0),
                rand::gen_range(bounds.y + 20.0, bounds.y + bounds.h - 20.0),
            );
            self.resources.push(Resource {
                pos,
                value: 10.0,
                radius: 8.0,
            });
        }
    }
}
