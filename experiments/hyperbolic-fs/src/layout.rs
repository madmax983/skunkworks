use crate::fs::FsNode;
use num_complex::Complex;
use poincare_disk::Point;
use std::f64::consts::PI;

#[derive(Debug, Clone)]
pub struct LayoutNode {
    pub fs: FsNode,
    pub global_pos: Point,
    pub children: Vec<LayoutNode>,
}

pub fn build_layout(fs_node: FsNode) -> LayoutNode {
    // We consume the fs_node to avoid cloning heavy path buffers if possible,
    // but FsNode is small enough to clone if needed.
    // Here we decompose.
    place_node(fs_node, Complex::new(0.0, 0.0), 0.0, 2.0 * PI, 0)
}

fn place_node(
    fs: FsNode,
    pos: Point,
    angle_start: f64,
    angle_end: f64,
    depth: usize,
) -> LayoutNode {
    let mut children_layout = Vec::new();
    let count = fs.children.len();

    // We need to keep the fs node info.
    // But we need to iterate children.
    // FsNode owns its children.
    // So we reconstruct the LayoutNode.

    let FsNode {
        path,
        is_dir,
        children,
    } = fs;
    let fs_info = FsNode {
        path,
        is_dir,
        children: vec![],
    }; // Stripped version for storage

    if count > 0 {
        let total_angle = angle_end - angle_start;
        let angle_per_child = total_angle / count as f64;

        // Exponential radius spacing: r = 1 - 0.6^(depth)
        // Tune this base to control crowding.
        // depth 0: 0.0
        // depth 1: 1 - 0.6 = 0.4
        // depth 2: 1 - 0.36 = 0.64
        // depth 3: 1 - 0.216 = 0.784
        let base = 0.6f64;
        let next_depth = depth + 1;
        let radius = 1.0 - base.powi(next_depth as i32);

        for (i, child) in children.into_iter().enumerate() {
            let child_angle_start = angle_start + i as f64 * angle_per_child;
            let child_angle_end = child_angle_start + angle_per_child;
            let mid_angle = (child_angle_start + child_angle_end) / 2.0;

            let child_pos = Complex::from_polar(radius, mid_angle);

            children_layout.push(place_node(
                child,
                child_pos,
                child_angle_start,
                child_angle_end,
                next_depth,
            ));
        }
    }

    LayoutNode {
        fs: fs_info,
        global_pos: pos,
        children: children_layout,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn mock_fs(depth: usize) -> FsNode {
        if depth == 0 {
            return FsNode {
                path: PathBuf::from("leaf"),
                is_dir: false,
                children: vec![],
            };
        }
        FsNode {
            path: PathBuf::from("node"),
            is_dir: true,
            children: vec![mock_fs(depth - 1), mock_fs(depth - 1)],
        }
    }

    #[test]
    fn test_layout_bounds() {
        let root = mock_fs(3);
        let layout = build_layout(root);

        assert_eq!(layout.global_pos.norm(), 0.0);

        // Check children
        for child in &layout.children {
            assert!(child.global_pos.norm() < 1.0);
            assert!(child.global_pos.norm() > 0.0);

            for grand in &child.children {
                assert!(grand.global_pos.norm() < 1.0);
                assert!(grand.global_pos.norm() > child.global_pos.norm());
            }
        }
    }
}
