use crate::world::{Portal, Room};
use macroquad::prelude::*;
use std::fs;
use std::path::Path;

pub fn scan_dir(path: &Path) -> Room {
    let mut portals = Vec::new();

    // Count files and subdirs to determine size
    let mut file_count = 0;
    let mut dir_count = 0;

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    dir_count += 1;
                    // Add a portal for this subdir
                    // Position portals on walls
                    // Simple logic: Place on walls in a circle or grid
                    // For now: Just random or linear placement
                } else {
                    file_count += 1;
                }
            }
        }
    }

    // Adjust room size based on content
    // Logarithmic scaling
    let content_factor = (file_count + dir_count) as f32;
    let base_size = 10.0 + content_factor.sqrt() * 2.0;
    let room_size = vec3(base_size, 8.0, base_size); // Fixed height for now

    // Re-scan to place portals with known room size
    let mut portal_idx = 0;
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    // Place portal
                    let wall_idx = portal_idx % 4; // 0=Back, 1=Right, 2=Front, 3=Left
                    let wall_offset = (portal_idx / 4) as f32 * 4.0 - (room_size.x / 2.0) + 4.0;
                    // This is very rough placement

                    let mut pos = Vec3::ZERO;
                    let size = vec2(2.5, 4.0); // Door size

                    match wall_idx {
                        0 => {
                            // Back Wall (Negative Z)
                            pos = vec3(wall_offset, -2.0, -room_size.z / 2.0);
                        }
                        1 => {
                            // Right Wall (Positive X)
                            pos = vec3(room_size.x / 2.0, -2.0, wall_offset);
                        }
                        2 => {
                            // Front Wall (Positive Z)
                            pos = vec3(-wall_offset, -2.0, room_size.z / 2.0);
                        }
                        3 => {
                            // Left Wall (Negative X)
                            pos = vec3(-room_size.x / 2.0, -2.0, -wall_offset);
                        }
                        _ => {}
                    }

                    // Clamp to room bounds to avoid floating doors
                    // This logic is flawed but okay for moonshot prototype

                    portals.push(Portal {
                        pos,
                        size,
                        target_path: entry.path(),
                        loaded_room: None,
                    });

                    portal_idx += 1;
                }
            }
        }
    }

    Room {
        path: path.to_path_buf(),
        size: room_size,
        color: generate_color_from_path(path),
        portals,
    }
}

fn generate_color_from_path(path: &Path) -> Color {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    path.hash(&mut hasher);
    let hash = hasher.finish();

    let r = ((hash >> 16) & 0xFF) as f32 / 255.0;
    let g = ((hash >> 8) & 0xFF) as f32 / 255.0;
    let b = (hash & 0xFF) as f32 / 255.0;

    Color::new(r, g, b, 1.0)
}
