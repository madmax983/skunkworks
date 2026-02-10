use crate::git::CommitData;
use locus::Vec2;
use rand::Rng;

pub struct Fissure {
    pub points: Vec<Vec2>,
    pub intensity: f64,
    pub active: bool,
    pub direction: f64, // -1 or 1 (up or down growth)
}

pub struct Strata {
    pub commit: CommitData,
    pub y_pos: f64,    // Logical Y position
    pub offset_x: f64, // Tectonic shift
    pub color_idx: u8,
}

pub struct StrataManager {
    pub strata: Vec<Strata>,
    pub fissures: Vec<Fissure>,
    pub scroll_y: f64,
    pub width: f64,
}

impl StrataManager {
    pub fn new(width: f64) -> Self {
        Self {
            strata: Vec::new(),
            fissures: Vec::new(),
            scroll_y: 0.0,
            width,
        }
    }

    pub fn add_commit(&mut self, commit: CommitData) {
        let mut rng = rand::thread_rng();

        let y_pos = self.strata.len() as f64 * 5.0; // 5 units height per strata

        // Calculate shift based on previous strata + stress
        let prev_offset = self.strata.last().map(|s| s.offset_x).unwrap_or(0.0);
        let shift = if commit.stress_level > 5.0 {
            rng.gen_range(-2.0..2.0)
        } else {
            0.0
        };

        self.strata.push(Strata {
            commit: commit.clone(),
            y_pos,
            offset_x: prev_offset + shift,
            color_idx: (commit.timestamp % 6) as u8, // Simple color cycle
        });

        // Trigger Fissure if stress is high
        if commit.stress_level > 0.0 {
            // Fissure starts at random X within width
            let start_x = rng.gen_range(self.width * 0.2..self.width * 0.8);

            // Fissure grows DOWN into history (or up? let's say down/up doesn't matter, just visual)
            // Let's make it grow perpendicular to strata
            self.fissures.push(Fissure {
                points: vec![Vec2::new(start_x, y_pos + 2.5)],
                intensity: commit.stress_level,
                active: true,
                direction: if rng.gen_bool(0.5) { 1.0 } else { -1.0 },
            });
        }
    }

    pub fn update(&mut self) {
        let mut rng = rand::thread_rng();

        // Grow fissures
        for fissure in &mut self.fissures {
            if !fissure.active {
                continue;
            }

            if fissure.points.len() > (fissure.intensity * 5.0).max(5.0) as usize {
                fissure.active = false;
                continue;
            }

            let last = *fissure.points.last().unwrap();

            // Jagged growth
            let angle_base = if fissure.direction > 0.0 {
                std::f64::consts::PI / 2.0
            } else {
                -std::f64::consts::PI / 2.0
            };
            let angle_var = rng.gen_range(-0.5..0.5);
            let len = rng.gen_range(1.0..3.0);

            let next_x = last.x + (angle_base + angle_var).cos() * len;
            let next_y = last.y + (angle_base + angle_var).sin() * len;

            fissure.points.push(Vec2::new(next_x, next_y));
        }
    }
}
