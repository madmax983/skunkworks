use crate::terrain::{FileNode, FileType};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct GaitParameters {
    pub step_height: f32,
    pub speed: f32,
    pub bounce: f32,
}

pub fn calculate_gait(node: &FileNode) -> GaitParameters {
    match node.file_type {
        FileType::Directory => GaitParameters {
            step_height: 40.0,
            speed: 150.0,
            bounce: 20.0, // High bounce for directories (hopping)
        },
        FileType::File => {
            // Heavier files = Slower speed, lower bounce
            // Log scale for size
            let size_factor = (node.size as f32 + 1.0).ln();
            // Max size ~ 100MB -> ln(100M) ~ 18.4

            let speed = (200.0 - size_factor * 8.0).max(20.0);
            let bounce = (10.0 - size_factor * 0.5).max(0.0);
            let step_height = (30.0 - size_factor * 1.0).max(5.0);

            GaitParameters {
                step_height,
                speed,
                bounce,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_gait_variation() {
        let large_file = FileNode {
            path: PathBuf::from("large.bin"),
            file_type: FileType::File,
            size: 10_000_000,
        };

        let small_file = FileNode {
            path: PathBuf::from("small.txt"),
            file_type: FileType::File,
            size: 100,
        };

        let dir = FileNode {
            path: PathBuf::from("src"),
            file_type: FileType::Directory,
            size: 4096,
        };

        let gait_large = calculate_gait(&large_file);
        let gait_small = calculate_gait(&small_file);
        let gait_dir = calculate_gait(&dir);

        // Large files should be heavier (slower)
        assert!(gait_large.speed < gait_small.speed, "Large files should be slower");

        // Directories should be bouncy
        assert!(gait_dir.bounce > gait_small.bounce, "Directories should be bouncy");
    }
}
