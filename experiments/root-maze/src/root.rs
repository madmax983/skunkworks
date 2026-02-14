use crate::maze::{Grid, Soil};
use ::rand::Rng;
use macroquad::prelude::*; // Use root rand crate

#[derive(Clone, Copy)]
pub struct RootSegment {
    pub pos: Vec2,
    pub parent: Option<usize>,
    pub thickness: f32,
}

pub struct RootTip {
    pub pos: Vec2,
    pub dir: Vec2,
    pub energy: f32,
    pub parent_segment_idx: usize,
}

pub struct RootSystem {
    pub segments: Vec<RootSegment>,
    pub tips: Vec<RootTip>,
}

impl RootSystem {
    pub fn new(start_pos: Vec2) -> Self {
        let root = RootSegment {
            pos: start_pos,
            parent: None,
            thickness: 2.0,
        };
        let tip = RootTip {
            pos: start_pos,
            dir: vec2(0.0, 1.0), // Downwards
            energy: 1000.0,
            parent_segment_idx: 0,
        };
        Self {
            segments: vec![root],
            tips: vec![tip],
        }
    }

    pub fn grow(&mut self, grid: &Grid) {
        let mut rng = ::rand::thread_rng();
        let mut next_tips = Vec::new();
        let mut new_branches = Vec::new();

        // Move tips out to avoid borrow checker issues
        let current_tips = std::mem::take(&mut self.tips);

        let growth_step = 0.5;
        let sensor_dist = 2.0;

        for mut tip in current_tips {
            if tip.energy <= 0.0 {
                continue; // Die
            }

            // --- Sensing ---
            let mut best_dir = tip.dir;
            let mut best_score = -9999.0;
            let mut found_valid = false;

            let angles: [f32; 5] = [-0.78, -0.35, 0.0, 0.35, 0.78]; // approx -45 to +45 deg

            for angle in angles {
                let sin_a = angle.sin();
                let cos_a = angle.cos();
                let check_dir = vec2(
                    tip.dir.x * cos_a - tip.dir.y * sin_a,
                    tip.dir.x * sin_a + tip.dir.y * cos_a,
                )
                .normalize();

                let check_pos = tip.pos + check_dir * sensor_dist;

                let gx = check_pos.x.round() as i32;
                let gy = check_pos.y.round() as i32;

                if gx < 0 || gy < 0 || gx >= grid.width as i32 || gy >= grid.height as i32 {
                    continue;
                }

                let soil = grid.get(gx as usize, gy as usize);
                let mut score = 0.0;

                match soil {
                    Soil::Water => score += 100.0,
                    Soil::HardRock => score -= 100.0,
                    Soil::SoftSoil => score += 5.0,
                    Soil::Empty => score += 1.0,
                }

                // Tropism: Bias downwards (positive Y)
                score += check_dir.y * 2.0;
                // Random noise
                score += rng.gen_range(-1.0..1.0);

                if score > best_score {
                    best_score = score;
                    best_dir = check_dir;
                    if soil != Soil::HardRock {
                        found_valid = true;
                    }
                }
            }

            if !found_valid {
                // Stuck and died
                continue;
            }

            // --- Move ---
            let wobble: f32 = rng.gen_range(-0.1..0.1);
            let sin_w = wobble.sin();
            let cos_w = wobble.cos();
            let final_dir = vec2(
                best_dir.x * cos_w - best_dir.y * sin_w,
                best_dir.x * sin_w + best_dir.y * cos_w,
            )
            .normalize();

            let new_pos = tip.pos + final_dir * growth_step;

            // Add Segment to self.segments (safe now)
            let new_segment_idx = self.segments.len();
            self.segments.push(RootSegment {
                pos: new_pos,
                parent: Some(tip.parent_segment_idx),
                thickness: (tip.energy / 1000.0).max(0.5) * 2.0,
            });

            // Update Tip State
            tip.pos = new_pos;
            tip.dir = final_dir;
            tip.parent_segment_idx = new_segment_idx;
            tip.energy -= 1.0;

            // --- Branching ---
            // If healthy and lucky, spawn a branch
            if tip.energy > 500.0 && rng.gen_bool(0.02) {
                let split_energy = tip.energy * 0.4;
                tip.energy *= 0.6;

                let branch_angle: f32 = if rng.gen_bool(0.5) { 1.0 } else { -1.0 }; // ~60 degrees
                let sin_b = branch_angle.sin();
                let cos_b = branch_angle.cos();
                let branch_dir = vec2(
                    tip.dir.x * cos_b - tip.dir.y * sin_b,
                    tip.dir.x * sin_b + tip.dir.y * cos_b,
                )
                .normalize();

                new_branches.push(RootTip {
                    pos: tip.pos,
                    dir: branch_dir,
                    energy: split_energy,
                    parent_segment_idx: new_segment_idx,
                });
            }

            next_tips.push(tip);
        }

        next_tips.append(&mut new_branches);
        self.tips = next_tips;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_root_growth() {
        let grid = Grid::new(20, 20);
        let start_pos = vec2(grid.start.0 as f32, grid.start.1 as f32);
        let mut system = RootSystem::new(start_pos);

        assert_eq!(system.segments.len(), 1);
        assert_eq!(system.tips.len(), 1);

        system.grow(&grid);

        // Should have grown unless start position is surrounded by rock (unlikely with Grid::new logic)
        // If start pos is SoftSoil, it should grow.
        // grid.start is guaranteed soft.
        assert!(system.segments.len() >= 2);
    }
}
