# Agent Guidelines for Skunkworks

This repository is a playground for autonomous coding agents (Jules, Claude, etc.) to create TUI experiments and other projects.

## Adding a New Experiment

1. **Create your crate** in `experiments/your-experiment-name/`

2. **Add to workspace members** in root `Cargo.toml` - keep alphabetical order:
   ```toml
   members = [
       "crates/tui-shared",
       "experiments/automata-warfare",
       "experiments/cellular-beats",
       # ... add yours in alphabetical position ...
       "experiments/your-experiment-name",
       # ...
   ]
   ```

3. **Use workspace dependencies** in your `Cargo.toml`:
   ```toml
   [package]
   name = "your-experiment-name"
   version = "0.1.0"
   edition = "2024"

   [dependencies]
   anyhow.workspace = true
   crossterm.workspace = true
   rand.workspace = true
   ratatui.workspace = true
   # Optional: shared TUI utilities
   tui-shared.workspace = true
   ```

## Available Workspace Dependencies

| Dependency | Version | Notes |
|------------|---------|-------|
| `anyhow` | 1.0 | Error handling for binaries |
| `crossterm` | 0.28 | Terminal manipulation |
| `rand` | 0.8 | Random number generation |
| `ratatui` | 0.29 | TUI framework |
| `tui-shared` | local | Shared TUI utilities |

## Important Notes

### Cargo.lock is not tracked
Don't commit `Cargo.lock` - it's in `.gitignore` to avoid merge conflicts.

### Rust 2024 Edition
If using `edition = "2024"`, the `gen` keyword is reserved. Use `r#gen` to call rand methods:
```rust
let mut rng = rand::thread_rng();
let value: f64 = rng.r#gen();  // Note the r# prefix
let range = rng.gen_range(0..10);  // gen_range is fine
```

### Adding New Workspace Dependencies
If you need a dependency used by multiple experiments, consider adding it to `[workspace.dependencies]` in the root `Cargo.toml` rather than pinning versions in each experiment.

## Shared Code

The `crates/tui-shared` crate contains common TUI utilities. Feel free to add reusable components there.
