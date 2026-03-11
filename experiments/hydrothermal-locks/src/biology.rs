use crate::fluid::FluidSim;
use crate::grid::{CellType, Grid};
use macroquad::prelude::*;

pub struct Worm {
    pub root: IVec2,
    pub segments: Vec<Vec2>,
    pub length: f32,
    pub max_length: f32,
    pub angle: f32,
    pub health: f32,
}

pub struct Worms {
    pub worms: Vec<Worm>,
}

impl Worms {
    pub fn new() -> Self {
        Self { worms: Vec::new() }
    }

    pub fn update(&mut self, grid: &Grid, fluid: &FluidSim, dt: f32) {
        // Spawn new worms
        // Iterate grid? O(W*H) might be slow every frame.
        // Optimization: Only check random subsets or near vents.
        // For now, let's just check random spots.
        for _ in 0..20 {
            let x = rand::gen_range(1, grid.width - 1);
            let y = rand::gen_range(1, grid.height - 1);

            if let Some(cell) = grid.get(x, y) {
                if cell.cell_type == CellType::Chimney {
                    // Check if occupied
                    let occupied = self
                        .worms
                        .iter()
                        .any(|w| w.root.x == x as i32 && w.root.y == y as i32);
                    if !occupied {
                        let temp = fluid.get_temp(x, y);
                        if temp > 0.2 && temp < 0.6 {
                            // Spawn worm
                            self.worms.push(Worm {
                                root: IVec2::new(x as i32, y as i32),
                                segments: vec![vec2(0.0, 0.0)],
                                length: 0.0,
                                max_length: rand::gen_range(10.0f32, 30.0f32),
                                angle: rand::gen_range(
                                    -std::f32::consts::PI / 4.0,
                                    std::f32::consts::PI / 4.0,
                                ) - std::f32::consts::PI / 2.0, // Upward bias
                                health: 1.0,
                            });
                        }
                    }
                }
            }
        }

        // Update worms
        let mut dead_indices = Vec::new();
        for (i, worm) in self.worms.iter_mut().enumerate() {
            let x = worm.root.x as usize;
            let y = worm.root.y as usize;

            // Check root survival
            let temp = fluid.get_temp(x, y);
            if !(0.1..=0.8).contains(&temp) {
                worm.health -= dt;
            } else {
                worm.health = (worm.health + dt).min(1.0);
            }

            if worm.health <= 0.0 {
                dead_indices.push(i);
                continue;
            }

            // Grow
            if worm.length < worm.max_length {
                worm.length += 10.0 * dt;
            }

            // Animate segments
            // Inverse kinematics or just simple sine wave sway
            let sway = (get_time() as f32 * 2.0 + x as f32).sin() * 0.1;
            worm.angle += sway * dt;

            // Rebuild segments based on length
            let seg_count = (worm.length / 5.0) as usize + 1;
            worm.segments.clear();
            let mut current_pos = vec2(0.0, 0.0);
            worm.segments.push(current_pos);

            for j in 0..seg_count {
                let angle = worm.angle + (j as f32 * 0.2 + get_time() as f32).sin() * 0.2;
                current_pos += vec2(angle.cos() * 5.0, angle.sin() * 5.0);
                worm.segments.push(current_pos);
            }
        }

        for idx in dead_indices.into_iter().rev() {
            self.worms.swap_remove(idx);
        }
    }

    pub fn draw(&self) {
        for worm in &self.worms {
            let root_pos = vec2(
                worm.root.x as f32 * 8.0 + 4.0,
                worm.root.y as f32 * 8.0 + 4.0,
            );
            for i in 0..worm.segments.len() - 1 {
                let start = root_pos + worm.segments[i];
                let end = root_pos + worm.segments[i + 1];
                let color = Color::new(1.0, 0.4 + (worm.health * 0.6), 0.4, 1.0);
                draw_line(start.x, start.y, end.x, end.y, 2.0, color);
            }
        }
    }
}
