use std::path::{Path, PathBuf};
use anyhow::Result;
use std::f32::consts::PI;

#[derive(Clone, Debug)]
pub struct FsNode {
    pub path: PathBuf,
    pub is_dir: bool,
    pub name: String,
    // Parametric coordinates
    pub u: f32,
    pub v: f32,
}

pub fn scan_directory(path: &Path) -> Result<Vec<FsNode>> {
    let mut nodes = Vec::new();

    // We only want immediate children for a "browser" feel,
    // but the prompt says "File system browser" and also "Moonshots".
    // If I just show current dir, it's easier to navigate.
    // Let's do immediate children (depth 1).

    // Actually, std::fs::read_dir is better for immediate children.
    // But WalkDir is robust.

    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
        let is_dir = path.is_dir();

        nodes.push(FsNode {
            path,
            is_dir,
            name,
            u: 0.0, // Placeholder
            v: 0.0, // Placeholder
        });
    }

    // Sort by name (dirs first)
    nodes.sort_by(|a, b| {
        b.is_dir.cmp(&a.is_dir).then(a.name.cmp(&b.name))
    });

    // Layout
    layout_nodes(&mut nodes);

    Ok(nodes)
}

fn layout_nodes(nodes: &mut [FsNode]) {
    let count = nodes.len();
    if count == 0 {
        return;
    }

    // Distribute along the tube (u)
    // We want to fill the bottle.
    // u goes from 0 to 2PI.
    // v goes from 0 to 2PI.

    // Simple spiral layout:
    // u increases linearly from 0 to 2PI * (cycles).
    // v spirals.

    // Actually, let's keep u in [0, 2PI) to represent "one loop".
    // If we have many files, we might want multiple loops?
    // But the Klein bottle is a closed surface.
    // So 0 and 2PI are the same u.

    for (i, node) in nodes.iter_mut().enumerate() {
        let t = i as f32 / count as f32;

        // u goes from 0 to 2PI
        node.u = t * 2.0 * PI;

        // v spirals 3 times around the cross section
        node.v = t * 2.0 * PI * 3.0;
    }
}
