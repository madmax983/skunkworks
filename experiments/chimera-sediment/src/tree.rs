use crate::lsystem::Grammar;
use crate::monitor::ProcessStats;
use crate::sediment::SedimentParticle;
use ::rand::rngs::StdRng;
use ::rand::{Rng, SeedableRng};
use macroquad::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct Tree {
    pub stats: ProcessStats,
    pub grammar: Grammar,
    pub position: Vec2,
    pub color: Color,
    pub scale: f32,
    pub angle_step: f32,
}

impl Tree {
    pub fn new(stats: ProcessStats, position: Vec2) -> Self {
        let mut hasher = DefaultHasher::new();
        stats.pid.hash(&mut hasher);
        let seed = hasher.finish();
        let mut rng = StdRng::seed_from_u64(seed);

        // Procedural Grammar Generation
        let tree_type = rng.gen_range(0..3);
        let (axiom, rules, angle) = match tree_type {
            0 => ("F", vec![('F', "F[+F]F[-F]F")], 25.7),
            1 => ("X", vec![('X', "F+[[X]-X]-F[-FX]+X"), ('F', "FF")], 22.5),
            _ => ("F", vec![('F', "FF-[-F+F+F]+[+F-F-F]")], 22.5),
        };

        let grammar = Grammar::new(axiom, rules);

        let r = rng.gen_range(0.3..0.8);
        let g = rng.gen_range(0.5..1.0);
        let b = rng.gen_range(0.3..0.8);

        // Scale based on memory (logarithmic scale)
        let mb = stats.memory as f32 / 1024.0 / 1024.0;
        let scale = (mb.log10() * 0.5).clamp(0.5, 3.0);

        Self {
            stats,
            grammar,
            position,
            color: Color::new(r, g, b, 1.0),
            scale,
            angle_step: angle,
        }
    }

    pub fn collapse(&self) -> Vec<SedimentParticle> {
        let mut particles = Vec::new();
        let mut rng = ::rand::thread_rng();

        // Amount of sediment proportional to memory size
        let mb = self.stats.memory as f32 / 1024.0 / 1024.0;
        let count = (mb * 0.5).clamp(5.0, 50.0) as usize;

        for _ in 0..count {
            let offset_x = rng.gen_range(-20.0..20.0) * self.scale;
            let offset_y = rng.gen_range(-100.0..0.0) * self.scale; // Falling from height

            let pos = vec2(self.position.x + offset_x, self.position.y + offset_y);
            let size = rng.gen_range(2.0..5.0);

            // Color is a decayed version of tree color
            let color = Color::new(
                self.color.r * 0.8,
                self.color.g * 0.8,
                self.color.b * 0.8,
                1.0
            );

            particles.push(SedimentParticle::new(pos, color, size));
        }

        particles
    }

    pub fn contains(&self, point: Vec2) -> bool {
        let mb = self.stats.memory as f32 / 1024.0 / 1024.0;
        let root_radius = (mb.sqrt() * 0.5).clamp(2.0, 20.0);

        if self.position.distance(point) < root_radius.max(15.0) {
            return true;
        }

        let height_est = 150.0 * self.scale;
        let width_est = 30.0 * self.scale;
        let dx = (point.x - self.position.x).abs();
        let dy = self.position.y - point.y;

        dx < width_est && dy > 0.0 && dy < height_est
    }

    pub fn draw(&self, is_scheduled: bool) {
        let iterations = if is_scheduled { 4 } else { 3 };
        // Simple recursive drawing (simplified for brevity here, logic same as original)
        // Note: The original used recursion or stack.
        // Let's reimplement stack based drawing from original file since I overwrote it.

        // Re-implementing logic from original file read:
        let instructions = self.grammar.expand(iterations);
        let mut stack: Vec<(Vec2, f32)> = Vec::new();
        let mut pos = self.position;
        let mut angle = -90.0f32.to_radians();
        let length = 5.0 * self.scale;
        let turn_angle = self.angle_step.to_radians();

        for c in instructions.chars() {
            match c {
                'F' | 'G' | 'X' => {
                    if c == 'F' || c == 'G' {
                        let next_pos =
                            vec2(pos.x + angle.cos() * length, pos.y + angle.sin() * length);

                        let thickness = (if is_scheduled { 3.0 } else { 1.5 }) * self.scale;
                        let line_color = if is_scheduled { LIGHTGRAY } else { self.color };

                        draw_line(pos.x, pos.y, next_pos.x, next_pos.y, thickness, line_color);

                        // Foliage
                        /*
                        if stack.len() > 1 {
                             draw_circle(next_pos.x, next_pos.y, 2.0 * self.scale, if self.stats.cpu_usage > 50.0 { RED } else { DARKGREEN });
                        }
                        */

                        pos = next_pos;
                    }
                }
                '+' => angle += turn_angle,
                '-' => angle -= turn_angle,
                '[' => stack.push((pos, angle)),
                ']' => {
                    if let Some((p, a)) = stack.pop() {
                        pos = p;
                        angle = a;
                    }
                }
                _ => {}
            }
        }

        // Draw Root
        let mb = self.stats.memory as f32 / 1024.0 / 1024.0;
        let root_radius = (mb.sqrt() * 0.5).clamp(2.0, 20.0);
        draw_circle(self.position.x, self.position.y, root_radius, BROWN);

        // Draw Label
        if self.stats.cpu_usage > 5.0 || is_scheduled {
            draw_text(
                &self.stats.name,
                self.position.x - 10.0,
                self.position.y + 15.0,
                16.0,
                WHITE,
            );
        }
    }
}
