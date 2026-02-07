use macroquad::prelude::*;
use crate::optimizer::{OptimizerState, OptimizerType};
use crate::landscape::ObjectiveFunction;

pub struct Tip {
    pub pos: Vec2,
    pub optimizer: OptimizerState,
    pub is_root: bool, // true = descent, false = ascent
    pub history: Vec<Vec2>,
    pub active: bool,
    pub color: Color,
}

impl Tip {
    pub fn new(pos: Vec2, opt_type: OptimizerType, is_root: bool) -> Self {
        let color = if is_root {
            Color::new(0.55, 0.27, 0.07, 0.8) // SaddleBrown
        } else {
            opt_type.color()
        };

        Self {
            pos,
            optimizer: OptimizerState::new(opt_type),
            is_root,
            history: vec![pos],
            active: true,
            color,
        }
    }

    pub fn grow(&mut self, landscape: &dyn ObjectiveFunction, learning_rate: f32, bounds: Rect) {
        if !self.active { return; }

        let grad = landscape.gradient(self.pos.x, self.pos.y);

        // Check for convergence (small gradient)
        if grad.length_squared() < 0.000001 {
            self.active = false;
            return;
        }

        let step = self.optimizer.compute_step(grad, learning_rate);

        // If step is tiny, stop
        if step.length_squared() < 0.000001 {
            self.active = false;
            return;
        }

        // Check for NaN or Inf
        if step.is_nan() {
            self.active = false;
            return;
        }

        let next_pos = if self.is_root {
            self.pos - step // Minimize
        } else {
            self.pos + step // Maximize
        };

        // Check bounds
        if !bounds.contains(next_pos) {
            self.active = false;
            return;
        }

        self.pos = next_pos;
        self.history.push(self.pos);

        // Limit history to prevent OOM on long runs, but we want full trails.
        // Maybe downsample? For now, keep all.
    }

    pub fn draw(&self, _offset: Vec2) {
        if self.history.len() < 2 { return; }

        // We only draw if active or history exists.
        // Transforming points to screen space should happen in main, but here we can use passed transforms.
        // Actually, let's assume world coordinates match screen coordinates for now,
        // or the camera is applied via `set_camera` in macroquad (but that's for 2D/3D).
        // Since we are doing a 2D top-down view, we can just draw lines.

        // Optimization: draw as line strip
        for i in 0..self.history.len()-1 {
            let p1 = self.history[i];
            let p2 = self.history[i+1];

            // Apply simple transform if needed, but macroquad has Camera2D.
            // Let's assume Camera2D is active.

            draw_line(
                p1.x, p1.y,
                p2.x, p2.y,
                if self.is_root { 0.15 } else { 0.1 }, // Fixed world thickness (scales with zoom)
                self.color
            );
        }

        // Draw tip
        if self.active {
            draw_circle(self.pos.x, self.pos.y, 0.2, RED);
        }
    }
}

pub struct Plant {
    pub tips: Vec<Tip>,
}

impl Plant {
    pub fn new(pos: Vec2, opt_type: OptimizerType) -> Self {
        Self {
            tips: vec![
                Tip::new(pos, opt_type, true), // Root
                Tip::new(pos, opt_type, false), // Stem
            ],
        }
    }

    pub fn update(&mut self, landscape: &dyn ObjectiveFunction, learning_rate: f32, bounds: Rect) {
        for tip in &mut self.tips {
            tip.grow(landscape, learning_rate, bounds);
        }
    }

    pub fn draw(&self, offset: Vec2) {
        for tip in &self.tips {
            tip.draw(offset);
        }
    }
}
