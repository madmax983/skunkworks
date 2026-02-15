use crate::tracer::Event;
use macroquad::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Node {
    pub id: usize,
    pub parent_id: Option<usize>,
    pub children: Vec<usize>,
    pub depth: usize,

    // Visual properties
    pub angle_offset: f32,
    pub length: f32,
    pub thickness: f32,

    // State
    pub active: bool,
    pub completed: bool,
    pub result: Option<String>,
}

pub struct Garden {
    pub nodes: HashMap<usize, Node>,
    pub root_id: Option<usize>,
    pub active_stack: Vec<usize>,

    // Visual settings
    base_length: f32,
    branch_angle: f32,
}

impl Garden {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            root_id: None,
            active_stack: Vec::new(),
            base_length: 100.0,
            branch_angle: 0.5, // Radians
        }
    }

    pub fn process_event(&mut self, event: Event) {
        match event {
            Event::Enter {
                id,
                name: _,
                args: _,
                parent_id,
            } => {
                let depth = if let Some(pid) = parent_id {
                    self.nodes.get(&pid).map(|n| n.depth + 1).unwrap_or(0)
                } else {
                    0
                };

                // Determine angle based on sibling index
                let angle_offset = if let Some(pid) = parent_id {
                    if let Some(parent) = self.nodes.get_mut(&pid) {
                        let sibling_count = parent.children.len();
                        parent.children.push(id);

                        // Simple binary distribution for now
                        // 0 -> -angle, 1 -> +angle, 2 -> -angle * 0.5 ...
                        // Better: Alternating
                        let sign = if sibling_count % 2 == 0 { -1.0 } else { 1.0 };
                        let magnitude = 1.0; // Could decay for many siblings
                        sign * self.branch_angle * magnitude
                    } else {
                        0.0
                    }
                } else {
                    0.0 // Root grows straight up
                };

                let node = Node {
                    id,
                    parent_id,
                    children: Vec::new(),
                    depth,
                    angle_offset,
                    length: self.base_length / (1.0 + depth as f32 * 0.3),
                    thickness: 10.0 / (1.0 + depth as f32 * 0.5),
                    active: true,
                    completed: false,
                    result: None,
                };

                self.nodes.insert(id, node);
                self.active_stack.push(id);

                if parent_id.is_none() {
                    self.root_id = Some(id);
                }
            }
            Event::Exit { id, result } => {
                if let Some(node) = self.nodes.get_mut(&id) {
                    node.active = false;
                    node.completed = true;
                    node.result = Some(result);
                }
                // Remove from stack if it's the top (should be)
                if let Some(&top) = self.active_stack.last() {
                    if top == id {
                        self.active_stack.pop();
                    }
                }
            }
        }
    }

    pub fn draw(&self) {
        if let Some(root_id) = self.root_id {
            self.draw_node(
                root_id,
                screen_width() / 2.0,
                screen_height(),
                -std::f32::consts::FRAC_PI_2,
            );
        }
    }

    fn draw_node(&self, id: usize, x: f32, y: f32, angle: f32) {
        if let Some(node) = self.nodes.get(&id) {
            let time = get_time() as f32;

            // Wind sway
            let sway = (time * 2.0 + node.depth as f32).sin() * 0.05 * (node.depth as f32 * 0.1);
            let final_angle = angle + node.angle_offset + sway;

            let end_x = x + final_angle.cos() * node.length;
            let end_y = y + final_angle.sin() * node.length;

            // Color
            let color = if node.active {
                Color::new(1.0, 0.5, 0.0, 1.0) // Orange glow
            } else if node.completed {
                // Gradient from brown (trunk) to green (branches)
                let t = (node.depth as f32 / 10.0).min(1.0);
                Color::new(0.4 * (1.0 - t), 0.3 * (1.0 - t) + 0.5 * t, 0.1, 1.0)
            } else {
                GRAY // Should not happen often
            };

            draw_line(x, y, end_x, end_y, node.thickness, color);

            // Draw result as fruit/flower
            if node.completed && node.children.is_empty() {
                draw_circle(end_x, end_y, 5.0, PINK);
                if let Some(_res) = &node.result {
                    // Too much text, maybe only on hover?
                    // draw_text(res, end_x, end_y, 15.0, WHITE);
                }
            }

            // Recursion
            for &child_id in &node.children {
                self.draw_node(child_id, end_x, end_y, final_angle);
            }
        }
    }
}
