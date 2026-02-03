use crate::process_tree::{ProcessNode, ProcessTree};
use ratatui::{
    style::{Color, Style},
    text::Span,
    widgets::canvas::{Context, Line},
};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct BonsaiRenderer {
    pub max_depth: usize,
    pub zoom: f64,
    pub pan_x: f64,
    pub pan_y: f64,
}

impl BonsaiRenderer {
    pub fn new() -> Self {
        Self {
            max_depth: 8,
            zoom: 1.0,
            pan_x: 0.0,
            pan_y: 0.0,
        }
    }

    pub fn render(&self, ctx: &mut Context, tree: &ProcessTree) {
        // Find main root (pid 1 usually) or just center the roots
        let start_x = 0.0 + self.pan_x;
        let start_y = -30.0 + self.pan_y; // Start at bottom

        let initial_length = 20.0 * self.zoom;
        let initial_angle = 90.0_f64.to_radians();

        // If multiple roots, spread them out
        let spread_x = 20.0 * self.zoom;
        let total_width = (tree.roots.len() as f64 - 1.0) * spread_x;
        let mut x = start_x - total_width / 2.0;

        for root in &tree.roots {
            self.draw_node(ctx, root, x, start_y, initial_angle, initial_length, 0);
            x += spread_x;
        }
    }

    fn draw_node(
        &self,
        ctx: &mut Context,
        node: &ProcessNode,
        x: f64,
        y: f64,
        angle: f64,
        length: f64,
        depth: usize,
    ) {
        if depth >= self.max_depth {
            // Draw leaf
            ctx.print(x, y, Span::styled("🌿", Style::default().fg(Color::Green)));
            return;
        }

        // Calculate end point
        let x2 = x + length * angle.cos();
        let y2 = y + length * angle.sin();

        // Color based on CPU
        let color = if node.cpu_usage > 50.0 {
            Color::Red
        } else if node.cpu_usage > 10.0 {
            Color::Yellow
        } else {
            Color::DarkGray // Wood color
        };

        // Draw branch
        ctx.draw(&Line {
            x1: x,
            y1: y,
            x2,
            y2,
            color,
        });

        // Children
        if node.children.is_empty() {
            ctx.print(
                x2,
                y2,
                Span::styled("🍃", Style::default().fg(Color::Green)),
            );
            return;
        }

        // Limit children to prevent mess
        let max_children = 5;
        let children: Vec<&ProcessNode> = node.children.iter().take(max_children).collect();

        let angle_spread = 90.0_f64.to_radians(); // Total spread
        let start_angle = angle - angle_spread / 2.0;

        // If only one child, just continue straight (with slight jitter)
        if children.len() == 1 {
            let mut hasher = DefaultHasher::new();
            node.pid.hash(&mut hasher);
            children[0].pid.hash(&mut hasher);
            let hash = hasher.finish();
            let jitter = ((hash % 100) as f64 - 50.0) / 100.0 * 0.2;

            self.draw_node(
                ctx,
                children[0],
                x2,
                y2,
                angle + jitter,
                length * 0.9,
                depth + 1,
            );
            return;
        }

        let angle_step = angle_spread / (children.len() as f64 - 1.0);

        for (i, child) in children.iter().enumerate() {
            let mut child_angle = start_angle + i as f64 * angle_step;

            // Add deterministic jitter
            let mut hasher = DefaultHasher::new();
            node.pid.hash(&mut hasher);
            child.pid.hash(&mut hasher);
            let hash = hasher.finish();
            let jitter = ((hash % 100) as f64 - 50.0) / 100.0 * 0.5; // +/- 0.25 rad
            child_angle += jitter;

            self.draw_node(
                ctx,
                child,
                x2,
                y2,
                child_angle,
                length * 0.8, // Decay length
                depth + 1,
            );
        }
    }
}
