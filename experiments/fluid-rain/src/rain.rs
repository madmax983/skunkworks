use rand::Rng;
use std::fs::File;
use std::io::{BufRead, BufReader};
use walkdir::WalkDir;

pub struct RainDrop {
    pub x: f64,
    pub y: f64,
    pub speed: f64,
    pub stream: Vec<char>,
}

impl RainDrop {
    pub fn new(x: f64, _width: f64, lines: &[String]) -> Self {
        let mut rng = rand::thread_rng();
        let content = if !lines.is_empty() {
            let line = &lines[rng.gen_range(0..lines.len())];
            line.chars().collect::<Vec<char>>()
        } else {
            (0..20)
                .map(|_| rng.gen_range(33..126) as u8 as char)
                .collect()
        };

        let stream = if content.is_empty() {
            (0..10)
                .map(|_| rng.gen_range(33..126) as u8 as char)
                .collect()
        } else {
            content
        };

        Self {
            x,
            y: rng.gen_range(-100.0..0.0),
            speed: rng.gen_range(0.5..2.0),
            stream,
        }
    }
}

pub struct RainManager {
    pub drops: Vec<RainDrop>,
    pub source_lines: Vec<String>,
}

impl RainManager {
    pub fn new() -> Self {
        let source_lines = scan_files(".");
        Self {
            drops: Vec::new(),
            source_lines,
        }
    }

    pub fn update(&mut self, width: f64, height: f64) -> Vec<(f64, f64)> {
        let mut rng = rand::thread_rng();
        let mut splash_points = Vec::new();

        // Spawn
        let target_drops = (width / 2.0) as usize;
        if self.drops.len() < target_drops {
            if rng.gen_bool(0.1) {
                let x = rng.gen_range(0.0..width);
                self.drops.push(RainDrop::new(x, width, &self.source_lines));
            }
        }

        // Update
        for drop in &mut self.drops {
            drop.y += drop.speed;
        }

        // Check for splash (hitting bottom)
        let mut retained = Vec::new();
        for drop in self.drops.drain(..) {
            // The drop "head" is at drop.y.
            // If drop.y > height, the head hit the bottom.
            if drop.y >= height {
                // Splash!
                // Add particles at the impact point.
                // Depending on drop size, add more?
                // Let's add 3 particles slightly dispersed.
                for _ in 0..3 {
                    let offset = rng.gen_range(-1.0..1.0);
                    splash_points.push((drop.x + offset, height - rng.gen_range(0.0..2.0)));
                }
            } else {
                retained.push(drop);
            }
        }
        self.drops = retained;

        splash_points
    }
}

fn scan_files(root: &str) -> Vec<String> {
    let mut lines = Vec::new();
    for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            if let Some(ext) = entry.path().extension() {
                if ext == "rs" {
                    if let Ok(file) = File::open(entry.path()) {
                        let reader = BufReader::new(file);
                        for line in reader.lines().filter_map(|l| l.ok()) {
                            let trimmed = line.trim();
                            if !trimmed.is_empty() {
                                lines.push(trimmed.to_string());
                            }
                        }
                    }
                }
            }
        }
    }
    if lines.is_empty() {
        lines.push("let x = 42;".to_string());
        lines.push("fn main() {}".to_string());
        lines.push("println!(\"Hello World\");".to_string());
    }
    lines
}
