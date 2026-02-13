use crate::fs::DirNode;
use poincare_disk::Point;
use std::f64::consts::PI;

#[derive(Debug, Clone)]
pub struct LayoutNode {
    pub pos: Point,
    pub node: DirNode,
    pub children: Vec<LayoutNode>,
    pub angle_start: f64,
    pub angle_end: f64,
}

pub fn layout_tree(root: DirNode) -> LayoutNode {
    // Wrap root in LayoutNode
    // Note: root.total_size is already computed by fs::scan_dir
    let mut root_layout = build_layout_node(root);

    // Compute positions recursively
    update_layout_positions(&mut root_layout, 0.0, 2.0 * PI, 0);

    root_layout
}

fn build_layout_node(mut node: DirNode) -> LayoutNode {
    // Take children to process them
    let children = std::mem::take(&mut node.children);

    let children_layout: Vec<LayoutNode> = children
        .into_iter()
        .map(|child| build_layout_node(child))
        .collect();

    LayoutNode {
        pos: Point::new(0.0, 0.0), // Placeholder, set by update_layout_positions
        node,
        children: children_layout,
        angle_start: 0.0,
        angle_end: 0.0,
    }
}

fn update_layout_positions(node: &mut LayoutNode, angle_start: f64, angle_end: f64, depth: usize) {
    // Hyperbolic radius increases with depth
    // step_h determines how crowded the center is vs the edge
    let step_h = 0.8;
    let r_h = depth as f64 * step_h;

    // Convert to Euclidean radius in Poincaré disk
    // r_e = tanh(r_h / 2)
    let r_e = (r_h / 2.0).tanh();

    let angle_center = (angle_start + angle_end) / 2.0;

    node.pos = if depth == 0 {
        Point::new(0.0, 0.0)
    } else {
        // Use polar coordinates (r_e, angle_center)
        // Note: Point is Complex<f64>
        use num_complex::Complex;
        Complex::from_polar(r_e, angle_center)
    };

    node.angle_start = angle_start;
    node.angle_end = angle_end;

    let child_count = node.children.len();
    if child_count > 0 {
        let total_angle = angle_end - angle_start;

        // Calculate weights based on total_size
        // We use sqrt(size) to dampen the disparity between tiny and huge files
        // We also enforce a minimum virtual size so small files don't disappear
        let weights: Vec<f64> = node
            .children
            .iter()
            .map(|c| {
                let s = c.node.total_size as f64;
                (s.max(1000.0)).sqrt()
            })
            .collect();

        let total_weight: f64 = weights.iter().sum();
        let mut current_angle = angle_start;

        for (i, child) in node.children.iter_mut().enumerate() {
            let weight = weights[i];

            // Calculate allocated angle fraction
            let fraction = if total_weight > 0.0 {
                weight / total_weight
            } else {
                1.0 / child_count as f64
            };

            let allocated_angle = total_angle * fraction;
            let child_end = current_angle + allocated_angle;

            update_layout_positions(child, current_angle, child_end, depth + 1);

            current_angle = child_end;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fs::FileType;
    use std::path::PathBuf;

    fn mock_dir(size: u64) -> DirNode {
        DirNode {
            path: PathBuf::from("dir"),
            name: "dir".to_string(),
            is_dir: true,
            file_type: FileType::Directory,
            children: Vec::new(),
            self_size: 4096,
            total_size: size, // Pre-calculated for test
        }
    }

    // Helper to create a node with specific total_size directly (bypassing calculation)
    // Note: In real usage, scan_dir calculates it.
    fn make_node(size: u64) -> DirNode {
        DirNode {
            path: PathBuf::from("file"),
            name: "file".to_string(),
            is_dir: false,
            file_type: FileType::Text,
            children: Vec::new(),
            self_size: size,
            total_size: size,
        }
    }

    #[test]
    fn test_layout_allocation() {
        let mut root = mock_dir(2000000);
        let small = make_node(1000);
        let big = make_node(1000000);

        root.children.push(small);
        root.children.push(big);

        // Fix total_size manually for test since we aren't using scan_dir
        root.total_size = 1000 + 1000000 + 4096;

        let layout = layout_tree(root);

        let small_l = &layout.children[0];
        let big_l = &layout.children[1];

        let angle_small = small_l.angle_end - small_l.angle_start;
        let angle_big = big_l.angle_end - big_l.angle_start;

        assert!(angle_big > angle_small, "Big file should get more angle");
    }
}
