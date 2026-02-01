use crate::scanner::Node;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Block {
    pub x: f64,
    pub z: f64,
    pub width: f64,
    pub depth: f64,
    pub height: f64,
    pub is_dir: bool,
    pub path: PathBuf,
}

pub fn generate_layout(node: &Node, x: f64, z: f64, w: f64, d: f64) -> Vec<Block> {
    let mut blocks = Vec::new();

    // Gap between nested blocks
    let padding = 2.0;

    match node {
        Node::File { path, size } => {
            // Height logic: Logarithmic scale works best for file sizes
            // +1.0 to ensure even 0-byte files have some presence
            let h = (*size as f64 + 1.0).log2().max(1.0);

            blocks.push(Block {
                x,
                z,
                width: w,
                depth: d,
                height: h,
                is_dir: false,
                path: path.clone(),
            });
        }
        Node::Dir { path, children } => {
            // District floor
            blocks.push(Block {
                x,
                z,
                width: w,
                depth: d,
                height: 0.1, // Flat
                is_dir: true,
                path: path.clone(),
            });

            let total_size: u64 = children.iter().map(|c| c.size()).sum();

            // If empty or size 0, we are done
            if total_size == 0 {
                return blocks;
            }

            // Calculate inner area
            let inner_x = x + padding;
            let inner_z = z + padding;
            let inner_w = w - 2.0 * padding;
            let inner_d = d - 2.0 * padding;

            // If too small to recurse, stop
            if inner_w <= 1.0 || inner_d <= 1.0 {
                return blocks;
            }

            let split_horizontal = inner_w > inner_d;
            let mut cursor = if split_horizontal { inner_x } else { inner_z };

            // Sort children by size for better packing visual (optional but nice)
            // Actually, stable sort might be better to keep file order?
            // Let's just use them as is for now.

            for child in children {
                let share = child.size() as f64 / total_size as f64;

                if split_horizontal {
                    let child_w = inner_w * share;
                    // Only recurse if the child has enough space
                    if child_w > 0.1 {
                        blocks.extend(generate_layout(child, cursor, inner_z, child_w, inner_d));
                    }
                    cursor += child_w;
                } else {
                    let child_d = inner_d * share;
                    if child_d > 0.1 {
                        blocks.extend(generate_layout(child, inner_x, cursor, inner_w, child_d));
                    }
                    cursor += child_d;
                }
            }
        }
    }
    blocks
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::Node;
    use std::path::PathBuf;

    #[test]
    fn test_layout_generation() {
        let node = Node::Dir {
            path: PathBuf::from("root"),
            children: vec![
                Node::File {
                    path: PathBuf::from("a"),
                    size: 100,
                },
                Node::File {
                    path: PathBuf::from("b"),
                    size: 300,
                },
            ],
        };

        let blocks = generate_layout(&node, 0.0, 0.0, 100.0, 100.0);

        // Should have:
        // 1 root dir floor
        // 2 files
        assert_eq!(blocks.len(), 3);

        // Verify total area roughly preserved (minus padding)
        // Root floor
        assert_eq!(blocks[0].width, 100.0);
    }
}
