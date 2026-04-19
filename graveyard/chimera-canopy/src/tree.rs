use crate::lsystem::Grammar;
use crate::monitor::ProcessStats;
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
        // Choose between 3 types
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

    pub fn contains(&self, point: Vec2) -> bool {
        let mb = self.stats.memory as f32 / 1024.0 / 1024.0;
        let root_radius = (mb.sqrt() * 0.5).clamp(2.0, 20.0);

        // Check base circle (generous hit area)
        if self.position.distance(point) < root_radius.max(15.0) {
            return true;
        }

        // Check vertical column for foliage (approximate)
        let height_est = 150.0 * self.scale;
        let width_est = 30.0 * self.scale;

        let dx = (point.x - self.position.x).abs();
        let dy = self.position.y - point.y; // positive up

        dx < width_est && dy > 0.0 && dy < height_est
    }

    pub fn draw(&self, is_scheduled: bool) {
        // Calculate iterations based on CPU usage? Or just fixed?
        // Let's use fixed for stability, but thickness based on CPU?
        let iterations = if is_scheduled { 4 } else { 3 };

        let instructions = self.grammar.expand(iterations);

        let mut stack: Vec<(Vec2, f32)> = Vec::new(); // (Pos, Angle)
        let mut pos = self.position;
        let mut angle = -90.0f32.to_radians(); // Up
        let length = 5.0 * self.scale;
        let turn_angle = self.angle_step.to_radians();

        for c in instructions.chars() {
            match c {
                'F' | 'G' | 'X' => {
                    // Treat X as draw too if not handled, or ignore.
                    // Usually X is just control logic, F is draw.
                    // But in "F", F is draw.
                    // In "X", F is draw.
                    if c == 'F' || c == 'G' {
                        let next_pos =
                            vec2(pos.x + angle.cos() * length, pos.y + angle.sin() * length);

                        // Thickness
                        let thickness = if is_scheduled { 3.0 } else { 1.5 } * self.scale;
                        let mut line_color = self.color;
                        if is_scheduled {
                            line_color = LIGHTGRAY; // Highlight branches
                        }

                        draw_line(pos.x, pos.y, next_pos.x, next_pos.y, thickness, line_color);

                        // Draw leaf at end of segment if deeply nested?
                        // Simplified: Just draw small circles at joints
                        if stack.len() > 1 {
                            draw_circle(
                                next_pos.x,
                                next_pos.y,
                                2.0 * self.scale,
                                if self.stats.cpu_usage > 50.0 {
                                    RED
                                } else {
                                    DARKGREEN
                                },
                            );
                        }

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

        // Draw Root / Memory Indicator
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
            draw_text(
                &format!("{:.1}% CPU", self.stats.cpu_usage),
                self.position.x - 10.0,
                self.position.y + 30.0,
                12.0,
                GOLD,
            );
        }
    }
}
