use crate::git_loader::CommitData;
pub use quipu::{Cord, Knot};

#[derive(Debug, Clone)]
pub struct CommitCord {
    pub cord: Cord,
    pub data: CommitData,
    pub x_offset: f64, // For wind animation
}

impl CommitCord {
    pub fn from_commit(data: CommitData) -> Self {
        let mut clusters = Vec::new();

        // 1. Files Changed (Top Cluster)
        // Represented as Simple Knots
        let mut files_knots = Vec::new();
        let fc = data.files_changed.min(20); // Cap
        for _ in 0..fc {
            files_knots.push(Knot::Simple);
        }
        clusters.push(files_knots);

        // 2. Insertions (Middle Cluster)
        // Represented using Quipu number system (Tens + Units)

        let (tens, units) = Self::decompose(data.insertions);
        if tens > 0 {
            let mut k = Vec::new();
            for _ in 0..tens {
                k.push(Knot::Simple);
            }
            clusters.push(k);
        } else {
            clusters.push(Vec::new()); // Empty space for zero tens
        }

        let mut unit_knots = Vec::new();
        if units == 1 {
            unit_knots.push(Knot::FigureEight);
        } else if units > 1 {
            unit_knots.push(Knot::Long(units as u8));
        }
        clusters.push(unit_knots);

        // 3. Deletions (Bottom Cluster)
        let (tens, units) = Self::decompose(data.deletions);
        if tens > 0 {
            let mut k = Vec::new();
            for _ in 0..tens {
                k.push(Knot::Simple);
            }
            clusters.push(k);
        } else {
            clusters.push(Vec::new());
        }

        let mut unit_knots = Vec::new();
        if units == 1 {
            unit_knots.push(Knot::FigureEight);
        } else if units > 1 {
            unit_knots.push(Knot::Long(units as u8));
        }
        clusters.push(unit_knots);

        Self {
            cord: Cord { clusters },
            data,
            x_offset: 0.0,
        }
    }

    fn decompose(val: usize) -> (usize, usize) {
        let val = val.min(99);
        (val / 10, val % 10)
    }

    pub fn apply_wind(&mut self, time: f64, index: usize) {
        // Simple harmonic motion based on time and cord index
        let phase = index as f64 * 0.5;
        self.x_offset = (time + phase).sin() * 0.5;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_knot_conversion() {
        let data = CommitData {
            hash: "abc".into(),
            message: "msg".into(),
            author: "me".into(),
            time: Utc::now(),
            insertions: 12,
            deletions: 5,
            files_changed: 3,
        };
        let cord = CommitCord::from_commit(data);

        // 5 clusters: Files, InsTens, InsUnits, DelTens, DelUnits
        assert_eq!(cord.cord.clusters.len(), 5);

        // Files: 3 Simple
        assert_eq!(cord.cord.clusters[0].len(), 3);
        assert_eq!(cord.cord.clusters[0][0], Knot::Simple);

        // InsTens: 1 Simple (12 -> 1 ten)
        assert_eq!(cord.cord.clusters[1].len(), 1);
        assert_eq!(cord.cord.clusters[1][0], Knot::Simple);

        // InsUnits: 1 Long(2) (12 -> 2 units)
        assert_eq!(cord.cord.clusters[2].len(), 1);
        assert_eq!(cord.cord.clusters[2][0], Knot::Long(2));

        // DelTens: 0 (5 -> 0 tens)
        assert_eq!(cord.cord.clusters[3].len(), 0);

        // DelUnits: 1 Long(5) (5 -> 5 units)
        assert_eq!(cord.cord.clusters[4].len(), 1);
        assert_eq!(cord.cord.clusters[4][0], Knot::Long(5));
    }
}
