use crate::fs_scanner::Node;
use crate::surface::figure_8_immersion;
use glam::Vec3;
use macroquad::color::{Color, GOLD};
use std::f32::consts::PI;

pub fn layout_nodes(nodes: &[Node]) -> Vec<(Vec3, Color)> {
    let mut positions = Vec::new();
    let n = nodes.len();
    if n == 0 {
        return positions;
    }

    // Parameters for layout
    // We map the linear list of nodes to a spiral on the Klein bottle surface.
    // u: Major parameter [0, 2PI]
    // v: Minor parameter [0, 2PI]

    // Using a spiral ensures good distribution.
    // spiral_factor determines how many times the spiral winds around the tube (minor circle)
    // as it traverses the length of the tube (major circle).
    let spiral_factor = 20.0;

    for (i, node) in nodes.iter().enumerate() {
        let t = i as f32 / n as f32;

        // u maps linearly to t
        let u = t * 2.0 * PI;

        // v rotates rapidly
        let v = t * 2.0 * PI * spiral_factor;

        let pos = figure_8_immersion(u, v);

        if pos.is_nan() {
            eprintln!("Warning: NaN position generated for node {}", i);
            continue;
        }

        let color = if node.is_dir {
            GOLD
        } else {
            // Gradient based on depth: deeper = darker/purple, shallower = blue/cyan
            let depth_f = (node.depth as f32 * 0.15).min(1.0);
            Color::new(0.2, 0.8 - depth_f * 0.6, 1.0 - depth_f * 0.3, 1.0)
        };

        positions.push((pos, color));
    }

    positions
}
