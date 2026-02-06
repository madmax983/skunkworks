use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Room {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
    pub path: PathBuf,
    pub is_dir: bool,
}

#[derive(Debug)]
pub struct Node {
    pub path: PathBuf,
    pub size: u64,
    pub children: Vec<Node>,
    pub is_dir: bool,
}

impl Node {
    pub fn new(path: PathBuf, size: u64, is_dir: bool) -> Self {
        Self {
            path,
            size,
            children: Vec::new(),
            is_dir,
        }
    }

    pub fn add_child(&mut self, node: Node) {
        self.size += node.size;
        self.children.push(node);
    }
}

pub fn build_tree(path: &Path) -> Node {
    if path.is_dir() {
        let mut node = Node::new(path.to_path_buf(), 0, true);
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let child = build_tree(&entry.path());
                node.add_child(child);
            }
        }
        if node.size == 0 {
            node.size = 1;
        }
        // Sort children by size descending for better packing
        node.children.sort_by(|a, b| b.size.cmp(&a.size));
        node
    } else {
        let size = fs::metadata(path).map(|m| m.len()).unwrap_or(1);
        Node::new(path.to_path_buf(), size.max(1), false)
    }
}

pub fn generate_layout(root: &Node, width: usize, height: usize) -> (Vec<bool>, Vec<Room>) {
    let mut walls = vec![false; width * height];
    let mut rooms = Vec::new();

    let pad = 1;
    if width <= 2 * pad || height <= 2 * pad {
        return (walls, rooms);
    }

    layout_recursive(
        root,
        pad,
        pad,
        width - 2 * pad,
        height - 2 * pad,
        true, // start vertical split
        width, // stride
        &mut walls,
        &mut rooms,
    );

    (walls, rooms)
}

fn layout_recursive(
    node: &Node,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    split_vertical: bool, // if true, split width. if false, split height.
    stride: usize,
    walls: &mut [bool],
    rooms: &mut Vec<Room>,
) {
    if w < 2 || h < 2 {
        return;
    }

    rooms.push(Room {
        x,
        y,
        w,
        h,
        path: node.path.clone(),
        is_dir: node.is_dir,
    });

    if node.is_dir {
        // Draw walls around this room
        draw_rect(x, y, w, h, walls, stride);

        // Children
        if node.children.is_empty() {
            return;
        }

        let total_size: u64 = node.children.iter().map(|c| c.size).sum();
        let mut current_offset = 0;

        // Inset for children so they are inside the walls
        let inner_x = x + 1;
        let inner_y = y + 1;
        let inner_w = w.saturating_sub(2);
        let inner_h = h.saturating_sub(2);

        if inner_w == 0 || inner_h == 0 {
            return;
        }

        for child in &node.children {
            let fraction = child.size as f64 / total_size as f64;

            if split_vertical {
                 let mut child_w = (inner_w as f64 * fraction).round() as usize;
                 child_w = child_w.max(1);
                 let available = inner_w - current_offset;
                 child_w = child_w.min(available);

                 // If last child, take all remaining space to avoid gaps
                 if std::ptr::eq(child, node.children.last().unwrap()) {
                     child_w = available;
                 }

                 if child_w > 0 {
                    layout_recursive(child, inner_x + current_offset, inner_y, child_w, inner_h, !split_vertical, stride, walls, rooms);
                    current_offset += child_w;
                 }
            } else {
                 let mut child_h = (inner_h as f64 * fraction).round() as usize;
                 child_h = child_h.max(1);
                 let available = inner_h - current_offset;
                 child_h = child_h.min(available);

                 // If last child, take all remaining space
                 if std::ptr::eq(child, node.children.last().unwrap()) {
                     child_h = available;
                 }

                 if child_h > 0 {
                    layout_recursive(child, inner_x, inner_y + current_offset, inner_w, child_h, !split_vertical, stride, walls, rooms);
                    current_offset += child_h;
                 }
            }
        }
    }
}

fn draw_rect(x: usize, y: usize, w: usize, h: usize, walls: &mut [bool], stride: usize) {
    let set_wall = |wx, wy, walls: &mut [bool]| {
        if wy * stride + wx < walls.len() {
            walls[wy * stride + wx] = true;
        }
    };

    // Top & Bottom
    for i in x..x + w {
        set_wall(i, y, walls);
        set_wall(i, y + h - 1, walls);
    }
    // Left & Right
    for j in y..y + h {
        set_wall(x, j, walls);
        set_wall(x + w - 1, j, walls);
    }

    // Door: Gap in the middle of Top (connects to parent)
    let mid = x + w / 2;
    if mid < stride {
        let idx = y * stride + mid;
        if idx < walls.len() { walls[idx] = false; }
        if mid + 1 < stride {
             let idx2 = y * stride + mid + 1;
             if idx2 < walls.len() { walls[idx2] = false; }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_generation() {
        let mut root = Node::new(PathBuf::from("root"), 100, true);
        root.add_child(Node::new(PathBuf::from("child1"), 40, false));
        root.add_child(Node::new(PathBuf::from("child2"), 60, false));

        let width = 20;
        let height = 20;
        let (walls, rooms) = generate_layout(&root, width, height);

        assert_eq!(walls.len(), width * height);
        assert!(!rooms.is_empty());

        // Root room should cover most of the area (inset by 1 pad)
        // Check if there are walls
        assert!(walls.iter().any(|&x| x));

        // Check for door (false in the top wall)
        // Root is at 1,1 size 18x18. Top wall at y=1. Mid x=10.
        // walls[1*20 + 10] should be false.
        let stride = width;
        assert_eq!(walls[1 * stride + 10], false, "Door should exist at top of root");
        assert_eq!(walls[1 * stride + 9], true, "Wall should exist next to door");
    }
}
