use anyhow::Result;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct RoomId {
    pub path: PathBuf,
    pub page: usize,
}

#[derive(Clone, Debug)]
pub struct Room {
    #[allow(dead_code)]
    pub id: RoomId,
    /// Neighbors directions: 0=Right, 1=Up, 2=Left, 3=Down
    pub neighbors: [Option<RoomId>; 4],
    pub is_dir: bool,
    pub name: String,
}

pub struct FsCache {
    rooms: HashMap<RoomId, Room>,
}

impl FsCache {
    pub fn new() -> Self {
        Self {
            rooms: HashMap::new(),
        }
    }

    pub fn get_room(&mut self, id: &RoomId) -> Result<&Room> {
        if !self.rooms.contains_key(id) {
            let room = self.scan_room(id)?;
            self.rooms.insert(id.clone(), room);
        }
        Ok(self.rooms.get(id).unwrap())
    }

    fn scan_room(&self, id: &RoomId) -> Result<Room> {
        let path = &id.path;
        let page = id.page;
        let is_dir = path.is_dir();
        let mut neighbors = [None, None, None, None];

        // Neighbor 2 (Left) is Back:
        // If page > 0: Previous Page
        // If page == 0: Parent
        if page > 0 {
            neighbors[2] = Some(RoomId {
                path: path.clone(),
                page: page - 1,
            });
        } else {
            if let Some(parent) = path.parent() {
                neighbors[2] = Some(RoomId {
                    path: parent.to_path_buf(),
                    page: 0,
                });
            }
        }

        let name = if page == 0 {
            path.file_name()
                .unwrap_or(std::ffi::OsStr::new("Root"))
                .to_string_lossy()
                .to_string()
        } else {
            format!(
                "{} (p{})",
                path.file_name()
                    .unwrap_or(std::ffi::OsStr::new("Root"))
                    .to_string_lossy(),
                page
            )
        };

        if is_dir {
            let mut entries = Vec::new();
            if let Ok(read_dir) = std::fs::read_dir(path) {
                for entry in read_dir.flatten() {
                    entries.push(entry.path());
                }
            }
            // Sort to ensure stability
            entries.sort();

            // We have 3 slots available: 0 (Right), 1 (Up), 3 (Down).
            // We use Slot 3 for "Next Page" if needed.
            // So we display 2 items per page.

            let items_per_page = 2;
            let start_idx = page * items_per_page;

            // Slot 0: Item 1
            if start_idx < entries.len() {
                neighbors[0] = Some(RoomId {
                    path: entries[start_idx].clone(),
                    page: 0,
                });
            }

            // Slot 1: Item 2
            if start_idx + 1 < entries.len() {
                neighbors[1] = Some(RoomId {
                    path: entries[start_idx + 1].clone(),
                    page: 0,
                });
            }

            // Slot 3: Next Page
            if start_idx + 2 < entries.len() {
                neighbors[3] = Some(RoomId {
                    path: path.clone(),
                    page: page + 1,
                });
            }
        }

        Ok(Room {
            id: id.clone(),
            neighbors,
            is_dir,
            name,
        })
    }
}
