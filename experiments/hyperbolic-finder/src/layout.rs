use crate::fs::DirNode;
use poincare_disk::Point;
use std::f64::consts::PI;

#[derive(Debug, Clone)]
pub struct LayoutNode {
    pub pos: Point,
    pub node: DirNode,
    pub children: Vec<LayoutNode>,
    pub total_size: u64,
    pub angle_start: f64,
    pub angle_end: f64,
}

pub fn layout_tree(root: DirNode) -> LayoutNode {
    let mut root_layout = build_sized_tree(root);
    update_layout_positions(&mut root_layout, 0.0, 2.0 * PI, 0);
    root_layout
}

fn build_sized_tree(mut node: DirNode) -> LayoutNode {
    let mut total_size = node.size;
    let mut children_layout = Vec::new();

    // Take children to avoid partial move of node
    let children = std::mem::take(&mut node.children);

    for child in children {
        let child_layout = build_sized_tree(child);
        total_size += child_layout.total_size;
        children_layout.push(child_layout);
    }

    LayoutNode {
        pos: Point::new(0.0, 0.0), // Placeholder
        node,
        children: children_layout,
        total_size,
        angle_start: 0.0, // Placeholder
        angle_end: 0.0,   // Placeholder
    }
}

fn update_layout_positions(
    node: &mut LayoutNode,
    angle_start: f64,
    angle_end: f64,
    depth: usize,
) {
    // Reduced step size to make deeper nodes visible in the disk before hitting the boundary.
    let step_h = 0.8;
    let r_h = depth as f64 * step_h;
    let r_e = (r_h / 2.0).tanh(); // r_euclidean = tanh(r_hyperbolic / 2)

    let angle_center = (angle_start + angle_end) / 2.0;

    node.pos = if depth == 0 {
        Point::new(0.0, 0.0)
    } else {
        Point::from_polar(r_e, angle_center)
    };

    node.angle_start = angle_start;
    node.angle_end = angle_end;

    let child_count = node.children.len();
    if child_count > 0 {
        let total_angle = angle_end - angle_start;

        // Calculate weights for proportional allocation
        // Use sqrt(size) to dampen the effect of massive files
        // Ensure a minimum weight so small files don't disappear
        let weights: Vec<f64> = node.children.iter().map(|c| {
            let s = c.total_size as f64;
            // 1000.0 is arbitrary minimum 'virtual' bytes for visibility
            (s.max(1000.0)).sqrt()
        }).collect();

        let total_weight: f64 = weights.iter().sum();

        let mut current_angle = angle_start;

        for (i, child) in node.children.iter_mut().enumerate() {
            let weight = weights[i];
            let angle_fraction = if total_weight > 0.0 {
                weight / total_weight
            } else {
                1.0 / child_count as f64
            };

            let allocated_angle = total_angle * angle_fraction;
            let child_end = current_angle + allocated_angle;

            update_layout_positions(child, current_angle, child_end, depth + 1);

            current_angle = child_end;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn mock_dir(size: u64) -> DirNode {
        DirNode::new(PathBuf::from("dir"), true, size)
    }

    fn mock_file(size: u64) -> DirNode {
        DirNode::new(PathBuf::from("file"), false, size)
    }

    #[test]
    fn test_layout_bounds() {
        let mut root = mock_dir(0);
        let child = mock_file(100);
        root.children.push(child);
        let layout = layout_tree(root);

        check_bounds(&layout);
    }

    fn check_bounds(node: &LayoutNode) {
        assert!(
            node.pos.norm() < 1.0,
            "Point {:?} is outside disk",
            node.pos
        );
        for child in &node.children {
            check_bounds(child);
        }
    }

    #[test]
    fn test_proportional_layout() {
        // Root with two children: one tiny, one huge
        let mut root = mock_dir(0);
        let small = mock_file(100); // 100 bytes
        let large = mock_file(1_000_000_000); // 1 GB

        root.children.push(small);
        root.children.push(large);

        // Layout
        let layout = layout_tree(root);

        assert_eq!(layout.children.len(), 2);

        // Find children (order might be maintained or not, scan_dir sorts but we pushed manually)
        // update_layout_positions iterates in order.
        let child1 = &layout.children[0];
        let child2 = &layout.children[1];

        // Identify by total_size
        let (small_layout, large_layout) = if child1.total_size < child2.total_size {
            (child1, child2)
        } else {
            (child2, child1)
        };

        let width_small = small_layout.angle_end - small_layout.angle_start;
        let width_large = large_layout.angle_end - large_layout.angle_start;

        println!("Small Width: {}, Large Width: {}", width_small, width_large);

        // Expectation: Large width should be significantly larger
        // Weight ratio: sqrt(1e9) / sqrt(1e3) = 31622 / 31.6 = ~1000
        // Or if minimal size kicks in: sqrt(1e9) / sqrt(1000) (if 100 was clamped to 1000)
        // 31622 / 31.6 = ~1000.
        // So width_large should be ~1000x width_small.

        assert!(width_large > width_small * 2.0, "Large file should have significantly more angular space");
    }
}
