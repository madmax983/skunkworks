use glam::Vec2;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct Node {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
    pub position: Vec2,
    pub size: f32,
}

pub struct World {
    pub nodes: Vec<Node>,
    pub current_path: PathBuf,
    pub selected_index: usize,
}

impl World {
    pub fn new() -> Self {
        let mut world = Self {
            nodes: Vec::new(),
            current_path: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            selected_index: 0,
        };
        world.scan(&world.current_path.clone()); // Initialize
        world
    }

    pub fn scan(&mut self, path: &Path) {
        self.nodes.clear();
        self.current_path = path.to_path_buf();

        // Add ".." node if not root
        if let Some(parent) = path.parent() {
            // Check if we can actually go up
            if parent.exists() {
                self.nodes.push(Node {
                    path: parent.to_path_buf(),
                    name: "..".to_string(),
                    is_dir: true,
                    position: Vec2::new(0.0, -30.0), // Top
                    size: 5.0,
                });
            }
        }

        // Read dir
        if let Ok(entries) = std::fs::read_dir(path) {
            let mut entries: Vec<_> = entries.flatten().collect();

            // Sort: directories first, then alphabetical
            entries.sort_by(|a, b| {
                let adir = a.file_type().map(|t| t.is_dir()).unwrap_or(false);
                let bdir = b.file_type().map(|t| t.is_dir()).unwrap_or(false);
                if adir == bdir {
                    a.path().cmp(&b.path())
                } else {
                    bdir.cmp(&adir) // true > false
                }
            });

            let _count = entries.len();
            // Layout: Spiral or Concentric circles to handle many files
            // Simple Spiral
            let spacing = 10.0;
            let mut angle: f32 = 0.0;
            let mut radius = 20.0;

            for entry in entries {
                let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);

                // Calculate position
                let x = angle.cos() * radius;
                let y = angle.sin() * radius;

                self.nodes.push(Node {
                    path: entry.path(),
                    name: entry.file_name().to_string_lossy().to_string(),
                    is_dir,
                    position: Vec2::new(x, y),
                    size: if is_dir { 3.0 } else { 1.5 },
                });

                // Increment spiral
                let circumference = 2.0 * std::f32::consts::PI * radius;
                let step = spacing / circumference * std::f32::consts::TAU;
                angle += step.max(0.1); // Min angle step
                radius += 0.2; // Slowly grow radius
            }
        }

        self.selected_index = 0;
    }
}
