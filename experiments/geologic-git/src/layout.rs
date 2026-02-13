use macroquad::math::Rect;
use std::path::{Path, PathBuf};
use std::fs;

#[derive(Debug, Clone)]
pub struct Node {
    pub rect: Rect,
    pub path: PathBuf,
    pub size: u64,
    pub children: Vec<Node>,
}

pub fn scan_repo(path: &Path) -> Node {
    let metadata = fs::metadata(path).ok();
    let size = metadata.as_ref().map(|m| m.len()).unwrap_or(0);
    let mut children = Vec::new();

    if let Some(meta) = metadata {
        if meta.is_dir() {
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    // Skip hidden files/dirs (like .git)
                    if path.file_name().and_then(|n| n.to_str()).map(|s| s.starts_with('.')).unwrap_or(false) {
                        continue;
                    }
                    children.push(scan_repo(&path));
                }
            }
        }
    }

    // If directory, size is sum of children (plus some base size?)
    // Let's say directory size is sum of children size.
    // If file, size is file size.
    let total_size = if !children.is_empty() {
        children.iter().map(|c| c.size).sum()
    } else {
        // Ensure non-zero size for visibility
        size.max(100)
    };

    Node {
        rect: Rect::new(0.0, 0.0, 0.0, 0.0),
        path: path.to_path_buf(),
        size: total_size,
        children,
    }
}

pub fn layout_treemap(node: &mut Node, area: Rect) {
    node.rect = area;

    if node.children.is_empty() {
        return;
    }

    // Sort children by size descending for better packing?
    // Or keep alphabetical? Slice-and-dice works with any order.
    // Let's keep original order to preserve directory structure "feeling".

    // We alternate split direction based on depth?
    // Or just based on aspect ratio of the area?
    // "Slice and Dice" usually alternates.
    // Let's decide based on whether width > height.
    let horizontal_split = area.w > area.h;

    let total_size: u64 = node.children.iter().map(|c| c.size).sum();

    // Avoid division by zero
    let total_size_f = total_size as f32;
    if total_size_f <= 0.0 {
        return;
    }

    let mut current_pos = if horizontal_split { area.x } else { area.y };

    for child in &mut node.children {
        let fraction = child.size as f32 / total_size_f;

        let child_area = if horizontal_split {
            let width = area.w * fraction;
            let r = Rect::new(current_pos, area.y, width, area.h);
            current_pos += width;
            r
        } else {
            let height = area.h * fraction;
            let r = Rect::new(area.x, current_pos, area.w, height);
            current_pos += height;
            r
        };

        layout_treemap(child, child_area);
    }
}

pub fn collect_leaves(node: &Node) -> Vec<Node> {
    let mut leaves = Vec::new();
    if node.children.is_empty() {
        leaves.push(node.clone());
    } else {
        for child in &node.children {
            leaves.extend(collect_leaves(child));
        }
    }
    leaves
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout() {
        let mut root = Node {
            rect: Rect::default(),
            path: PathBuf::from("root"),
            size: 100,
            children: vec![
                Node { rect: Rect::default(), path: PathBuf::from("a"), size: 40, children: vec![] },
                Node { rect: Rect::default(), path: PathBuf::from("b"), size: 60, children: vec![] },
            ],
        };

        let area = Rect::new(0.0, 0.0, 100.0, 100.0);
        layout_treemap(&mut root, area);

        // root should cover area
        assert_eq!(root.rect, area);

        // Children should sum to area
        // Split should be horizontal (100 > 100 is false? No, 100 > 100 is false. Vertical split?)
        // Wait, width > height. 100 > 100 is false.
        // So Vertical split (slices along Y axis).
        // Rect(x, current_pos, w, height)
        // Child A: 40% height. Rect(0, 0, 100, 40).
        // Child B: 60% height. Rect(0, 40, 100, 60).

        assert_approx_eq(root.children[0].rect, Rect::new(0.0, 0.0, 100.0, 40.0));
        assert_approx_eq(root.children[1].rect, Rect::new(0.0, 40.0, 100.0, 60.0));
    }

    fn assert_approx_eq(r1: Rect, r2: Rect) {
        let epsilon = 0.001;
        assert!((r1.x - r2.x).abs() < epsilon, "x mismatch: {} vs {}", r1.x, r2.x);
        assert!((r1.y - r2.y).abs() < epsilon, "y mismatch: {} vs {}", r1.y, r2.y);
        assert!((r1.w - r2.w).abs() < epsilon, "w mismatch: {} vs {}", r1.w, r2.w);
        assert!((r1.h - r2.h).abs() < epsilon, "h mismatch: {} vs {}", r1.h, r2.h);
    }
}
