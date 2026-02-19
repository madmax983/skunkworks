use crate::substrate::StegoSubstrate;
use macroquad::prelude::*;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Clone, Copy, PartialEq)]
pub struct Node {
    pos: IVec2,
    cost: f32,
}

impl Eq for Node {}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse for Min-Heap
        other.cost.partial_cmp(&self.cost).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct HyphaeNetwork {
    pub width: usize,
    pub height: usize,
    pub visited: Vec<bool>,
    pub parent: Vec<Option<IVec2>>,
    pub open_set: BinaryHeap<Node>,
    pub cost_map: Vec<f32>,
    pub active_tips: Vec<IVec2>,
    pub start_pos: IVec2,
}

impl HyphaeNetwork {
    pub fn new(width: usize, height: usize, start: IVec2) -> Self {
        let mut open_set = BinaryHeap::new();
        let visited = vec![false; width * height];
        let parent = vec![None; width * height];
        let mut cost_map = vec![f32::INFINITY; width * height];

        let start_idx = (start.y as usize) * width + (start.x as usize);
        cost_map[start_idx] = 0.0;
        open_set.push(Node { pos: start, cost: 0.0 });

        Self {
            width,
            height,
            visited,
            parent,
            open_set,
            cost_map,
            active_tips: Vec::new(),
            start_pos: start,
        }
    }

    pub fn update(&mut self, substrate: &StegoSubstrate, steps: usize) {
        self.active_tips.clear();

        for _ in 0..steps {
            if let Some(node) = self.open_set.pop() {
                let idx = (node.pos.y as usize) * self.width + (node.pos.x as usize);

                if self.visited[idx] {
                    continue;
                }
                self.visited[idx] = true;
                self.active_tips.push(node.pos);

                // Neighbors (8-way)
                let neighbors = [
                    IVec2::new(1, 0), IVec2::new(-1, 0),
                    IVec2::new(0, 1), IVec2::new(0, -1),
                    IVec2::new(1, 1), IVec2::new(-1, -1),
                    IVec2::new(1, -1), IVec2::new(-1, 1),
                ];

                for &offset in &neighbors {
                    let next_pos = node.pos + offset;

                    if next_pos.x < 0 || next_pos.y < 0 || next_pos.x >= self.width as i32 || next_pos.y >= self.height as i32 {
                        continue;
                    }

                    let next_idx = (next_pos.y as usize) * self.width + (next_pos.x as usize);
                    if self.visited[next_idx] {
                        continue;
                    }

                    let move_cost = if offset.x != 0 && offset.y != 0 { 1.414 } else { 1.0 };
                    let cell_cost = substrate.get_cost(next_pos.x, next_pos.y);

                    let new_cost = node.cost + move_cost * cell_cost;

                    if new_cost < self.cost_map[next_idx] {
                        self.cost_map[next_idx] = new_cost;
                        self.parent[next_idx] = Some(node.pos);
                        self.open_set.push(Node { pos: next_pos, cost: new_cost });
                    }
                }
            } else {
                break;
            }
        }
    }

    // Trace back from the furthest tip to decode message
    pub fn get_message(&self, substrate: &StegoSubstrate) -> String {
        // Find tip with max cost (furthest)?
        // Actually, we want the tip that is at the end of the vein.
        // The vein ends at high x.
        // Let's just pick the active tip with highest X?
        // Or highest cost_map value among visited?

        // Let's find the visited node with max x.
        let mut best_pos = self.start_pos;
        let mut max_x = -1;

        for (i, &v) in self.visited.iter().enumerate() {
            if v {
                let x = (i % self.width) as i32;
                let y = (i / self.width) as i32;
                if x > max_x {
                    max_x = x;
                    best_pos = IVec2::new(x, y);
                }
            }
        }

        // Backtrace
        let mut bits = Vec::new();
        let mut curr = best_pos;

        // Safety break
        let mut loops = 0;
        while curr != self.start_pos && loops < 10000 {
            if let Some(bit) = substrate.get_bit(curr.x, curr.y) {
                bits.push(bit);
            }
            let idx = (curr.y as usize) * self.width + (curr.x as usize);
            match self.parent[idx] {
                Some(p) => curr = p,
                None => break,
            }
            loops += 1;
        }
        // Add start bit if any
        if let Some(bit) = substrate.get_bit(self.start_pos.x, self.start_pos.y) {
            bits.push(bit);
        }

        // Reverse to get start->end order
        bits.reverse();

        // Decode
        let mut text = String::new();
        for chunk in bits.chunks(8) {
            if chunk.len() == 8 {
                let mut byte = 0u8;
                for (i, &b) in chunk.iter().enumerate() {
                    if b == 1 {
                        byte |= 1 << i;
                    }
                }
                if (32..=126).contains(&byte) {
                    text.push(byte as char);
                } else {
                    text.push('.');
                }
            }
        }

        text
    }

    pub fn draw(&self, cell_w: f32, cell_h: f32) {
        // Draw visited (Mycelium body)
        // Optimization: Draw points for visited nodes
        // Or lines.

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;
                if self.visited[idx] {
                    // Draw pixel
                    draw_rectangle(x as f32 * cell_w, y as f32 * cell_h, cell_w, cell_h, Color::new(0.8, 1.0, 0.8, 0.3));
                }
            }
        }

        // Draw Tips
        for tip in &self.active_tips {
            draw_rectangle(tip.x as f32 * cell_w, tip.y as f32 * cell_h, cell_w, cell_h, GREEN);
        }
    }
}
