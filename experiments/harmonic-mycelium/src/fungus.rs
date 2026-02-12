use macroquad::prelude::*;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub trait CostMap {
    fn get_cost(&self, x: i32, y: i32) -> f32;
}

#[derive(Clone, Copy, PartialEq)]
struct Node {
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
    pub cost_so_far: Vec<f32>, // Dijkstra cost
    pub parent: Vec<Option<IVec2>>, // Tree structure
    frontier: BinaryHeap<Node>,
    pub width: usize,
    pub height: usize,
    pub active_tips: Vec<IVec2>, // For visualization
}

impl HyphaeNetwork {
    pub fn new(width: usize, height: usize, start: IVec2) -> Self {
        let mut frontier = BinaryHeap::new();
        frontier.push(Node { pos: start, cost: 0.0 });

        let mut cost_so_far = vec![f32::INFINITY; width * height];
        let idx = (start.y as usize) * width + (start.x as usize);
        cost_so_far[idx] = 0.0;

        Self {
            cost_so_far,
            parent: vec![None; width * height],
            frontier,
            width,
            height,
            active_tips: Vec::new(),
        }
    }

    pub fn reset(&mut self, start: IVec2) {
        self.frontier.clear();
        self.frontier.push(Node { pos: start, cost: 0.0 });
        self.cost_so_far.fill(f32::INFINITY);
        self.parent.fill(None);
        self.active_tips.clear();

        let idx = (start.y as usize) * self.width + (start.x as usize);
        self.cost_so_far[idx] = 0.0;
    }

    pub fn update(&mut self, map: &impl CostMap, steps: usize) {
        self.active_tips.clear();
        for _ in 0..steps {
            if let Some(Node { pos, cost }) = self.frontier.pop() {
                self.active_tips.push(pos);
                let idx = (pos.y as usize) * self.width + (pos.x as usize);

                // If we found a shorter path previously (lazy deletion), skip
                if cost > self.cost_so_far[idx] { continue; }

                // Neighbors (8-way)
                 let neighbors = [
                    IVec2::new(0, 1), IVec2::new(0, -1),
                    IVec2::new(1, 0), IVec2::new(-1, 0),
                    IVec2::new(1, 1), IVec2::new(1, -1),
                    IVec2::new(-1, 1), IVec2::new(-1, -1),
                ];

                for &offset in &neighbors {
                    let next = pos + offset;
                    if next.x < 0 || next.y < 0 || next.x >= self.width as i32 || next.y >= self.height as i32 {
                        continue;
                    }

                    let next_idx = (next.y as usize) * self.width + (next.x as usize);
                    let step_cost = if offset.x != 0 && offset.y != 0 { 1.414 } else { 1.0 };
                    let terrain_cost = map.get_cost(next.x, next.y);

                    let new_cost = cost + step_cost * terrain_cost;

                    if new_cost < self.cost_so_far[next_idx] {
                        self.cost_so_far[next_idx] = new_cost;
                        self.parent[next_idx] = Some(pos);
                        self.frontier.push(Node { pos: next, cost: new_cost });
                    }
                }
            } else {
                break;
            }
        }
    }

    // Check if a point is reachable (connected)
    pub fn is_connected(&self, pos: IVec2) -> bool {
        if pos.x < 0 || pos.y < 0 || pos.x >= self.width as i32 || pos.y >= self.height as i32 {
            return false;
        }
        let idx = (pos.y as usize) * self.width + (pos.x as usize);
        self.cost_so_far[idx] < f32::INFINITY
    }
}
