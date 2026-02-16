
#[derive(Clone, Copy, Debug)]
struct Vector {
    x: i32,
    y: i32,
}

impl Vector {
    fn length_sq(&self) -> i32 {
        self.x * self.x + self.y * self.y
    }
}

pub fn generate_sdf(grid: &[bool], width: usize, height: usize) -> Vec<f32> {
    let mut vectors = vec![Vector { x: 9999, y: 9999 }; width * height];

    // Initialize
    // We want distance from "inside" (true) to "outside" (false).
    // So "outside" pixels have distance 0 (vector 0,0).
    // "Inside" pixels start with infinity.
    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            if !grid[idx] {
                vectors[idx] = Vector { x: 0, y: 0 };
            }
        }
    }

    // Pass 1: Top-Left to Bottom-Right
    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            let current = vectors[idx];

            // Check Left
            if x > 0 {
                let neighbor = vectors[idx - 1];
                let candidate = Vector { x: neighbor.x + 1, y: neighbor.y };
                if candidate.length_sq() < vectors[idx].length_sq() {
                    vectors[idx] = candidate;
                }
            }
            // Check Up
            if y > 0 {
                let neighbor = vectors[idx - width];
                let candidate = Vector { x: neighbor.x, y: neighbor.y + 1 };
                if candidate.length_sq() < vectors[idx].length_sq() {
                    vectors[idx] = candidate;
                }
            }
            // Check Up-Left
            if x > 0 && y > 0 {
                 let neighbor = vectors[idx - width - 1];
                 let candidate = Vector { x: neighbor.x + 1, y: neighbor.y + 1 };
                 if candidate.length_sq() < vectors[idx].length_sq() {
                     vectors[idx] = candidate;
                 }
            }
            // Check Up-Right
             if x + 1 < width && y > 0 {
                 let neighbor = vectors[idx - width + 1];
                 let candidate = Vector { x: neighbor.x - 1, y: neighbor.y + 1 };
                 if candidate.length_sq() < vectors[idx].length_sq() {
                     vectors[idx] = candidate;
                 }
            }
        }
    }

    // Pass 2: Bottom-Right to Top-Left
    for y in (0..height).rev() {
        for x in (0..width).rev() {
            let idx = y * width + x;

            // Check Right
            if x + 1 < width {
                let neighbor = vectors[idx + 1];
                let candidate = Vector { x: neighbor.x - 1, y: neighbor.y };
                if candidate.length_sq() < vectors[idx].length_sq() {
                    vectors[idx] = candidate;
                }
            }
            // Check Down
            if y + 1 < height {
                let neighbor = vectors[idx + width];
                let candidate = Vector { x: neighbor.x, y: neighbor.y - 1 };
                if candidate.length_sq() < vectors[idx].length_sq() {
                    vectors[idx] = candidate;
                }
            }
             // Check Down-Right
             if x + 1 < width && y + 1 < height {
                 let neighbor = vectors[idx + width + 1];
                 let candidate = Vector { x: neighbor.x - 1, y: neighbor.y - 1 };
                 if candidate.length_sq() < vectors[idx].length_sq() {
                     vectors[idx] = candidate;
                 }
            }
             // Check Down-Left
             if x > 0 && y + 1 < height {
                 let neighbor = vectors[idx + width - 1];
                 let candidate = Vector { x: neighbor.x + 1, y: neighbor.y - 1 };
                 if candidate.length_sq() < vectors[idx].length_sq() {
                     vectors[idx] = candidate;
                 }
            }
        }
    }

    // Convert to float distances
    vectors.iter().map(|v| (v.length_sq() as f32).sqrt()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sdf_basic() {
        // 3x3 grid
        // O I O
        // I I I
        // O I O
        let width = 3;
        let height = 3;
        let grid = vec![
            false, true, false,
            true, true, true,
            false, true, false,
        ];

        let sdf = generate_sdf(&grid, width, height);

        // Center (1,1) should be furthest from edge.
        // Neighbors (0,1), (1,0), (2,1), (1,2) are distance 0 from "outside"??
        // Wait, my logic: "Outside" pixels have dist 0. "Inside" pixels have dist to nearest outside.
        // In this grid, (0,0) is outside.
        // (0,1) is inside. Distance to nearest outside (0,0) is 1.
        // (1,1) is inside. Nearest outside is (0,0) (dist sqrt(2)) or (0,2) or (2,0) or (2,2).
        // Actually (1,0) is true (inside).

        // Let's trace (1,1).
        // (0,0) is false -> dist 0.
        // (1,1) is true.
        // Nearest false is at (0,0), (2,0), (0,2), (2,2).
        // Distance is sqrt(1^2 + 1^2) = 1.414.

        let center_idx = 1 * width + 1;
        assert!(sdf[center_idx] > 1.0);

        // (0,1) is true. Nearest false is (0,0) or (0,2). Distance 1.
        let left_idx = 0 * width + 1; // x=1, y=0 -> wait.
        // x=0, y=1 is index 3.
        assert_eq!(sdf[3], 1.0);
    }

    #[test]
    fn test_sdf_empty() {
        let width = 5;
        let height = 5;
        let grid = vec![false; width * height];
        let sdf = generate_sdf(&grid, width, height);
        for d in sdf {
            assert_eq!(d, 0.0);
        }
    }
}
