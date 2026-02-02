use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn process_line(line: &str, width: usize) -> (usize, u8) {
    if width == 0 {
        return (0, 0);
    }

    // 1. Determine Bucket (X-axis)
    // We want similar log lines to hit the same bucket.
    // Let's hash the first 20 chars or the whole line?
    // If we hash the timestamp, it will spread randomly.
    // We usually want to cluster by "Message Type".
    // Simple heuristic: Hash the line, but if it looks like a CLF log, skip the timestamp?
    // For now, simple hash of the whole line.
    let mut hasher = DefaultHasher::new();
    line.hash(&mut hasher);
    let hash = hasher.finish();
    let bucket = (hash as usize) % width;

    // 2. Determine Height (Intensity)
    let mut height = 1;

    // Length contribution (capped)
    height += (line.len() / 20).min(5) as u8;

    // Keyword contribution
    let upper = line.to_uppercase();
    if upper.contains("ERROR") || upper.contains("FAIL") || upper.contains("CRITICAL") {
        height += 8;
    } else if upper.contains("WARN") {
        height += 4;
    } else if upper.contains("INFO") {
        height += 1;
    }

    // Cap total height
    (bucket, height.min(20))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_line_deterministic() {
        let line = "INFO: User logged in";
        let width = 50;
        let (b1, h1) = process_line(line, width);
        let (b2, h2) = process_line(line, width);
        assert_eq!(b1, b2);
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_process_line_bounds() {
        let width = 10;
        for i in 0..100 {
            let line = format!("Log line {}", i);
            let (b, _) = process_line(&line, width);
            assert!(b < width);
        }
    }

    #[test]
    fn test_process_line_keywords() {
        let width = 100;
        let (_, h_info) = process_line("INFO: something", width);
        let (_, h_error) = process_line("ERROR: something", width);
        assert!(h_error > h_info);
    }
}
