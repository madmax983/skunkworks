use git_associates::GitModel;
use anyhow::Result;
use std::collections::VecDeque;

pub struct EruptionSource {
    commits: VecDeque<git_associates::model::Commit>,
    active_eruption: Option<EruptionEvent>,
}

pub struct EruptionEvent {
    pub hash: String,
    pub intensity: usize, // Number of particles to spawn
    pub remaining: usize, // Particles left to spawn
    pub color: ratatui::style::Color, // Color based on author/type?
}

impl EruptionSource {
    pub fn new(path: &str, limit: usize) -> Result<Self> {
        let model = GitModel::open(path)?;
        let history = model.history(limit)?;
        let mut commits = VecDeque::new();
        // Reverse history so we start from oldest? Or newest?
        // Geological strata: Oldest at bottom. So we should erupt oldest first.
        for c in history.into_iter().rev() {
            commits.push_back(c);
        }

        Ok(Self {
            commits,
            active_eruption: None,
        })
    }

    pub fn update(&mut self) -> Option<Vec<crate::physics::MagmaParticle>> {
        // If no active eruption, start one from the queue
        if self.active_eruption.is_none() {
            if let Some(commit) = self.commits.pop_front() {
                // Calculate intensity based on stats
                let stats = commit.stats.unwrap_or_default();
                let intensity = (stats.insertions + stats.deletions).min(500) as usize; // Cap at 500

                self.active_eruption = Some(EruptionEvent {
                    hash: commit.short_hash,
                    intensity,
                    remaining: intensity,
                    color: ratatui::style::Color::Red, // Default
                });
            } else {
                return None; // No more history
            }
        }

        // Spawn particles for the current eruption
        if let Some(event) = &mut self.active_eruption {
            let spawn_count = event.remaining.min(10); // Spawn 10 per frame
            event.remaining -= spawn_count;

            let mut new_particles = Vec::new();
            for _ in 0..spawn_count {
                // Spawn at top center with some random spread
                let x = 50.0 + (rand::random::<f32>() - 0.5) * 10.0;
                let y = 0.0;
                new_particles.push(crate::physics::MagmaParticle::new(x, y, event.hash.clone()));
            }

            if event.remaining == 0 {
                self.active_eruption = None;
            }

            Some(new_particles)
        } else {
            None
        }
    }
}
