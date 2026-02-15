use crate::scanner::FileNode;
use macroquad::prelude::Rect;

#[derive(Debug, Clone)]
pub struct LayoutNode {
    pub node: FileNode,
    pub rect: Rect,
}

pub fn generate_layout(root: &FileNode, area: Rect) -> Vec<LayoutNode> {
    let mut layout = Vec::new();

    // Add the current node (container)
    layout.push(LayoutNode {
        node: root.clone(),
        rect: area,
    });

    if root.is_dir && !root.children.is_empty() {
        let total_size: u64 = root.children.iter().map(|c| c.size).sum();

        // If directory has 0 size (empty files), treat as equal distribution
        let use_count = total_size == 0;
        let effective_total = if use_count { root.children.len() as f32 } else { total_size as f32 };

        let horizontal_split = area.w > area.h;

        let mut current_offset = if horizontal_split { area.x } else { area.y };

        for child in &root.children {
            let share = if use_count {
                1.0
            } else {
                child.size as f32
            } / effective_total;

            // Avoid 0-size rects if possible, or just let them be 0
            if share <= 0.0 && !use_count {
                continue;
            }

            let child_rect = if horizontal_split {
                let w = area.w * share;
                let r = Rect::new(current_offset, area.y, w, area.h);
                current_offset += w;
                r
            } else {
                let h = area.h * share;
                let r = Rect::new(area.x, current_offset, area.w, h);
                current_offset += h;
                r
            };

            // Recurse
            let child_layout = generate_layout(child, child_rect);
            layout.extend(child_layout);
        }
    }

    layout
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_layout_generation() {
        let root = FileNode {
            path: PathBuf::from("root"),
            is_dir: true,
            size: 100,
            children: vec![
                FileNode {
                    path: PathBuf::from("child1"),
                    is_dir: false,
                    size: 60,
                    children: vec![],
                },
                FileNode {
                    path: PathBuf::from("child2"),
                    is_dir: false,
                    size: 40,
                    children: vec![],
                },
            ],
        };

        let area = Rect::new(0.0, 0.0, 100.0, 100.0);
        let layout = generate_layout(&root, area);

        // Root + 2 children = 3 nodes
        assert_eq!(layout.len(), 3);

        // Root should cover full area
        assert_eq!(layout[0].rect.w, 100.0);
        assert_eq!(layout[0].rect.h, 100.0);

        // Children should sum up to area (roughly)
        // Horizontal split because 100 > 100 is false? Wait, 100 > 100 is false. Vertical split.
        // Wait, w > h. 100 > 100 is false. So vertical split (stacking vertically).
        // Child 1 (60%) -> Height 60.
        // Child 2 (40%) -> Height 40.

        let child1 = &layout[1];
        let _child2 = &layout[2];

        assert!((child1.rect.h - 60.0).abs() < 0.1 || (child1.rect.w - 60.0).abs() < 0.1);
    }
}
