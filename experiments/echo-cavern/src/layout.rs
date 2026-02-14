use macroquad::prelude::*;
use resonance_audio::physics::Material;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct LayoutItem {
    pub rect: Rect,
    pub material: Material,
    pub path: PathBuf,
}

struct Node {
    path: PathBuf,
    is_dir: bool,
    size: u64, // Used for area calculation
    children: Vec<Node>,
}

impl Node {
    fn scan(path: &Path, depth: usize) -> Option<Self> {
        // Limit depth to avoid too much nesting
        if depth > 4 {
            return None;
        }

        let metadata = fs::metadata(path).ok()?;
        let is_dir = metadata.is_dir();
        let mut size = metadata.len();
        let mut children = Vec::new();

        if is_dir {
            // Give directory a base size to ensure it has space even if empty
            size = 4096;
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries.flatten() {
                    // Ignore hidden files
                    if entry.file_name().to_string_lossy().starts_with('.') {
                        continue;
                    }

                    if let Some(child) = Node::scan(&entry.path(), depth + 1) {
                        size += child.size;
                        children.push(child);
                    }
                }
            }
        } else {
            // Clamping size for layout stability, so tiny files don't disappear and huge files don't dominate
            size = size.clamp(100, 50000);
        }

        Some(Node {
            path: path.to_path_buf(),
            is_dir,
            size,
            children,
        })
    }
}

pub fn generate_layout(root_path: &Path, bounds: Rect) -> Vec<LayoutItem> {
    let root = match Node::scan(root_path, 0) {
        Some(n) => n,
        None => return vec![],
    };

    let mut items = Vec::new();
    layout_node(&root, bounds, &mut items);
    items
}

fn layout_node(node: &Node, rect: Rect, items: &mut Vec<LayoutItem>) {
    // If rect is too small, stop recursion
    if rect.w < 1.0 || rect.h < 1.0 {
        return;
    }

    // Determine material based on extension
    let material = if node.is_dir {
        Material::Air
    } else {
        match node.path.extension().and_then(|s| s.to_str()) {
            Some("rs") => Material::Wall,
            Some("toml") | Some("lock") => Material::Slow,
            Some("md") | Some("txt") => Material::Fast,
            _ => Material::Void, // Binary / unknown / other
        }
    };

    // If it's a file, we add it as a block.
    if !node.is_dir {
        // Add padding to make it distinct
        let padded = Rect::new(
            rect.x + 0.5,
            rect.y + 0.5,
            (rect.w - 1.0).max(0.5),
            (rect.h - 1.0).max(0.5),
        );
        items.push(LayoutItem {
            rect: padded,
            material,
            path: node.path.clone(),
        });
        return;
    }

    // For directories, we create a recursive layout of children
    // Sort by size desc
    let mut children_refs: Vec<&Node> = node.children.iter().collect();
    children_refs.sort_by_key(|n| std::cmp::Reverse(n.size));

    let mut current_rect = rect;
    // Padding for directory structure (walls of the cavern) - create gaps between folders
    let padding = 1.0;
    current_rect.x += padding;
    current_rect.y += padding;
    current_rect.w -= padding * 2.0;
    current_rect.h -= padding * 2.0;

    if current_rect.w <= 0.0 || current_rect.h <= 0.0 {
        return;
    }

    layout_children(&children_refs, current_rect, items);
}

fn layout_children(nodes: &[&Node], rect: Rect, items: &mut Vec<LayoutItem>) {
    if nodes.is_empty() {
        return;
    }

    if nodes.len() == 1 {
        layout_node(nodes[0], rect, items);
        return;
    }

    // Split based on total size to maintain area proportionality
    let total_size: u64 = nodes.iter().map(|n| n.size).sum();
    // Prevent division by zero
    if total_size == 0 {
        return;
    }

    // Find split point where left size is closest to half
    let mut left_size = 0;
    let mut split_idx = 0;
    for (i, node) in nodes.iter().enumerate() {
        if left_size + node.size > total_size / 2 && i > 0 {
            break;
        }
        left_size += node.size;
        split_idx = i + 1;
    }
    // Ensure at least one item on each side if possible
    if split_idx >= nodes.len() {
        split_idx = nodes.len() - 1;
    }
    if split_idx == 0 {
        split_idx = 1;
    }

    let (left_nodes, right_nodes) = nodes.split_at(split_idx);

    // Recalculate actual sizes for precise ratio
    let left_actual_size: u64 = left_nodes.iter().map(|n| n.size).sum();
    let ratio = left_actual_size as f32 / total_size as f32;

    // Split along longest axis
    let (r1, r2) = if rect.w > rect.h {
        let w1 = rect.w * ratio;
        (
            Rect::new(rect.x, rect.y, w1, rect.h),
            Rect::new(rect.x + w1, rect.y, rect.w - w1, rect.h),
        )
    } else {
        let h1 = rect.h * ratio;
        (
            Rect::new(rect.x, rect.y, rect.w, h1),
            Rect::new(rect.x, rect.y + h1, rect.w, rect.h - h1),
        )
    };

    layout_children(left_nodes, r1, items);
    layout_children(right_nodes, r2, items);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_generation() {
        let rect = Rect::new(0.0, 0.0, 100.0, 100.0);
        // We can't easily mock filesystem, so we test on current dir
        let path = Path::new(".");
        let items = generate_layout(path, rect);
        assert!(!items.is_empty());

        for item in &items {
            assert!(item.rect.x >= 0.0);
            assert!(item.rect.y >= 0.0);
            assert!(item.rect.w > 0.0);
            assert!(item.rect.h > 0.0);
            assert!(item.rect.x + item.rect.w <= 100.0);
            assert!(item.rect.y + item.rect.h <= 100.0);
        }
    }
}
