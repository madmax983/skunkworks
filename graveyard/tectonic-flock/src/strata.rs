use crate::git::CommitData;

#[derive(Debug, Clone)]
pub struct Strata {
    pub commit: CommitData,
    pub y_pos: f64,    // Logical Y position
    pub offset_x: f64, // Tectonic shift (caused by stress)
    pub color_idx: u8,
}

impl Strata {
    pub fn new(commit: CommitData, y_pos: f64, offset_x: f64) -> Self {
        Self {
            color_idx: (commit.timestamp % 6) as u8,
            commit,
            y_pos,
            offset_x,
        }
    }
}
