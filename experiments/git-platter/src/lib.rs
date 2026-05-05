use git_associates::{GitModel, Commit};
use platter::Platter;

/// A hybrid entity that maps Git commit history onto a continuous 2D scalar field.
pub struct GitPlatter {
    pub model: GitModel,
    pub platter: Platter,
    pub width: usize,
    pub height: usize,
    pub decay_rate: f64,
    pub scale_factor: f64,
}

impl GitPlatter {
    /// Creates a new GitPlatter scanning the given repository path.
    pub fn new<P: AsRef<std::path::Path>>(
        repo_path: P,
        width: usize,
        height: usize,
    ) -> Result<Self, anyhow::Error> {
        let model = GitModel::open(repo_path)?;
        Ok(Self {
            model,
            platter: Platter::new(width, height),
            width,
            height,
            decay_rate: 0.95,
            scale_factor: 0.05, // How much heat an insertion adds
        })
    }

    /// Processes a single commit, applying its file changes as heat/cold to the platter.
    pub fn process_commit(&mut self, commit: &Commit) {
        if let Some(stats) = &commit.stats {
            let insertions = stats.insertions as f64;
            let deletions = stats.deletions as f64;

            // Map the commit author or hash to a coordinate (simple mapping for now)
            let author_len = commit.author.len() as f64;
            let x = ((author_len * 13.0) % self.width as f64) as usize;
            let y = ((commit.message.len() as f64 * 7.0) % self.height as f64) as usize;

            // Apply heat (insertions)
            if insertions > 0.0 {
                self.platter.accumulate(x, y, insertions * self.scale_factor);
            }

            // Apply cold (deletions)
            if deletions > 0.0 {
                self.platter.accumulate(x, y, -deletions * self.scale_factor);
            }
        }
    }

    /// Decays the platter field over time.
    pub fn decay(&mut self) {
        self.platter.decay(self.decay_rate);
    }
}
