# Chrontext ⏳

**"See the heartbeat of your code."**

Chrontext is a TUI tool that visualizes the age of your code. It uses `git blame` data to color-code each line, revealing the "strata" of development.

- **Hot (White/Yellow):** Freshly committed code. The ink is still wet.
- **Warm (Green/Cyan):** Recent changes.
- **Cold (Blue/Gray):** Ancient bedrock. Code that hasn't changed in eons.

## Usage

```bash
cargo run -p chrontext -- <file_path>
```

Example:
```bash
cargo run -p chrontext -- src/main.rs
```

## Features

- **Heatmap Visualization:** Instantly spot where the active development is happening.
- **Detail View:** Hover over any line to see the Commit Hash, Author, Date, and Message.
- **Git Integration:** Automatically discovers the repository from the file path.

## The Philosophy

Codebases are not static text files; they are living histories. Chrontext brings that history to the surface, allowing you to read the *time* dimension of your code as easily as the spatial dimension.
