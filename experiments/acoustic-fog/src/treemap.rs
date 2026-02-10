use macroquad::prelude::Rect;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct FileRect {
    pub rect: Rect,
    pub path: PathBuf,
    pub size: u64,
}

pub fn generate_treemap(root: &Path, width: f32, height: f32) -> Vec<FileRect> {
    let mut files: Vec<(PathBuf, u64)> = WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| (e.path().to_path_buf(), e.metadata().map(|m| m.len()).unwrap_or(0)))
        .filter(|(_, size)| *size > 0)
        .collect();

    // Sort by size (not strictly needed for slice-and-dice but good for consistency)
    files.sort_by(|a, b| b.1.cmp(&a.1));

    if files.is_empty() {
        return Vec::new();
    }

    let total_size: u64 = files.iter().map(|(_, s)| *s).sum();
    let initial_rect = Rect::new(0.0, 0.0, width, height);

    let mut results = Vec::with_capacity(files.len());
    slice_layout(&files, initial_rect, total_size, true, &mut results);

    results
}

fn slice_layout(
    items: &[(PathBuf, u64)],
    rect: Rect,
    total_size: u64,
    vertical: bool, // Cut vertically (divide width) or horizontally (divide height)
    results: &mut Vec<FileRect>,
) {
    if items.is_empty() {
        return;
    }

    if items.len() == 1 {
        // Base case
        results.push(FileRect {
            rect,
            path: items[0].0.clone(),
            size: items[0].1,
        });
        return;
    }

    // Find split point
    let target_size = total_size / 2;
    let mut current_size = 0;
    let mut split_idx = 0;

    for (i, (_, size)) in items.iter().enumerate() {
        current_size += size;
        if current_size >= target_size {
            split_idx = i + 1;
            break;
        }
    }

    // Ensure at least one item on each side if possible
    if split_idx == 0 {
        split_idx = 1;
        current_size = items[0].1;
    } else if split_idx == items.len() {
        split_idx = items.len() - 1;
        current_size -= items.last().unwrap().1;
    }

    let left_size = current_size;
    let right_size = total_size - left_size;

    // Avoid division by zero
    if total_size == 0 {
         // Just give up and put everything in one rect? Or split evenly?
         // If total size is 0, we filtered them out.
         return;
    }

    let ratio = left_size as f32 / total_size as f32;

    let (rect1, rect2) = if vertical {
        // Split width
        let w1 = rect.w * ratio;
        let w2 = rect.w - w1;
        (
            Rect::new(rect.x, rect.y, w1, rect.h),
            Rect::new(rect.x + w1, rect.y, w2, rect.h),
        )
    } else {
        // Split height
        let h1 = rect.h * ratio;
        let h2 = rect.h - h1;
        (
            Rect::new(rect.x, rect.y, rect.w, h1),
            Rect::new(rect.x, rect.y + h1, rect.w, h2),
        )
    };

    slice_layout(&items[..split_idx], rect1, left_size, !vertical, results);
    slice_layout(&items[split_idx..], rect2, right_size, !vertical, results);
}

#[cfg(test)]
mod tests {
    use super::*;
    use macroquad::prelude::*;

    #[test]
    fn test_treemap_generation() {
        // Mock files? WalkDir hits disk.
        // We can test slice_layout directly if we expose it or mock WalkDir (hard).
        // Let's just create a dummy directory structure or trust the math logic.
        // Actually, let's create a temp dir.

        let temp_dir = std::env::temp_dir().join("acoustic_fog_test");
        let _ = std::fs::remove_dir_all(&temp_dir);
        std::fs::create_dir_all(&temp_dir).unwrap();

        std::fs::write(temp_dir.join("a.txt"), "12345").unwrap();
        std::fs::write(temp_dir.join("b.txt"), "1234567890").unwrap();

        let rects = generate_treemap(&temp_dir, 100.0, 100.0);

        assert_eq!(rects.len(), 2);

        // Total area should match relative sizes roughly
        // 5 bytes vs 10 bytes -> 1:2 ratio.
        // Total 15.
        // Rect 1 area approx 3333.
        // Rect 2 area approx 6666.

        let area1 = rects[0].rect.w * rects[0].rect.h;
        let area2 = rects[1].rect.w * rects[1].rect.h;

        let total_area = area1 + area2;
        assert!((total_area - 10000.0).abs() < 1.0);

        // One should be roughly double the other
        if rects[0].size == 5 {
             assert!((area2 / area1 - 2.0).abs() < 0.5);
        } else {
             assert!((area1 / area2 - 2.0).abs() < 0.5);
        }

        let _ = std::fs::remove_dir_all(&temp_dir);
    }
}
