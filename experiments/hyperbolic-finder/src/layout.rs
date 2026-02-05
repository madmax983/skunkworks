use crate::fs::DirNode;
use num_complex::Complex;
use std::f64::consts::PI;

#[derive(Debug, Clone)]
pub struct LayoutNode {
    pub pos: Complex<f64>,
    pub node: DirNode,
    pub children: Vec<LayoutNode>,
}

pub fn layout_tree(root: DirNode) -> LayoutNode {
    layout_recursive(root, 0.0, 2.0 * PI, 0)
}

fn layout_recursive(
    mut node: DirNode,
    angle_start: f64,
    angle_end: f64,
    depth: usize,
) -> LayoutNode {
    let step_h = 1.5;
    let r_h = depth as f64 * step_h;
    let r_e = (r_h / 2.0).tanh();

    let angle_center = (angle_start + angle_end) / 2.0;
    let pos = if depth == 0 {
        Complex::new(0.0, 0.0)
    } else {
        Complex::from_polar(r_e, angle_center)
    };

    // Extract children to iterate, leaving `node` with empty children
    let children = std::mem::take(&mut node.children);
    let child_count = children.len();

    let mut children_layout = Vec::new();

    if child_count > 0 {
        let total_angle = angle_end - angle_start;
        let angle_per_child = total_angle / child_count as f64;

        for (i, child) in children.into_iter().enumerate() {
            let child_start = angle_start + i as f64 * angle_per_child;
            let child_end = child_start + angle_per_child;

            children_layout.push(layout_recursive(child, child_start, child_end, depth + 1));
        }
    }

    LayoutNode {
        pos,
        node, // node now has empty children
        children: children_layout,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn mock_node(depth: usize) -> DirNode {
        if depth == 0 {
            DirNode::new(PathBuf::from("leaf"), false, 0)
        } else {
            let mut node = DirNode::new(PathBuf::from("root"), true, 0);
            node.children.push(mock_node(depth - 1));
            node.children.push(mock_node(depth - 1));
            node
        }
    }

    #[test]
    fn test_layout_bounds() {
        let root = mock_node(3);
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
}
