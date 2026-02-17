use crate::math::mobius_add;
use crate::scanner::FSNode;
use macroquad::prelude::Vec3;
use std::f32::consts::PI;

pub struct LayoutNode {
    pub position: Vec3,
    pub node: FSNode,
    pub children: Vec<LayoutNode>,
}

pub fn layout_tree(root: FSNode) -> LayoutNode {
    build_layout(root, Vec3::ZERO, 0)
}

fn build_layout(mut node: FSNode, pos: Vec3, depth: usize) -> LayoutNode {
    let mut layout_children = Vec::new();
    // Take children out of the node to avoid partial move issues
    // We replace them with an empty vector, so 'node' is still valid but has no children.
    let children = std::mem::take(&mut node.children);
    let child_count = children.len();

    // Configuration
    let base_radius = 0.4;
    let radius = base_radius;

    // Fibonacci Sphere Distribution
    let golden_ratio = (1.0 + 5.0f32.sqrt()) / 2.0;

    for (i, child) in children.into_iter().enumerate() {
        // Fibonacci Sphere
        let idx = i as f32 + 0.5;

        let phi = (2.0 * PI * idx) / golden_ratio;
        let cos_theta = 1.0 - 2.0 * idx / (child_count as f32);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let x = sin_theta * phi.cos() * radius;
        let y = sin_theta * phi.sin() * radius;
        let z = cos_theta * radius;

        let displacement = Vec3::new(x, y, z);
        let child_pos = mobius_add(pos, displacement);

        layout_children.push(build_layout(child, child_pos, depth + 1));
    }

    LayoutNode {
        position: pos,
        node,
        children: layout_children,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::NodeType;
    use std::path::PathBuf;

    fn make_dummy_node(name: &str, children_count: usize) -> FSNode {
        let mut children = Vec::new();
        for i in 0..children_count {
            children.push(FSNode::new(
                PathBuf::from(format!("{}/child_{}", name, i)),
                NodeType::File,
                100,
            ));
        }

        let mut node = FSNode::new(PathBuf::from(name), NodeType::Directory, 0);
        node.children = children;
        node
    }

    #[test]
    fn test_layout_generation() {
        let root = make_dummy_node("root", 5);
        let layout = layout_tree(root);

        assert_eq!(layout.position, Vec3::ZERO);
        assert_eq!(layout.children.len(), 5);

        for child in layout.children {
            // Check if child is inside unit ball
            assert!(
                child.position.length() < 1.0,
                "Child at {:?} escaped unit ball",
                child.position
            );
            // Check distance from origin (should be radius since parent is at origin)
            // Note: displacement length is 0.4.
            // mobius_add(0, v) = v.
            assert!((child.position.length() - 0.4).abs() < 1e-5);
        }
    }

    #[test]
    fn test_deep_layout() {
        // Create a deep chain
        let mut root = FSNode::new(PathBuf::from("root"), NodeType::Directory, 0);
        let mut current = &mut root;
        // This is tricky with ownership, let's just make a recursive helper or simple manual chain
        // root -> child -> grandchild

        let grandchild = FSNode::new(PathBuf::from("grandchild"), NodeType::File, 10);
        let mut child = FSNode::new(PathBuf::from("child"), NodeType::Directory, 0);
        child.children.push(grandchild);
        root.children.push(child);

        let layout = layout_tree(root);
        let child_layout = &layout.children[0];
        let grandchild_layout = &child_layout.children[0];

        assert!(grandchild_layout.position.length() < 1.0);
        println!("Grandchild Pos: {:?}", grandchild_layout.position);
    }
}
