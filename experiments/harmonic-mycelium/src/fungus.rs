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
    pub targets: Vec<IVec2>,
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
            targets: Vec::new(),
            start,
        }
    }

    pub fn set_targets(&mut self, targets: Vec<IVec2>) {
        self.targets = targets;
    }

    pub fn reset(&mut self) {
        self.open_set.clear();
        self.came_from.fill(None);
        self.g_score.fill(f32::INFINITY);
        self.active_tips.clear();

        let start_idx = (self.start.y as usize) * self.width + (self.start.x as usize);
        self.g_score[start_idx] = 0.0;
        self.open_set.push(Node {
            pos: self.start,
            f_score: 0.0,
        });
    }

    pub fn update<F>(&mut self, cost_provider: F, steps: usize)
    where
        F: Fn(i32, i32) -> f32,
    {
        if self.open_set.is_empty() {
            return;
        }

        self.active_tips.clear();

        for _ in 0..steps {
            if let Some(current) = self.open_set.pop() {
                self.active_tips.push(current.pos);

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

                    let dist_mult = if offset.x != 0 && offset.y != 0 {
                        1.414
                    } else {
                        1.0
                    };
                    let cell_cost = cost_provider(neighbor.x, neighbor.y);

                    let move_cost = dist_mult * cell_cost;

                    let tentative_g = self.g_score[curr_idx] + move_cost;

                    if tentative_g < self.g_score[idx] {
                        self.came_from[idx] = Some(current.pos);
                        self.g_score[idx] = tentative_g;

                        // Heuristic: Min dist to any target
                        let mut min_h = f32::MAX;
                        if self.targets.is_empty() {
                            min_h = 0.0;
                        } else {
                            for t in &self.targets {
                                let d = neighbor.as_vec2().distance(t.as_vec2());
                                if d < min_h {
                                    min_h = d;
                                }
                            }
                        }

                        // Add tie-breaker to favor straight lines or specific paths?
                        // Just use standard A*
                        self.open_set.push(Node {
                            pos: neighbor,
                            f_score: tentative_g + min_h * 1.5,
                        });
                    }
                }
            } else {
                break;
            }
        }
    }

    pub fn draw(&self, cell_size: Vec2) {
        // Draw established paths (Hyphae)
        // This can be slow if we iterate all pixels.
        // Optimization: Only draw if visible?
        // Or just iterate came_from.

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;
                if let Some(parent) = self.came_from[idx] {
                    let p1 =
                        Vec2::new(parent.x as f32, parent.y as f32) * cell_size + cell_size / 2.0;
                    let p2 = Vec2::new(x as f32, y as f32) * cell_size + cell_size / 2.0;

                    // Fade out distant branches?
                    // Use g_score for color?
                    let cost = self.g_score[idx];
                    let alpha = (100.0 / (cost + 1.0)).clamp(0.1, 0.6);

                    draw_line(
                        p1.x,
                        p1.y,
                        p2.x,
                        p2.y,
                        1.0,
                        Color::new(0.5, 0.8, 0.5, alpha),
                    );
                }
            }
        }

        // Draw active tips
        for tip in &self.active_tips {
            draw_rectangle(
                tip.x as f32 * cell_size.x,
                tip.y as f32 * cell_size.y,
                cell_size.x,
                cell_size.y,
                GREEN,
            );
        }
    }
}
