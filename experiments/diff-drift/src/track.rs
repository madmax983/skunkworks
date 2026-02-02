use anyhow::{Context, Result};
use std::process::Command;

#[derive(Debug, Clone, PartialEq)]
pub struct Segment {
    pub curvature: f64,
    pub length: f64,
    pub description: String,
}

#[derive(Debug, Clone, Default)]
pub struct Track {
    pub segments: Vec<Segment>,
    // Cached geometry: (y, x, angle)
    pub points: Vec<(f64, f64, f64)>,
}

impl Track {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_git(path: &str, limit: usize) -> Result<Self> {
        let output = Command::new("git")
            .arg("log")
            .arg("--pretty=format:%h %s")
            .arg("-n")
            .arg(limit.to_string())
            .current_dir(path)
            .output()
            .context("Failed to run git log")?;

        let stdout = String::from_utf8(output.stdout).context("Git output was not valid UTF-8")?;

        Ok(Self::parse(&stdout))
    }

    pub fn parse(output: &str) -> Self {
        let mut segments = Vec::new();

        for line in output.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.splitn(2, ' ').collect();
            if parts.len() < 2 {
                continue;
            }

            let hash = parts[0];
            let message = parts[1];

            // Curvature from first char of hash
            let curvature = if let Some(c) = hash.chars().next() {
                match c.to_ascii_lowercase() {
                    '0'..='3' => -0.8, // Sharp Left
                    '4'..='7' => -0.2, // Slight Left
                    '8'..='b' => 0.2,  // Slight Right
                    'c'..='f' => 0.8,  // Sharp Right
                    _ => 0.0,
                }
            } else {
                0.0
            };

            // Length from message length
            // Normalize: 1 char = 1 unit? Let's say 1 char = 2.0 units.
            let length = (message.len() as f64 * 2.0).max(10.0);

            segments.push(Segment {
                curvature,
                length,
                description: message.to_string(),
            });
        }

        // Ensure there is at least one segment
        if segments.is_empty() {
            segments.push(Segment {
                curvature: 0.0,
                length: 1000.0,
                description: "Empty Repo - Enjoy the silence".to_string(),
            });
        }

        let mut track = Self {
            segments,
            points: Vec::new(),
        };
        track.compute_geometry();
        track
    }

    pub fn compute_geometry(&mut self) {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut angle = 0.0;
        let step = 2.0; // Resolution

        self.points.clear();
        self.points.push((y, x, angle));

        for segment in &self.segments {
            let mut covered = 0.0;
            while covered < segment.length {
                y += step;
                angle += segment.curvature * 0.05; // Scaling factor for curvature
                x += angle.sin() * step;

                self.points.push((y, x, angle));
                covered += step;
            }
        }
    }

    pub fn get_segment_at(&self, distance: f64) -> Option<&Segment> {
        let mut current_dist = 0.0;
        for segment in &self.segments {
            if distance >= current_dist && distance < current_dist + segment.length {
                return Some(segment);
            }
            current_dist += segment.length;
        }
        None
    }

    // Get track center X at specific Y distance
    pub fn get_x_at(&self, distance: f64) -> f64 {
        // Find closest point
        // Points are sorted by Y. Binary search?
        // Or just linear if we assume resolution.

        // Binary search
        let idx = self.points.partition_point(|&(py, _, _)| py < distance);
        if idx == 0 {
            return self.points.first().map(|p| p.1).unwrap_or(0.0);
        }
        if idx >= self.points.len() {
            return self.points.last().map(|p| p.1).unwrap_or(0.0);
        }

        let p0 = self.points[idx - 1];
        let p1 = self.points[idx];

        // Lerp
        let t = (distance - p0.0) / (p1.0 - p0.0);
        p0.1 + (p1.1 - p0.1) * t
    }

    pub fn total_length(&self) -> f64 {
        self.segments.iter().map(|s| s.length).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geometry() {
        let segments = vec![
            Segment {
                curvature: 0.0,
                length: 10.0,
                description: "Straight".into(),
            },
            Segment {
                curvature: 1.0,
                length: 10.0,
                description: "Right".into(),
            },
        ];
        let mut track = Track {
            segments,
            points: Vec::new(),
        };
        track.compute_geometry();

        assert!(!track.points.is_empty());

        // First point should be 0,0,0
        assert_eq!(track.points[0], (0.0, 0.0, 0.0));

        // Check interpolation
        let x = track.get_x_at(5.0);
        assert!((x - 0.0).abs() < 0.1); // Straight
    }
}
