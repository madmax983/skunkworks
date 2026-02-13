use crate::chaos::ChaosSubstrate;
use macroquad::prelude::*;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Clone, Copy, PartialEq)]
struct Node {
    pos: IVec2,
    f_score: f32, // g + h
}

impl Eq for Node {}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for Min-Heap behavior in BinaryHeap
        other
            .f_score
            .partial_cmp(&self.f_score)
            .unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct HyphaeNetwork {
    open_set: BinaryHeap<Node>,
    pub came_from: Vec<Option<IVec2>>,
    pub g_score: Vec<f32>,
    pub active_tips: Vec<IVec2>,
    pub width: usize,
    pub height: usize,
    pub target: Option<IVec2>,
    pub found: bool,
    pub start: IVec2,
}

impl HyphaeNetwork {
    pub fn new(width: usize, height: usize, start: IVec2) -> Self {
        let mut open_set = BinaryHeap::new();
        let came_from = vec![None; width * height];
        let mut g_score = vec![f32::INFINITY; width * height];

        // Initialize start
        let start_idx = (start.y as usize) * width + (start.x as usize);
        g_score[start_idx] = 0.0;
        open_set.push(Node {
            pos: start,
            f_score: 0.0,
        });

        Self {
            open_set,
            came_from,
            g_score,
            active_tips: Vec::new(),
            width,
            height,
            target: None,
            found: false,
            start,
        }
    }

    pub fn set_target(&mut self, target: IVec2) {
        self.target = Some(target);
        // Reset search but keep start
        self.found = false;
        self.open_set.clear();
        self.came_from.fill(None);
        self.g_score.fill(f32::INFINITY);

        let start_idx = (self.start.y as usize) * self.width + (self.start.x as usize);
        self.g_score[start_idx] = 0.0;

        // Initial heuristic
        let h = ((self.start.x - target.x).abs() + (self.start.y - target.y).abs()) as f32;
        self.open_set.push(Node {
            pos: self.start,
            f_score: h,
        });
    }

    pub fn reset(&mut self) {
        if let Some(target) = self.target {
            self.set_target(target);
        }
    }

    pub fn update(&mut self, substrate: &ChaosSubstrate, steps: usize) {
        if self.found {
            return;
        }
        if self.target.is_none() {
            return;
        }

        let target = self.target.unwrap();

        self.active_tips.clear();

        for _ in 0..steps {
            if let Some(current) = self.open_set.pop() {
                self.active_tips.push(current.pos);

                if current.pos == target {
                    self.found = true;
                    return;
                }

                // Neighbors (8-way)
                let neighbors = [
                    IVec2::new(0, 1),
                    IVec2::new(0, -1),
                    IVec2::new(1, 0),
                    IVec2::new(-1, 0),
                    IVec2::new(1, 1),
                    IVec2::new(1, -1),
                    IVec2::new(-1, 1),
                    IVec2::new(-1, -1),
                ];

                for &offset in &neighbors {
                    let neighbor = current.pos + offset;

                    if neighbor.x < 0
                        || neighbor.y < 0
                        || neighbor.x >= self.width as i32
                        || neighbor.y >= self.height as i32
                    {
                        continue;
                    }

                    let idx = (neighbor.y as usize) * self.width + (neighbor.x as usize);
                    let curr_idx = (current.pos.y as usize) * self.width + (current.pos.x as usize);

                    // Cost function from ChaosSubstrate
                    let dist = if offset.x != 0 && offset.y != 0 {
                        1.414
                    } else {
                        1.0
                    };
                    let region_cost = substrate.get_cost(neighbor.x, neighbor.y);

                    let move_cost = dist * region_cost;

                    let tentative_g = self.g_score[curr_idx] + move_cost;

                    if tentative_g < self.g_score[idx] {
                        self.came_from[idx] = Some(current.pos);
                        self.g_score[idx] = tentative_g;

                        // Heuristic: Euclidean * weight
                        let h = neighbor.as_vec2().distance(target.as_vec2());

                        self.open_set.push(Node {
                            pos: neighbor,
                            f_score: tentative_g + h * 1.5,
                        });
                    }
                }
            } else {
                break;
            }
        }
    }

    pub fn draw(&self, cell_size: f32) {
        // Draw established paths (Hyphae)
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;
                if let Some(parent) = self.came_from[idx] {
                    let p1 = Vec2::new(parent.x as f32, parent.y as f32) * cell_size
                        + Vec2::splat(cell_size / 2.0);
                    let p2 =
                        Vec2::new(x as f32, y as f32) * cell_size + Vec2::splat(cell_size / 2.0);

                    // Color based on G-Score (gradient)?
                    // let score = self.g_score[idx];
                    // let alpha = (100.0 / (score + 1.0)).min(1.0);

                    draw_line(
                        p1.x,
                        p1.y,
                        p2.x,
                        p2.y,
                        1.0 * (cell_size / 2.0).max(1.0),
                        Color::new(1.0, 1.0, 1.0, 0.4),
                    );
                }
            }
        }

        // Draw active tips
        for tip in &self.active_tips {
            draw_rectangle(
                tip.x as f32 * cell_size,
                tip.y as f32 * cell_size,
                cell_size,
                cell_size,
                GREEN,
            );
        }

        // Draw Path if found
        if self.found {
            if let Some(target) = self.target {
                let mut curr = target;
                while let Some(parent) =
                    self.came_from[(curr.y as usize) * self.width + (curr.x as usize)]
                {
                    let p1 = Vec2::new(parent.x as f32, parent.y as f32) * cell_size
                        + Vec2::splat(cell_size / 2.0);
                    let p2 = Vec2::new(curr.x as f32, curr.y as f32) * cell_size
                        + Vec2::splat(cell_size / 2.0);

                    // Thicker, pulsating artery
                    let time = get_time() as f32;
                    let thickness = 2.0 + (time * 5.0).sin() * 0.5;
                    let pulse_color = Color::new(1.0, 0.2, 0.2, 0.8);

                    draw_line(
                        p1.x,
                        p1.y,
                        p2.x,
                        p2.y,
                        thickness * cell_size.max(1.0),
                        pulse_color,
                    );
                    curr = parent;
                }
            }
        }
    }
}
