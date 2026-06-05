# Git Associates 🤝

A friendly, high-level wrapper around `git2` for analyzing repository history, diffs, and file changes.

This crate simplifies common git operations needed for visualization tools or analysis scripts,
abstracting away the complexities of `git2`'s low-level API.

## Features

- **History Traversal**: Easily fetch commit logs with metadata.
- **Diff Analysis**: Get detailed stats on insertions, deletions, and file modifications.
- **Working Directory**: Diff the current working directory against `HEAD`.
- **Hunk Extraction**: Parse diffs into structured hunks and lines.

## Example

```no_run
use git_associates::GitModel;

fn main() -> anyhow::Result<()> {
    // Open the repository in the current directory
    let model = GitModel::open(".")?;

    // Fetch the last 10 commits
    let history = model.history(10)?;

    for commit in history {
        println!("{} - {}", commit.short_hash, commit.message);
    }

    Ok(())
}
```
