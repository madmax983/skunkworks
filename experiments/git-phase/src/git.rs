use anyhow::{Context, Result};
use git2::Repository;
use std::path::Path;

pub struct GitSource {
    repo: Repository,
}

impl GitSource {
    pub fn open(path: &str) -> Result<Self> {
        let repo = Repository::discover(path).context("Failed to discover git repository")?;
        Ok(Self { repo })
    }

    pub fn read_file_at_revision(&self, path: &str, rev: &str) -> Result<Vec<u8>> {
        // Resolve revision to an object (usually a commit)
        let obj = self
            .repo
            .revparse_single(rev)
            .context(format!("Invalid revision '{}'", rev))?;

        // Peel to tree (if commit or tag)
        let tree = obj
            .peel_to_tree()
            .context(format!("'{}' does not point to a tree-ish object", rev))?;

        // Find the file in the tree
        match tree.get_path(Path::new(path)) {
            Ok(entry) => {
                let object = entry.to_object(&self.repo)?;
                if let Some(blob) = object.as_blob() {
                    Ok(blob.content().to_vec())
                } else {
                    // Not a blob (e.g. directory), return empty
                    Ok(Vec::new())
                }
            }
            Err(_) => {
                // File not found in this revision
                Ok(Vec::new())
            }
        }
    }
}
