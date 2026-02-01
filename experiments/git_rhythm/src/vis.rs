use crate::harvester::MusicalCommit;

pub struct VisualState {
    pub current_commit: Option<MusicalCommit>,
    pub history_len: usize,
    pub waveform_buffer: Vec<f32>,
    pub buffer_capacity: usize,
}

impl VisualState {
    pub fn new() -> Self {
        Self {
            current_commit: None,
            history_len: 0,
            waveform_buffer: Vec::new(),
            buffer_capacity: 100,
        }
    }

    pub fn update(&mut self, commit: MusicalCommit, sample: f32) {
        // Check if new commit
        let is_new = match &self.current_commit {
            Some(c) => c.hash != commit.hash,
            None => true,
        };

        if is_new {
            self.history_len += 1;
            self.current_commit = Some(commit);
        }

        self.waveform_buffer.push(sample);
        if self.waveform_buffer.len() > self.buffer_capacity {
            self.waveform_buffer.remove(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_visual_state() {
        let mut vis = VisualState::new();
        let commit = MusicalCommit {
            hash: "123".to_string(),
            author: "Me".to_string(),
            timestamp: 100,
            churn: 50,
        };

        vis.update(commit.clone(), 0.5);

        assert!(vis.current_commit.is_some());
        assert_eq!(vis.current_commit.unwrap().hash, "123");
        assert_eq!(vis.waveform_buffer.len(), 1);
        assert_eq!(vis.waveform_buffer[0], 0.5);
    }
}
